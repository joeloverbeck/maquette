# Saffi's hand-made views as numbers

Saffi is the comparison case: the one character whose accepted armature is kept out of every proportion table, so that what a table allows can be tested against her. This folder holds her hand-made front view and three-quarter view as landmark positions, and what the two views say when they are set against each other.

Her comparison case is her two grey S2 underdrawings with the ears and the lids of her two final drawings in their place. Jon ruled so on 2026-11-16: the larger ears and the hooded upper lids of her final drawings, and everything else as recorded from S2 ("The final drawings' ears and lids" below).

**These numbers never enter a proportion table.** The positions are also not her set of proportions: that is `proportions.toml`, worked out from them once the list of proportions existed ("Her set of proportions" below).

| File | What it holds |
|---|---|
| [`proportions.toml`](proportions.toml) | Her set of proportions: one value for each proportion of [`proportions/list.toml`](../../proportions/list.toml), with where it came from and the rules used. Written by the program from `hand-made-views.toml` and `final-drawings.toml`. It is a set for one head, not a table: it has no ranges and no works behind it. |
| [`hand-made-views.toml`](hand-made-views.toml) | The S2 underdrawings: 118 landmark positions in the front view and 122 in the three-quarter view, each with the element it came from and how it was taken. Also what the views give for each guide, and what was left out. |
| [`views-compared.toml`](views-compared.toml) | The S2 underdrawings set against each other: the turn of the three-quarter view, worked out pair by pair, and every place where the two views do not describe one head, as a share of head height. |
| [`final-drawings.toml`](final-drawings.toml) | The final drawings: the 36 landmark positions of the ears and the lids in each view, each with the element it came from and how it was taken, how far each stands from the S2 landmark it replaces, and where the two final views do not agree. |

No image, tracing, crop, path data or copy of the drawings is here. The repository is public.

## Her drawings in mudsong

Every drawing of Saffi that maquette relies on is in mudsong's portrait study folder, `art/studies/saffi-portrait/work/`. Git ignores that folder in mudsong, so no commit names a drawing: the SHA-256 digest of its SVG file is its only identity. Mudsong is private, and nothing here needs it to be present.

| Drawing | SHA-256 | Stage | Standing |
|---|---|---|---|
| `s2-base-underdrawing-A-front.svg` | `34ee7b92a8422031a20f7d2f195d6952d16e1099de3002f5fffcb6697a57f76f` | S2: grey underdrawing, front view, made on 2026-11-15 | It was on the sheet Jon accepted in mudsong on 2026-11-15, without being the question put to him. Read on 2026-11-16. It gives every front-view number here but the ears and the lids, for which `s6-front.svg` superseded it on 2026-11-16. |
| `s2-base-underdrawing-A.svg` | `c50bb0529f254fe12ac0af7b3f6175ae175df14b61d24f16bfec54afcc2efb02` | S2: grey underdrawing, three-quarter view, made on 2026-11-15 | The armature Jon accepted in mudsong on 2026-11-15. Read on 2026-11-16. It gives every three-quarter number here but the ears and the lids, for which `s8-final-rig.svg` superseded it on 2026-11-16. The turn was pinned from it. |
| `s6-front.svg` | `15d6fe02177427db7b0ae956411b97295f250d3328554f598134bb8e91c8136e` | S6: the model sheet's front, made on 2026-11-16 over the S2 construction | Pointed to by Jon on 2026-11-16 as the front view of her final drawings. Read on 2026-11-17. It gives the front view's ears and lids and nothing else. |
| `s8-final-rig.svg` | `23251a2e5c196738514f324cec1c25c171fb0785112044d45168541d060d304a` | S8: the final rig, a three-quarter view turned about 30°, made on 2026-11-16 over the S2 construction | Pointed to by Jon on 2026-11-16 as the three-quarter view of her final drawings. Read on 2026-11-17. It gives the three-quarter view's ears and lids and nothing else. |

A later drawing of her in that folder does not change these numbers until Jon points to it and it is read under its own digest. If one of these four is replaced there, its digest no longer matches and the reading command refuses it.

## Where the numbers came from

