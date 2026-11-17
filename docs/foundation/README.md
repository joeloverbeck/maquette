# Foundations

Repository-specific foundations for `maquette`, written at founding on 2026-11-16 and revised the same day when Jon reoriented the first increment. They specialize the constitution at `docs/principles/principles-for-repos.md` with this repository's facts and record what Jon decided, so that no session has to work it out again. They authorize no work: GitHub Issues and pull requests do (`GOV-2`).

| Document | What it settles | Read it before |
|---|---|---|
| `01-mission.md` | What maquette is; purpose class, benefit, displacement and appetite; the primary outcome and non-goals; the mannequin rule; the consumer and the two real inputs; what mudsong's record supports; the two pipelines; authority; limits | Accepting, activating or scoping any work; continuing near a limit; a terminal decision; a merge |
| `02-first-table.md` | The bottleneck; the incumbent; the first increment and what it is checked against; the unproven claim about painting; the handoff to mudsong; the SVG contract | Writing or working the first tickets; measuring a reference; writing or reading an armature's SVG; exporting anything to mudsong; judging what an accepted armature or table shows |
| `03-direction.md` | What Jon wants later, what is known about each part and what would earn it. It authorizes nothing | Proposing controls, part types, non-human parts, more views, bodies, a wider batch of references or an interface |
| `04-evidence-and-references.md` | What the research lets maquette say; the rules for reference drawings and tables; what is never committed; how to ask for Jon's eyes | Using a research finding, a reference drawing or a measurement; committing any data |
| `05-code-and-design-tools.md` | Rust, with TypeScript only for the web interface's thin layer; no Python; what VectorCraft and FigCraft each make; their MCP servers; where designs live | Writing code or tooling; adding a dependency; drawing, laying out or mocking up anything |

## Founding record

**Constitution.** Revision 2026-11-15.1, copied byte for byte from the tag of that name in `joeloverbeck/principles-for-repos`. The founding read the complete core and all six modules.

| Module | Adopted | Why its trigger fires, or does not |
|---|---|---|
| `fidelity.md` | yes | The armature must stay compatible with SVG as VectorCraft reads it. "Matches measured anime proportions" is also a fidelity claim, which `FID-4` keeps apart from usefulness. |
| `agency-simulation.md` | yes | The benefit is a counterfactual claim, that a character painted over an armature comes out better than one painted without, and the painter varies from run to run (`SIM-2`). |
| `search-adaptation.md` | yes | The armature is an upstream stage credited for a result that painting transforms (`SEARCH-4`). Fitted tables and any score come later (`SEARCH-3`, `SEARCH-5`). |
| `evaluators-scores.md` | yes | An agent that reads proportions off a drawing is an artificial evaluator (`EVAL-2`). An "animeness" number would be a composite score (`EVAL-1`). |
| `integration.md` | yes | Armatures are exported to mudsong, and private reference images are mounted from outside (`INT-1` to `INT-4`). |
| `longitudinal.md` | no | No state here is stranded when a participant disappears. |

**Decided by Jon.** In the founding interview of 2026-11-16:

- The name, maquette.
- Purpose class necessary support. Saffi's portrait continues in mudsong on the hand-made armature, and maquette takes sessions from mudsong's other work.
- Public from the start.

And in conversation on 2026-11-15 and 2026-11-16: armatures only, with no painting, clothes or animation; front, three-quarter and side views; characters of all ages; and the direction recorded in `03-direction.md`.

And by his rulings after the interview, on 2026-11-16: all code is Rust except the thin layer of TypeScript the web interface needs; the interface is a web app; every design made by hand is made in VectorCraft or FigCraft, whose MCP servers are registered in this repository (`05-code-and-design-tools.md`).

**Proposed by the agent and accepted by Jon.** He accepted the founding whole on 2026-11-16, at commit `6460d14`. These are the parts that were the agent's proposals and not his own decisions. The reorientation below replaced item 3 and the appetite.

1. The module set above.
2. The wording of the primary outcome and the non-goals (`01-mission.md`).
3. The first armature, its checks and its handoff.
4. The mannequin rule (`01-mission.md`).
5. The rules for reference drawings, and what is never committed (`04-evidence-and-references.md`).
6. Measuring references by hand is a step here; a corpus-scale measuring system would be a second pipeline.
7. What counts as a design made by hand, and that the two servers write only under `design/` (`05-code-and-design-tools.md`).
8. The MIT licence, as in Jon's other public repositories.
9. Who merges (`CLAUDE.md`).

## Reorientation record, 2026-11-16

The revision read the complete core and the five adopted modules, and mudsong #69 again. No module's trigger changed.

**Decided by Jon,** in the interview that prepared the first spec. Each ruling is recorded where it applies.

- Measured references are the base, and Saffi's accepted armature is the comparison case, never an input (`01-mission.md`).
- The first increment is the first measured table, drawn, and its appetite is that increment and eight looks (`01-mission.md`, `02-first-table.md`).
- Measuring and generating both stay in this repository (`01-mission.md`).
- The turn of the three-quarter view, the hair's volume and the shoulder line, the first batch, and the one read of mudsong's hand-made drawings (`02-first-table.md`).
- References are checked on a sheet with every drawing read twice, and a valid range is the lowest and highest value measured (`04-evidence-and-references.md`).
- The interface and every control stay out of the first increment (`03-direction.md`).

