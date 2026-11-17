# Look 5: the first table, drawn

The armature review sheet for the fifth look of #3, made for #10. It started from the sheet of look 3 in `../look-3-saffi-side-view/`. The sheet accepts nothing itself, and Jon has not looked at it yet.

- `armature-review-sheet.figcraft`: the FigCraft source.
- `armature-review-sheet.png`: its export, 2400 pixels wide.
- `armature-review-sheet-full-size.png`: the same at full size, 4000 pixels wide, for the small text.
- `saffi-with-table-values.toml`: the set of proportions the head on the sheet is drawn from.

## The decision

One decision: is the table in `tables/adult-women/table.toml` accepted as measured? And one question that binds nothing: does a better Saffi lie inside the ranges?

Jon decided on 2026-11-17 what is drawn for this look: Saffi's own head with the table's values put in where the references give them, with a sheet that says so.

## The head

`saffi-with-table-values.toml` is what `maquette set-from-table` wrote from the record of Jon's check and `data/saffi/proportions.toml`, copied here unchanged. It says of each of its 73 proportions where the number comes from:

| Source | Proportions | What it is |
|---|---|---|
| The table's middle value | 27 | The median of the five or six works that give it. It has a valid range |
| A thin average | 17 | The average of the two to four works that give it. It has no range |
| Borrowed from Saffi | 29 | Her own number, as it stands: no work gives it, or one work alone, or it is a convention |

- **A thin value is the average and not the middle.** The middle of three works is one work's own number, and all eight depths that three works give had their middle in the same work: drawn, that would have published one character's side view.
- **A proportion that one work gives is borrowed,** for the same reason.
- **The ear is taken whole.** Two works give the ear's outer extent, and none gives its outer base point or the offset of its top. An ear cannot be built from the batch, and the batch's ears are a human's at the side of the head where Saffi's stand on top of it, so all twelve ear proportions are hers. Without this rule the head would have had her ear with its outer extent below the eye line.
- **No correction.** The set carries none. Its chin and mouth stand where three side views put them on average, 15.8 and 42.7 units in front of the eyes at the default placement; Saffi's side view, with her correction, draws hers at 12.3 and 44.7. Run on this set's own depths, the rule of `lower_face_set_back` would set the chin back by 2.3 units at a share of 1, which is 0.5% of head height, and the mouth by nothing. That is not carried.

It is one head to look at. It is no table and no part of one, and nothing reads it but `maquette draw`, `maquette overlay-sources` and the tests.

## What the sheet shows

- **Top row:** the drawn head in its front, three-quarter and side views, on shared level lines. Beside them, the table in short.
- **Second row:** Saffi's accepted armature in the same three views, the files of `armatures/saffi/`, under the drawn head's level lines. Beside them, her 27 proportions that have a range against it, with the 8 outside marked and how far, and a line that her other 46 have no range to be set against.
- **Third row:** the drawn head again, with a dot on every point a guide passes through, coloured by the least supported number that places the point in that view: green for the table, yellow for a thin average, orange for borrowed. Beside them, the legend, the count of proportions and of dots of each kind, and which proportions are thin and which borrowed.
- **Fourth row:** what the drawn head claims and what it does not; the decision asked, why no check can make it, what it covers and when it stops; and what the agent's own look found, which accepts nothing.
- **Foot:** the works the table was measured on, by work and character designer, as the table lists them.

The sheet says it is look 5 of 8, and that looks 1, 3 and 4 were accepted and look 2 was not. It shows generated guides only: no reference drawing is on it.

## What the agent's look found

A diagnostic, which accepts nothing. Nothing is broken: each outline is one curve with no loop, and every guide is there under its name.

- **The side view has a pointed nose.** Its front runs out to a point 51 units in front of the eye line's, 10% of head height, where Saffi's runs 21. In a profile the reading's `nose-mark-bottom` is under the tip of the nose, which is another point than the front view's nose mark, and the outline is drawn through it. In the three-quarter view the centre line swings out to it and back.
- **The chin is a narrow wedge.** The six works put the jaw point almost on the chin, 14 units out and 2 up, where Saffi's is 34 out and 11 up. The dry run found that point poorly defined, and Saffi is outside the range of both of its proportions. Its depth is borrowed from her and stands 24 units in front of the averaged chin, so in the three-quarter view the flat of the chin lies to one side of the centre line.
- **The brows stand high and slope down outward, the mouth is wide with its ends above its centre, and the neck is thin.** Each is a middle or an average taken by itself.

