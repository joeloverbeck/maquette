# Look 1: Saffi, front and three-quarter

The armature review sheet put to Jon for the first look of #3, made for #5. It follows the second mock-up in `../first-table-sheets/`. Nothing on it is accepted.

- `armature-review-sheet.figcraft`: the FigCraft source.
- `armature-review-sheet.png`: its export, 1920 pixels wide.
- `armature-review-sheet-full-size.png`: the same at full size, 3200 pixels wide, for the small text.

## What it is made from

Every drawing on the sheet is a file the program wrote, imported into FigCraft unchanged and placed at scale 1; FigCraft only lays them out. The generated guides inside the source are copies of those files, not designs.

| On the sheet | File |
|---|---|
| Top row, and under each comparison | `target/saffi/front.svg`, `target/saffi/three-quarter.svg` |
| The orange dots, rings, lines and numbers of the comparison | `target/saffi/front.hand-made-overlay.svg`, `target/saffi/three-quarter.hand-made-overlay.svg` |
| The list of differences, and the footnotes | `target/saffi/differences.toml` |
| The level lines of the top row | the heights in `target/saffi/front.landmarks.toml`: hair top 111.202, skull top 135.977, eye line 400, nose level 480.694, mouth level 531.97, chin 600, shoulder line 779.72 |

The files were written from `data/saffi/proportions.toml` and `data/saffi/hand-made-views.toml` at the default placement. Their SHA-256 digests when the sheet was composed on 2026-11-16:

```
af5a4fcf6d838ce82bc36bce4d142bdb19e69a46bdba9c634da874e02b318857  front.svg
e9fedc58d44e93a71d082f162b6e83335692e5cf344a3412bcac2554b8715069  front.landmarks.toml
102cad3030a2e31e5cf784f1266666ad97e8cab1f93b0159878ca99dfd181572  three-quarter.svg
1abe17279bae6d2bd0ec95993cf9b665b8f2448f49960c7eb61de59d817e91dc  three-quarter.landmarks.toml
becdd50f517d196a8c9fa1aa596feb6debc7106a72486263edca342f916bfc39  differences.toml
2ab86ede121226d6eb95f3e4d7ff1e82ef86e16dbea540daa100034123a6a62f  front.hand-made-overlay.svg
3edee73385e73980e8930e5a7e3d6d4396bf4bc98f3fbf14703c9673fb9f9bcf  three-quarter.hand-made-overlay.svg
```

## Composing it again

1. Write the files: `draw`, `differences-from-hand-made-views` and `overlay-hand-made-views`, as `README.md` at the root gives them, into `target/saffi`. Run the read-back and the hard case on the two views before a sheet goes to Jon.
2. In FigCraft, open `armature-review-sheet.figcraft`. Each imported drawing is a frame named after its file. Replace it by importing the new file (`file.import` with the same `x` and `y`, then `node.reparent` into the sheet frame): the front view at 80, 260 and 80, 1380, the three-quarter view at 1100, 260 and 1100, 1380, each overlay at the same place as the view it lies over. The sheet frame is at 0, 0.
3. If any height in a landmark file changed, move the level lines to 260 plus that height. If the differences changed, retype the list from the differences file.
4. Export the sheet frame at 1920 pixels wide and at scale 1.
