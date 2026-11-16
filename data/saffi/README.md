# Saffi's hand-made views as numbers

Saffi is the comparison case: the one character whose accepted armature is kept out of every proportion table, so that what a table allows can be tested against her. This folder holds her hand-made front view and three-quarter view as landmark positions, and what the two views say when they are set against each other.

**These numbers never enter a proportion table.** They are also not her set of proportions: the list of proportions is fixed by a later ticket, and her proportions are worked out from these positions then.

| File | What it holds |
|---|---|
| [`hand-made-views.toml`](hand-made-views.toml) | 118 landmark positions in the front view and 122 in the three-quarter view, each with the element it came from and how it was taken. Also what the views give for each guide, and what was left out. |
| [`views-compared.toml`](views-compared.toml) | The turn of the three-quarter view, worked out pair by pair, and every place where the two views do not describe one head, as a share of head height. |

No image, tracing, crop, path data or copy of the drawings is here. The repository is public.

## Where the numbers came from

- **The drawings.** Two grey underdrawings of Saffi's bust, made by hand in mudsong's portrait study on 2026-11-15: a front view and a three-quarter view. She is Jon's own character.
- **Their identity.** They sit in mudsong's study folder, which Git ignores there, so no commit names them. The SHA-256 digest of each SVG file is its only identity:
  - front: `34ee7b92a8422031a20f7d2f195d6952d16e1099de3002f5fffcb6697a57f76f`
  - three-quarter: `c50bb0529f254fe12ac0af7b3f6175ae175df14b61d24f16bfec54afcc2efb02`
- **The read.** Jon approved one read of them on 2026-11-16 (`docs/foundation/02-first-table.md`, "The handoff"). They were read that day, and both digests matched before anything was used.
- **Custody.** Private copies are kept in maquette's reference folder, which is outside every repository, with a note of the source, the date, the digests and the approval. The three other drawings of the accuracy test of 2026-11-16 are kept there too, for later tests of an agent's reading on drawings with known positions. They were not measured.
- **The derivation.** A throwaway Rust program, kept beside the private copies and not in this repository, wrote both files. Its SHA-256 digest is `7e81dcdeeb6b5e1ef32f7531adde180e4864e047f89b1467677d517ea8bd51cd`. Nothing in either file was edited by hand.

Positions were taken by arithmetic on the files' own coordinates and never estimated from a rendered picture. The two renders were looked at once, only to tell which element is which.

## How to read the positions

- **Units.** SVG user units of the source files. Both are 1000 by 1000 with no transform on any element, so one unit is one pixel of their renders. x grows toward the viewer's right and y grows downward.
- **Left and right** are the character's own. Her right is toward the viewer's left in both views. In the three-quarter view her right is the near side.
- **Head height** is `chin.y` minus `hair_top.y`: 480.318 units in the front view and 481.400 in the three-quarter view. Every share in `views-compared.toml` is of the front view's figure, so 1% is 4.803 units.
- **Each landmark** has a `kind` that says how far it can be trusted across views. A `node` is a point the drawing's own path data gives exactly, and it marks a feature. A `highest` or `lowest` is the top or bottom of an outline, found where a curve's slope is zero. A `side_extent`, a `crossing` and an `outline_node` are points on a silhouette or a guide line; the other view shows a different point of the head there, so they are never compared between views. The file defines every kind.
- **`under`** lists the shapes painted over a point. A landmark with `under` is in the file's coordinates and cannot be seen in the drawing.

The accuracy test of 2026-11-16 used 479 units as the head's height, which is the chin minus the first node of the hair's outline (y = 88). The outline rises 1.3 units above that node, to y = 86.682. The difference is 0.3% of the yardstick and moves no figure here across the 1% line.

## What the views give for each guide

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

**The shoulders.** By their own spacing the shoulders are turned 4.2°, against the head's 28.96°. They are level in both views (y = 744.0). The neck ends at one height in the front view and at two heights 36 units apart in the three-quarter view. The shoulder line of a figure that turns whole will differ from the hand-made one by these amounts.

**Levels.** Of 78 landmarks recorded in both views and comparable in height, 53 are at exactly the same height and 25 are not. The three level lines and the chin are identical. Five differences are above 1% and are in the table. The largest of the rest are the top of the near ear (0.85%) and four points at 0.83%: the far brow's outer top corner, the far ear's upper node, the near ear's outer base and the outer tip of the far lid line. The hair's top is 1.08 units higher in the three-quarter view (0.23%), which is why the two head heights differ.

**The centre line.** In the front view the skull's top, the nose mark, the mouth's centre and the chin all stand on the centre line. In the three-quarter view the nose mark's top is 1.3 units off it, the mouth's centre 2.0 and the chin 1.0, all under 0.5%. The skull's top is 5.0 units off, in the table above.

**The front view is symmetric.** Of its 50 left and right pairs, 49 mirror each other exactly about x = 500. The one that does not is the hair's side extents, by 0.002 unit sideways and 0.513 in height.

## What this is evidence of

It shows what the two drawings' coordinates say, and nothing more.

- It does not say the armature looks right. Jon's eyes decide that.
- It does not say which view is the correct one where they differ. A misfit in spacing belongs to the pair, not to one eye or one ear. The record gives the three-quarter view more standing: Jon accepted it, and the front view was on the same sheet without being the question put to him (`docs/foundation/01-mission.md`).
- The turn assumes a flat drawing of a level head turning about an upright axis. If the drawing was made with another idea of a three-quarter view, the figure describes the drawing and not that idea.
- A landmark of kind `node` is where the drawer put a path node. That a node is the same point of the head in both views is an assumption, firm for an eye corner and loose for the base of an ear.

## How it was checked

Three checks, none of which shares code with the derivation:

1. **A second walk of the drawings.** An `awk` script reads each SVG as text, steps along every path 2000 times a segment, and checks every recorded landmark of that view against the walk: a node must be a node of the stated element, a top or an extent must equal the extreme the walk finds, a crossing must lie on the outline and meet what it claims to cross, a centre must be the mean its `how` names. All 118 and all 122 agree. With three positions changed on purpose it reported them.
2. **A recomputation from the committed numbers alone.** A separate Rust program parses both TOML files and recomputes the turn, every misfit, the level differences, the centre-line offsets and the shoulders from `hand-made-views.toml`. All agree. So every number in `views-compared.toml` follows from `hand-made-views.toml` alone, with no private input. The program is kept with the private copies and is not committed, so nothing in a clean clone runs this check.
3. **By hand.** The hair's top, an ear's top, the nose mark's top, an iris's outer extent, and the face outline's widest point in the front view were worked from the raw attributes with `bc`, as were the pupil and outer-corner ratios. Left and right were confirmed as the character's own: in both views every `left` landmark is toward the viewer's right of its `right` twin, and in the front view they sit on either side of x = 500.

The `awk` walk and the `bc` sums were one-off checks run outside the repository: nothing that is not Rust is committed, and the repeatable check belongs to the Rust package the next ticket brings.

## Rerunning it

The derivation needs the private copies, and a clean clone does not have them. Without them these files stand as recorded numbers under the digests above; nothing else in the repository depends on the private folder.

Where the reference folder exists, its `known-positions/README.md` gives the commands: compile `derivation/saffi_numbers.rs` with `rustc`, run it on the folder, and `diff` its two output files against the two files here. The program refuses to run if a drawing's digest differs, and says so plainly if the folder is absent.

If the private copies are ever lost, these numbers can no longer be rederived, and this file must say so.
