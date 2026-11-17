# Side-face study: the same face drawn over the side view of look 6

A one-off test of one armature, made at Jon's request on 2026-11-17. It is not a product: maquette makes armatures and does not paint. The question it serves is whether the side view of look 6 looks right once an anime-style face is drawn over it, now that the nose's tip stands at its own height. The drawing accepts nothing; Jon's eyes decide.

It is the woman of `../../look-5-first-table/side-face-study/` drawn again: the same hair, ear, eye, brow, colours and manner, so that the two faces differ only where the armature does. She is original and has no name.

## The files

- `side-face.vectorcraft`: the VectorCraft source. Three layers, bottom to top: `paper`, `armature` (`target/look-6/side.svg`, opened unchanged as the base) and `face` (sublayers `neck`, `garment`, `head`, `features`, `hair`, `ear`).
- `side-face.png`: the face alone, the `armature` layer hidden. 2000 by 2000 pixels, 2 pixels to the unit, covering the armature's whole 1000 by 1000 units.
- `side-face-over-armature.png`: the same face with the `face` layer at 45% opacity and the armature showing through it.
- `side-face-before-and-after.png`: the look 5 face and this one side by side at the same size, each labelled, no armature. 2640 by 2020 pixels.
- `side-face-before-and-after.figcraft`: the FigCraft source of that sheet.

The armature is the one `maquette draw --proportions design/look-6-first-table/saffi-with-table-values.toml --out target/look-6` writes. Its digests on the day were those listed in `../README.md`.

## What follows the armature

Each of these is an anchor of the drawn path set on the landmark's own coordinates, so the line's centre stands 0 units from it.

- **The profile:** `skull-front`, `centre-line-at-eye-level`, `nose-tip` (the tip of the nose), `mouth-line-centre`, `chin`, `jaw-line-right-at-mouth-level`, `jaw-line-right-at-nose-level`. The forehead above `skull-front` runs along the skull circle. The ear's front edge stands on the jaw guide above the nose level.
- **The nose's base:** `nose-mark-bottom` lies on no guide in this view and gives a height only. The underside runs back from the tip to the lip's root at 698, 486, which is at that height.
- **The mouth, the eye, the brow:** as in the look 5 study, on the same landmarks, which did not move.
- **The neck:** both lines at the guides' x. The front line starts at `neck-front-top`, which lies on the jaw's line. The shoulder's arc passes through `shoulder-right-point`.

No landmark was moved.

## Where the line leaves the guide's curve between landmarks

| Stretch | Drawn as | Off the guide's curve |
|---|---|---|
| Bridge of the nose | Nearly straight, where the guide bulges forward | up to 7 units behind it |
| Under the tip, upper lip | A straight underside back to the lip's root, then the lip, where the guide rounds on below the tip as far as the nose level | up to 15 |
| Lower lip, chin | As in the look 5 study | up to 4 |
| Jaw, chin to mouth level | As in the look 5 study | up to 5 |
| Eye opening, front half | As in the look 5 study | up to 9 |
| Lid line, front half | As in the look 5 study | up to 3 |

## What changed from the look 5 study, and what did not

- **Changed:** the bridge, the tip, the underside and the upper lip of the nose; the neck's two lines and the shadow on it; the collar and the garment, which follow the neck (the collar sits at the neck's base, 43 units higher than before, and the garment is 56 units deeper).
- **Not changed:** the forehead, the lower lip, the chin, the jaw, the mouth, the eye, the brow, the hair, the ear, the shoulder's arc and every colour. In the side view the armature's jaw outline is the one of look 5; the borrowed jaw point shows in the other two views.

## What was drawn freely

As in the look 5 study: the hair, the human ear, the eye's details, the garment, the collar, the colours and the three shadows. The neck's front line runs 13 units below `neck-front-base` to the collar.

## Making the exports again

- Open `side-face.vectorcraft`. For `side-face.png`, hide the `armature` layer and export the artboard at scale 2. For `side-face-over-armature.png`, show it, set the `face` layer's opacity to 45% and export at scale 2. The source is saved with the armature showing and the face at 100%.
- Open `side-face-before-and-after.figcraft`. The `sheet` frame is 1320 by 1010. Each face is its `side-face.png` at 1000 by 1000 in a window frame of 600 by 860 that clips it, placed at -200, -70 inside the window. Replace a face by importing the new file, moving it into its window and setting that place. Export the `sheet` frame at scale 2.
