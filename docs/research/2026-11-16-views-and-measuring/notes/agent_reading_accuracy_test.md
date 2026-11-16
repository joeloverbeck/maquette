# How accurately an agent reads points off a drawing: a test on known drawings

Test note, 2026-11-16. Evidence only (`EVID-2`, `EVID-4`, `EVAL-2`, principles revision 2026-11-15.1). It was a throwaway test: nothing it used is kept in the repository except this note.

## What it was for

- **Decision served:** whether "an agent places the points, Jon checks by eye" can be the measuring method of the first proportion table, and whether it needs crops and grids to work.
- **Claim tested:** a vision agent can place named points on a head drawing to within 2% of the head's height.
- **Possible outcomes named beforehand:** good enough unaided; good enough only with aids; not good enough, so a different instrument is needed.

## Material

Five grey construction drawings made by hand in mudsong's portrait study on 2026-11-15: one front view and four three-quarter views, each a 1000 by 1000 pixel PNG rendered from an SVG. They are Jon's own character, they are not in this repository, and Jon approved reading them once for this test. Their true positions come from the path nodes of the SVG sources, whose SHA-256 digests are:

| Drawing | SVG digest |
|---|---|
| A, front | `34ee7b92a8422031a20f7d2f195d6952d16e1099de3002f5fffcb6697a57f76f` |
| A, three-quarter | `c50bb0529f254fe12ac0af7b3f6175ae175df14b61d24f16bfec54afcc2efb02` |
| B, three-quarter | `5217ce80fd3fd0f3a65a6d8cfc8eace49f54f79ec153095ec3db223da8f19db8` |
| C, three-quarter | `dfda8410543f8ec7ce0e7ece054ac02cc9deb58964e1a2fa89694fae2d6e8d23` |
| D, three-quarter | `50a9b37bf0cd6d3725812eaeea851e09802a20404cdb3b374b3153834657437d` |

## Method

- **Points:** up to twelve per drawing, 52 in all: the two corners of each eye white, both pupil centres, the lowest point and the two upper corners of the nose shape, the two ends of the mouth line, and the lowest point of the chin. A point was scored only where the SVG has a node for it.
- **Readers:** fresh agents (Claude Fable 5.2) that were given only the PNG files and the definitions of the points, and were told not to look for any other copy.
- **Unaided reading**, twice, by separate agents: each looked at every image once, as it is, and reported coordinates. About 35 seconds and six tool calls for all five drawings.
- **Aided reading**, twice, by two more agents: each cropped every feature with `ffmpeg`, enlarged it five times with a grid line every ten original pixels, and read the enlargement. Ten to nineteen minutes and 55 to 80 tool calls for all five drawings.
- **Score:** the distance between the reported and the true position, as a share of the head's height (479 pixels, hair top to chin).

## Result

| Reading | Mean error | Median | 90th percentile | Worst | Points over 1% | Points over 2% |
|---|---|---|---|---|---|---|
| Unaided, first agent | 0.42% | 0.30% | 0.86% | 1.52% | 4 | 0 |
| Unaided, second agent | 0.42% | 0.42% | 0.66% | 1.32% | 2 | 0 |
| Aided, first agent | 0.17% | 0.10% | 0.42% | 1.04% | 1 | 0 |
| Aided, second agent | 0.20% | 0.21% | 0.42% | 0.93% | 0 | 0 |

- The two unaided readings differ from each other by 0.17% on average and by 0.84% at most. The two aided readings differ by 0.07% on average and by 0.21% at most.
- Eye corners were the least accurate kind of point (mean 0.7% unaided), because the lid shape covers the corner of the eye white. Pupils and the chin were the most accurate (under 0.2%).
- The worst point in every reading was the same one: the outer corner of a flat-topped eye in drawing D.

## What this does and does not establish

**It establishes** that on clean, flat grey drawings of this size an agent's point placement is well inside 2% of the head's height, even without aids, and that two independent readings agree closely enough for their disagreement to be used as a warning.

**It does not establish:**

- **Accuracy on published drawings.** Those have line art, colour, shading and compression, hair over the top of the skull, and heads far smaller than 480 pixels. These drawings are the easy case, and the figures are a floor for the error to expect.
- **Judging the view.** Whether a head is level, or how far it is turned, was not tested. That is the fault Jon saw in mudsong, where heads looking slightly down were measured as level.
- **Points with no drawn feature,** such as the top of the skull under hair or the jaw corner.
- **Independence from the drawer.** The drawings were made by an agent of the same model family as the readers.

**Known flaw.** The instructions to every reader showed the reply format with one line that happened to carry a true position (the first point of the first drawing). Leaving that point out changes no figure above by more than 0.01 percentage points.

## Outcome

The method is not ruled out, and the aids are not needed for precision at this size. The next evidence it needs is on real references: two independent readings of each, with their disagreement shown, and Jon's check of the view and of the points on a sheet.
