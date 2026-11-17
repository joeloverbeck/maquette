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

**The result.** Not run when this note was first written. It is added here when it is.

## 6. Limits

- **Clean drawings only.** The pictures are flat grey underdrawings with hard edges, 1000 pixels square, with heads about 480 pixels high. A published drawing has line work, colour and compression. Nothing here says how a reader does on one (`EVAL-2`).
- **Level is untested.** Both drawings are level, and her ears say nothing about level. The judgement that failed in mudsong is the one this dry run cannot test.
- **Two drawings, one character.** The dry run tells apart a front view from a three-quarter view, once. It is no test of judging the view.
- **The side view is untested.** No side drawing with known positions exists. The reading instructions say what a side view asks for; taking depth from one is not built.
- **Human ears are untested.** The ear's landmarks were defined on Saffi. How they are read on an ear at the side of the head is stated and has never been run.
- **The drawer and the readers are the same kind of model.** The underdrawings were made by an agent, and agents read them.
- **The pictures are taken as renders of the drawings at one pixel to a unit.** An agent's look at the record's positions drawn over the pictures found them on the features. That is a diagnostic, not a check.
- **The sheet has not been judged.** No look was asked. Whether Jon can check a character on it is unknown until he does.
