# Look 6: the first table, drawn again

The armature review sheet for the sixth look of #3, made for #10 after look 5 did not accept the table. It started from the sheet of look 5 in `../look-5-first-table/`, whose README says what Jon saw there and why he did not accept it. The sheet accepts nothing itself, and Jon has not looked at it yet.

- `armature-review-sheet.figcraft`: the FigCraft source.
- `armature-review-sheet.png`: its export, 2400 pixels wide.
- `armature-review-sheet-full-size.png`: the same at full size, 4000 pixels wide, for the small text.
- `saffi-with-table-values.toml`: the set of proportions the head on the sheet is drawn from.

## The decision

One decision: is the table in `tables/adult-women/table.toml` accepted as measured? And one question that binds nothing: does a better Saffi lie inside the ranges?

## What changed since look 5

**1. The tip of the nose has its own height.** The list of proportions has a new proportion, `nose_tip_drop`: how far below the eye line the tip of the nose is.

- A side view gives it: the height at which it places `nose-mark-bottom`, which in a profile is the point under the tip of the nose, in the side view's own unit. The three side readings of the batch already held it, so nothing was read again and the reading instructions did not change.
- The head model draws the tip as a landmark of its own, `nose-tip`, at that height and at the depth `nose_mark_bottom_depth`. The side view's face outline, the three-quarter view's centre line and the nose mark of both run through it. The front view's nose mark still runs through its own bottom, where the front view puts it.
- Where a set has no measured side view the tip stands at the nose mark's bottom, by rule, and the two are one point. Saffi's set is such a one: her six accepted files are the same bytes as before, and the test that pins them is untouched.
- In the drawn head the tip stands 64.6 units below the eye line, where look 5 drew it at 85.9. Its depth has not changed: 95.1 units in front of the eyes.

**2. The jaw point left the table.** Jon ruled after look 5 that it is left out until the landmark is redefined. The record of his check marks it as off on every front, the table was rebuilt, and the drawn head's jaw point is Saffi's. `tables/adult-women/README.md` says why.

**3. The neck is borrowed whole.** A neck is taken from one source, as an ear is: its width, its base and its depth are all Saffi's.

## The head

`saffi-with-table-values.toml` is what `maquette set-from-table` wrote from the record of Jon's check and `data/saffi/proportions.toml`, copied here unchanged. It says of each of its 74 proportions where the number comes from:

| Source | Look 6 | Look 5 | What it is |
|---|---|---|---|
| The table's middle value | 25 | 27 | The median of the five or six works that give it. It has a valid range |
| A thin average | 16 | 17 | The average of the two to four works that give it. It has no range |
| Borrowed from Saffi | 33 | 29 | Her own number, as it stands: no work gives it, or one work alone, or it is a convention, or it belongs to a form that is taken whole |

- **From the table to borrowed:** the jaw point's drop and half-width.
- **From thin to borrowed:** the neck's width and its base.
- **New, and thin:** the height of the tip of the nose, the average of three side views.
- **No correction.** The set carries none. Run on its own depths, the rule of `lower_face_set_back` would set the chin back by 2.3 units at a share of 1, which is 0.5% of head height, and the mouth by nothing.

**Saffi against the table:** inside 19 ranges, outside 6, and 49 of her proportions have no range to be set against. At look 5 she was outside 8. The two that left with the jaw point were her widest, by 2.1% and 1.2% of head height. The six that stay are all under 0.6%.

## What the sheet shows

- **Top band:** what changed since look 5 and why, and the counts now.
- **First row:** the drawn head in its front, three-quarter and side views, on shared level lines. Beside the side view, a panel kept empty for a face drawn over it.
- **Second row:** Saffi's accepted armature in the same three views, under the drawn head's level lines. Beside them, her 25 proportions that have a range against it, with the 6 outside marked and how far.
- **Third row:** the drawn head again, with a dot on every point a guide passes through, coloured by the least supported number that places the point in that view. Beside them, the legend and the counts.
- **Fourth row:** what the drawn head claims and what it does not; the decision asked, why no check can make it, what it covers and when it stops; and what the agent's own look found.
- **Fifth row and foot:** the table in short, and the works it was measured on, by work and character designer, as the table lists them.

The sheet says it is look 6 of 8, and that looks 1, 3 and 4 were accepted and looks 2 and 5 were not. It shows generated guides only.

## The panel kept for a face

The panel "A face over this side view" is empty on purpose. A face is drawn over the new side view after this sheet, as it was after look 5, and the two are put to Jon together.

- **Where it goes:** the panel's drawing area is at 2820, 580 in the sheet frame and 1000 units square, the size of the side view's whole drawing. An image of the face that covers the armature's 1000 by 1000 units, such as a 2000-pixel export at 2 pixels to the unit, is placed there at half size. It then stands at the size of the three views, and the level lines of the row, which run on through the panel, fall on it where they fall on the side view.
- **What to draw over:** `target/look-6/side.svg`, which `maquette draw --proportions design/look-6-first-table/saffi-with-table-values.toml --out target/look-6` writes. Its digest is below. In it the nose point of the profile is the landmark `nose-tip`, not `nose-mark-bottom`.

## What the agent's look found

