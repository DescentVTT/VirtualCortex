---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0024
---

# ADR-0028: Edge behaviour under audit — a gate is closed until it is evaluated, a reflex is released with its override, every packed or shifted input is read within its bound, and the loader refuses what the writer would never produce

## Context and Problem Statement

A review of the eight crates changed by ADR-0026 and ADR-0027 found six defects of the same few classes (a shift that can reach the integer width, a negation of the most negative value, an off-by-one bound, a mutator that reports success when nothing changed, a state nothing can clear, an undocumented no-op). The same classes were then looked for in the other twenty-four state crates and the runtime, each candidate reproduced in a scratch crate before it was counted. Twelve were reproduced:

1. `stp_decay_factor_q16(elapsed, tau_shift)` shifted a 64-bit one by `tau_shift`: a shift of 64 or more panics in a debug build, and any shift above 16 silently yielded 1.0 forever, since a base of $1 - 2^{-17}$ rounds to 1.0 in Q16.16.
2. `EthicalEvaluationGate::is_permitted` was true for a default record: the flag's zero meant "permitted", so a gate nobody had evaluated was open.
3. `SalienceNodeState::evaluate_threat` set the freeze reflex, the valence and the replay tag when a threat appeared and released only the override when it passed; no method cleared the other three, and the crate's own test pinned that "rather than endorsing it".
4. `OP_REM` on `i128::MIN` and $-1$ reported `ERR_OVERFLOW`, though the remainder is exactly 0 and fits; the flag is defined as "does not fit 128 bits".
5. `CerebellarMicrozone::step_forward_model` read the head, the delay and the fill count out of the packed `delay_ctl` without bounds: a head of 7 indexed outside the seven-slot ring and a delay of 8 with eight entries filled underflowed. Reachable through the public field or an image, since `set_plant_delay` clamps.
6. `literal_of_term(term, negated)` added one to any index: at $2^{31} - 1$ the sum landed on the sign bit (the literal read as negated and as no literal), at `u32::MAX` it overflowed.
7. `ThalamicRelayNode::relay` with a gating mode the setter refuses, written through the public field or an image, relayed nothing, which the documentation did not say.
8. `mailbox_push` stored the payload into the caller's node before it read the head, so a push refused for a corrupt head was not "refused, with nothing changed".
9. `DendriticSuperNeuron::is_at_rest_image` ignored the reserved bytes at `[52..54)`, which the documentation says must be zero.
10. `Image::decode` accepted a delta whose slot its block does not have, a delta with a non-zero padding byte, a synapse block with non-zero reserved bytes, and (through 9) a unit with non-zero reserved bytes, all under a valid section checksum.
11. `WriteAheadLog::append` for a unit outside the log wrote the entry to the file, then panicked on the index.
12. `Image::decode` sized the directory allocation from a sealed header's `section_count` before checking it against the file: a 64-byte file whose header claimed $2^{32} - 1$ entries aborted the process on a 275 GB allocation instead of returning `Truncated`.

A thirteenth observation was a design gap rather than a defect: `Executor::is_quiescent` ignored the injector ring, so an image written between an injection and the tick that drains it would silently lose the pair.

Which of these are rule changes that need a decision, and what is the rule?

## Decision Drivers

- Whitepaper §8.1: every state update saturates; no input may panic a record method.
- Whitepaper §8.7: an image reader fails closed; the writer writes only at a quiescent point; reserved bytes are zero at rest.
- Whitepaper §2.3 and ADR-0016: a name is descriptive and a claim needs a rule and a test; a documented "refused, with nothing changed" must be true.
- Principle 5 of `CLAUDE.md`: a structural boundary beats a reviewed one; each fix carries the test that failed before it.
- Principle 7: say what was not done.

## Considered Options

1. **Fix each defect locally, document the refined rule where it lives, and record the round here**, with one rule change that touches a record's encoding (the gate's decision flag) and one that changes observable dynamics (the salience release).
2. Fix only the panics and the loader gaps; leave the gate's default and the salience latch as they are, since both were documented.
3. Add a `debug_assert!` at each site and leave release behaviour unchanged.

## Decision Outcome

Option 1.

