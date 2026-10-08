---
status: proposed
date: 2026-10-08
depends-on: ADR-0152
decision-makers: VirtualCortex maintainers
---

# ADR-0153: Three answers, measured — the readouts dealt by ADR-0151's rule and H-29 run once

## Context and Problem Statement

[ADR-0151](0151-three-answers.md) wrote H-29 before any run: on H-25's configuration, stimuli, schedule and arms, with three readouts and an answer for each stimulus that moves to the next readout at a flip, does the engine learn every mapping by each stimulus, with every coupling bounded, the network outside the couplings held and the critic holding each stimulus's expected reward. [ADR-0152](0152-three-answers-built.md) built the selection and the mapping. This ADR is the measurement: the readouts' deal, H-29's protocol, its calibration, its one run and its reading.

It is written in the order the round ran. **This section and the next were committed before any coupling was read.**

## The readouts' deals and their order (written before any coupling was read)

ADR-0151's rule: each readout is four of the fourteen places of a period that lie within both stimuli's windows, one of them a place the prior makes inhibitory, and the deal taken is the first, in an order written before any coupling is read, whose six stimulus–readout couplings on the settled image are equal within ten per cent with none zero.

**The places**, held to the rule by the gate (`the_clauses_of_h_29_the_deals_in_their_order_and_the_readings_rules`, `runtime/cortex-runtime/tests/inhibition.rs`):
- the fourteen within both windows are 3 to 8 and 12 to 19 (`SHARED_PLACES`);
- three of them are inhibitory, 4, 14 and 19 (`INHIBITORY_PLACES`), so each readout holds exactly one;
- eleven are free: 3, 5, 6, 7, 8, 12, 13, 15, 16, 17 and 18 (`FREE_PLACES`). Each readout takes three, nine in all, and two are left in no set.

**The deals.** The readouts are numbered by their inhibitory place: readout 0 holds place 4, readout 1 place 14 and readout 2 place 19. Two deals that differ by the readouts' names alone are then one deal. A deal is readout 0's three free places, then readout 1's three of the eight left, then readout 2's three of the five left: $165 \times 56 \times 10 = 92\,400$ deals (`DEALS`).

**The order** (`deals`). Each readout's three free places are written as an ascending triple. The deals are in lexicographic order of the three triples, readout 0's outermost, then readout 1's, then readout 2's, and triples compare by their first place, then their second, then their third.
- The first deal: readout 0 is places 3, 4, 5 and 6; readout 1 is 7, 8, 12 and 14; readout 2 is 13, 15, 16 and 19.
- The second moves readout 2's last place: 13, 15, 17 and 19.
- The last: readout 0 is 4, 16, 17 and 18; readout 1 is 12, 13, 14 and 15; readout 2 is 6, 7, 8 and 19.

**Why this order.** It is the order of the places' numbers and reads nothing of the network. By the prior's rule a local synapse lands on each of the sixteen places of its source's window with the same chance, and its delay is drawn from the local band whatever the distance, so no place within both windows is nearer a stimulus than another in what the rule gives it. An order that preferred spread places to neighbouring ones would be a choice with no rule behind it.

**The rule's reading** (`dealt`): the largest of the six couplings at most eleven tenths of the smallest, and the smallest above zero — ADR-0065's rule for the rotation (`balanced`), over six couplings where it read four. A deal's six couplings are sums of each stimulus's couplings into single places (`place_couplings`, `deal_couplings`), so the whole order is read from twenty-eight numbers of the image.

**What the gate holds before any coupling is read**: the three lists of places against the rule that derives them; the deals' number; the first, the second, the eleventh, the 561st and the last deal as the order states them; every one of the 92 400 a deal of the rule's — four places a readout, of the fourteen, one inhibitory place and the readout's own, no place twice — and after the one before it in the order; a deal's sets at 1 024 units, each readout 204 units, 51 of them inhibitory; the rule at its edges; and the first deal that passes over tables written by hand.

**One thing read of the geometry and not in ADR-0151.** ADR-0151 wrote that each stimulus unit has four places of each readout in its window. One unit does not: unit 0, stimulus A's first, lies at the ring's seam, where the four units past the last whole period take the place of places 12 to 15 of a period before. It sees a readout's places among 3 to 8 and 16 to 19 only. The other 101 stimulus units see four of each readout. The two-answer geometry has the same seam (`the_geometry_holds_against_the_census_at_both_sizes` reads between four and eight units of a readout in a stimulus unit's window). It is one unit of 51, and the rule reads the couplings the image holds, seam included.
