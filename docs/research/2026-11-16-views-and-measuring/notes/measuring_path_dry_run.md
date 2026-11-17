# The measuring path, run where the answer is known

Written 2026-11-17 for ticket #8 of spec #3. It records what the measuring commands gave on inputs whose right answer is known, before any published drawing is read. Evidence only: it accepts nothing.

Every share is of head height, hair top to chin. "The record" is `data/saffi/hand-made-views.toml`: the positions of Saffi's two S2 underdrawings, taken from the drawings' own files by arithmetic.

## 1. What was run

| Input | Run by | What it can show |
|---|---|---|
| Made-up heads: a few landmarks in space, projected at known turns | `cargo test` | That the arithmetic is right: the commands recover what was put in |
| The record's own positions, written under a reader's names | `cargo test`, and by hand in the reference folder | What the method makes of exact positions of real drawings: the part of any later result that is not a reader's error |
| The skull rule, on Saffi's set of proportions | `maquette skull-by-rule` | How close the rule comes to a circle that was drawn |
| Two readers' readings of pictures of the two S2 underdrawings, and two more readers' after the instructions were revised | Sections 5 and 5c | How far a reader's placement is from the known positions |

## 2. Made-up heads

The tests in `tests/measuring.rs`, `tests/proportions_from_readings.rs` and `tests/known_positions.rs` state their arithmetic. They show that:

- a disagreement between two readings of 1.042% is numbered, and one of 0.9375% is not;
- the turn is recovered to 0.001° from heads projected at 12°, 20°, 29°, 36.87°, 45° and 60°, and the same from a head that faces the viewer's left;
- the fit is 0 for a rigid head, and 1.25%, 1.333% and above 1% for heads with one landmark moved in height, off the centre line and sideways;
- readings of a generated head give back its proportions to 0.0002 of a unit, with the skull built by rule;
- readings that hide the ears give the other 58 proportions and say why 15 are not given;
- a reading's error against a made-up record is the distance typed in, by kind of landmark.

Added after the review of the ticket, on the same day:

- a reading that does not pass its check, or whose picture has changed or is gone, is not measured;
- one reading named twice, two readings under one reader's label, and two readings of a drawing under two revisions of the instructions are refused, and a reading under an earlier revision is measured and said to be;
- a front view whose pairs have no spacing, and a three-quarter view whose pairs stand a quarter wider apart than the front view's, give a failure and no turn;
- a sheet still renders when two readings of a drawing have no head height between them, and says so on that drawing's card.

The dry run's figures below were worked out before these checks existed and have not been worked out again under them. Sections 5 and 5c record that every reading of the dry run passed its check when it was made.

They show nothing about drawings. A made-up head is a plain projection by construction.

## 3. The record's own positions, with no reader

`maquette known-positions-as-reading` writes the record's positions under the names a reader is asked for: what a reader who placed every visible point exactly would write. Run through the measuring commands, they give:

| What | Under a reader's names | Already recorded |
|---|---|---|
| The turn | 28.15° over 8 pairs (cosine 0.8817) | 28.96° over 18 pairs under the record's names; 27.85° over the 15 with no covered point; pinned at 29° |
| The pairs one by one | 0° to 29.45° | 0° to 32.74° |
| The fit | 1.25% | No single figure. Pairs within 0.8% but two; levels within 1% but five |
| The worst misfits | The far jaw point's level, 1.25%; the far brow's outer end, 0.94%; the nose mark's corners in spacing, 0.74% | The record compares its own points: among those nothing covers, the lid lines' crests in spacing, 1.61%, and the far brow's outer lower corner in level, 1.04% |
| Heights, against her S2 proportions | 24 compared, all identical | |
| Widths | 24 compared: mean 0.08%, largest 0.29% | |
| Depths | 17 compared: mean 0.40%, largest 1.10%; two above 1%, the neck's and the shoulders' | |
| Proportions given | 65 of 73 | 73 |

**What this says.** The turn the commands work out is the recorded turn to within a degree, which is how firmly the record says it is known. Heights do not depend on the turn and come back exactly. Widths and depths differ only by that degree: the S2 proportions were taken at the pinned 29°, and a depth carries about twice what a width does.

**The eight not given.** A reader is not asked for the skull, and the hair covers both temples and both eye-line crossings of the face outline in her front view, and the near side's outline at the nose level in her three-quarter view. So no reading of these two pictures can give the widths of the face at its top and at the eye line, three depths that rest on them, or the skull, which the rule builds from the temples. This is a property of these drawings, not of the reader.

**A choice made after seeing this.** The first list of pairs counted the top and the bottom of the eye opening, the lid line's crest and the ear's top. On the record's exact positions that gave 26.12° over 12 pairs: the top of the opening and the crest of the lid line each gave 21.6°, and the ears' tops 22.1°. Each of those is where an outline runs level, so its sideways position is the least certain thing about it, and a level stretch of a drawn outline need not be the same point of the head in two drawings. They were taken out of the pairs for what they are, and the tests changed with them. The choice was made on Saffi's drawings, and no other drawing has tested it.

