# The side-view retest drawing

An original painted head in profile, drawn on 2026-11-17 for issue #18. It is the first side drawing with known positions that is a painted head and not an armature's guides. It tests readers under revision 2026-11-17.4 of the reading instructions, which first asked a side view for the forehead, the bridge of the nose, the jaw's angle and the top ends of the neck's two lines. The rule is in `docs/agents/reading-instructions.md`: a revision is tested on a side drawing whose positions are known before a published side view is read under it.

It is no character, copies no drawing and is a test of readers, not a design of a head: its proportions are not anyone's to adopt. Its nose has a tip and a base that stand apart, which no made-up head of the tests had before look 5.

## The files

- `side-profile.vectorcraft`: the VectorCraft source. Each path is named for what it is (`skin`, `profile-and-jaw-line`, `neck-front-line`, `neck-back-line`, `eye-white`, `iris`, `lid-line`, `brow`, `nose-wing`, `mouth-line`, `hair`, `hair-bun`, `ear`, and the rest).
- `side-profile.png`: the picture readers are given, 1000 by 1000 pixels, exported at one pixel to a unit.
- `side-profile.svg`: the SVG export, whose path data are the source's coordinates.
- `known-positions.toml`: the record that `maquette score-reading --known-positions` reads, written as a landmark file is.

## How each known position is fixed

The eye line is at y = 450, midway between the opening's top (428) and bottom (472), and the chin is at y = 680, so the unit is 230. Depth zero is midway between the eye's corners, at x = 623.5.

| Landmark | Where it is in the drawing |
|---|---|
| `hair-top` | The anchor at the crown of `hair`, whose two handles are level. |
| `hair-front-extent` | The fringe's tip, a corner of `hair`; the path runs back from it on both sides. |
| `hair-back-extent` | The back of `hair-bun`, an anchor whose two handles are upright. |
| `chin` | The corner of `profile-and-jaw-line` where the front of the face, coming down, turns to run back under the jaw. |
| `nose-mark-bottom` | The tip of the nose, a corner where the bridge turns back into the nose's underside. |
| `mouth-line-centre` | The bottom of the notch between the lips, a corner of the profile, where `mouth-line` reaches it. |
| `centre-line-at-brow-level` | The profile's anchor at y = 383, the height of the brow's front end. |
| `centre-line-at-eye-level` | The profile's anchor at y = 450, the eye line. |
| `jaw-right-angle` | The corner of `profile-and-jaw-line` where the straight underside of the jaw turns up toward the ear. |
| `jaw-line-right-at-nose-level`, `-at-mouth-level` | Where the straight back edge of the jaw, from the angle (560, 640) to the lobe (553, 508), crosses the nose level (525.75, midway between the wing's middle and the tip) and the mouth level (578). |
| `neck-front-top` | The corner of `skin` and `neck-front-line` where the neck's front edge meets the jaw's underside. |
| `neck-front-base`, `neck-back-base` | The corners where the neck's front edge turns into the chest and its back edge into the shoulder. |
| `neck-back-top` | The corner of `neck-back-line` where the neck's back edge meets the skin below the hair at the nape. |
| Eye opening | `eye-white`: its corner at the back (the outer corner), and anchors with level handles at its top and bottom and upright ones at its front (the inner corner). |
| Iris | `iris`: anchors with level handles at its top and bottom, upright ones at its back and front. A sliver of white stands in front of it. |
| Lid line, brow | `lid-line` and `brow` are open paths with butt caps, so each end is a path end; the lid line's crest is the anchor with level handles. |
| `nose-mark-right-corner` | The middle anchor of `nose-wing`, which is symmetric about it. |
| `mouth-line-right-end` | The back end of `mouth-line`. |
| Ear | `ear`: the base points are the ends of its front edge, where the rim meets the head and where the lobe does; the top has level handles; the back and the front have upright ones. |

The shoulder is under the garment and has no known position.

## Making the exports again

Open `side-profile.vectorcraft` and export the first artboard as PNG at scale 1 and as SVG. The record does not change unless a path does.