- **The gate is closed until it is evaluated.** `veto_decision_flag` is `DECISION_UNEVALUATED` (0), `DECISION_VETOED` (1) or `DECISION_PERMITTED` (2); `is_permitted` is true only for `DECISION_PERMITTED` with `VETO_NONE`. Zero means "no verdict yet", in the same spirit as the index + 1 encodings (ADR-0017, ADR-0022): a record of zeros, whether default or read from an older image, holds no permission. Image format version **9**: a version-8 image's zero flag reads as not yet evaluated, which fails closed; nothing else moved.
- **The reflex is released with the override.** When neither the shock nor the conditioned weight crosses its threshold, `evaluate_threat` clears the valence, the freeze flag, the override and the replay tag together; what persists is `fear_conditioning_w`, which is what was learned. The previous test that pinned the latch is replaced by one that pins the release.
- **Every packed or shifted input is read within its bound.** `stp_decay_factor_q16` reads a time constant above $2^{16}$ ticks as $2^{16}$, the longest a Q16.16 base resolves, and states it. `CerebellarMicrozone` reads the head modulo the ring, the delay and the fill count clamped to seven, so a word from the field or an image cannot index outside the ring; `set_plant_delay`'s documentation now says `0..=MAX_PLANT_DELAY`, which is what it did. `literal_of_term` returns `Option<u32>`, `None` at or above $2^{31} - 1$ (the last index whose `+ 1` stays below the sign bit is $2^{31} - 2$). `OP_REM` wraps after the zero check, since the one wrapping case has an exact result.
- **A refusal changes nothing.** `mailbox_push` reads the head before it stores the payload; a head that turns corrupt during the retry leaves the payload in the caller's own node, and the documentation says so. `WriteAheadLog::append` refuses a unit outside the log with `InvalidInput` before it writes.
- **The loader refuses what the writer would never produce.** `is_at_rest_image` requires the reserved bytes to be zero; `Image::decode` refuses a delta slot at or above four (`DanglingIndex`, slot 5), a non-zero delta padding byte or synapse reserved byte (`ImageError::ReservedNotZero { section, index }`), and a directory longer than the bytes after the header (`Truncated`) before the count sizes anything.
- **An unknown gating mode relays nothing**, like a closed gate, and the documentation says so; the setter still refuses it.
- **Quiescence includes the injector ring.** `Injector::is_empty` (exact while no producer pushes, which is the case between ticks) is part of `Executor::is_quiescent`, so `Image::encode` refuses `NotQuiescent` while a pair injected before the tick sits in no record.

Each item carries the test that reproduced it. The public surface changed in two places: `literal_of_term` returns `Option<u32>` (its four callers are tests) and `ImageError` gains `ReservedNotZero`.

### Consequences

- Good: a record method no longer panics on any value of its inputs, including values only an image or a public field can supply; a gate of zeros permits nothing; an image is refused for every byte the writer would not have produced; an injection cannot be lost to an image.
- Good: the defect classes are named here, so the next review looks for them first.
- Bad: a version-8 image must be re-written to be read as 9. Not a cost today: no image outside the test suite exists.
- Bad: `ThalamicRelayNode` and `CerebellarMicrozone` still expose the fields that made 5 and 7 reachable; ADR-0001 chose public fields for records, and this decision does not reopen that.
- Not done: no lint enforces "every shift amount is bounded" or "every mutator that returns `bool` changes nothing on `false`"; both remain review items (the classes are listed above so a reviewer can grep for `>>` on a caller-supplied shift and for `return false` after a store). `clippy::arithmetic_side_effects`, which would catch the plain `+ 1` of item 6, fires 229 times in the tree today and is a separate decision.

## Alternatives considered and why rejected

- Option 2 keeps a documented fail-open gate. Whitepaper §5.2.28 says the gate fails closed; a default record that reads as permitted is the opposite, whatever the comment beside the field said.
- Option 3 leaves release builds with the same out-of-bounds reads; a `debug_assert!` is a test that runs only when nobody is looking.
- Making `evaluate_threat` decay the valence rather than clear it: there is no time base in the record, and a decay without a time constant is a number chosen for no reason. The release is immediate; a slower release is a rule for a later brief with a stated constant.

## Confirmation

- `cortex-core`: `a_time_constant_past_the_resolution_is_the_longest_one_and_never_a_panic`, `a_corrupt_head_refuses_a_push_before_the_payload_is_stored`, `a_reserved_byte_that_is_not_zero_is_not_at_rest`.
- `cortex-ethics`: `a_gate_that_was_never_evaluated_is_closed_whatever_its_reason_byte_says`; the default-gate test asserts the closed default.
- `cortex-salience`: `the_reflex_is_released_with_the_override_and_the_conditioned_weight_persists`.
- `cortex-arithmetic`: `overflow_is_flagged_and_the_result_is_zero` asserts `OP_REM` of `i128::MIN` by $-1$ is `(true, 0, 0)`.
- `cortex-cerebellum`: `a_control_word_outside_its_bounds_is_read_within_them`.
- `cortex-reasoning`: `a_literal_the_encoding_cannot_hold_is_refused_and_the_last_one_round_trips`.
- `cortex-thalamus`: `an_unknown_mode_in_the_field_relays_nothing_like_a_closed_gate`.
- `cortex-runtime`: `a_record_that_is_not_at_rest_in_its_reserved_bytes_or_its_slot_is_refused_at_load`, `a_sealed_header_claiming_more_directory_than_the_file_holds_is_truncated_not_an_allocation`, `the_log_refuses_a_unit_outside_it_before_writing`, `a_pair_still_in_the_injector_ring_is_not_a_quiescent_point`; the injector's own test covers `is_empty` across a wrap.
- Whitepaper §5.2.1, §5.2.6, §5.2.7, §5.2.19, §5.2.28, §5.2.30, §5.2.31, §6.7, §8.6, §8.7 state the refined rules; §5.2.2 records format version 9; §11 records the round as finding F-24, resolved.
