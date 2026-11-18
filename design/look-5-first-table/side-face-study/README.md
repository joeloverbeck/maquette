# Side-face study: a face drawn over the side view of look 5

A one-off test of one armature, made at Jon's request on 2026-11-17. It is not a product and not part of any look: maquette makes armatures and does not paint. The question it serves is whether the side view of look 5, and its nose above all, looks right once an anime-style face is drawn over it. The drawing accepts nothing; Jon's eyes decide.

The woman drawn is original and has no name. She is a plain bust, a mannequin in a plain collar.

## The files

- `side-face.vectorcraft`: the VectorCraft source. Three layers, bottom to top: `paper` (a white ground), `armature` (`target/look-5/side.svg`, opened unchanged as the base) and `face` (the drawing, in sublayers `neck`, `garment`, `head`, `features`, `hair`, `ear`).
- `side-face.png`: the face alone, the `armature` layer hidden. 2000 by 2000 pixels, 2 pixels to the unit.
- `side-face-over-armature.png`: the same face with the `face` layer at 45% opacity and the armature showing through it.

The armature is the one `maquette draw --proportions design/look-5-first-table/saffi-with-table-values.toml --out target/look-5` writes. Its digests on the day were those listed in `../README.md`.

## What follows the armature

Each of these is an anchor of the drawn path set on the landmark's own coordinates, so the line's centre stands 0 units from it.

- **The profile:** `skull-front`, `centre-line-at-eye-level`, `nose-mark-bottom` (the tip of the nose), `mouth-line-centre` (where the lips meet), `chin`, `jaw-line-right-at-mouth-level`, `jaw-line-right-at-nose-level`. Above `skull-front` the forehead runs along the skull circle to the hairline. Above the nose level the jaw guide is hidden by the ear, whose front edge stands on it.
- **The mouth:** from `mouth-line-centre` to `mouth-line-right-end`.
- **The eye:** the opening through its four points, the upper lash's top edge through the three points of the lid line, the iris centred on the iris line.
- **The brow:** tapered, from `brow-line-right-inner-end` to `brow-line-right-outer-end`.
- **The neck:** both lines at the guides' x. The shoulder's arc passes through `shoulder-right-point`.

No landmark was moved.

## Where the line leaves the guide's curve between landmarks

The guide's smooth curve between two landmarks is a guide. The drawn line leaves it by about these distances:

| Stretch | Drawn as | Off the guide's curve |
|---|---|---|
| Bridge of the nose | Nearly straight, slightly hollow, where the guide bulges forward | up to 9 units behind it |
| Under the tip, upper lip | A short underside back to the lip's root, then the lip, where the guide swings below the tip | up to 10 |
| Lower lip, chin | A small lower lip, a hollow, a round chin | up to 4 |
| Jaw, chin to mouth level | Straighter | up to 5 |
| Eye opening, front half | Kept tall to its front point, as a profile eye is, where the guide closes to a point | up to 9 |
| Lid line, front half | Carried high, then down to its front end | up to 3 |

The eye is the one place where the freedom was needed to make the feature work at all: on the guide's own curve the opening is about 12 units tall at the iris line, and an iris 43 units tall would show as a sliver in the eye's front corner.

## What was drawn freely

The armature does not settle these.

- **The hair:** a short bob swept back off the forehead and tucked behind the ear, with a lock in front of the ear. It stays inside the hair volume.
- **The ear:** a human ear between the eye line and the nose level, from x = 469 to 519, just behind the middle of the head's depth. The armature's own ear, an animal's on top of the head, is ignored.
- **The neck beyond its guides:** above `neck-front-top` the front line curves forward to meet the jaw's corner; below `neck-front-base` it runs 13 units further to the collar.
- **The eye's details:** the iris's width (20 units), the pupil, the highlight, the lower lid's length, the crease above the lash and the lash's tail behind `lid-line-right-outer-end`.
- **The garment, the collar, every colour and the three shadows** (under the jaw, inside the ear, under the hair).

## Making the exports again

Open `side-face.vectorcraft`. For `side-face.png`, hide the `armature` layer and export the artboard at scale 2. For `side-face-over-armature.png`, show it, set the `face` layer's opacity to 45% and export at scale 2. The source is saved with the armature showing and the face at 100%.