- **The S2 underdrawings.** Two grey underdrawings of Saffi's bust, made by hand in mudsong's portrait study on 2026-11-15: a front view and a three-quarter view. She is Jon's own character.
- **The read.** Jon approved one read of them on 2026-11-16 (`docs/foundation/02-first-table.md`, "The handoff"). They were read that day, and both digests matched before anything was used. His ruling of 2026-11-16 on her ears and lids named the two final drawings, which were read on 2026-11-17 under the digests above.
- **Custody.** Private copies of the S2 underdrawings are kept in maquette's reference folder, which is outside every repository, with a note of the source, the date, the digests and the approval. The three other drawings of the accuracy test of 2026-11-16 are kept there too, for later tests of an agent's reading on drawings with known positions. They were not measured. The final drawings are recorded where they stay, in mudsong's study folder: no copy of them is kept, but for the face studies of look 9 (#22), which Jon asked to show Saffi as she looks. They are made from her final front drawing and kept with the trial in the reference folder.
- **The derivation.** `maquette record-hand-made-drawings` writes `hand-made-views.toml`, `views-compared.toml` and `final-drawings.toml` from the four drawings ("Rerunning it" below). Nothing in them was edited by hand. The first two were written on 2026-11-16 by a throwaway Rust program kept beside the private copies, whose SHA-256 digest is `7e81dcdeeb6b5e1ef32f7531adde180e4864e047f89b1467677d517ea8bd51cd`; that program was moved into the package on 2026-11-17 as this command, and the command writes both files again byte for byte.

Positions were taken by arithmetic on the files' own coordinates and never estimated from a rendered picture. The renders of the S2 underdrawings were looked at once, only to tell which element is which. No render of the final drawings was looked at: which shape is which was read from the files' own groups and names, as `maquette list-drawing` lists them.

## How to read the positions

- **Units.** SVG user units of the source files. All four are 1000 by 1000 with no transform on any element, so one unit is one pixel of their renders. x grows toward the viewer's right and y grows downward.
- **One frame for a view.** Each final drawing was made over its S2 underdrawing's construction: its skull circle and its chin are the underdrawing's exactly, which the command checks before it writes. So a position of `final-drawings.toml` and a position of `hand-made-views.toml` in the same view stand in one frame.
- **Left and right** are the character's own. Her right is toward the viewer's left in both views. In the three-quarter view her right is the near side.
- **Head height** is `chin.y` minus `hair_top.y`: 480.318 units in the S2 front view and 481.400 in the S2 three-quarter view. Every share in `views-compared.toml` and in `final-drawings.toml` is of the front view's figure, so 1% is 4.803 units.
- **Each landmark** has a `kind` that says how far it can be trusted across views. A `node` is a point the drawing's own path data gives exactly, and it marks a feature. A `highest` or `lowest` is the top or bottom of an outline, found where a curve's slope is zero. A `side_extent`, a `crossing` and an `outline_node` are points on a silhouette or a guide line; the other view shows a different point of the head there, so they are never compared between views. The file defines every kind.
- **`under`** lists the shapes painted over a point. A landmark with `under` is in the file's coordinates and cannot be seen in the drawing.

The accuracy test of 2026-11-16 used 479 units as the head's height, which is the chin minus the first node of the hair's outline (y = 88). The outline rises 1.3 units above that node, to y = 86.682. The difference is 0.3% of the yardstick and moves no figure here across the 1% line.

## What the views give for each guide

This section, "The turn" and "Where the two views do not describe one head" are about the S2 underdrawings, as `hand-made-views.toml` and `views-compared.toml` record them. "The final drawings' ears and lids" says what changed.

The armature's guides are listed in issue #3. `[guides]` in `hand-made-views.toml` says what stands behind each. In short:

