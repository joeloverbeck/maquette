# The measuring path, run where the answer is known

Written 2026-11-17 for ticket #8 of spec #3. It records what the measuring commands gave on inputs whose right answer is known, before any published drawing is read. Evidence only: it accepts nothing.

Every share is of head height, hair top to chin. "The record" is `data/saffi/hand-made-views.toml`: the positions of Saffi's two S2 underdrawings, taken from the drawings' own files by arithmetic.

## 1. What was run

| Input | Run by | What it can show |
|---|---|---|
| Made-up heads: a few landmarks in space, projected at known turns | `cargo test` | That the arithmetic is right: the commands recover what was put in |
| The record's own positions, written under a reader's names | `cargo test`, and by hand in the reference folder | What the method makes of exact positions of real drawings: the part of any later result that is not a reader's error |
| The skull rule, on Saffi's set of proportions | `maquette skull-by-rule` | How close the rule comes to a circle that was drawn |
| Two readers' readings of pictures of the two S2 underdrawings | Section 5 | How far a reader's placement is from the known positions |

## 2. Made-up heads

The tests in `tests/measuring.rs`, `tests/proportions_from_readings.rs` and `tests/known_positions.rs` state their arithmetic. They show that:

- a disagreement between two readings of 1.042% is numbered, and one of 0.9375% is not;
- the turn is recovered to 0.001° from heads projected at 12°, 20°, 29°, 36.87°, 45° and 60°, and the same from a head that faces the viewer's left;
- the fit is 0 for a rigid head, and 1.25%, 1.333% and above 1% for heads with one landmark moved in height, off the centre line and sideways;
- readings of a generated head give back its proportions to 0.0002 of a unit, with the skull built by rule;
- readings that hide the ears give the other 58 proportions and say why 15 are not given;
- a reading's error against a made-up record is the distance typed in, by kind of landmark.

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

**The retest.** The spec has a reader's reading tested again on drawings with known positions whenever the instructions change. Two new readers read the same two pictures under 2026-11-17.2, once. Not run when this was written; its scores are added here when it has.

## 5b. What a reader can give the skull rule

The rule needs the ear's two base points, for the centre, and the temple, for the radius.

- **The ear's base points.** Read to 0.03% to 0.17% where a reader placed them. One reader of two placed both; the other stated the inner one as not seen, under the fringe. From the first reader's readings the rule's centre comes out at a rise of 1.0048 and a depth of −0.7031, where the record's positions give 1.0054 and −0.6997. So where the ear's base shows, the centre is as good as the record's. On her it is still a cat-folk's ear (section 4).
- **The temple.** No reader placed one, in either drawing, under either reading of the instructions: hair covers both temples in the front view, and the far one in the three-quarter view runs under a lock. The rule's radius has no input from these pictures. A fringe or a side lock covers the temple in most drawings of women.

What else is drawn, and how near a sphere through it comes to her drawn circle, from her drawn centre (worked from `data/saffi/proportions.toml`; her S2 set in brackets):

| The sphere reaches | Radius | Against her drawn 0.9835 | Read by the dry run's readers |
|---|---|---|---|
| The temple (the rule as it stands) | 0.9720 | 0.46% short (0.49%) | Never: covered |
| The face outline at the eye line | 0.9646 | 0.76% short (0.95%) | Never in the front view: covered |
| The outer corner of the eye | 0.9318 | 2.09% short (2.33%) | Always, to 0.02% to 0.42% |
| The outer end of the brow line | 0.8727 | 4.48% short (4.66%) | Always, to 0.13% |

The points nearest her circle are the ones the hair covers. The outer corner of the eye is drawn in every front and three-quarter view, and it gives a skull about 2% of head height smaller in radius than hers. This is evidence for a decision that is Jon's: he set the rule.


## 6. Limits

- **Clean drawings only.** The pictures are flat grey underdrawings with hard edges, 1000 pixels square, with heads about 480 pixels high. A published drawing has line work, colour and compression. Nothing here says how a reader does on one (`EVAL-2`).
- **Level is untested.** Both drawings are level, and her ears say nothing about level. The judgement that failed in mudsong is the one this dry run cannot test.
- **Two drawings, one character.** The dry run tells apart a front view from a three-quarter view, once. It is no test of judging the view.
- **The side view is untested.** No side drawing with known positions exists. The reading instructions say what a side view asks for; taking depth from one is not built.
- **Human ears are untested.** The ear's landmarks were defined on Saffi. How they are read on an ear at the side of the head is stated and has never been run.
- **The drawer and the readers are the same kind of model.** The underdrawings were made by an agent, and agents read them.
- **Each reader saw both drawings.** A reader's judgement of one view was made with the other in sight, as it will be for a character's sheet.
- **Isolation was by instruction.** Each reader was told to open nothing but its own folder, which held the two pictures. Nothing enforced it.
- **Two readers, once.** The figures are four readings. They say nothing of how much a reader varies from run to run.
- **The pictures are taken as renders of the drawings at one pixel to a unit.** An agent's look at the record's positions drawn over the pictures found them on the features. That is a diagnostic, not a check.
- **The sheet has not been judged.** No look was asked. Whether Jon can check a character on it is unknown until he does.
