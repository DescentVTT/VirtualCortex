---
status: accepted
date: 2026-09-12
decision-makers: VirtualCortex maintainers
depends-on: ADR-0036
---

# ADR-0037: Sleep as a state machine on the window cadence — a two-process sleep pressure against a circadian threshold, three stages under an ultradian budget, a wake as an input; the homeostasis record's sleep fields; format version 13

## Context and Problem Statement

`HomeostaticDrivePool` carried a sleep gate: `update_circadian_tick(dt_ticks)` advanced the sixteen-bit circadian phase by the low bits of its argument and set `sleep_mode_active` to 1 when `sensory_fatigue` exceeded `0x8000_0000` (32 768.0 in Q16.16) or the phase exceeded `0xC000`. Nothing in the runtime called it, nothing wrote the fatigue, and no value a Q16.16 quantity in $[0, 1]$ can take reaches the threshold: the gate was a rule nothing composed on an input nothing produced (finding F-29, with ADR-0038's countdown). Whitepaper R-6 (§6.6) described sleep as Specified: replay, rescaling and scrubbing while the flag is set. The proposal of brief 019 asked for the flag to become an ultradian state machine, awake to slow-wave sleep to REM and back, driven by "fatigue / circadian", scheduled on a cadence, with an emergency wake from a sensory burst, and for slow-wave sleep to downscale weights and REM to recalibrate affect.

What is the machine, what drives it, on what schedule does it step, what is an input to it, and what of the proposal's sleep-phase work is the executor's?

## Decision Drivers

- ADR-0035: a rule slower than the tick runs on a cadence, between ticks on the coordinator; the window cadence ($2^{17}$ ticks, 1.31 s) already exists, and sixteen bits of windows are $2^{33}$ ticks, 23.86 h at the fine tick: a phase advanced once per window is a day to within one per cent.
- §8.3 and ADR-0036's precedent: anything that changes what a run does is in the image, always written and required, or is an input between ticks; the reference dynamics are unchanged at the default.
- ADR-0016: one owner per quantity. The sleep pressure and the stage are the homeostasis record's; what replay does is `cortex-hippocampus`'s (ADR-0038); affect is `cortex-affect`'s and composed by nothing.
- "Latest ≠ Newest": the two-process model of sleep regulation (Borbély 1982; Daan, Beersma and Borbély 1984) is four decades old, with a homeostatic process S that rises during wake and falls during sleep and a circadian process C that sets its thresholds; its failure modes (a pressure that saturates, thresholds that cross) are known and bounded here by construction.
- §8.1: a relaxation toward a target moves by at least one LSB, so 1.0 and 0 are reached exactly.
- ADR-0010 and principle 3: the thresholds and budgets are numbers chosen to make a cycle observable in a test; what they should be at the reference scale is a Target.

## Considered Options

1. Keep the flag and the gate; compose `update_circadian_tick` every tick with `dt = 1` (a 0.65 s "day") and have the caller write the fatigue.
2. **A state machine stepped once per window on the window cadence: a sleep pressure $S$ in $[0, 1]$ (process S) that rises while awake by $(1 - S) \gg k$ and falls while asleep by $S \gg \max(k - 2, 0)$, at least one LSB each; a circadian phase advanced by one per window (process C: sixteen bits are a day); onset and wake thresholds lower in the night quarter; three stages, awake, slow-wave and REM, the two sleep stages alternating under a budget of windows, the wake test first; a wake as an input between ticks; the shift $k$ in the image, 0 by default (off).**
3. A use-dependent pressure: $S$ integrating the bin's activity (local sleep, Krueger), with the same thresholds.
4. A completion-driven alternation: slow-wave to REM when the ledger has nothing left to replay, REM to slow-wave when nothing is left to depotentiate.

## Decision Outcome

Option 2.

- **The record** (`cortex-homeostasis`, format version 13). `[4..8)` is `sleep_pressure_q16: u32` ($S$; `sensory_fatigue` renamed, since the rule now defines it), `[18)` `sleep_stage: u8` (`STAGE_AWAKE` 0, `STAGE_SWS` 1, `STAGE_REM` 2; `sleep_mode_active` renamed), `[22)` `sleep_shift: u8` ($k$, at most `SLEEP_SHIFT_MAX` 15) and `[23)` `stage_windows: u8` (windows in the current stage, saturating); the two reserved bytes are gone. `new()` is awake with the pressure and the shift at zero. `update_circadian_tick` is removed.
- **The step** (`step_sleep() -> u8`, once per window). The circadian phase advances by one, wrapping. With $k = 0$ nothing else moves: the pressure and the stage stay where the image put them, which is the default and the reference dynamics. With $k > 0$: awake, $S \leftarrow \min(1, S + \max((1 - S) \gg k, 1))$; asleep, $S \leftarrow S - \max(S \gg \max(k - 2, 0), 1)$, saturating at zero: the fall is two shifts faster than the rise (Borbély's ratio is about 4.5), floored at a whole step. Then the stage: awake to slow-wave when $S \ge$ the onset threshold (`SLEEP_ONSET_DAY_Q16` 0.875, or `SLEEP_ONSET_NIGHT_Q16` 0.5 when the phase is at or above `NIGHT_PHASE`, `0xC000`, the last quarter); either sleep stage to awake when $S \le$ the wake threshold (`WAKE_DAY_Q16` 0.375, `WAKE_NIGHT_Q16` 0.125); slow-wave to REM after `SWS_WINDOWS` (4) windows in the stage and REM to slow-wave after `REM_WINDOWS` (2), the wake test first; a stage the constants do not name wakes. At $k = 15$ the rise's time constant is $2^{15}$ windows (11.9 h) and the fall's $2^{13}$ (3.0 h), near the human values; at $k = 5$ a cycle fits a test: awake for 66 windows from zero, four of slow-wave sleep, two of REM, one more of slow-wave, awake at 0.375.
- **The wake** (`wake() -> bool`, an input between ticks): whatever the stage, awake, the stage's windows at zero, the pressure kept, as an alarm leaves a sleeper; false when already awake. A sleeper woken with the pressure still at or above the onset is awake for one window and asleep again at the next step: the alarm does not lower the pressure. A stage the constants do not name (reachable through the public field; the loader refuses it) reads as asleep to `is_asleep`, is left alone at $k = 0$, wakes at the next step otherwise, and replays nothing. It is the executor's `wake()`, the form the proposal's "emergency wake from a sensory burst" can take in a tree where the injector is the only input to the loop: the salience rule that would raise it (`evaluate_threat`) is composed by nothing, and a caller that runs it calls `wake`.
- **Well-formedness.** `is_well_formed` gains: the stage at most 2, the pressure at most 1.0, the shift at most 15, a slow-wave or REM record's windows below its budget (the step moves the stage in the window that reaches it). The loader refuses the rest (`MalformedHomeostasis`) and a shift above its bound as it refuses the control step (`Config(SleepShiftOutOfRange)`).
- **The composition** (`cortex-runtime`). `Config::sleep_shift` (default 0; refused above 15). On the window cadence, after the gain's regulation, the coordinator calls `step_sleep`; `Executor::{sleep_stage, wake}` between ticks. The image's shift, stage and pressure outrank the configuration's. The clock sweep is not tied to a stage: it is the caller's call between ticks, and `sleep_stage()` is what a caller gates on.
- **What is not adopted.** *Synaptic downscaling in slow-wave sleep as a weight sweep* (Tononi and Cirelli's synaptic homeostasis): the multiplicative actuator exists as the gain of ADR-0036, whose loop runs asleep as awake; a uniform downscale of every weight differs from a gain only in the pair rule's relative step; nothing in the tree shows weight saturation under a rule whose depression exceeds its potentiation ($A_- / A_+ = 1.05$); and an $O(S)$ sweep over the block arena is what ADR-0036 removed. *Affect recalibration in the executor*: `InteroceptiveState` is composed by nothing, its recovery is its caller's argument (`integrate`), and REM's affective work is the tag's decay in ADR-0038. *Sensory gating during sleep*: the injector is the trace and the trace defines the run (§8.3); the thalamic gate is `cortex-thalamus`'s and Specified. *Option 3*: Borbély's form is time-based and canonical; the bin's activity is the input a use-dependent variant would take, and it is Specified as that. *Option 4*: a completion signal makes the stage's length a function of the ledger (an empty ledger would flip stages every window); the alternation is intrinsic (the REM-on/REM-off oscillator), and a budget of windows says so.
- **What is not claimed.** The thresholds (0.875, 0.5, 0.375, 0.125), the budgets (4 and 2 windows: 5.2 s and 2.6 s, against a 90-minute human cycle) and the shift's ratio are placeholders that make a cycle observable in a test; their values at the reference scale are Targets. No claim that the engine "rests", "recovers" or "consolidates" beyond the rules stated here and in ADR-0038.

### Consequences

- Good: sleep is a rule with a schedule, an input and bounds, persisted in the image; at the default it changes nothing, and the determinism pin holds.
- Good: the phase is a day at one step per window, with no new cadence.
- Good: the two fields with no composer and no writer (F-29) are a rule.
- Bad: format version 13 makes version-12 images unreadable, as every bump does.
- Bad: the record has no reserved bytes left; the next field in it is a new decision on its shape.
- Bad: the cycle in the tree is seconds long; the values that make it hours are untested at that length.

## Alternatives considered and why rejected

- **Option 1** keeps a threshold no value reaches and a day of 0.65 s, and leaves the fatigue to a caller with no rule.
- **A circadian cosine** for the thresholds: a table or a polynomial for what a two-level threshold by quarter gives in one comparison; the quarter is the existing rule's.
- **Separate rise and fall shifts** in one byte (two nibbles): two parameters for one ratio the literature fixes; a byte that is zero would mean both "off" and "instant".
- **The stage in the amendment registry** (ADR-0031): it changes what the engine does.

## Confirmation

`cortex-homeostasis`: the rise from zero at $k = 3$ pinned window by window to the onset; the onset at exactly the threshold and one LSB below it; the night quarter's thresholds, its first window and the wrap at dawn; the fall at $k = 5$ pinned through two ultradian cycles to the wake; the wake test before the budget at the threshold and one LSB above it; the budgets one window short; the whole pressure in one window at $k \le 2$; 1.0 and 0 reached exactly; $k = 0$ moving nothing but the phase; the wake from each stage and from an unknown stage; each well-formedness clause alone with its neighbours; the bytes; a property walk (every step keeps the record well formed and the stage one of three; a shift of one to six cycles within 4 000 windows). `cortex-runtime/tests/sleep.rs`: the machine on the executor equals an oracle record through a whole cycle (transitions at windows 66, 70, 72 and 73 at $k = 5$), one tick short of a boundary nothing has moved, the wake between ticks, the defaults never sleeping; `tests/image.rs`: the shift above its bound, a stage of three, a pressure above 1.0, a sleep stage at its budget, and the image's sleep state read back over the configuration's. The mutation gate on the changed lines (ADR-0030) passes in CI.