- **Drawn in both views:** the skull circle, the centre line, the three level lines, each eye's opening, lid line and iris, the brows, the nose mark, the mouth line and the ears.
- **Partly there:** the face outline (in the three-quarter view the near side has nothing drawn above the nose level) and the neck (its edges are edges of the torso shape, and the near edge in the three-quarter view is only that shape's straight closing edge).
- **Nothing behind it:** the hair volume as a plain envelope (only the extents of the styled hair), the shoulder line as an element (only nodes of the torso outline), and depth. There is no hand-made side view, and no position in either view says how far forward a feature sits.
- **Bands, not lines:** each lid line and each brow is a filled shape with thickness. A tilt or a single line follows from the recorded corners by a rule the list of proportions has yet to state.
- **The eye level line is not through the pupils.** It is at y = 366 in both views. The pupil centres are at y = 370.5, which is 0.9% of head height lower, and the centres of the openings at about y = 370.

## The turn

A level head that turns about an upright axis, drawn flat, keeps every point at its height. Two landmarks that mirror each other across the centre line stand `cos(turn)` times as far apart as in the front view, whatever their depth. So each pair gives a turn of its own, `acos(three-quarter spacing / front spacing)`, and one turn for the head is the one that leaves the least squared misfit in spacing over all the pairs.

**The turn is 28.96°** toward the viewer's right (cosine 0.8750), over the 18 pairs of the head recorded in both views.

| Pair | Front spacing | Three-quarter spacing | Ratio | Turn it gives | Misfit at 28.96° |
|---|---|---|---|---|---|
| `ear_base_outer` (covered) | 340.0 | 286.0 | 0.8412 | 32.74° | −11.50 (**2.39%**) |
| `ear_upper_node` | 268.0 | 236.0 | 0.8806 | 28.29° | +1.50 (0.31%) |
| `lid_line_outer_tip` (covered) | 254.0 | 222.0 | 0.8740 | 29.07° | −0.25 (0.05%) |
| `lid_line_outer_lower` | 234.0 | 206.0 | 0.8803 | 28.32° | +1.25 (0.26%) |
| `brow_outer_top` | 234.0 | 204.0 | 0.8718 | 29.33° | −0.75 (0.16%) |
| `eye_outer_corner` | 232.0 | 202.0 | 0.8707 | 29.46° | −1.00 (0.21%) |
| `brow_outer_bottom` | 226.0 | 201.0 | 0.8894 | 27.20° | +3.25 (0.68%) |
| `ear_base_inner` (covered) | 164.0 | 146.0 | 0.8902 | 27.10° | +2.50 (0.52%) |
| `pupil_centre` | 156.0 | 137.0 | 0.8782 | 28.57° | +0.50 (0.10%) |
| `iris_centre` | 155.392 | 136.0 | 0.8752 | 28.93° | +0.03 (0.01%) |
| `lid_line_lower_mid` | 142.0 | 132.0 | 0.9296 | 21.63° | +7.75 (**1.61%**) |
| `brow_inner_bottom` | 84.0 | 77.0 | 0.9167 | 23.56° | +3.50 (0.73%) |
| `eye_inner_corner` | 78.0 | 72.0 | 0.9231 | 22.62° | +3.75 (0.78%) |
| `brow_inner_top` | 78.0 | 70.0 | 0.8974 | 26.18° | +1.75 (0.36%) |
| `lid_line_inner_lower` | 72.0 | 66.0 | 0.9167 | 23.56° | +3.00 (0.62%) |
| `lid_line_inner_upper` | 62.0 | 57.0 | 0.9194 | 23.17° | +2.75 (0.57%) |
| `mouth_line_end` | 52.0 | 48.0 | 0.9231 | 22.62° | +2.50 (0.52%) |
| `nose_mark_corner` | 30.0 | 30.0 | 1.0000 | 0.00° | +3.75 (0.78%) |

The misfit is the three-quarter spacing minus the front spacing times 0.8750, in units and as a share of head height. "Covered" means one of the pair's four points lies under another shape.

**The spread.** The pairs give turns from 0° to 32.74°, with a median of 27.15°. The ten pairs wider than 150 units give 27.10° to 32.74°, and 27.10° to 29.46° without the far ear's covered base. The eight narrower pairs all give less, 0° to 26.18°: the features near the centre line stand further apart in the three-quarter view than one head at this turn would show them, by 1.75 to 7.75 units. A narrow pair moves the angle a long way for a small distance, so the misfit column is the fairer measure of the spread: 16 of the 18 pairs are within 0.8% of head height.

**How firm the figure is.** Leaving out the three pairs with a covered point gives 27.85° over 15 pairs. Between the two figures, the spacing one head would show changes by 2.1 units for the outer eye corners and 3.1 units for the ears' outer bases, which is 0.4% and 0.7% of head height. The turn is known to about a degree.

Five pairs are not counted, each for what the landmark is and not for its result: the pupil's top and bottom and the lid line's upper middle share a sideways position with a pair already counted, the opening's centre is the mean of the two corners, and the neck is a column whose width does not narrow when it turns. The reasons are in `[turn.not_counted]`.

## Where the two views do not describe one head

Each line is a share of the front view's head height. The full list, with the 20 smaller level differences, is in `views-compared.toml`.

**Above 1%:**

| What | Front | Three-quarter | Difference | Share |
|---|---|---|---|---|
| Spacing of the shoulder points | 744.0 | 742.0, where a turn of 28.96° gives 651.0 | 91.0 units too wide | **18.95%** |
| Level of `neck_right_base`, where the neck ends on the near side | y = 590.0 | y = 560.0 | 30.0 higher | **6.25%** |
| Level of `ear_left_base_inner` (far ear, covered in the three-quarter view) | y = 170.0 | y = 150.0 | 20.0 higher | **4.16%** |
| Level of `ear_left_base_outer` (far ear, covered in the three-quarter view) | y = 172.0 | y = 160.0 | 12.0 higher | **2.50%** |
| Spacing of `ear_base_outer` (the far one covered) | 340.0 | 286.0, where the turn gives 297.5 | 11.5 units too narrow | **2.39%** |
| Spacing of `lid_line_lower_mid`, the crests of the two lid lines | 142.0 | 132.0, where the turn gives 124.2 | 7.75 units too wide | **1.61%** |
| Level of `neck_left_base`, where the neck ends on the far side | y = 590.0 | y = 596.0 | 6.0 lower | **1.25%** |
| Level of `brow_left_outer_bottom`, the far brow's outer lower corner | y = 329.0 | y = 324.0 | 5.0 higher | **1.04%** |
| The skull's top against the top of the centre line | on it | 5.0 units toward the viewer's right of it | 5.0 | **1.04%** |

**The shoulders.** By their own spacing the shoulders are turned 4.2°, against the head's 28.96°. They are level in both views (y = 744.0). The neck ends at one height in the front view and at two heights 36 units apart in the three-quarter view. The shoulder line of a figure that turns whole will differ from the hand-made one by these amounts. Jon ruled at look 1 that the shoulder line turns with the head: the hand-made shoulders are the flaw.

**Levels.** Of 78 landmarks recorded in both views and comparable in height, 53 are at exactly the same height and 25 are not. The three level lines and the chin are identical. Five differences are above 1% and are in the table. The largest of the rest are the top of the near ear (0.85%) and four points at 0.83%: the far brow's outer top corner, the far ear's upper node, the near ear's outer base and the outer tip of the far lid line. The hair's top is 1.08 units higher in the three-quarter view (0.23%), which is why the two head heights differ.

**The centre line.** In the front view the skull's top, the nose mark, the mouth's centre and the chin all stand on the centre line. In the three-quarter view the nose mark's top is 1.3 units off it, the mouth's centre 2.0 and the chin 1.0, all under 0.5%. The skull's top is 5.0 units off, in the table above.

**The front view is symmetric.** Of its 50 left and right pairs, 49 mirror each other exactly about x = 500. The one that does not is the hair's side extents, by 0.002 unit sideways and 0.513 in height.

## The final drawings' ears and lids

**Jon's ruling, 2026-11-16.** Saffi's comparison case takes the larger ears and the hooded upper lids of her final drawings in mudsong, not those of the grey S2 underdrawing. Everything else stays as recorded from S2. Each view's ears and lids come from its own drawing, so nothing is worked out through the turn.

**What is taken.** `final-drawings.toml` holds, for each view, 36 landmarks under the names `hand-made-views.toml` gives them, and each replaces the landmark of the same name in the same view:

- **Each ear whole:** the `back` shape's two base points, its top, its two side extents and its two other nodes.
- **Each eye's lids:** the opening they leave, which is the `white` shape (its two corners, its top, its bottom and its centre), and the lid line. The S2 underdrawings draw the lid line as the `lid_upper` band. In the final drawings that shape is a blink lid with no opacity, and the dark band along the upper lid is `lashes`: its inner and outer ends are nodes, and its crest is midway between the highest points of its lower and its upper edge.

**What is not.** The iris and the pupil stay S2's (Jon left this to the agent, 2026-11-16). The final front view sets its pupils 3 units further out on each side than the S2 front view, and the final three-quarter view keeps the S2 pupils; taking the front view's would make the two final views disagree by about 1% of head height. So an S2 iris stands 3 to 4.4 units above the top of a final opening: the hooded lid covers that much of it. The skull, the jaw, the nose, the mouth, the brows, the neck, the shoulders and the hair's extents are S2's too.

**What changed,** as `[views.….moved_from_s2]` gives it for every landmark:

- **The ears are larger and stand differently on the head.** In the front view and on the near ear of the three-quarter view the outer base point is 15 to 16 units lower than in S2 and the inner base point as much higher; on the far ear it is 7.6. The base is tilted about its middle, which has not moved. The outline reaches 12 to 18 units further out, and its top stands 21 to 26 units further out and within 7 units of its S2 height.
- **The upper lids are hooded.** The top of each opening is 3.8 units lower in the front view and on the near eye, and 5.2 lower on the far eye, and it has moved toward the outer corner by 13 to 23 units. The lid line's crest is 11 to 13 units lower.
- **The eye line is lower,** because it is midway between the top and the bottom of the opening: y = 371.893 in the front view and 372.281 in the three-quarter view, where S2 gives 370.000 and 370.027. The unit, eye line to chin, is 195.107 and 194.719 where S2 gives 197.000 and 196.973. Every proportion is divided by the unit, so every value in `proportions.toml` changed, while the positions those values draw changed only for the ears, the lids and what is built on the eye line.

**Where the two final views do not agree.** `final-drawings.toml` lists it under `[levels]` and `[[spacing.pair]]`, as `views-compared.toml` does for S2. It is reported and not corrected.

| What | Front | Three-quarter | Difference | Share |
|---|---|---|---|---|
| Level of `ear_left_base_outer` (far ear, covered in the three-quarter view) | y = 186.989 | y = 167.565 | 19.424 higher | **4.04%** |
| Level of `ear_left_base_inner` (far ear, covered in the three-quarter view) | y = 155.011 | y = 142.435 | 12.576 higher | **2.62%** |
| Level of `ear_left_upper_node` (far ear) | y = 78.827 | y = 72.393 | 6.434 higher | **1.34%** |
| Level of `ear_right_base_outer` (near ear) | y = 186.989 | y = 191.834 | 4.845 lower | **1.01%** |

Of 30 landmarks comparable in height, 14 are at exactly one height in both views and 16 are not; the 12 others differ by under 1%. The near eye is the same shape in both final drawings. The spacing of the outer eye corners narrows from 232.0 to 203.0, which is a turn of 28.96°, and the outer tips of the lid lines give the same; the ears' outer bases give 33.45°, as the far one is covered. The pinned turn of 29° stands.

## Her set of proportions

`proportions.toml` holds one value for each proportion of [`proportions/list.toml`](../../proportions/list.toml), and the two amounts of the named correction `lower_face_set_back`, which "Her side view" below explains. It was not typed in by hand. The program wrote it from the two records:

```sh
cargo run -- proportions-from-hand-made-views --record data/saffi/hand-made-views.toml --final-record data/saffi/final-drawings.toml --lower-face-set-back 0.5 --out data/saffi/proportions.toml
```

`cargo test` checks that the committed file is what that command writes, and checks twenty-one of its numbers against arithmetic done by hand on the positions the two records give. It also checks that the command without `--final-record` still gives her S2 proportions, and the correction's two amounts against arithmetic done by hand at three shares. None of these checks says her armature looks right.

**How it was taken.** The head model has two directions: from proportions to landmarks, which the generator uses, and from landmark positions in a front view and a three-quarter view back to proportions. The second was run on her two views, with the three-quarter view taken as turned 29°. Every proportion is divided by the unit, eye line to chin: 195.107 units in her front view and 194.719 in her three-quarter view. Depth is taken through the turn, from where a pair's midpoint or a point of the centre line stands in the three-quarter view.

**The rules, where her views disagree or one does not show a point.** The file's `[source]` records them in full, as the list of proportions states them.

1. A height is the three-quarter view's, and for a left and right pair the mean of its two heights there. The record gives that view more standing: Jon accepted it.
2. An offset from the centre line is the three-quarter view's, when it shows both points of the pair: half their spacing over the cosine of the turn.
3. A depth is the three-quarter view's.
4. What the three-quarter view does not show comes from the front view: the far ear's base and inner extent, which the head covers there, and the half-widths of the face outline and the hair.
5. The neck and the shoulder line take their widths from the front view.

`[source]` also records how the record's positions were read as the landmarks the list asks for: a lid line and a brow line as the middle of the drawn band, and the face outline's crossings with the drawn level lines as its crossings with the eye line and the nose and mouth levels.

**What follows from them.** Each is arithmetic on the positions above.

- Her eye line is at y = 372.281 in the three-quarter view, 6.3 units below the level line drawn in both views (1.3% of head height). With the S2 lids it was at 370.027, 4 units below.
- A head drawn from this set shows its shoulder points 649.4 units apart in the three-quarter view, where hers are 742.0 apart: 46.3 units a side (9.6%). Hers hardly turn. Jon ruled at look 1 (2026-11-16) that the shoulder line turns with the head, so this difference is a flaw of the hand-made drawing, not of the generator.
- The neck ends at y = 578.0, the mean of 560.0 and 596.0. That is 18 units from each in her three-quarter view (3.7%) and 12 from her front view's 590.0 (2.5%).
- The ear's outer extent stands at y = 112.5, the mean of 126.1 on the near ear and 98.9 on the far one: 13.6 units from each (2.8%), and 16.0 from the front view's 127.6 (3.3%). Her two ears in the three-quarter view are not one ear seen from two sides, in her final drawings as in her S2 underdrawings.
- Where a pair's spacing in her two views does not fit the turn, the three-quarter view is kept and the front view moves: the lid lines' crests by 2.5 units a side (0.5%), the inner eye corners by 2.4 (0.5%).

The list of every difference between a head drawn from this set and her hand-made views belongs to the armature review sheet, not to this file. The program writes it from this folder's records and the generator's landmark files:

```sh
cargo run -- draw --proportions data/saffi/proportions.toml --out target/saffi
cargo run -- differences-from-hand-made-views --front target/saffi/front.landmarks.toml --three-quarter target/saffi/three-quarter.landmarks.toml --record data/saffi/hand-made-views.toml --final-record data/saffi/final-drawings.toml --out target/saffi/differences.toml
```

The file it writes states how each generated view is laid over its hand-made view: the unit, the eye line and the upright axis the head turns about are made to coincide, by one scale and one shift for the whole view. It is not committed, because it changes whenever the generator does and nothing in it is accepted.

## Her side view

She has no hand-made side view. The generator draws one from this set, and nothing in this folder says anything about it: Jon's look is its only check.

**Look 2 (2026-11-17): not accepted.** Jon found the face pushed too far forward from the nose down. Every depth of hers is her three-quarter view's sideways shift over the sine of 29°, which is 0.48. That view puts the mouth line's centre 1 unit behind the nose mark's bottom and the chin 15 behind it, which as a rigid head is a mouth at depth 62.9 and a chin at 33.2 where the nose mark is at 65.0 and the forehead at 58.3, in units of a head 494.5 high. A side view drawn flat from that has a muzzle. Her three-quarter view is accepted as it is, so her depths stay as measured.

**The correction.** Her set carries `lower_face_set_back`, which the list of proportions defines: in the side view only, the chin and the mouth are drawn further back than their depths. No drawing lesson gives a distance, so the amounts come from two rules of the lessons in `docs/research/2026-11-16-views-and-measuring/notes/head_construction_in_three_views.md`, section 1:

- **The chin** is drawn a share of the way back to straight below the iris's centre, at depth -8.5. One lesson (B) puts the chin's tip straight below the pupil; her pupil's centre is 0.4 unit from her iris's.
- **The mouth line's centre** is then drawn on the straight line from the nose mark's bottom to the chin as drawn. Four lessons (A, C, F, H) put the lips on that line or just behind it; one (B) lets them pass it slightly.

| Share | Chin set back | Chin drawn at | Mouth set back | Mouth drawn at |
|---|---|---|---|---|
| 0, as at look 2 | 0 | 33.2 | 0 | 62.9 |
| 0.5, in `proportions.toml`, accepted at look 3 | 20.9 (4.2% of head height) | 12.3 | 18.2 (3.7%) | 44.7 |
| 1 | 41.7 (8.4%) | -8.5 | 26.2 (5.3%) | 36.7 |

**Look 3 (2026-11-17): accepted.** Jon was shown the share of 0.5 as candidate A beside the share of 1 as candidate B, and chose A. No rule gives the share: his look did. `design/look-3-saffi-side-view/` holds the sheet he saw.

That accepts her armature in three views: the front and three-quarter views, which he had said at look 2 read as one head, and the side view with the correction at the amounts of the 0.5 row. The six files are in [`armatures/saffi/`](../../armatures/saffi/README.md), and `cargo test` checks that `maquette draw` still writes them byte for byte from `proportions.toml`.

It accepts nothing else. Candidate B is not accepted. Her proportions are not made a standard, or good ones for another character. No table is accepted, and her numbers still never enter one.

## What this is evidence of

It shows what the four drawings' coordinates say, and nothing more.

- It does not say the armature looks right. Jon's eyes decide that.
- It does not say which view is the correct one where they differ. A misfit in spacing belongs to the pair, not to one eye or one ear. The record gives the three-quarter view more standing: Jon accepted it, and the front view was on the same sheet without being the question put to him (`docs/foundation/01-mission.md`).
- The turn assumes a flat drawing of a level head turning about an upright axis. If the drawing was made with another idea of a three-quarter view, the figure describes the drawing and not that idea.
- A landmark of kind `node` is where the drawer put a path node. That a node is the same point of the head in both views is an assumption, firm for an eye corner and loose for the base of an ear.

## How it was checked

Three checks, none of which shares code with the derivation:

1. **A second walk of the drawings.** An `awk` script reads each SVG as text, steps along every path 2000 times a segment, and checks every recorded landmark of that view against the walk: a node must be a node of the stated element, a top or an extent must equal the extreme the walk finds, a crossing must lie on the outline and meet what it claims to cross, a centre must be the mean its `how` names. All 118 and all 122 agree. With three positions changed on purpose it reported them.
2. **A recomputation from the committed numbers alone.** A separate Rust program parses both TOML files and recomputes the turn, every misfit, the level differences, the centre-line offsets and the shoulders from `hand-made-views.toml`. All agree. So every number in `views-compared.toml` follows from `hand-made-views.toml` alone, with no private input. The program is kept with the private copies and is not committed, so nothing in a clean clone runs this check.
3. **By hand.** The hair's top, an ear's top, the nose mark's top, an iris's outer extent, and the face outline's widest point in the front view were worked from the raw attributes with `bc`, as were the pupil and outer-corner ratios. Left and right were confirmed as the character's own: in both views every `left` landmark is toward the viewer's right of its `right` twin, and in the front view they sit on either side of x = 500.

The `awk` walk and the `bc` sums were one-off checks run outside the repository on 2026-11-16, on the S2 underdrawings: nothing that is not Rust is committed.

Since 2026-11-17 the checks that can be repeated are in the package:

4. **The move of the derivation.** With the drawings present, `maquette record-hand-made-drawings` writes `hand-made-views.toml` and `views-compared.toml` byte for byte as the throwaway program wrote them on 2026-11-16, so the command is that program and the S2 numbers did not change in the move.
5. **The final record against the S2 record,** in `cargo test`, with no private input: every landmark of `final-drawings.toml` replaces one the S2 record has; each `moved_from_s2` is the one position minus the other; each view's skull circle and chin are the S2 record's; every level difference, share, spacing and turn of the final record follows from its own positions; and only ears and lids are in it.
6. **Her set of proportions by hand,** in `cargo test`: twenty-one of its numbers are worked from the positions the two records give, by arithmetic the test states.

The final drawings' landmarks have had no second walk like check 1. What stands behind them is the reading in `src/hand_made_drawings.rs`, which states for each landmark the node or the edge it is, the checks the command makes while it reads (each corner of an opening is the opening's sideways extreme, each lid band has eight nodes and its ends lie the right way round, every `left` landmark is toward the viewer's right of its `right` twin), and checks 5 and 6.

## Rerunning it

The derivation needs the four drawings, and a clean clone does not have them. Without them these files stand as recorded numbers under the digests above; nothing else in the repository depends on them.

Where the drawings are, the command reads each from its folder under the name in "Her drawings in mudsong", and refuses one whose digest differs:

```sh
cargo run -- record-hand-made-drawings --underdrawings <the folder of the S2 underdrawings> --final-drawings <the folder of the final drawings> --out data/saffi
cargo run -- proportions-from-hand-made-views --record data/saffi/hand-made-views.toml --final-record data/saffi/final-drawings.toml --lower-face-set-back 0.5 --out data/saffi/proportions.toml
```

Mudsong's study folder holds all four. The private copies of the S2 underdrawings are in the reference folder's `known-positions/`. With both folders named in the environment, a test checks that the three committed records are what the command writes:

```sh
MAQUETTE_UNDERDRAWINGS=<folder> MAQUETTE_FINAL_DRAWINGS=<folder> cargo test --test hand_made_drawings -- --ignored
```

If the drawings are ever lost, these numbers can no longer be rederived, and this file must say so.
