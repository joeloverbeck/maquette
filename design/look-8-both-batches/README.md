# Look 8: the table of both batches, drawn

The sheets for the eighth and last look under the ceiling of eight, made for #18 of spec #3. The armature review sheet started from look 6's, in `../look-6-first-table/`, whose README says how its layout is built; the face study started from look 6's face. Nothing on either sheet is accepted.

- `armature-review-sheet.figcraft`: the FigCraft source of the review sheet.
- `armature-review-sheet.png`: its export, 2400 pixels wide; `armature-review-sheet-full-size.png`, 4000 wide, for the small text.
- `head-from-the-table.toml`: the set of proportions the head on the sheet is drawn from, as `maquette set-from-table` wrote it.
- `side-face-study/`: the face drawn over the head's side view, and look 6's face beside it.

## The decision

One decision: is the table in `tables/women-and-teenage-girls/table.toml` accepted as measured, with what it counts? And one question that binds nothing: does the face over the side view read better than at look 6?

## What changed since look 6

1. **The references.** The table is built from both batches: the six characters Jon accepted at look 4 and the 14 of the second batch he accepted at look 7, 20 characters of 12 works, 14 adult and 6 teenage. A side view of 9 gives depth, where 3 did.
2. **The profile.** A side view is read since revision 2026-11-17.4 for the forehead, the bridge of the nose, the angle of the jaw and the neck's top ends. The generator draws them where a set gives them: the outline runs through the forehead and the bridge, and under the chin the jaw's underside runs back through the top of the neck's front, where the neck's front line starts, before it turns up toward the ear. At look 6 the jaw ran from the chin to the ear as one diagonal, on Saffi's neck.
3. **Counting.** The table counts characters, one drawing of each in each view, and says beside each count how many works stand behind it and how many come from one work. Counted by works, 43 of its 56 ranges would stand.

## The head

`head-from-the-table.toml` takes 48 proportions from the table's middles, 6 thin averages of two to four characters, and 26 borrowed from Saffi: the skull, the ears, taken whole, the jaw point, placed from the head's own chin, the top end of the face outline, two depths of the cheek, and the shoulders. The angle of the jaw and the four proportions of the neck's back are left out: one character each gives them, and Saffi's set has none. So the jaw turns up after the neck's front by the construction's curve, and the neck's back line is the column of its width at the depth two characters give.

## What the agent's look found, before the look

A diagnostic, which accepts nothing.

- **A fault, repaired before the sheet was made.** The side view's outline sagged below the chin on its way back to the neck, by up to 11 units: a reader would have put the chin there, and not at the landmark. The chin is the outline's lowest point by definition, so the outline now runs level there and on through the top of the neck's front (commits `e1e056a` and `9b1f210`). Saffi's armature is the same bytes.
- **The jaw reads as a jaw.** Its underside runs back almost level from the chin to the top of the neck's front, 80 units back, and turns up toward the ear after it.
- **The nose stands out:** its tip is 40 units in front of the bridge.
- **The neck is deep from front to back,** 170 to 200 units against 135 wide, because its back line is the column at a depth two characters give.
- **The ears are Saffi's cat ears,** as at look 6. The face study draws a human ear where the jaw line is.

## What it is made from

| On the sheet | Files | Drawn from |
|---|---|---|
| The drawn head, first and third rows | `target/look-8/front.svg`, `three-quarter.svg`, `side.svg` | `head-from-the-table.toml` |
| The dots of the third row | `target/look-8/*.sources-overlay.svg` | The same set, and the three landmark files |
| The face over the side view | `side-face-study/side-face.png` | Drawn over `target/look-8/side.svg` |
| Saffi's accepted armature | `armatures/saffi/front.svg`, `three-quarter.svg`, `side.svg` | `data/saffi/proportions.toml` |
| Saffi against the ranges | What `maquette set-against-table --proportions data/saffi/proportions.toml --table tables/women-and-teenage-girls/table.toml` writes | The table |

The program was built from commit `9b1f210` on `spec-3-first-table`; the table and the head's set were written again after the review that followed, which changed two counts of thin proportions and drew the same head. SHA-256 digests on 2026-11-18:

```
254d30d2cce2e46ffa7173c785efff1a91956af4bad7e80938d4786affd67bec  head-from-the-table.toml
86a39df07b216c19df56f7427e54aade0652f2dde30b56ca8d2de3acecebf474  tables/women-and-teenage-girls/table.toml
165a68d5425bf7820bafcf9c8a4ea8c67ff0a627d871e96da93511466e47d6d9  front.svg
1a1505cdb965fba0f6a8ecf8d7d4c44ba5d63f8fc1281c5669bf8a09cb408aa6  front.landmarks.toml
deaf7977227ec9e596734aa64f8e9d73615d3a4168c8e3ab5effc417515a1511  three-quarter.svg
aa0983e3592dc2f0908e089e9c51780458ad565f7c2b9eb192f9ffa11f0ec5f7  three-quarter.landmarks.toml
307ff582afb2ea250acdb99d64db6107ea7a227de9cec4c300c16d4ef1477c3f  side.svg
43b93e1b26ced2723904ff656d89c921254d9de5e2f120daa08ab29f672c681b  side.landmarks.toml
147f5b6c9c6bea69c6ed7d3038f3dcff2a4f4dadea90814cbb184673c5a1f668  front.sources-overlay.svg
4e6591ce2d36c949a17842ca1e905d3ee3d192e4593bff3f720d6b53d115d20f  three-quarter.sources-overlay.svg
8b2ae3b1dae6690e3ec5c2f87bb1d8a674c8e09bb05dfd602b6e2d979d2206b2  side.sources-overlay.svg
```

**The checks the drawn head passed:**

- `cargo test` (`tests/first_table.rs`): the head takes the table's middles and borrows Saffi's numbers as it says; every landmark is at one height in all three views and stands sideways where its depth puts it. `tests/armature.rs`: the side view's outline is lowest at the chin, with an angle of the jaw and without one.
- The read-back in VectorCraft 0.24.1 on 2026-11-18, for each view: no import warning, every name once, every anchor within 0.001 unit, every guide visible, stroked and unfilled.
- The hard case of the SVG contract on the three-quarter view: all 10 lines held.

## Composing it again

1. From the repository's root:
   - `maquette draw --proportions design/look-8-both-batches/head-from-the-table.toml --out target/look-8`
   - `maquette overlay-sources --proportions design/look-8-both-batches/head-from-the-table.toml --armature target/look-8 --out target/look-8`
   - `maquette read-back` on each landmark file, and `maquette read-back-hard-case` on the three-quarter view.
2. In FigCraft, open `armature-review-sheet.figcraft` and replace a drawing as `../look-6-first-table/README.md` says: import the file (`file.import`), move it into the sheet frame or its window (`node.reparent`), and set its place. The face sits in the frame "window on the face over the side view", at 2820, 580, 1000 wide and 940 high, the image at 1000 by 1000 inside it.
3. Export the sheet frame at 2400 pixels wide, and at scale 1 for the full size.

`head-from-the-table.toml` itself cannot be written again from a clean clone: `maquette set-from-table` reads the readings Jon accepted, which are in the reference folder.