A diagnostic, which accepts nothing. Nothing is broken: each outline is one curve with no loop, and every guide is there under its name.

- **The nose is shorter than at look 5.** Its tip stands where the three side views put it on average, in height and in depth. The guide round the tip is still a rounded wedge, because the outline is one smooth curve through the tip. Only the nose mark, which rises 9 units from the nostril to the tip, says that the nose turns up.
- **The chin is lopsided in the three-quarter view.** The jaw point is Saffi's, and its depth stands 24 units in front of the averaged chin, so the chin's round lies to the far side of the centre line, with a small notch on the near side. On her own head the jaw point stands 6 units in front of her chin.
- **The brows stand high and slope down outward, and the mouth is wide with its ends above its centre.** Each is a middle or an average taken by itself.

## What it is made from

Every drawing on the sheet is a file the program wrote, imported into FigCraft unchanged and placed at scale 1. Each side view is seen through a window 680 units wide, which shows the drawing from x = 195 to 875.

| On the sheet | Files | Drawn from |
|---|---|---|
| The drawn head, first and third rows | `target/look-6/front.svg`, `three-quarter.svg`, `side.svg` | `saffi-with-table-values.toml` |
| The dots of the third row | `target/look-6/front.sources-overlay.svg`, `three-quarter.sources-overlay.svg`, `side.sources-overlay.svg` | The same set, and the three landmark files |
| Saffi's accepted armature | `armatures/saffi/front.svg`, `three-quarter.svg`, `side.svg` | `data/saffi/proportions.toml` |
| Saffi against the ranges | What `maquette set-against-table --proportions data/saffi/proportions.toml --table tables/adult-women/table.toml` writes | The table |

Their SHA-256 digests on 2026-11-17:

```
ba63b73a9591939c3027da1fb7894a94c848e43773deaef94b1244cf2fa56f7c  saffi-with-table-values.toml
395e3d0603118d2155947c6605471a2a04ca8c00bdd1c44e843b18048d2a5ee4  tables/adult-women/table.toml
7bf7da29d057bae86b78161ab6adeb6c4dddda2b38ae12228e30067a64818d96  front.svg
60590881843de65dbc80d1026cc0f537858b3c34c458ce723a815c99a764cbfe  front.landmarks.toml
eb427c944221d0702c314e47584abca420c1ed781f64331c3301cf8822a35d0d  three-quarter.svg
be213a524455b724d9aaa115160fcd9048652b15df0fb6142814c27e3ce4ffb0  three-quarter.landmarks.toml
04f605850119051ecf0b25f515597c230944c352bb1df55e0a0eae3b15464076  side.svg
37894aa9b585bfe98c178b006594d53e8e8f833fca2de69346eb441e43584fb6  side.landmarks.toml
7582a76c178195c761f2b48e0f92ef340b7541c596d449976cee20ef54771204  front.sources-overlay.svg
410df802a95b54e4f4f2e2d499ea4a26214aed182926be88a670ec2942498d61  three-quarter.sources-overlay.svg
b649da5927cddcefa88028ce84e11523b9d9664063a24bc6cffc5c5da523629c  side.sources-overlay.svg
```

**The checks the drawn head passed,** the ones Saffi's armature passes:

- `cargo test` (`tests/first_table.rs`): every landmark is at one height in all three views, the tip of the nose among them, and stands sideways in the three-quarter and side views where its front position and its depth put it.
- The read-back in VectorCraft 0.24.1 on 2026-11-17, for each view: no import warning, every name once, every anchor within 0.001 unit, every guide visible, stroked and unfilled.
- The hard case of the SVG contract on the three-quarter view: all 10 lines held.

## Composing it again

1. Write the files, from the repository's root:
   - `maquette draw --proportions design/look-6-first-table/saffi-with-table-values.toml --out target/look-6`
   - `maquette overlay-sources --proportions design/look-6-first-table/saffi-with-table-values.toml --armature target/look-6 --out target/look-6`
   - `maquette read-back` on each of the three landmark files, and `maquette read-back-hard-case` on the three-quarter view, before a sheet goes to Jon.
2. In FigCraft, open `armature-review-sheet.figcraft`. The sheet frame is at 0, 0, and each imported drawing is a frame named after its file.
   - First row: the front view at 80, 580 and the three-quarter view at 1100, 580. The side view sits in a window frame at 2120, 580, 680 by 940, that clips it, with the drawing inside at -195, 0.
   - Second row: the same places 1120 further down, with Saffi's three files.
   - Third row: the same places 2240 further down, with the drawn head's files again and each view's overlay over it at the same place.
   Replace a drawing by importing the new file (`file.import`), moving it into the sheet frame or its window (`node.reparent`) and setting its place.
3. If a height in a landmark file changed, move the level lines of each row to that row's top plus the height. If a number changed, retype the captions, the ranges and the counts.
4. Export the sheet frame at 2400 pixels wide and at scale 1.

FigCraft's server writes only under the main checkout's `design/`. Composed from another worktree, the folder is made there and moved here afterwards.

`saffi-with-table-values.toml` itself cannot be written again from a clean clone: `maquette set-from-table` reads the readings Jon accepted, which are in the reference folder.
