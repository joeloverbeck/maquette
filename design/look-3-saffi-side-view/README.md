# Look 3: Saffi's side view, two candidates

The armature review sheet for the third look of #3, made for #6 after look 2 did not accept the side view. It started from the sheet of look 2 in `../look-2-saffi-three-views/`. Nothing on it is accepted.

- `armature-review-sheet.figcraft`: the FigCraft source.
- `armature-review-sheet.png`: its export, 2400 pixels wide. The sheet is 4000 units wide where look 2's was 3200, so this keeps look 2's text size.
- `armature-review-sheet-full-size.png`: the same at full size, 4000 pixels wide, for the small text.
- `candidate-b-proportions.toml`: the set of proportions candidate B was drawn from. Candidate A's is `data/saffi/proportions.toml`.

## The decision

One decision: which side view, if either, is right to paint over. Jon said at look 2 that the front and three-quarter views read as one head, and that from the nose down the side view was pushed too far forward. Choosing A or B accepts the three views together, with that amount of the correction.

## What it shows

- **Top row:** the front and three-quarter views on shared level lines, the same files as at look 2. Beside them, on the same level lines, the two candidate side views, A and B. Then look 2's side view at half size, as "before", over a table of the depths of the forehead, the centre line at the eye line, the nose mark, the mouth and the chin in each of the three.
- **Second row:** the front and three-quarter views over her hand-made landmarks with the list of differences, unchanged since look 2 and shown again because the choice accepts all three views. Then the correction: its name, what it moves, the rule each amount comes from, and the amounts of A and of B.
- **Third row:** what the drawing lessons say about a profile and what the agent's own look found, which accepts nothing; the decision asked, why no check can make it, what it covers and when it stops; and which candidate the agent recommends, with its reasons.
- **Foot:** the proportions against a table's ranges, which do not exist yet.

The sheet says it is look 3 of 8, that look 1 was accepted and look 2 was not.

## The correction and the two candidates

`lower_face_set_back` is the first named correction. `proportions/list.toml` defines it: in the side view only, the chin and the mouth are drawn further back than their depths, which stay as measured. The front and three-quarter views do not change by a byte.

No drawing lesson gives a distance for a profile, so each amount comes from a rule. The lessons are those of `docs/research/2026-11-16-views-and-measuring/notes/head_construction_in_three_views.md`, section 1, which names them by letter. Those letters are not the candidates' letters, so the sheet leaves them out.

- **Chin.** The full amount draws the chin straight below the iris's centre: `chin_depth` minus `iris_depth`. The note's lesson B puts the chin's tip straight below the pupil, and it is the only lesson that places the chin against a point the armature has. The pupil is the painter's, so the iris's centre stands for it; her pupil's centre is 0.4 unit from it. A candidate takes a share of the full amount.
- **Mouth.** Whatever the share, the mouth line's centre is drawn on the straight line from the nose mark's bottom to the chin as drawn. The note's lessons A, C, F and H put the lips on the line from the nose tip to the chin or just behind it. Lesson G draws that line and names a chin that sticks out as a mistake. Lesson B lets the lips pass it slightly.
- **Nose.** Not moved. The note's lesson C wants the line from the nose down level with the forehead or ahead of it: the nose mark stands 6.7 units in front of the forehead.

| | Before, look 2 | A, share 0.5 | B, share 1 |
|---|---|---|---|
| Chin set back | 0 | 20.864 (4.22% of head height) | 41.728 (8.44%) |
| Mouth set back | 0 | 18.166 (3.67%) | 26.205 (5.30%) |
| Forehead, the skull's front | 58.296 | 58.296 | 58.296 |
| Centre line at the eye line | 44.367 | 44.367 | 44.367 |
| Nose mark's bottom | 64.988 | 64.988 | 64.988 |
| Mouth line's centre, as drawn | 62.870 | 44.704 | 36.664 |
| Chin, as drawn | 33.209 | 12.345 | -8.519 |

Depths are in front of the eyes, in the SVG's units at the default placement, where her head is 494.456 high from hair top to chin. The jaw point beside the chin moves with the chin, and the mouth line's two ends with its centre. In space the mouth and the chin stay at 62.870 and 33.209, which is where the front and three-quarter views show them.

**B's mouth amount is not twice A's.** Halving both amounts would leave the mouth 5 units in front of the line from the nose mark to the chin, and so break the rule that four lessons share. The share applies to the chin, which rests on one lesson, and the mouth follows by its own rule.

**The recommendation is A,** and it is the amount in `data/saffi/proportions.toml`. It is the smaller departure: turned back to 29°, A's chin would stand 10.1 units (2.0%) from where the three-quarter view draws it, and B's 20.2 (4.1%). It meets the lips' rule in full and follows lesson B's chin half-way. That lesson draws a round head, was read without its diagram, and itself lets the lips pass the line, so its chin with the other lessons' lips sets the mouth further back than any one lesson describes. Nothing gives the half: it is the midpoint between her own drawing and that lesson. A recommendation accepts nothing, and Jon's look decides.

## What the agent's look found

A diagnostic, which accepts nothing. In A and in B the face outline is one curve with no loop and no corner, and the mouth line ends on it. The nose mark's bottom stands on the outline, and its top 2.0 units in front of it in A and 1.3 in B, against 3.6 at look 2.

