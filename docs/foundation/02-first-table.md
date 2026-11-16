# 02 · The first table, drawn

Jon reoriented the first increment on 2026-11-16. It had been Saffi's head in three views from fixed values. It is now his whole path, from references to a drawn table, run once at small scale.

## Bottleneck and primary path (`WORK-1`)

No measured proportions qualify as a table. Family A's came from three faces of one series, in one view. Nothing yet says which proportions to take from a drawing, and nothing can draw a set of proportions for Jon to judge. The primary path is the shortest one to a first table measured on several works by several hands, drawn in three views that agree, and accepted by Jon.

## Incumbent (`PURP-4`)

Without maquette, an agent in mudsong measures a few published faces of the style family, draws the armature in VectorCraft and shows Jon a sheet.

- Observed on 2026-11-15: two looks to four accepted three-quarter armatures.
- Observed by Jon on 2026-11-16: the agent took proportions from drawings that looked slightly down, as if they were level.

The increment must keep the incumbent's result for Saffi: the generator can draw her accepted three-quarter view. It is judged on looks to acceptance, agreement between views, what one reference costs Jon, and the work the next character would need.

## The increment

Six parts, run on one small batch before any wide gathering (`EVID-3`).

1. **The list of proportions.** Every named proportion a head needs in the three views, each with a definition. A proportion is defined on features that are drawn in a reference. A guide with nothing drawn behind it, such as the skull under the hair, is built by a stated rule and labelled as a convention.
2. **The measuring method.** Separate agents read each reference twice, and a disagreement above 1% of head height is shown. The agent states the view, the turn and whether the head is level. Jon checks each character on a reference check sheet.
3. **A first batch of references.** Adult women, one character per work, from at least six works by six character designers, in official publications only. An agent proposes the works and Jon strikes any he does not rate.
4. **The table and its valid ranges.** For each proportion, the lowest, middle and highest value measured and the number of works behind it.
5. **The generator.** It draws any set of proportions as a head in front, three-quarter and side views: an SVG of named guides for each, with a landmark file beside it. Every landmark is one point in space, and each view shows the same points from another side. The three-quarter view is turned as far as Saffi's hand-made armature, about 30°.
6. **The comparison with Saffi.** The generator draws her from her own proportions, laid over the hand-made views. Each of her proportions is then set against the table's valid range. Her numbers never enter a table.

The armature carries the guides `01-mission.md` lists, the centre line, level lines for the eyes, nose and mouth, the hair's volume as a plain envelope, and the shoulder line (Jon, 2026-11-16).

It is built as the least complex thing that can do this: no controls, no types of part, no interface, no body below the shoulder line, and no rule for which proportions go together. Those come from `03-direction.md` when they are earned (`EVID-5`).

## What it is checked against

- **Drawings whose positions are known.** An agent's reading is an artificial evaluation, trusted only as far as it has matched such drawings (`EVAL-2`). On 2026-11-16, four readings of five hand-made drawings were all within 1.5% of head height. Published drawings, and judging whether a head is level, are untested.
- **Two readings of every reference, and Jon's check.** He decides whether each head is level and in the view stated, and whether the points sit on the right features.
- **Mechanical checks of the generator.** Each proportion taken back from the written files equals the one put in, and every landmark is the same point in all three views. These are validators (`AUTH-3`) and prove only that.
- **The comparison case.** Saffi's numbers stay out of every table, so setting her against one is an independent test (`EVID-4`).
- **An agent's look at a render.** A diagnostic. Finding nothing wrong establishes nothing (`EVAL-2`).
- **Jon's look.** The acceptance.

## The claim about painting (`SEARCH-4`, `SIM-2`)

That an armature improves the drawing made over it is a separate claim. Its evidence is the same bust painted with the armature and without it, from the same brief, more than once each, with Jon comparing them. One good painting does not carry it. Until that comparison is made the claim is unproven, and nothing may rest on it.

## The handoff (`INT-1`, `FID-1`)

Maquette exports Saffi's accepted armature at a tagged commit. Mudsong copies the files into its own study folder, records the commit and imports them into VectorCraft itself. Maquette does not depend on mudsong's working tree. Jon approved one exception on 2026-11-16: the hand-made drawings in mudsong's study folder are read once, pinned by digest, and only numbers taken from them are committed, with where they came from (`INT-4`).

The compatibility promise is narrow: the exported SVG, as read by the VectorCraft version mudsong uses, keeps every guide's name and position. Nobody has yet checked what VectorCraft keeps on import. AniCraft's importer, read but not run, drops group structure and hidden elements. The controlling sources and the oracle (import, then read the names and coordinates back) are fixed before anything is built on the format.

## What this cannot show

Other groups of characters, ages or species. Bodies. A control's safe range. Which combinations of proportions go together. Whether a wider batch moves the ranges. Whether the side view is right beyond Jon's look: nothing hand-made exists to check it against.

## Stops

The limits in `01-mission.md`, and `OPS-4`.
