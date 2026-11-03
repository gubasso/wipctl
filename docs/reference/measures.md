# Measures

The flow measures and the point arithmetic. A reader looks these values up here. The specs state the rules, and each section names the rules that bind it.

## The three flow measures

| Measure | Definition                                                                                           |
| ------- | ---------------------------------------------------------------------------------------------------- |
| age     | from the first entry into the in-flight lane to the entry's last event when closed, otherwise to now |
| dwell   | the summed intervals spent in the review lane, counting an open interval through now                 |
| rework  | entries into the in-flight lane beyond the first                                                     |

A duration is displayed in whole days, floored.

Rules that bind this section: `metrics:a-duration-is-shown-in-whole-days`.

## The point arithmetic

| Outcome    | Contribution                                                      |
| ---------- | ----------------------------------------------------------------- |
| `done`     | delivers its own points                                           |
| `cut`      | contributes nothing and stays in the total                        |
| `reshaped` | delivers its own points exactly when its successor closed as done |
| reopened   | counts as open                                                    |

Rules that bind this section: `epics:the-arithmetic-has-one-implementation`.