## 4. The skull rule on Saffi

The rule (`proportions/list.toml`): the skull's centre stands at the height and at the depth of the middle of the ear's base, midway between its two base points, and its radius is the distance in space from that centre to the temple, the top end of the face outline. The hair's top is not used.

`maquette skull-by-rule --proportions data/saffi/proportions.toml`, against her drawn construction circle, in units, with head height 2.4723:

| Part of the rule | By rule | Her drawn circle | Apart |
|---|---|---|---|
| Centre height above the eye line | 1.0286 | 0.3635 | 0.6651, **26.90%** |
| Centre depth | −0.7134 | −0.6920 | 0.0214, **0.86%** |
| Radius, from her drawn centre to the temple: the radius part alone | 0.9720 | 0.9835 | 0.0114 short, **0.46%** |
| Radius, from the rule's own centre: the whole rule | 1.2874 | 0.9835 | 0.3040, **12.30%** |
| Top of the skull above the eye line | 2.3160 | 1.3470 | 0.9690, **39.20%**; her hair's top is at 1.4723 |

On her S2 proportions alone the four shares are 26.90%, 0.78%, 0.49% and 12.23%.

**What this says, part by part.**

- *Depth:* the rule puts the centre within 0.9% of her drawn one. Her ears stand at the depth of her skull's centre.
- *Radius:* from a centre that is right, the sphere through the temple is her drawn circle to within 0.5%. The radius has to be a distance in space: within the front view alone the temple is well inside the circle.
- *Height:* the rule fails on her, and it cannot be tested on her. She is of the cat-folk and her ears are set on top of her skull, a unit above the eye line, where the lessons the rule follows mean a human ear at the side of the head. With the centre that high, the whole rule gives a skull that stands far above her hair. The rule was not changed to suit her.

