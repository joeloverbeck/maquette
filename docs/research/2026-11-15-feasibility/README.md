# Feasibility research, 2026-11-15

Reference-only evidence behind the founding. It authorizes nothing and is not a design. `docs/foundation/04-evidence-and-references.md` says what maquette may claim from it.

| File | What it is |
|---|---|
| `report.md` | The synthesis: whether a slider-driven anime character generator that emits SVG is feasible |
| `notes/prior_art_character_makers.md` | Existing character makers and avatar generators |
| `notes/datasets_detectors_ml_routes.md` | What can be measured from drawings automatically, and the machine-learning alternatives |
| `notes/proportions_and_craft_practice.md` | Published proportion numbers, and how studios keep a character on model |
| `notes/parametric_vector_construction_and_views.md` | Ways to build a parametric vector model with agreeing views |
| `notes/animation_readiness_and_game_use.md` | What AniCraft keeps from an imported SVG; structure for animation and game sprites |
| `notes/legal_and_licensing.md` | Copyright and licensing of measuring published drawings. Not legal advice |

## How to read it

- **Scope.** It was written for a wider tool that also drew finished characters and handed them to AniCraft. Maquette makes armatures only, so the parts on line quality, hair design, clothing and animation hand-off describe work maquette does not do.
- **Method.** Six research passes by AI agents on 2026-11-15, each saving notes with sources, then one synthesis written from the notes alone.
- **Nothing was built or run.** Every statement about a tool's behaviour is a reading of its documentation or code.
- **Evidence marks.** Each note tags a finding as read on a primary source or taken from a search summary. The report flags the search-summary claims that bear on a decision.
- **Known gaps.** The closest prior project was not read at source level. The patent on 2.5D cartoon models was not examined. The 3D posing mannequins that artists draw over were not surveyed. The law on drawn figures of minors was not researched.
- **Paths.** `~/src/anicraft` is the local checkout of AniCraft 0.3.0 that one note read.
