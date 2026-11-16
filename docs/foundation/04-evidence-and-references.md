# 04 · Evidence and references

## The research

`docs/research/2026-11-15-feasibility/` holds a report and six notes from 2026-11-15. It is reference-only evidence. It was written for a wider tool that also drew finished characters, nothing in it was built or run, and several of its claims rest on search summaries, which the notes mark. A paper, product or architecture in it shows only what was observed under its own conditions (`PURP-2`).

`docs/research/2026-11-16-views-and-measuring/` holds three notes and one test from the reorientation: how heads are constructed in side and three-quarter views, where level references are published, and how far an agent's reading of a drawing can be trusted. It is reference-only evidence too.

## What it lets maquette say

| Finding | Limit |
|---|---|
| Published measurements exist: a face divided 0.29 / 0.37 / 0.12 / 0.21 from hairline to chin, with an eye opening of 0.16 and a nose width of 0.12 of face height (Liu et al. 2019). | 100 film characters in a conservative style. Mudsong's family tables are nearer the consumer. |
| Measured proportions decide which category a face reads as: anime, an age band, a build. | They do not supply appeal, the eyes or the other views. No study tested this directly. |
| Age shows in the ratios between 10 and 18. | No measurements of adult women, young children or the old were found. |
| Anime landmark detectors work on near-frontal faces only and publish no accuracy. On a pointing benchmark, general vision models put a point inside the right region 30% to 47% of the time. | Neither is evidence without cases Jon has checked (`EVAL-2`). |
| The closest existing project is `rhedak/anime-character-creator` (MIT), which draws front-view characters in code. | Its source was read on 2026-11-16 and not run. It has no depth, no second view, no landmark file and no named parts, so there is nothing to fork. |
| Drawing lessons agree that in a side view the ear sits at or just behind the middle of the head's depth, between the brow and the base of the nose. | No source gives an anime head's depth as a number, or measures how drawn three-quarter and side views depart from geometry. |
| Official sites publish level front heads freely. Level three-quarter heads are rare, and side heads are mostly in official setting-material books. | Seven sources, picked by the agent that looked. No head sheet of an adult woman was among them. |
| An agent read 52 points off five clean hand-made drawings to within 1.5% of head height, unaided. | Published drawings are harder, and judging whether a head is level was not tested. |
| MIT lists US patent 8,791,942 on 2.5D cartoon models. | Not examined. Check it before publishing a method that blends authored views around points in space. |

Matching measured sources and helping Jon are separate claims with separate evidence (`FID-4`). A table can be faithful to its sources and still give an armature he rejects.

## Reference drawings

A reference is a published drawing whose proportions are measured for a table.

1. **Jon decides which work is competent.** He has named Delicious in Dungeon and Studio Ghibli's films. For a batch he has selected, an agent proposes works with what each official source shows, and he strikes any he does not rate (`EVID-7`).
2. **The view is stated and checked.** Each reference is front, three-quarter or side, with the head level and upright and the expression neutral. Only official publications are sources, and official model sheets are preferred, because they are drawn that way on purpose. Jon looks before a reference's numbers count: he saw mudsong's agent measure heads that looked slightly down as if they were level.
3. **A tilted or turned head is rejected, or measured as what it is.** It is never averaged in as level. A three-quarter drawing is measured at the turn it actually shows.
4. **What each reference is, is noted:** the work, the view, and the character's age band, sex and species.
5. **An agent's reading is an artificial evaluation** (`EVAL-2`). It is trusted only as far as it has matched drawings whose positions are known and references Jon has checked. Separate agents read every reference twice, and Jon is shown where they disagree.
6. **A table is measured over several works by several hands.** One series, one artist or one character is not a table, and a view backed by fewer than five works is too thin to be one. Family A's proportions in mudsong came from three faces of one series, so they are not a table and are not brought here.
7. **The comparison case stays out.** Saffi's numbers never enter a table.

## What is committed, and what never is

Committed: the tables, and the list of works each was measured from. A table gives each proportion's lowest, middle and highest value and the number of works behind it.

Never committed, because the repository is public and the works are other people's:

- reference images, or crops, tracings and overlays of them, and so no reference check sheet;
- the reading of a single reference, or any landmark set or settings that reproduce one identifiable character;
- anything named after a character, series, studio or artist. Family names are descriptive: A, "modern television cel".

Reference images are kept outside the repository, in a folder named by configuration. When it is absent, measuring stops with a plain message and everything else still works (`BOUND-5`, `OPS-7`).

The legal reading behind this is in the research's `notes/legal_and_licensing.md`. It is background research, not legal advice.

## Asking for Jon's eyes (`EVID-8`)

Every request names the decision it serves, why no check can make it, how many drawings it covers and when it stops. Looks are counted against the ceiling in `01-mission.md`.
