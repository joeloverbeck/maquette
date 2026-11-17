# Look 2: Saffi in three views

The armature review sheet for the second look of #3, made for #6. It started from the sheet of look 1 in `../look-1-saffi-front-three-quarter/`, and follows the second mock-up in `../first-table-sheets/`. Nothing on it is accepted.

- `armature-review-sheet.figcraft`: the FigCraft source.
- `armature-review-sheet.png`: its export, 1920 pixels wide.
- `armature-review-sheet-full-size.png`: the same at full size, 3200 pixels wide, for the small text.

## What it shows

- **Top row:** the generated front, three-quarter and side views alone, on shared level lines. The side view is marked as new, with nothing hand-made to lay it over.
- **Second row:** the front and three-quarter views over her hand-made landmarks, with every difference above 1% of head height numbered, and the list of them. The legend says what the numbers are measured against: her S2 underdrawing with the ears and the lids of her final drawings in their place, and her irises and pupils still S2's.
- **Third row:** what the side view's only check is, what drawing lessons agree on for a side view as things to look for (where the ear sits, where the lips fall against the line from the nose tip to the chin, how the neck leans), and how the side view is drawn; then the decision asked, why no check can make it, what it covers and when it stops.
- **Foot:** the proportions against a table's ranges, which do not exist yet.

No correction exists, and the sheet says so. One that Jon's look calls for is named, given an amount and shown on the next sheet.

## What it is made from

Every drawing on the sheet is a file the program wrote, imported into FigCraft unchanged and placed at scale 1; FigCraft only lays them out. The generated guides inside the source are copies of those files, not designs.

| On the sheet | File |
|---|---|
| Top row | `target/saffi/front.svg`, `target/saffi/three-quarter.svg`, `target/saffi/side.svg` |
| Under each comparison | `target/saffi/front.svg`, `target/saffi/three-quarter.svg` |
| The orange dots, rings, lines and numbers of the comparison | `target/saffi/front.hand-made-overlay.svg`, `target/saffi/three-quarter.hand-made-overlay.svg` |
| The list of differences, and the footnotes | `target/saffi/differences.toml`, and the levels of `data/saffi/final-drawings.toml` |
| The level lines of the top row | the heights in `target/saffi/front.landmarks.toml`: hair top 105.544, skull top 130.605, eye line 400, nose level 479.313, mouth level 531.183, chin 600, shoulder line 781.8 |

The files were written from `data/saffi/proportions.toml`, `data/saffi/hand-made-views.toml` and `data/saffi/final-drawings.toml` at the default placement. Their SHA-256 digests on 2026-11-17:

```
1d2dd2501f81ca9c7c4f51c335fb9e19f5af4ad81ef29807627fab95056c5569  front.svg
2017d428249c0de463d054abc3c43ec83e3674999a0c8aaf75cf14960c6d9689  front.landmarks.toml
41ce28ee32df11612415e36268b5fdcceff35dd204e56158c7592ae67da4518e  three-quarter.svg
25d4c7278a67629b7e30b59eb11e48deeb89fa8cb4de60b6ae4cfdac334f864c  three-quarter.landmarks.toml
45c2919c5c90bb13775f513cc5f5cdabd2bb774fc7ec27de1e09d5ea14a66226  side.svg
bc23617514fe40fc867e64f924373f28bd854b510aa98d02a35038f31b0fcb96  side.landmarks.toml
b9ceb8046ee1c9f4d310cd6385c1af4261bbd55fbb53430e696371c127dbef57  differences.toml
c6895d5e755a017bed5814bbb640290839a8e14d384ccf3a79ca6811bc496b8b  front.hand-made-overlay.svg
65ea5c636257d62cbf9e99cfcfa880f77fbe9b13f6a2a9ccebb02d70829e65a0  three-quarter.hand-made-overlay.svg
```

The differences file and the overlays name the landmark files by the paths they were given, so they have these digests only when written into `target/saffi` from the repository's root, as below.

All three views passed the read-back in VectorCraft 0.24.1 on 2026-11-17, and the three-quarter view held the contract's hard case, before the sheet was composed.

## Composing it again

1. Write the files into `target/saffi`: `draw`, `differences-from-hand-made-views` and `overlay-hand-made-views`, the last two with `--final-record data/saffi/final-drawings.toml`, as `README.md` at the root gives them. Run the read-back on the three views and the hard case on the three-quarter view before a sheet goes to Jon.
2. In FigCraft, open `armature-review-sheet.figcraft`. Each imported drawing is a frame named after its file. Replace it by importing the new file (`file.import` with the same `x` and `y`, then `node.reparent` into the sheet frame): the front view at 80, 260 and 80, 1380, the three-quarter view at 1100, 260 and 1100, 1380, the side view at 2120, 260, and each overlay at the same place as the view it lies over. The sheet frame is at 0, 0.
3. If any height in a landmark file changed, move the level lines to 260 plus that height. If the differences changed, retype the list from the differences file: a row is a numbered dot, a share and a name, 33 units below the row before.
4. Export the sheet frame at 1920 pixels wide and at scale 1.

FigCraft's server writes only under the main checkout's `design/`. Composed from another worktree, the folder is made there and moved here afterwards.
