# 02 · The first armature

## Bottleneck and primary path (`WORK-1`)

Saffi has one accepted armature, three-quarter right, made by hand. No front or side view exists, and mudsong's method would measure each new view separately, from drawings that are seldom level and straight on. The bottleneck is an armature whose views agree. The primary path is the shortest one to Saffi's head in three agreeing views, accepted by Jon.

## Incumbent (`PURP-4`)

Without maquette, an agent in mudsong measures about ten published faces of the style family, draws the armature in VectorCraft and shows Jon a sheet.

- Observed on 2026-11-15: two looks to four accepted three-quarter armatures.
- Observed by Jon on 2026-11-16: the agent took proportions from drawings that looked slightly down, as if they were level.

The first armature must keep the incumbent's result, which is that Jon accepts the three-quarter view. It is judged on three things: looks to acceptance, agreement between views, and the work the next character would need.

## The increment

Saffi's head, family A, in front, three-quarter right and side views, with her ears, produced by a program from one set of values. Each view is an SVG of named guides with a landmark file beside it. Jon selected the three views. Mudsong's S6 uses the front and the three-quarter; the side has no consumer there yet.

It is built as the least complex thing that can do this: fixed values for one character, and no controls, part types, statistics or interface. Those come from `03-direction.md` only when a second real character shows they are needed (`EVID-5`).

## What it is checked against

- **The accepted hand-made armature.** The three-quarter view is laid over it, or over its table of working proportions from mudsong #69. It is the independent expectation (`EVID-4`): the program does not supply its own answer.
- **Mechanical checks.** Each ratio equals family A's table, and each view's landmarks are the same points seen from another side. These are validators (`AUTH-3`) and prove only that.
- **An agent's look at the render.** A diagnostic. Finding nothing wrong establishes nothing (`EVAL-2`).
- **Jon's look.** The acceptance.

## The claim about painting (`SEARCH-4`, `SIM-2`)

That an armature improves the drawing made over it is a separate claim. Its evidence is the same bust painted with the armature and without it, from the same brief, more than once each, with Jon comparing them. One good painting does not carry it. Until that comparison is made the claim is unproven, and nothing may rest on it.

## The handoff (`INT-1`, `FID-1`)

Maquette exports the armature files at a tagged commit. Mudsong copies them into its own study folder, records the commit and imports them into VectorCraft itself. Maquette never reads mudsong's working tree.

The compatibility promise is narrow: the exported SVG, as read by the VectorCraft version mudsong uses, keeps every guide's name and position. Nobody has yet checked what VectorCraft keeps on import. AniCraft's importer, read but not run, drops group structure and hidden elements. The first ticket fixes the controlling sources and the oracle (import, then read the names and coordinates back) before anything is built on the format.

## What this cannot show

Other characters, ages or species. Bodies. A control's safe range. Whether tables measured from many works help. Other style families.

## Stops

The limits in `01-mission.md`, and `OPS-4`.
