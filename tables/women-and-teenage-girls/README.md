# The table of both batches: women and teenage girls

`table.toml` is the proportion table built from both batches of references. **It is a proposal.** Jon has not accepted it. It replaces the first table, built from the first batch alone, which he did not accept at look 5 and gave no verdict on at look 6; that table is in the history of this folder's parent, as `tables/adult-women/`. The pull request that carries this one merges only if he accepts it.

- **What it was measured on:** 20 characters of 12 works, one drawing of each in each view: the six adult women of the first batch, which Jon accepted at look 4, and 14 characters of the second batch, which he accepted at look 7. The second batch read the first batch's three side views again for the points of the profile asked for since revision 2026-11-17.4 of the reading instructions; those readings give only what the first ones were not asked for. `[[character]]` in the file lists each character with its work and designer as its readings name them.
- **What it counts:** characters, and beside each count the works they come from and how many the work that gives the most gives, so that a range resting on one hand can be seen. The floor of five counts characters. Five of the second batch's characters come from one work's sheets, and where a work is not known the characters are counted under the words their readings give.
- **The views:** a front view of all 20 characters, of 12 works, 5 of them from one work. A side view of 9, of 5 works, 4 of them from one. Three-quarter views of 2 give depth, which is too thin; three more characters have one, and their depth is taken from their side views.
- **What it gives:** 56 of the 86 proportions with a valid range. 20 are too thin, given by one to four characters; 7 are given by none; the skull's 3 are conventions. A thin one states its count and why the others do not give it, and no number.
- **Ranges resting mostly on one work:** five of the 56 have more than half their characters from one work: `jaw_line_depth_at_mouth_level` (4 of 7), `nose_mark_corner_depth` (5 of 7), `ear_base_inner_depth` (4 of 6), `ear_top_rise` (5 of 9) and `bridge_depth` (4 of 6).
- **The age bands, side by side:** 14 adult characters and 6 teenage ones. Each band has a range of its own in 24 proportions, and in all 24 the two ranges overlap. Overlapping ranges do not show that the bands are alike, and six teenage characters of three works are few: this shows what was measured, and tests nothing of whether the bands differ.
- **What it says of itself:** that its ranges are separate and do not rule on which combinations of proportions go together, and that a lowest and a highest value are each one reference's number.

## The profile

A side view has been asked since revision 2026-11-17.4 for the forehead, the bridge of the nose at the eye line, the angle of the jaw, and the top ends of the neck's front and back lines: where look 6 drew Saffi's outline on a measured face, and where Jon found it wrong.

| Proportions | Characters | Standing |
|---|---|---|
| `bridge_depth` | 6, of 3 works | given |
| `neck_front_top_drop`, `neck_front_top_depth` | 6, of 3 works | given |
| `forehead_depth` | 3 | too thin |
| `neck_front_base_drop`, `neck_front_base_depth` | 3 | too thin |
| `jaw_angle_drop`, `jaw_angle_depth` | 1 | too thin |
| `neck_back_top_*`, `neck_back_base_*` | 1 | too thin |

The angle of the jaw and the back of the neck are what the side views did not give. Of the nine side views, both readers placed the angle in two, and Jon marked one of those off by his rule for disagreements; one reader placed it in two more, and a point that one reader alone places is left out. The first batch's third side view was read before the point was asked. Elsewhere the readers found it under the hair, or found no angle drawn: the jaw's underside running back to the ear, or into the front of the neck, as one even curve. The top of the neck's back was under the hair in every side view but one, where one reader placed it.

## Saffi against it

Set against it, Saffi's proportions are inside 42 ranges and outside 11; 21 have no range to be set against, and she gives none of the 12 proportions of the profile. That is a finding, and nothing was changed to bring her in. Her numbers are at no end of a range: the table was not built on her.

## How it was made

```sh
maquette table-from-readings --references <the reference folder> \
  --check batch-1/jons-check.toml,batch-2/jons-check.toml --out <table>
maquette set-against-table --proportions data/saffi/proportions.toml \
  --table tables/women-and-teenage-girls/table.toml
```

The reference folder holds both batches, each in a folder of its own with the record of Jon's check that names its readings from there. The table was copied here unchanged.