## What it is made from

Every drawing on the sheet is a file the program wrote, imported into FigCraft unchanged and placed at scale 1. Each side view is seen through a window 680 units wide, which shows the drawing from x = 195 to 875.

| On the sheet | Files | Drawn from |
|---|---|---|
| The drawn head, top and third rows | `target/look-5/front.svg`, `three-quarter.svg`, `side.svg` | `saffi-with-table-values.toml` |
| The dots of the third row | `target/look-5/front.sources-overlay.svg`, `three-quarter.sources-overlay.svg`, `side.sources-overlay.svg` | The same set, and the three landmark files |
| Saffi's accepted armature | `armatures/saffi/front.svg`, `three-quarter.svg`, `side.svg` | `data/saffi/proportions.toml` |
| Saffi against the ranges | What `maquette set-against-table --proportions data/saffi/proportions.toml --table tables/adult-women/table.toml` writes | The table |

Their SHA-256 digests on 2026-11-17:

```
8fcb5b4671617d873977b98aebbc28aff73de3c3fd7febc9db3e5e2fca1e7146  saffi-with-table-values.toml
58b202c7c7cc6e3e404eccce818cc50b9d068543c4964d47d72075b651b1a56e  tables/adult-women/table.toml
bc003fcf8ca3c06b8adef031fc6cd2172890e58b529445726ff9dc393d099ff7  front.svg
be206a667f58b8bad278f2763a340f57f8b28796b91addca5df2844cacfa141c  front.landmarks.toml
edc43b32489e65e1f4fb92da468a8d977d943640fe144873fd9a5f7ab969e31d  three-quarter.svg
05b6c49fda2a109339709e8bfe9800c5f3ae5817157949688d68bed9efd72710  three-quarter.landmarks.toml
9c2ad9bf0559441f745eff57a3a83b642370469dc1d4ab9e6339369ed81954d1  side.svg
b92b3b041812cad3f65b7decc6299defa5e79edb62fd6f023efb78532262ed0f  side.landmarks.toml
3c47be83f2ad31497f2b610452e03043312cd63bd5f38dd86a14443680a3c463  front.sources-overlay.svg
b7c18d3e4a330e8a62faabb5ac061611490df6f4492c3a1cc3a03b8b106b092d  three-quarter.sources-overlay.svg
339143b6fc9f4f811605ad59170116e77520315f6ce845d576053bc6ebad7b94  side.sources-overlay.svg
```

**The checks the drawn head passed,** the ones Saffi's armature passes:

- `cargo test` (`tests/first_table.rs`): every landmark is at one height in all three views, and stands sideways in the three-quarter and side views where its front position and its depth put it.
- The read-back in VectorCraft 0.24.1 on 2026-11-17, for each view: no import warning, every name once, every anchor within 0.001 unit, every guide visible, stroked and unfilled.
- The hard case of the SVG contract on the three-quarter view: all 10 lines held.

## Composing it again

1. Write the files, from the repository's root:
   - `maquette draw --proportions design/look-5-first-table/saffi-with-table-values.toml --out target/look-5`
   - `maquette overlay-sources --proportions design/look-5-first-table/saffi-with-table-values.toml --armature target/look-5 --out target/look-5`
   - `maquette read-back` on each of the three landmark files, and `maquette read-back-hard-case` on the three-quarter view, before a sheet goes to Jon.
2. In FigCraft, open `armature-review-sheet.figcraft`. The sheet frame is at 0, 0, and each imported drawing is a frame named after its file.
   - Top row: the front view at 80, 260 and the three-quarter view at 1100, 260. The side view sits in a window frame at 2120, 260, 680 by 940, that clips it, with the drawing inside at -195, 0.
   - Second row: the same places 1120 further down, with Saffi's three files.
   - Third row: the same places 2240 further down, with the drawn head's files again and each view's overlay over it at the same place.
   Replace a drawing by importing the new file (`file.import`), moving it into the sheet frame or its window (`node.reparent`) and setting its place.
3. If a height in a landmark file changed, move the level lines of each row to that row's top plus the height. If a number changed, retype the captions, the ranges and the counts.
4. Export the sheet frame at 2400 pixels wide and at scale 1.

FigCraft's server writes only under the main checkout's `design/`. Composed from another worktree, the folder is made there and moved here afterwards.

`saffi-with-table-values.toml` itself cannot be written again from a clean clone: `maquette set-from-table` reads the readings Jon accepted, which are in the reference folder.
