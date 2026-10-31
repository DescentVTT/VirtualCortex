# Research reports

What each hypothesis asked of the engine and what its rounds read, one report a hypothesis ([ADR-0197](../adr/0197-one-place-for-each-kind-of-fact.md)).
A report holds the hypothesis's text, its stopping rule and its readings.
The decisions taken on them are the ADRs'.
A report is history to `spec-graph` once its round has merged.

## The hypotheses

`scripts/adr-index.mjs` writes this table from each report's front matter (`hypothesis:`, `question:`, `verdict:`) and from the ADRs whose `hypothesis:` names it.
`npm run spec:index` fails when the table is not what the script writes.
The question is the hypothesis's own name, as §11.1 wrote it.

<!-- adr-index: begin hypotheses -->
| H-n | Question | Verdict | ADRs | Report |
| :--- | :--- | :--- | :--- | :--- |
| H-1 | condensation ratio | open |  | [H-1](H-1.md) |
| H-2 | predictive-coding traffic reduction | open |  | [H-2](H-2.md) |
| H-3 | experience | open | [ADR-0020](../adr/0020-computational-phenomenology-and-synthetic-qualia.md) | [H-3](H-3.md) |
| H-4 | blends as metaphor | open | [ADR-0021](../adr/0021-native-cognitive-language-and-conceptual-blending.md) | [H-4](H-4.md) |
| H-5 | re-representation as synthesis | open | [ADR-0026](../adr/0026-social-acumen-and-re-representation.md) | [H-5](H-5.md) |
| H-6 | mirth as humor; the shaped voice as tone | open | [ADR-0027](../adr/0027-vocal-synthesis-and-computational-humor.md) | [H-6](H-6.md) |
| H-7 | the block arena as the binding constraint | open | [ADR-0034](../adr/0034-synaptic-density.md) | [H-7](H-7.md) |
| H-8 | the estimator at the reference scale | open | [ADR-0036](../adr/0036-criticality-control.md), [ADR-0044](../adr/0044-reference-network.md) | [H-8](H-8.md) |
| H-9 | replay at the reference scale | open | [ADR-0038](../adr/0038-episodic-ledger-and-replay.md), [ADR-0044](../adr/0044-reference-network.md), [ADR-0048](../adr/0048-episodes-tagged-from-the-train.md) | [H-9](H-9.md) |
| H-10 | the greedy reducer's coverage | open | [ADR-0040](../adr/0040-categorial-reduction.md) | [H-10](H-10.md) |
| H-11 | compression progress as a reward; first-fit matching | open | [ADR-0041](../adr/0041-induction-on-the-term-arena.md), [ADR-0043](../adr/0043-discovery-path.md), [ADR-0045](../adr/0045-clause-search.md), [ADR-0048](../adr/0048-episodes-tagged-from-the-train.md) | [H-11](H-11.md) |
| H-12 | a reward changes a behaviour the engine reads back | no | [ADR-0059](../adr/0059-a-task-a-readout-and-a-reward.md) to [ADR-0077](../adr/0077-the-background-side.md), 11 ADRs | [H-12](H-12.md) |
| H-13 | a reward raises the response it is delivered to | yes | [ADR-0078](../adr/0078-the-rewards-direction.md), [ADR-0079](../adr/0079-the-rewards-direction-measured.md) | [H-13](H-13.md) |
| H-14 | a reward the engine earns moves its selection | yes | [ADR-0080](../adr/0080-the-reinforced-form.md), [ADR-0081](../adr/0081-the-reinforced-form-measured.md) | [H-14](H-14.md) |
| H-15 | a reward the engine earns moves its selection when every synapse consolidates | no | [ADR-0082](../adr/0082-plasticity-everywhere.md), [ADR-0083](../adr/0083-plasticity-everywhere-measured.md) | [H-15](H-15.md) |
| H-16 | the reward's gate holds with inhibition off it | yes | [ADR-0085](../adr/0085-inhibition-off-the-reward-gate.md), [ADR-0087](../adr/0087-inhibition-off-the-reward-gate-measured.md) | [H-16](H-16.md) |
| H-17 | the engine revises a selection it has learned | no | [ADR-0089](../adr/0089-the-assignment-reversed.md), [ADR-0090](../adr/0090-the-old-answer-held-from-the-trial-after-the-flip.md), [ADR-0091](../adr/0091-the-assignment-reversed-measured.md) | [H-17](H-17.md) |
| H-18 | a punished pair loses what it gained, and the engine revises | yes | [ADR-0093](../adr/0093-the-punished-pair-consolidates-against-its-trace.md), [ADR-0095](../adr/0095-an-image-pin-moves-with-its-format.md), [ADR-0096](../adr/0096-the-punished-pair-measured.md) | [H-18](H-18.md) |
| H-19 | with the reward less its expectation, the engine learns, revises and settles | no | [ADR-0106](../adr/0106-the-reward-prediction-error.md), [ADR-0107](../adr/0107-the-critic-built.md), [ADR-0108](../adr/0108-the-reward-prediction-error-measured.md) | [H-19](H-19.md) |
| H-20 | the engine learns across a schedule of reversals, its couplings bounded | yes | [ADR-0109](../adr/0109-a-schedule-of-reversals.md), [ADR-0110](../adr/0110-a-schedule-of-reversals-measured.md) | [H-20](H-20.md) |
| H-21 | with the inhibitory rule's target at the settled network's own rate, H-20's schedule keeps its inhibition and still learns | yes | [ADR-0128](../adr/0128-a-target-the-network-fires-at.md), [ADR-0129](../adr/0129-a-target-the-network-fires-at-measured.md) | [H-21](H-21.md) |
| H-22 | on H-21's configuration, a critic of the engine's own stands in for the task's | no | [ADR-0130](../adr/0130-a-critic-of-the-engines-own.md), [ADR-0132](../adr/0132-a-critic-of-the-engines-own-measured.md) | [H-22](H-22.md) |
| H-23 | on H-22's configuration, the critic's window | yes | [ADR-0133](../adr/0133-the-critics-window.md), [ADR-0135](../adr/0135-the-critics-window-measured.md) | [H-23](H-23.md) |
| H-24 | on H-23's configuration, the reward delivered to every synapse | no | [ADR-0136](../adr/0136-the-reward-unaddressed.md), [ADR-0137](../adr/0137-the-reward-unaddressed-measured.md) | [H-24](H-24.md) |
| H-25 | on H-23's configuration, the address drawn by the engine | yes | [ADR-0138](../adr/0138-the-address-drawn.md), [ADR-0140](../adr/0140-the-address-drawn-measured.md) | [H-25](H-25.md) |
| H-26 | on H-25's runs, what releasing the address's targets would cost | no | [ADR-0141](../adr/0141-the-readouts-own-competition.md), [ADR-0142](../adr/0142-the-readouts-own-competition-measured.md) | [H-26](H-26.md) |
| H-27 | on H-25's configuration, the address's targets released and the gate's output delivered | no | [ADR-0143](../adr/0143-the-gates-output-delivered.md), [ADR-0145](../adr/0145-the-gates-output-measured.md) | [H-27](H-27.md) |
| H-28 | on H-25's configuration, a reward right seven times in eight | yes | [ADR-0147](../adr/0147-a-reward-right-seven-times-in-eight.md), [ADR-0149](../adr/0149-a-reward-right-seven-times-in-eight-measured.md) | [H-28](H-28.md) |
| H-29 | on H-25's configuration, a choice among three answers | no | [ADR-0151](../adr/0151-three-answers.md), [ADR-0153](../adr/0153-three-answers-measured.md) | [H-29](H-29.md) |
| H-30 | on H-29's readouts, a punishment the value does not soften | no | [ADR-0154](../adr/0154-a-punishment-the-value-does-not-soften.md), [ADR-0156](../adr/0156-a-punishment-the-value-does-not-soften-measured.md) | [H-30](H-30.md) |
| H-31 | on H-29's configuration, the span a choice among three is given | no | [ADR-0157](../adr/0157-the-span-a-choice-among-three-is-given.md), [ADR-0161](../adr/0161-the-span-a-choice-among-three-is-given-measured.md) | [H-31](H-31.md) |
| H-32 | on H-29's readouts, an exploration the value gates | no | [ADR-0162](../adr/0162-an-exploration-the-value-gates.md), [ADR-0164](../adr/0164-an-exploration-the-value-gates-measured.md) | [H-32](H-32.md) |
| H-33 | H-25's configuration on another network of the same size | yes | [ADR-0165](../adr/0165-another-network-of-the-same-size.md), [ADR-0167](../adr/0167-another-network-of-the-same-size-measured.md) | [H-33](H-33.md) |
| H-34 | H-25's configuration at twice the size | yes | [ADR-0168](../adr/0168-twice-the-size.md), [ADR-0169](../adr/0169-twice-the-size-measured.md) | [H-34](H-34.md) |
| H-35 | H-25's configuration under a reward right three times in four | no | [ADR-0170](../adr/0170-a-reward-right-three-times-in-four.md), [ADR-0172](../adr/0172-a-reward-right-three-times-in-four-measured.md) | [H-35](H-35.md) |
| H-36 | a reward right three times in four on the other networks of the size | no | [ADR-0173](../adr/0173-three-in-four-on-the-other-networks.md), [ADR-0174](../adr/0174-three-in-four-on-the-other-networks-measured.md) | [H-36](H-36.md) |
| H-37 | H-25's configuration with the criticality controller on, at a step that holds | no | [ADR-0175](../adr/0175-the-controllers-step.md), [ADR-0177](../adr/0177-the-controllers-step-measured.md) | [H-37](H-37.md) |
| H-38 | H-25's configuration with the controller holding the settled network's rate | yes | [ADR-0178](../adr/0178-a-rate-the-controller-holds.md), [ADR-0181](../adr/0181-a-rate-the-controller-holds-measured.md) | [H-38](H-38.md) |
| H-39 | the rate's rule at a step of a sixteenth, on three networks | yes | [ADR-0182](../adr/0182-a-sixteenth-on-three-networks.md), [ADR-0183](../adr/0183-a-sixteenth-on-three-networks-measured.md) | [H-39](H-39.md) |
| H-40 | a target the engine reads, at a sixteenth, on three networks | yes | [ADR-0188](../adr/0188-a-target-the-engine-reads.md), [ADR-0190](../adr/0190-a-target-the-engine-reads-measured.md) | [H-40](H-40.md) |
| H-41 | the inhibitory target by the same read, on three networks | yes | [ADR-0192](../adr/0192-the-inhibitory-target-by-the-same-read.md), [ADR-0195](../adr/0195-the-inhibitory-target-by-the-same-read-measured.md) | [H-41](H-41.md) |
| H-42 | a step the engine makes from its own loop, on three networks | yes | [ADR-0200](../adr/0200-a-step-the-engine-makes-from-its-own-loop.md), [ADR-0203](../adr/0203-a-step-the-engine-makes-from-its-own-loop-protocol.md), [ADR-0204](../adr/0204-a-step-the-engine-makes-from-its-own-loop-measured.md) | [H-42](H-42.md) |
| H-43 | a reward past the trial, its address kept, on three networks | yes | [ADR-0207](../adr/0207-a-reward-past-the-trial-its-address-kept.md), [ADR-0209](../adr/0209-a-reward-past-the-trial-its-address-kept-its-protocol.md), [ADR-0210](../adr/0210-a-reward-past-the-trial-its-address-kept-measured.md) | [H-43](H-43.md) |
| H-44 | a reward right three times in four, on the default, beside a control at step zero | yes | [ADR-0212](../adr/0212-three-in-four-on-the-default.md), [ADR-0213](../adr/0213-three-in-four-on-the-default-its-protocol.md), [ADR-0214](../adr/0214-three-in-four-on-the-default-measured.md) | [H-44](H-44.md) |
| H-45 | a delay the engine is not told, read from its kept addresses, on three networks | yes | [ADR-0216](../adr/0216-a-delay-the-engine-reads.md), [ADR-0218](../adr/0218-a-delay-the-engine-reads-its-protocol.md), [ADR-0219](../adr/0219-a-delay-the-engine-reads-measured.md) | [H-45](H-45.md) |
| H-46 | the couplings bounded over sixteen schedules by a zero sum at the stimulus and a quarter's depression exponent, on three networks | yes | [ADR-0226](../adr/0226-a-zero-sum-at-the-stimulus.md), [ADR-0229](../adr/0229-a-zero-sum-at-the-stimulus-its-protocol.md), [ADR-0230](../adr/0230-a-zero-sum-at-the-stimulus-measured.md) | [H-46](H-46.md) |
<!-- adr-index: end hypotheses -->

## Lines with no number

- [A rule held by the network](a-rule-held-by-the-network.md): six rounds from [ADR-0111](../adr/0111-a-rule-held-by-the-network.md) to [ADR-0126](../adr/0126-the-gate-raised-measured.md), after which the line pauses.
- [The operating regime the network is built to hold](the-operating-regime.md): an open question, with the readings of H-13 to H-18 added to it.

## How a report is written

From brief 078, a round that reads a hypothesis writes its report in the same pull request as its measured ADR ([briefs/README.md](../../briefs/README.md#evidencing-a-round)).
- **The front matter**: `hypothesis:`, `question:` of at most 160 characters, and `verdict:`, one of `yes`, `no` or `open`.
- **The body**: the hypothesis as it was written before any run, its stopping rule, the readings' tables, and the dispatches that are their evidence.
- **The directives**: each under the sentence it backs, as anywhere else.

The reports H-1 to H-41 and the two above were moved here verbatim from whitepaper 4.134.0 by [ADR-0199](../adr/0199-one-place-for-each-kind-of-fact-built.md).