**Proposed by the agent and accepted by Jon** with this revision:

1. The six parts of the increment and what it is checked against (`02-first-table.md`).
2. The corrections to what mudsong's record supports (`01-mission.md`).
3. Rules 5 to 7 for references, and the five-work floor for a view (`04-evidence-and-references.md`).
4. That a Rust tool renders reference check sheets from a layout designed in FigCraft (`05-code-and-design-tools.md`).

## SVG contract and patent record, 2026-11-16

The revision read the complete core and the five adopted modules. No module's trigger changed.

**Decided by Jon** on 2026-11-16, after a summary of the reading of its independent claims: maquette disregards US patent 8,791,942. It constrains no work, and the question about it leaves `03-direction.md` (`04-evidence-and-references.md`). He ruled with his acceptance that the contract is not pinned to a VectorCraft version, because VectorCraft is updated almost daily (`02-first-table.md`).

**Proposed by the agent and accepted by Jon** on 2026-11-16, in pull request #11:

1. The SVG contract (`02-first-table.md`): the version it was observed on, the order of its sources, what the armature uses, the oracle and its tolerance, what happens when a guide does not survive, and the hard case.
2. The removal of the sentence about AniCraft's importer. It was read and never run, and the contract covers VectorCraft only.
3. The sentence in `04-evidence-and-references.md` that says Saffi's hand-made views are committed as numbers, in `data/saffi/`.

## SVG contract wording, 2026-11-16

Running the oracle in Rust on the generator's own output for #5 found three sentences of the accepted contract that could not be run as written, and one observation the new runs made false.

**Proposed by the agent and accepted by Jon** on 2026-11-16, with the armature, in pull request #12 for issue #5. All in `02-first-table.md`:

1. The names the oracle compares are the landmark file's layer, groups and guides, not its guides alone.
2. The oracle compares every point a guide passes through. A position no guide passes through is not in the SVG, so the read-back names it and does not compare it.
3. In the hard case, left and right pairs differ in place, and in shape wherever the head model draws the two sides differently. The two irises are the same ellipse in two places.
4. Coordinates came back at most 0.00005 unit off in the generated views, not 0.00004.

## Look 1, 2026-11-16

Jon accepted Saffi's generated front and three-quarter views at look 1 (of 8), in pull request #12: they read as one head, and as her armature. He also ruled that the shoulder line turns with the head. Her hand-made three-quarter view, whose shoulders turn about 4° against the head's 29°, is wrong on that point, so differences there come from the hand-made drawing and not from the generator.

## Review sheets, 2026-11-16

At look 1 Jon could not tell from the sheet whether its percentages came from published references. **Decided by Jon** after the retrospective that followed: a review sheet says what every number on it is measured against (`04-evidence-and-references.md`, "Asking for Jon's eyes").

## Look 2, 2026-11-17

Jon did not accept Saffi's side view at look 2 (of 8), in pull request #14. The sheet showed her three views, with the ears and lids of her final drawings as he had ruled on 2026-11-16. **What Jon said:** the front and three-quarter views read as one head; from the nose down the side view is pushed too far forward; and where did its numbers come from, when her drawings have no side view?

They came only from her hand-made three-quarter view. A depth is a sideways shift there over the sine of 29°, so what that drawing does with the mouth and the chin arrives in the side view doubled. Her three-quarter view stays as accepted, and the repair was the first named correction, `lower_face_set_back`: in the side view only, the chin and the mouth are drawn further back than their depths. `proportions/list.toml` defines it and the rule of each amount. The rules come from drawing lessons, which give no distances, and no rule gives the share of the chin's full amount.

## Look 3, 2026-11-17

Jon was shown the side view at two shares, 0.5 as candidate A and 1 as candidate B, and chose A at look 3 (of 8), in pull request #14. **What that accepts:** Saffi's armature in three views. They are the front and three-quarter views of look 2, and the side view with `lower_face_set_back` drawing the chin 20.9 units back (4.2% of head height) and the mouth 18.2 (3.7%). The six files are in `armatures/saffi/`, a test pins their bytes, and issue #6 records the acceptance with their digests.

**What it does not:** candidate B, her proportions as a standard for another character, or any table. The side view's only check was his look. Looks used: three of eight, two accepted and one not.

## The GOV-1 count

Five foundations and this index. The count was about 4,100 words at founding, about 5,500 after the reorientation, about 6,700 with the SVG contract, about 6,900 after look 1 and is about 7,500 after look 3, over the 5,000 at which `GOV-1` asks for a recheck. The recheck: every addition records a ruling Jon made on 2026-11-16 or 2026-11-17 or a fact the first increment needs, and none describes work beyond that increment except in `03-direction.md`, which authorizes nothing. The contract's consumer is issue #5, which builds the generator's output on it, and its detail stays in `design/svg-contract/`. The research under `docs/research/` is reference-only source evidence and is not counted. A later addition to this directory needs an immediate decision or execution consumer, named in the pull request that adds it.