**What is not settled.** Which point of a human ear the lessons mean by "at the ears": the rule takes the middle of the ear's base, where the ear stands on the head, and Loomis's brow line, which is the ear's top, is the strongest alternative. No drawing with a human ear and a drawn skull is on record to test either. Jon judges the skull the rule builds when the table's middle head is drawn (#10).

**A second limit.** In a published drawing the hair often covers the ear, or the temple. The rule then has nothing to build from, and the three conventions are reported as not given. Section 3 shows it on Saffi's own front view.

## 5. The dry run on pictures

**The design.** Two agents, each in a folder that holds only the two pictures, read both S2 underdrawings by the reading instructions at revision 2026-11-17.1. Neither sees the record, the drawings' vector files, the repository or the other's reading. The pictures are named `drawing-1.png` and `drawing-2.png`, so the view is the reader's to judge. Their readings are scored against the record by kind of landmark, set against each other, and taken through the turn, the fit and the proportions, which are compared with section 3 and with her S2 proportions. A reference check sheet of the two readings is rendered in the reference folder. No reading and no sheet is committed.

**The run.** On 2026-11-17, under reading instructions 2026-11-17.1. Both readers called `drawing-1` a three-quarter view facing the viewer's right, turned 30°, and level, and `drawing-2` a front view, level; neither set a drawing aside. Both judgements of the view are right. One reader said it also looked at the other drawing's eye spacing while judging the turn. Every reading passed its check.

**A reader's error against the known positions,** over the landmarks that are placed, can be seen in the drawing and are on record:

| Reading | Scored | Mean | Median | 90th pct | Worst | Over 1% | Over 2% |
|---|---|---|---|---|---|---|---|
| First reader, front | 55 | 1.23% | 0.11% | 2.43% | 16.86% | 8 | 6 |
| Second reader, front | 53 | 0.72% | 0.06% | 1.16% | 10.01% | 6 | 4 |
| First reader, three-quarter | 53 | 1.39% | 0.12% | 2.50% | 26.25% | 8 | 6 |
| Second reader, three-quarter | 52 | 0.90% | 0.11% | 1.54% | 15.01% | 9 | 5 |

The medians are a tenth of a percent, and the means are carried by a few landmarks. By kind, as the mean and the worst of each reading in the order of the table above:

| Kind | Mean | Worst |
|---|---|---|
| Hair: top and side extents | 0.10%, 0.02%, 0.07%, 0.02% | 0.12% |
| Chin | 0.00%, 0.04%, 0.21%, 0.11% | 0.21% |
| Nose mark | 0.15%, 0.15%, 0.17%, 0.10% | 0.21% |
| Mouth line | 0.23%, 0.19%, 0.42%, 0.46% | 0.63%, its centre in the three-quarter view |
| Eye opening | 0.50%, 0.35%, 0.41%, 0.31% | 1.66%, the opening's top |
| Iris | 0.04%, 0.02%, 0.06%, 0.04% | 0.17% |
| Lid line | 0.33%, 0.51%, 0.13%, 0.39% | 1.25%, its crest |
| Brow line | 0.04%, 0.07%, 0.04%, 0.06% | 0.13% |
| Ear | 0.10%, 0.12%, 0.09%, 0.10% | 0.39% |
| Face outline | 5.67%, 1.57%, 6.08%, 1.28% | 26.25%, the jaw point |
| Neck | 10.06%, 10.01%, 10.57%, 11.96% | 15.01% |
| Shoulder line | 2.50%, 0.34%, 4.12%, 3.58% | 5.75% |

**Three landmarks carry the large errors, and the fault is in their definition.** The jaw point, the neck's base and the shoulder point are each a place on a smooth curve that the instructions did not pin down.

- *The jaw point.* One reader took the upper turn of the outline, where the cheek turns into the jaw, and was 8% to 26% off. The other took a point near the chin and was 1.4% to 4.6% off. The record's point is the lower turn, where the jaw turns into the chin.
- *The neck's base.* Both readers took the middle of the curve into the shoulder, where the edge runs at about 45°, and both were 9% to 15% off. The record's point is where the neck's straight edge ends and that curve begins.
- *The shoulder point.* 0.3% to 5.7% off, for the same reason.

Without those six landmarks of a view, the four readings score a mean of 0.17% to 0.21%, a median of 0.06% to 0.09% and a worst of 1.22% to 1.66%, with two or three landmarks over 1% and none over 2%. The mean is that of the aided readings of the accuracy test of 2026-11-16 (0.17% and 0.20%), on four times as many landmarks a drawing; the worst is higher than theirs (1.04%).

**What is left over 1% after those.** The top of an eye opening, in all four readings, 1.2% to 1.7% sideways: the opening runs level along its top for 15 to 25 pixels, the readers took the middle of the run, and the record's top is 6 to 8 pixels from it. The lid line's crest once, for the same reason. And the far side's outline at the eye line in the three-quarter view, 1.5% in both readings: both read the edge of the drawn skull circle, which runs 7 pixels outside the face outline there.

**What the readers rightly did not place.** Both temples and both eye-line crossings of the front view, and the near side's nose-level crossing in the three-quarter view, which the hair covers; the far ear's two base points, which the head covers. Both readers also stated the far temple of the three-quarter view as not seen, where the record says nothing covers it: they saw the outline run up under the far lock some 20 pixels below the record's point, which lies on the hair's edge. One reader stated the inner base point of each ear as not seen where the other placed it through a gap in the fringe, to 0.15%.

**Placed though covered.** Both readers placed the far ear's inner extent in the three-quarter view where the hair cuts the ear off, 1.1% to 1.2% from the extent the head covers.

**The agreement between the two readers.**

| Drawing | Compared | Mean | Over 1% | Which |
|---|---|---|---|---|
| Front | 54 | 0.97% | 4 | The two jaw points, 21.3%; the two shoulder points, 2.1% and 2.3% |
| Three-quarter | 53 | 0.99% | 4 | The two jaw points, 27.6% and 12.9%; the near neck base, 2.9%; the far shoulder point, 1.4% |

The disagreement caught the jaw point, where the readers chose differently. It did not catch the neck's base: in the front view the two readers put it within 0.1% of each other and 10% from the known position. Two readings that agree show that a definition is read one way, not that it is read rightly.

**The turn.**

| From | Turn | Pairs |
|---|---|---|
| The two readers' readings together | 28.24° | 9 |
| The first reader's alone | 28.29° | 9 |
| The second reader's alone | 28.19° | 9 |
| The record's exact positions, under a reader's names (section 3) | 28.15° | 8 |
| The record, under its own names | 28.96°, and 27.85° without covered points | 18, 15 |
| Each reader's own estimate | 30° | |

The readers' error moves the turn by a tenth of a degree. The fit of the two views, from both readers' readings together, is 1.30%, against 1.25% from exact positions: the same misfit, the far jaw point's level.

**The proportions, against her S2 set.** The two readers' readings together give 61 of 73 proportions; the first reader's alone give 64 and the second's 61.

| | Compared | Mean | Largest | Over 1% |
|---|---|---|---|---|
| Heights | 22 | 0.98% | 10.17%, the neck's base | 4 |
| Widths | 23 | 0.75% | 6.79%, the jaw point | 3 |
| Depths | 16 | 1.32% | 10.72%, the jaw point | 4 |

The eleven over 1% are the jaw point's three, the neck's three, the shoulders' two, the ear's inner extent's two, and the mouth centre's depth at 1.05%. All but the last come from the landmarks named above. Set against what exact positions give, the readers' share of the mouth centre's depth is 1.41%: 3 pixels sideways in the three-quarter view, doubled by the turn. The twelve not given are the eight of section 3, the temple's height, and the three of the ear's inner base point, which one reader did not place.

**What it shows.** On two clean drawings, a reader following the instructions places the features of a face (eyes, irises, lids, brows, nose, mouth, chin, ears and the hair's extent) to a few tenths of a percent of head height, judges a front view from a three-quarter view, and gives a turn within a tenth of a degree of what exact positions give. It shows that three definitions were not good enough to be read, and that a reader does not place what the hair covers.

**What it does not show.** Anything about a published drawing; anything about judging whether a head is level; anything about a side view or a human ear (section 6).

## 5a. The instructions revised, once

The two readers reported, independently, the same places where instructions 2026-11-17.1 were unclear. The instructions were revised once, to 2026-11-17.2. Where the readers had chosen differently, the rule kept is the one whose choice landed nearer the known position.

| What was unclear | Revision | What the scores say |
|---|---|---|
| The temple: "where the hair leaves it showing" against "under the hair, not seen" | The temple is where the outline meets the hairline, and is placed only where that end itself shows. Where hair hangs over the outline, it is not seen | Where the outline goes under the hair is 20 pixels from the record's temple in the three-quarter view and 113 in the front view |
| Telling the face outline from the edge of hair over it, or from a construction line beside it | A section on the face outline: follow it up from the chin; a hair edge runs straight on or ends in a point; a level that crosses a hair edge has no crossing | Both readers were right to place nothing where the hair covers, and both read the skull circle for the outline once, 1.5% off |
| The ear's base points where hair crosses the ear | They are the corners of the ear's own foot. One that shows through the hair is placed; where the hair crosses the ear is never one | The reader who placed the corner through the fringe was 0.15% off |
| The jaw point on a jaw that turns twice | The lower turn, the last before the chin. Not seen on an outline that is one even curve | The reader nearer the chin was 1.4% to 4.6% off; the reader at the upper turn 8% to 26% |
| The neck's base and the shoulder point on smooth curves | The start of the curve from the neck's straight edge; the point where the shoulder's outline stops running outward | Both readers' 45° points were 9% to 15% off for the neck, and up to 5.7% for the shoulder |
| The mouth line's centre in a turned view | Where the mouth line crosses the straight line from the nose mark's bottom to the chin | Both readers were 0.6% off by eye. This rule gives 585 where the record has 590: no better on this drawing, and stated so that two readers do the same |
| A top or a bottom that runs level | The middle of the run | Both did this. It leaves 1.2% to 1.7% sideways on the eye opening's top, which a reader cannot remove |
| A band's end cut on a slant | The middle of the cut | Both did this: brow ends to 0.13%, lid line ends to 0.5% |
| An extent that hair or the head cuts off | Not seen. A landmark is a point of the feature itself, never the place where another shape's edge crosses it | Placed at the cut, it was 1.1% to 1.2% off |
| The reading's `[reference]` comments assumed a published source | Reworded for any drawing | |
| A grid that crowds the drawing; an enlargement refused for its size | The grid's lines are drawn at half strength, and the instructions give the step and the largest part for each factor | |

One principle now stands behind four of these rules: a landmark is a point of the feature itself, and the place where a covering shape's edge crosses a feature is never a landmark.

The rules are general, and each was decided on two drawings of one character.

## 5c. The retest under the revised instructions

The spec has a reader's reading tested again on drawings with known positions whenever the instructions change. On 2026-11-17 two new readers, who had seen nothing of the first run, read the same two pictures under 2026-11-17.2, once. Both judged the views as the first two did, and both level; they estimated the turn at 30° and 25°. Every reading passed its check.

**A reader's error against the known positions,** first run beside retest:

| Reading | Scored | Mean | Median | 90th pct | Worst | Over 1% | Over 2% |
|---|---|---|---|---|---|---|---|
| First reader, front: first run | 55 | 1.23% | 0.11% | 2.43% | 16.86% | 8 | 6 |
| First reader, front: retest | 55 | 0.21% | 0.10% | 0.83% | 1.46% | 4 | 0 |
| Second reader, front: first run | 53 | 0.72% | 0.06% | 1.16% | 10.01% | 6 | 4 |
| Second reader, front: retest | 53 | 0.19% | 0.12% | 0.42% | 1.25% | 2 | 0 |
| First reader, three-quarter: first run | 53 | 1.39% | 0.12% | 2.50% | 26.25% | 8 | 6 |
| First reader, three-quarter: retest | 53 | 0.50% | 0.12% | 1.16% | 7.53% | 7 | 3 |
| Second reader, three-quarter: first run | 52 | 0.90% | 0.11% | 1.54% | 15.01% | 9 | 5 |
| Second reader, three-quarter: retest | 53 | 0.52% | 0.16% | 1.25% | 4.55% | 6 | 4 |

The readers of the retest are not those of the first run: "first" and "second" only name the two readers of each.

**By kind,** as the range of the four readings' means and the worst landmark of the four:

| Kind | First run: means | First run: worst | Retest: means | Retest: worst |
|---|---|---|---|---|
| Hair | 0.02% to 0.10% | 0.12% | 0.02% to 0.07% | 0.12% |
| Chin | 0.00% to 0.21% | 0.21% | 0.00% to 0.33% | 0.33% |
| Nose mark | 0.10% to 0.17% | 0.21% | 0.07% to 0.17% | 0.21% |
| Mouth line | 0.19% to 0.46% | 0.63% | 0.17% to 0.52% | 0.92% |
| Eye opening | 0.31% to 0.50% | 1.66% | 0.24% to 0.43% | 1.25% |
| Iris | 0.02% to 0.06% | 0.17% | 0.05% to 0.09% | 0.21% |
| Lid line | 0.13% to 0.51% | 1.25% | 0.29% to 0.54% | 1.08% |
| Brow line | 0.04% to 0.07% | 0.13% | 0.07% to 0.11% | 0.12% |
| Ear | 0.09% to 0.12% | 0.39% | 0.08% to 0.14% | 0.39% |
| Face outline | 1.28% to 6.08% | 26.25% | 0.07% to 1.60% | 4.41% |
| Neck | 10.01% to 11.96% | 15.01% | 0.06% to 4.40% | 7.53% |
| Shoulder line | 0.34% to 4.12% | 5.75% | 0.28% to 2.57% | 4.55% |

**The three landmarks the revision was for came down.**

| Landmark | First run | Retest |
|---|---|---|
| The jaw point, front view | 16.9% and 4.6% | 1.5%; the other reader stated it as not seen, as one even curve |
| The jaw point, three-quarter view | 26.2% and 8.5%; 1.4% and 4.5% | 3.1% and 4.4%; 3.5% and 4.4% |
| The neck's base, front view | 10.1% and 10.0% | 0.06% and 0.10% |
| The neck's base, three-quarter view | 12.5% and 8.6%; 15.0% and 8.9% | 7.5% and 1.3%; 4.0% and 0.1% |
| The shoulder point, front view | 2.6% and 2.4%; 0.4% | 0.28%; 0.44% |
| The shoulder point, three-quarter view | 5.7% and 2.5%; 4.5% and 2.7% | 1.2% and 0.2%; 4.6% and 0.6% |

In the front view all three are now read to under 1.5%. In the three-quarter view they are better and still not good: the jaw point is 3% to 4.4% off on both sides for both readers, the near neck base 4% to 7.5%, and one reader's far shoulder point 4.6%. Both readers put the three-quarter view's jaw points within 2 pixels of each other, so the rule is now read one way; it is not the record's way. The record's jaw points there are 26 and 32 pixels from the chin, on an outline with no turn that either reader could find so far out.

**The features held.** Without the jaw points, the neck's bases and the shoulder points, the four readings of the retest score a mean of 0.16% to 0.22%, a median of 0.08% to 0.13% and a worst of 1.08% to 1.59%, with two landmarks over 1% in each and none over 2%. The first run's were 0.17% to 0.21%, 0.06% to 0.09% and 1.22% to 1.66%.

**What the revision changed besides.**

- *The ear's base points.* All four readings now place the inner base point of every ear that shows it, to 0.03% to 0.19%. In the first run one reader of two did.
- *The far ear's inner extent, which the head covers.* Both readers of the first run placed it at the cut, 1.1% to 1.2% off. Both readers of the retest stated it as not seen.
- *The temples.* Stated as not seen by all, as in the first run.
- *Nothing that was rightly not seen is now placed.* The one covered point placed in the retest, the near lid line's outer end in the front view, was placed in the first run too, to 0.13% to 0.47%.

**What the revision made worse.** The mouth line's centre in the three-quarter view: 0.62% in the first run, by eye, and 0.87% and 0.92% in the retest, by the line from the nose mark's bottom to the chin. The two readers now put it 0.2 pixels apart, and 4 pixels from the record's. The rule made the reading repeatable and moved it 1.2 pixels further from this drawing's known position. Its depth, taken through the turn, went from 1.05% to 1.60% against her S2 set. Nothing else is worse beyond what two different pairs of readers differ by.

**What the revision did not mend.** The far side's outline at the eye line in the three-quarter view is still read on the drawn skull circle, 1.5% to 1.6% off, by both readers. The top of the eye opening and the lid line's crest are still 0.5% to 1.25% off sideways, as the middle of a level run must be.

**The agreement between the two readers.**

| Drawing | | Compared | Mean | Largest | Over 1% |
|---|---|---|---|---|---|
| Front | First run | 54 | 0.97% | 21.28% | 4 |
| Front | Retest | 54 | 0.13% | 0.69% | 0 |
| Three-quarter | First run | 53 | 0.99% | 27.61% | 4 |
| Three-quarter | Retest | 53 | 0.30% | 3.52% | 3: the two neck bases and the far shoulder point |

**The turn, the fit and the proportions.**

| | First run | Retest | Exact positions |
|---|---|---|---|
| The turn, both readers together | 28.24° | 28.30° | 28.15° |
| The turn, each reader alone | 28.29°, 28.19° | 28.28°, 28.32° | |
| The fit | 1.30% | 0.95% | 1.25% |
| Proportions given | 61 of 73 | 64 of 73 | 65 of 73 |
| Heights against her S2 set: mean, largest | 0.98%, 10.17% | 0.34%, 3.07% | 0.00%, 0.00% |
| Widths: mean, largest | 0.75%, 6.79% | 0.31%, 3.85% | 0.08%, 0.29% |
| Depths: mean, largest | 1.32%, 10.72% | 0.59%, 2.34% | 0.40%, 1.10% |
| Proportions over 1% | 11 | 7 | 2 |

The seven over 1% in the retest are the jaw point's drop and half-width, the neck's base drop and depth, the shoulders' drop and depth, and the mouth centre's depth. The fit is lower than exact positions give because the readers' jaw points agree between the two views better than the record's do; it is not a better reading of her.

**The verdict.** The revision made reading better where it was meant to and made one landmark a little worse. It did not make reading worse overall, so the instructions stand at 2026-11-17.2. Three landmarks are still not read well in a turned view: the jaw point, the neck's base and the shoulder point. Their proportions should not be trusted from a three-quarter drawing until their definitions are settled on more than one character.

**Left open, for the ticket that first reads published drawings.** Both readers of the retest reported these, and the instructions were not changed again: one retest was the limit, and any change calls for another.

- How to tell an ear's foot that shows through a gap in the hair from a separate mark, and which corner of such a patch is the base point.
- Where within a bend the jaw's turn is, and how much more a curve must bend to count as not one even curve. One reader placed the front view's jaw points and one did not.
- What "where the two cannot be told apart" means for a construction line that lies exactly on the outline. Both placed the far eye-line crossing there.
- The shoulder point when the arm's outline keeps running outward to the picture's edge.
- An extent on an upright run that goes under the hair.
- The mouth line where a drawn level line lies over it, and "in a turned view the centre line is not drawn" when an underdrawing draws one.
- No method is given for the turn estimate. The two readers said 30° and 25° of the same drawing.
- The height of an iris's side extent; a lid line that ends in an oblique cut with a tip beyond it.
- `enlarge-image` refuses a factor above 16, and the instructions do not say so.

Several of these come from the construction lines of an underdrawing, which a published drawing does not have.

## 5b. What a reader can give the skull rule

The rule needs the ear's two base points, for the centre, and the temple, for the radius.

- **The ear's base points.** Read to 0.03% to 0.17% where a reader placed them. One reader of two placed both; the other stated the inner one as not seen, under the fringe. From the first reader's readings the rule's centre comes out at a rise of 1.0048 and a depth of −0.7031, where the record's positions give 1.0054 and −0.6997. In the retest both readers placed both, through the fringe, and their readings together give 1.0041 and −0.7013. So where the ear's base shows, the centre is as good as the record's. On her it is still a cat-folk's ear (section 4).
- **The temple.** None of the four readers placed one, in either drawing, under either revision of the instructions: hair covers both temples in the front view, and the far one in the three-quarter view runs under a lock. The rule's radius has no input from these pictures. A fringe or a side lock may be expected to cover the temple in many published drawings of women as well. That is an expectation and not a finding: it rests on these two drawings of one character, and no published drawing has been read.

What else is drawn, and how near a sphere through it comes to her drawn circle, from her drawn centre (worked from `data/saffi/proportions.toml`; her S2 set in brackets):

| The sphere reaches | Radius | Against her drawn 0.9835 | Read by the dry run's readers |
|---|---|---|---|
| The temple (the rule as it stands) | 0.9720 | 0.46% short (0.49%) | Never: covered |
| The face outline at the eye line | 0.9646 | 0.76% short (0.95%) | Never in the front view: covered |
| The outer corner of the eye | 0.9318 | 2.09% short (2.33%) | Always, to 0.02% to 0.42% |
| The outer end of the brow line | 0.8727 | 4.48% short (4.66%) | Always, to 0.13% |

The points nearest her circle are the ones the hair covers. The outer corner of the eye is drawn in every front and three-quarter view, and it gives a skull about 2% of head height smaller in radius than hers. This is evidence for a decision that is Jon's: he set the rule.

## 5d. The side-view part revised, once

On 2026-11-17 twelve readers read the first batch of published drawings under 2026-11-17.2: six front heads and three side heads, each twice. The side-view part of the instructions had never been run. All six side readings reported the same failures, independently, and the part was revised once, to 2026-11-17.3.

**No sentence that governs how a front or a three-quarter view is read was changed.** The front readings of the batch and sections 5 to 5c rest on those sentences as they are. Where a general sentence does not hold in a profile, the side-view part now says so for a profile.

**Each definition was set against how the generator draws its side view.** A depth taken from a reading must be of the point the head model draws. The front of the generated outline runs through the nose mark's bottom, the mouth line's centre and the chin; the eye's inner corner is the one toward the front; the ear stands on its two base points.

| What failed in a profile | Revision | Why this point |
|---|---|---|
| `chin`: "the lowest point of the face outline", with the middle-of-a-run rule, landed on the underside of the jaw, 14 to 30 pixels behind the chin's front. Its height was sound and its depth was not | The chin's own lowest point: where the outline, coming down the front of the face, turns to run back under the jaw. On a level underside it is the front end of the run, not its middle | The model's chin is on the centre line, and the generated outline turns back there. Depth is taken from it |
| `nose-mark-bottom`: "where the underside of the nose meets the face" is a rounded turn, read up to 6 pixels apart, and it is not the point the model draws | Under the tip: where the outline, coming round the tip, turns to run back toward the face. The nose mark in a profile is the nose's underside, and this is its front end | A front or a three-quarter view draws the nose mark at the tip, so the model's point stands furthest forward of the lower face, as it does in Saffi's accepted side view. Where the nose meets the lip lies as far behind as the nose stands out |
| `nose-mark-<side>-corner`: "furthest toward that side" points at the viewer. Every reader stated it as not seen | The middle of the nostril, or of the curve drawn for the wing of the nose; not seen where neither is drawn. The near corner only | The generator draws the mark from the near corner to the bottom, and the wing is what a front view's corner marks |
| The nose level: undefined in a profile | Midway in height between that corner and the bottom; the bottom's height where the corner is not seen | The general rule, said for one corner |
| `mouth-line-centre`: unclear under an overhanging lip, or where the line stops short of the outline. `mouth-line-<side>-end`: not explained | On the outline, at the bottom of the notch between the lips; level with the line's front end where the line stops short. The end is the back end of the drawn line | The generated outline runs through the mouth line's centre |
| Inner and outer: defined for the eye's corner and the iris only | Inner is toward the front of the face and outer toward the back of the head, in every name | As the generator draws them: the inner ends of the brow line and the lid line stand in front of their outer ends |
| The profile eye: no lower lid, an opening not closed at the back, the iris at the front edge | The inner corner is the front end of the opening, drawn as a contour or as the iris's edge; the outer corner the back end of what is drawn; the opening's bottom the lowest of the iris or the white; the iris's inner extent and the inner corner may be one point | Depth zero is midway between these two corners. See below |
| The ear: the base-point sentence describes an ear seen from the front | An ear at the side stands on the head along its front edge: its upper end is `base-outer`, its lower end `base-inner`. On an ear on top of the head the front corner of the foot is `base-inner`. The extents are toward the front (inner) and the back (outer) | The front view's rule kept: the upper one is outer. Saffi's side view has the inner base point in front |
| What a side view does not ask for was not said | Said: no temple, no jaw point, no crossing with the eye line, no `hair-<side>-extent`, no `neck-<side>-base` | |
| "The back edge of the jaw": under hair, or only the edge of a shadow | The edge from under the chin back and up toward the ear. A shadow's edge is read where no line is drawn. Not seen under hair | |
| The signs of level are written for a front view | Signs for a profile: the lid line and the mouth line, the ear's span, the jaw's underside. A lean does not show. Any of the far side showing means the head is not in full profile | The readers' own signs, written down. None is tested |
| `hair-back-extent`: on long hair it landed on the hanging length | Read at or above the chin's height | The hair volume the generator draws ends at the chin's height |
| Head height between two points on different uprights | Said: a difference in height only | |

**What the revision does not mend, by what a profile is.**

- *A profile eye is not a projection of the model's eye.* The model's side view shows the opening as wide as the eye wraps round the face, with the iris an upright line toward its back. A drawn profile shows a wedge with the iris at its front. So `eye_wrap_depth` and `iris_depth` taken from a profile are that convention's. Depth zero, midway between the wedge's two ends, may stand a little way from where a front and a three-quarter view would put it, which shifts every depth of that work alike. Nothing in the batch can test this: no character has all three views.
- *The shoulder and the neck* were not seen in any side reading of the batch: hair, a collar or the picture's edge covers them.

**One addition outside the side view,** which changes no placement: how a reading records that the person checking overruled a set-aside (`set_aside_overruled`, with who overruled it), and that such a drawing is read with every drawn landmark placed. The reader's own judgement stays in the reading beside it.

**The retest.** The spec retests a reader whenever the instructions change. The only side drawing with known positions is the generator's own: Saffi's accepted side view, `armatures/saffi/side.svg`, whose landmark file says where each landmark is drawn. `score-reading` scores a side reading against that file, at the positions the view draws, which for her mouth and chin are the set-back ones. Two fresh readers read a picture of it under 2026-11-17.3. Both readings were made by Fable 5.2.

**The run.** On 2026-11-17, once. The picture is `side.svg` rendered at one pixel to a unit, 1000 pixels square, with the head 494 pixels high. Each reader was told which guide stands for which feature and nothing of where its points are, and saw nothing else. Both called it a side view facing the viewer's right, turned 90°, and level, and set nothing aside. Both readings passed their check. Each placed 29 landmarks and stated two as not seen: the iris's two side extents, which an iris seen from its edge does not have and the landmark file does not give. Nothing was placed though covered, and nothing was stated as not seen that the drawing shows.

**A reader's error against the positions the view draws,** over the 29 landmarks each placed:

| Reading | Scored | Mean | Median | 90th pct | Worst | Over 1% |
|---|---|---|---|---|---|---|
| First reader, side | 29 | 0.17% | 0.17% | 0.27% | 0.60%, the eye opening's top | 0 |
| Second reader, side | 29 | 0.16% | 0.12% | 0.36% | 0.50%, the eye opening's top | 0 |

**By kind,** as the mean of the first reading and of the second, and the worst of the two:

| Kind | Scored in each | Means | Worst |
|---|---|---|---|
| Hair: top, back and front extents | 3 | 0.02%, 0.02% | 0.03%, the top |
| Chin | 1 | 0.15%, 0.15% | 0.15% |
| Nose mark: bottom and near corner | 2 | 0.18%, 0.12% | 0.22%, the corner |
| Mouth line: centre and near end | 2 | 0.13%, 0.13% | 0.23%, the end |
| Eye opening | 4 | 0.28%, 0.25% | 0.60%, the top |
| Iris: top and bottom | 2 | 0.23%, 0.25% | 0.26%, the bottom |
| Lid line | 3 | 0.26%, 0.33% | 0.39%, the crest |
| Brow line | 2 | 0.18%, 0.12% | 0.18%, the inner end |
| Ear | 5 | 0.14%, 0.12% | 0.25%, the top |
| Face outline at the nose and mouth levels | 2 | 0.10%, 0.05% | 0.16%, at the mouth level |
| Neck: back and front base | 2 | 0.18%, 0.18% | 0.18%, the back |
| Shoulder point | 1 | 0.01%, 0.02% | 0.02% |

**The agreement between the two readers:** 29 landmarks compared, a mean of 0.09%, the largest 0.88% (the eye opening's top), none over 1%. Neither placed a landmark the other did not.

**What these figures say.** On this drawing of guides, both readers followed the side view's naming (the near side, inner toward the front, the ear's base points, the hair's back and front, the neck's back and front), and placed every point to under 0.61% of head height. That is as close as the features were read on the underdrawings of sections 5 and 5c (means of 0.16% to 0.22%, worsts of 1.08% to 1.66%). The check and the score ran on a side reading. Under the limits below, these figures say nothing of how a painted profile is read, of level, or of human ears; the side readings of the first batch rest on the reading instructions alone.

**Who read the batch.** Jon checked eighteen readings of the first batch at look 4, on 2026-11-17: thirteen made by Fable 5.2 and five by Opus 5.5. The two readings of this retest were Fable 5.2's. No test here has measured how accurately Opus 5.5 reads a drawing.

What it can show:

- whether a reader places points on a profile's guides as closely as sections 5 and 5c found on front and three-quarter underdrawings;
- whether the side view's naming is followed: the near side, inner toward the front, the corners of an ear's foot, the hair's back and front, the neck's back and front;
- that the check and the score run on a side reading.

What it cannot show:

- *How a painted profile is read.* The armature is guides: an outline with no nose, no lips and no underside of a jaw, a tick for the nose mark, an upright line for the iris. The definitions that were revised for painted profiles (the chin before a level jaw, the tip of the nose, the notch of the lips, the wedge eye, the front edge of a human ear) have nothing in it to be placed on. On its smooth outline the old and the new definition of the chin give one point.
- *Level.* It is level by construction.
- *That profiles are drawn as the model draws one.* It is the generator's own profile: agreement with it says a reader can read what the model draws.
- *A human ear.* Hers are a cat-folk's, on top of the head.

The readers of the retest must be told which guide stands for which feature. The underdrawings of sections 5 and 5c did not need that.


## 6. Limits

- **Clean drawings only.** The pictures are flat grey underdrawings with hard edges, 1000 pixels square, with heads about 480 pixels high. A published drawing has line work, colour and compression. Nothing here says how a reader does on one (`EVAL-2`).
- **Level is untested.** Both drawings are level, and her ears say nothing about level. The judgement that failed in mudsong is the one this dry run cannot test.
- **Two drawings, one character.** The dry run tells apart a front view from a three-quarter view, once. It is no test of judging the view.
- **The side view is untested on a painted head.** No painted side drawing with known positions exists. Taking depth from a front and a side view is built and has been run on made-up heads only. Section 5d says what a retest on a generated armature's side view can show.
- **Human ears are untested.** The ear's landmarks were defined on Saffi. How they are read on an ear at the side of the head is stated and has never been run.
- **The drawer and the readers are the same kind of model.** The underdrawings were made by an agent, and agents read them.
- **Each reader saw both drawings.** A reader's judgement of one view was made with the other in sight, as it will be for a character's sheet.
- **Isolation was by instruction.** Each reader was told to open nothing but its own folder, which held the two pictures. Nothing enforced it.
- **Four readers, once each.** Each run is four readings by two readers, and the retest's readers are not the first run's. The difference between the runs is the revision and the readers together; nothing here says how much a reader varies from run to run.
- **The revision was made on these two drawings.** Its rules are written generally, and the retest was on the same two pictures they were decided on. It is no test of them on another drawing.
- **The pictures are taken as renders of the drawings at one pixel to a unit.** An agent's look at the record's positions drawn over the pictures found them on the features. That is a diagnostic, not a check.
- **The sheet has not been judged.** No look was asked. Whether Jon can check a character on it is unknown until he does.