The outline is a smooth curve that at each point runs parallel to the line joining its two neighbours. The mouth is a neighbour of the nose mark's bottom, so moving the mouth also bends the stretch above it: between the eye line and the nose mark the outline differs from look 2's by up to 1.9 units in A and 2.7 in B (0.4% and 0.5% of head height). The landmarks there do not move. Where the outline crosses the nose level, the view point `face-outline-front-at-nose-level` stands 1.0 unit further forward in A and 1.4 in B, and the neck's front line starts 1.1 and 2.4 units higher, because it starts where it crosses the outline.

## What it is made from

Every drawing on the sheet is a file the program wrote, imported into FigCraft unchanged. The front and three-quarter views and the two candidates are placed at scale 1; look 2's side view is scaled to half. Each side view is seen through a window 680 units wide, which shows the drawing from x = 195 to 875 and cuts off only empty page.

| On the sheet | File | Drawn from |
|---|---|---|
| Front, three-quarter, and the comparisons | `target/saffi/front.svg`, `target/saffi/three-quarter.svg`, the two overlays and `target/saffi/differences.toml` | `data/saffi/proportions.toml` |
| Side A | `target/saffi/side.svg` | `data/saffi/proportions.toml`, share 0.5 |
| Side B | `target/saffi-full-amount/side.svg` | `candidate-b-proportions.toml`, share 1 |
| Before | `target/saffi-look-2/side.svg` | her set without the correction, which is the set of look 2 byte for byte |

Their SHA-256 digests on 2026-11-17. The first four, the differences and the two overlays are the digests in look 2's README: the correction did not change them.

```
1d2dd2501f81ca9c7c4f51c335fb9e19f5af4ad81ef29807627fab95056c5569  front.svg
2017d428249c0de463d054abc3c43ec83e3674999a0c8aaf75cf14960c6d9689  front.landmarks.toml
41ce28ee32df11612415e36268b5fdcceff35dd204e56158c7592ae67da4518e  three-quarter.svg
25d4c7278a67629b7e30b59eb11e48deeb89fa8cb4de60b6ae4cfdac334f864c  three-quarter.landmarks.toml
b9ceb8046ee1c9f4d310cd6385c1af4261bbd55fbb53430e696371c127dbef57  differences.toml
c6895d5e755a017bed5814bbb640290839a8e14d384ccf3a79ca6811bc496b8b  front.hand-made-overlay.svg
65ea5c636257d62cbf9e99cfcfa880f77fbe9b13f6a2a9ccebb02d70829e65a0  three-quarter.hand-made-overlay.svg
e3e35204e50620c8654f542a8593656da5e417d0c6a93920d8c5721d0f77f4b2  side.svg                 A, in target/saffi
22220acdfbcf5cb404ab97123c43eb96148880a59120affc0c18ab9685af17d8  side.landmarks.toml      A
c3ef285a82654d9ab3fdca40bc6c1d8909bb644f2bf069a123bca23e484c9498  side.svg                 B, in target/saffi-full-amount
0f53a19efae458b4a17e627fbc109b091ea80dc525de1251ed14d349e9db4d9e  side.landmarks.toml      B
45c2919c5c90bb13775f513cc5f5cdabd2bb774fc7ec27de1e09d5ea14a66226  side.svg                 before, look 2's
```

Drawn from candidate B's set, the front and three-quarter files have the same four digests.

`candidate-b-proportions.toml` is what `proportions-from-hand-made-views` writes at a share of 1. It differs from `data/saffi/proportions.toml` only in the share and the two amounts. Its opening lines are the command's own and say that the tests check the file: they check `data/saffi/proportions.toml`, and no test checks this copy.

Both candidate side views passed the read-back in VectorCraft 0.24.1 on 2026-11-17, before the sheet was final: no import warning, 19 names, 49 anchors of 16 guides within 0.001 unit, every guide visible, stroked and unfilled. The front and three-quarter views are the files that passed before look 2.

## Composing it again

1. Write the files, from the repository's root:
   - `target/saffi`: `draw`, `differences-from-hand-made-views` and `overlay-hand-made-views` from `data/saffi/proportions.toml`, as `README.md` at the root gives them.
   - `target/saffi-full-amount`: `draw --proportions design/look-3-saffi-side-view/candidate-b-proportions.toml`.
   - `target/saffi-look-2`: `proportions-from-hand-made-views` with the two records and no `--lower-face-set-back`, then `draw` from what it writes.
   - Run `read-back` on each candidate's `side.landmarks.toml` before a sheet goes to Jon.
2. In FigCraft, open `armature-review-sheet.figcraft`. The sheet frame is at 0, 0. Each imported drawing is a frame named after its file. The front and three-quarter views and the overlays are where look 2's README puts them. Each side view sits inside a window frame that clips it:
   - side A: the window at 2120, 260, 680 by 940, and the drawing inside it at -195, 0;
   - side B: the window at 2820, 260, the same size, and the drawing at -195, 0;
   - before: the window at 3540, 350, 360 by 400, and the drawing scaled by 0.5 (`transform.scale`) at -62, -35.
   Replace a drawing by importing the new file (`file.import`), moving it into its window (`node.reparent`) and setting its place.
3. If a height in a landmark file changed, move the level lines to 260 plus that height. If an amount changed, retype the two captions, the two tables and the recommendation.
4. Export the sheet frame at 2400 pixels wide and at scale 1.

FigCraft's server writes only under the main checkout's `design/`. Composed from another worktree, the folder is made there and moved here afterwards.
