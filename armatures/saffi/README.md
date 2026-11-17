# Saffi's accepted armature

Generated output that Jon accepted. It is not a design, which would be in `design/`, and not a proportion table: Saffi is the comparison case, and nothing of hers enters one.

The six files are what `maquette draw` writes from [`data/saffi/proportions.toml`](../../data/saffi/proportions.toml) at the default placement: each view's SVG of named guides, with its landmark file beside it.

| View | SVG | Landmark file |
|---|---|---|
| Front | `front.svg` | `front.landmarks.toml` |
| Three-quarter, turned 29° | `three-quarter.svg` | `three-quarter.landmarks.toml` |
| Side, turned 90° | `side.svg` | `side.landmarks.toml` |

## What Jon accepted

Her armature in three views, at look 3 of 8 on 2026-11-17, from the armature review sheet in [`design/look-3-saffi-side-view/`](../../design/look-3-saffi-side-view/README.md):

- the front and three-quarter views, which he had said at look 2 read as one head;
- the side view with the named correction `lower_face_set_back` at the amounts of that sheet's candidate A: in the side view only, the chin is drawn 20.9 units back (4.2% of head height) and the mouth 18.2 (3.7%).

The record of his acceptance is on issue #6, with the SHA-256 digest of each file. The same six digests are in the look's README, where the side view is "A". These files have those digests.

## What that does not say

- It does not make her proportions a standard, or good ones for another character.
- It accepts no table. None exists.
- It does not accept candidate B.
- The side view's only check was his look: she has no hand-made side view to lay it over.
- It does not say mudsong can use the files. That is proven only when mudsong has imported them and read the names and positions back (`docs/foundation/02-first-table.md`, "The handoff").

## Keeping it

No file here is edited by hand or written again from the program. `cargo test` checks that `maquette draw` still writes these six files byte for byte from her set of proportions (`tests/saffi.rs`).

When that test fails, the program no longer draws the armature Jon accepted. Either the change is undone, or her armature has changed and goes to Jon for a new look; only after he accepts it are these files replaced.
