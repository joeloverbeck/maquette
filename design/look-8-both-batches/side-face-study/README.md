# Side-face study: the same face drawn over the side view of look 8

A diagnostic study of one armature, made before look 8 as `docs/agents/looks.md` asks for a side view of a head Jon has not accepted. It is not a product: maquette makes armatures and does not paint. The question it serves is whether the side view drawn from the table of both batches looks right once an anime-style face is drawn over it, where look 6's jaw and neck did not. The drawing accepts nothing; Jon's eyes decide.

It is the woman of `../../look-6-first-table/side-face-study/` drawn again: the same hair, ear, eye, colours and manner, so that the two faces differ only where the armature does. She is original and has no name.

## The files

- `side-face.vectorcraft`: the VectorCraft source, started as a copy of look 6's. Three layers, bottom to top: `paper`, `armature` (`target/look-8/side.svg`, pasted in place unchanged) and `face` (sublayers `neck`, `garment`, `head`, `features`, `hair`, `ear`).
- `side-face.png`: the face alone, the `armature` layer hidden. 2000 by 2000 pixels, 2 pixels to the unit.
- `side-face-over-armature.png`: the same face with the `face` layer at 45% opacity and the armature showing through it.
- `side-face-before-and-after.png`: look 6's face and this one side by side at the same size, each labelled, no armature. 2640 by 2020 pixels.
- `side-face-before-and-after.figcraft`: the FigCraft source of that sheet.

## What follows the armature

Each of these is an anchor of the drawn path set on the landmark's own coordinates.

- **The profile:** `skull-front`, `centre-line-at-brow-level` (the forehead), `centre-line-at-eye-level` (the bridge), `nose-tip`, `mouth-line-centre`, `chin`, `neck-front-top`, `jaw-line-right-at-mouth-level`, `jaw-line-right-at-nose-level`. From the chin to the neck's front and on to the jaw line it takes the guide's own curve. The forehead above `skull-front` runs as at look 6.
- **The nose's base:** the underside runs back from the tip to the lip's root at `nose-mark-bottom`'s height, 693.2, 487.2, as look 6's did 35 behind the tip.
- **The neck:** the front line from `neck-front-top` to `neck-front-base`, the back line on the guide's, from `neck-back-top` to `neck-back-base`. The collar sits at the two bases, so it runs down toward the front.
- **The eye:** look 6's eye scaled in height to this opening, 32.2 units against 40.6, about the eye line, with the iris moved 3.9 units back to its centre.
- **The brow:** a new stroke between the brow line's two ends, which stand 31 and 11 units lower than look 6's.
- **The mouth:** look 6's line, moved 5.8 units forward and 4.9 down with its two landmarks.
- **The ear and the sideburn:** moved 35.7 units back with the jaw line, whose guide the ear's front edge stands on above the nose level.

No landmark was moved.

## Where the line leaves the guide's curve between landmarks

| Stretch | Drawn as | Off the guide's curve |
|---|---|---|
| Bridge of the nose | Nearly straight, where the guide bulges forward | up to about 8 units behind it |
| Under the tip, upper lip | A straight underside back to the lip's root, then the lip, as at look 6 | up to about 15 |
| Lower lip, chin | Look 6's lower lip and chin, moved with the mouth and the chin | up to about 5 |

## What was drawn freely

As at look 6: the hair, the human ear, the eye's details, the garment, the collar, the colours and the shadows.

## Making the exports again

- Open `side-face.vectorcraft`. For `side-face.png`, hide the `armature` layer and export the artboard at scale 2. For `side-face-over-armature.png`, show it, set the `face` layer's opacity to 45% and export at scale 2.
- Open `side-face-before-and-after.figcraft`. Each face is its `side-face.png` at 1000 by 1000 in a window frame of 600 by 860 that clips it, placed at -200, -70 inside the window. Export the `sheet` frame at scale 2.
