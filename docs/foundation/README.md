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

## Look 4 and the first batch, 2026-11-17

The first batch of references is one adult woman from each of six works by six character designers, from official sources. No level three-quarter head was found in a free official source, and a side view was found for three of the six. **Decided by Jon:** the batch is read in front and side views, depth is reported as too thin, and no book is bought to fill a view.

At look 4 (of 8) Jon checked the batch on a reference check sheet and accepted all six characters: a front view of each and a side view of three. **What that accepts:** that those heads are level, in the view stated, and that the points sit on the right features as he sees them. **What it does not:** that a published head is read rightly, which nothing in the batch tests, or any table. Issue #9 records it.

## Look 5, 2026-11-17

The first table was built from that batch and put to Jon at look 5 (of 8), with 27 proportions given a valid range. **Jon did not accept it.** His worry was the side view's nose, and a face drawn at his request through the side view's points came out with a long pointed nose, a narrow wedge of a lower face and a thin neck set far back.

**Decided by Jon** on 2026-11-17: the head drawn for the look is Saffi's own head with the table's values put in where the references give them, and the sheet says so. A proportion that two to four works give is drawn at their average and marked as thin; what one work gives, or none, is borrowed from Saffi and marked as borrowed; and the head carries no side-view correction. The table as published states a number only where a proportion has a range. `design/look-5-first-table/` holds the sheet, the face and what he said.

## Look 6, 2026-11-17

Three repairs followed look 5, and the table was put to Jon again at look 6.

- **The tip of the nose has its own height.** The head model had drawn the tip's depth, which a side view gives, at the height of the front view's nose mark, which is the base of the nose. The list of proportions now has `nose_tip_drop`, taken from the side view. Without a measured side view the tip stands at the nose mark's bottom, so Saffi's accepted armature is unchanged.
- **Decided by Jon** after look 5: the jaw point is left out of the table until the landmark is redefined, because the readers placed it at the corner of the chin. The table gives 25 proportions a valid range.
- **The neck is borrowed whole,** as the ear is.

Depth had three works and was not a table, no three-quarter view was measured, and no skull is built. `design/look-6-first-table/` holds the sheet and says how the head is made.

**Jon gave no verdict on the table.** He found the face drawn over the side view generally good, and the jaw line where it meets the neck "clearly a disaster": a symptom of too few references. He sent the work to more of them, and ruled the same day that a reference is a drawing he rates, whatever its source (`04-evidence-and-references.md`). The first table was never accepted, and the table of both batches replaced it before anything was published. Issue #10 records the look.

## Look 7 and the second batch, 2026-11-18

Jon gathered a second batch himself, of adult women and teenage girls. **Decided by Jon:** the two are read alike and make one table, each drawing keeping its age band, and the table reports the two bands' ranges side by side. Before any of its side views was read, a side view was asked for the forehead, the bridge of the nose, the angle of the jaw and the neck's top ends, where look 6 had drawn Saffi's outline on a measured face (reading instructions revision 2026-11-17.4, retested on known positions first).

At look 7 (of 8) Jon checked 22 characters on a reference check sheet: the second batch's 19, and the first batch's three side views read again for the new points. He accepted 15 of the second batch's and the three side views, and rejected four. **Decided by Jon:** where the two readers of a drawing differ by 2.5% of head height or more on a point that only shapes a feature, such as a lid line's end or crest, an ear's base point, a neck's ends or the jaw's angle, that point is off on that drawing. Issue #18 records the look.

## Look 8, 2026-11-18

**Jon accepted the table of both batches** at look 8 (of 8), the last under the ceiling: `tables/women-and-teenage-girls/`, as measured and with what it counts. It counts characters, one drawing of each in each view, with the works behind each count beside it, and the floor of five holds on characters. Its README says what it was measured on and what it gives, and a test holds those numbers to the table. **What Jon said** of the head drawn from it: he is happier with its side view than with look 6's, and it still has problems, mainly the curve at the back of the jaw.

**What it does not accept:** an armature of any character but Saffi. Nor does it show that the bands differ or are alike: wherever both have a range, the ranges overlap, and six teenage characters are few. The angle of the jaw and the back of the neck are given by one character each, so the head leaves them out and the construction's curve stands in for them. Revision 2026-11-18.1 of the reading instructions is not yet retested, and no side view is read under it until it is. Looks used: eight of eight, five accepted (1, 3, 4, 7 and 8), two not (2 and 5) and one without a verdict (6).

## Look 9 and parking, 2026-11-18

After look 8 Jon said he would not gather enough references for tables, and asked for a trial (#22): Saffi built from one **structure reference**, a character of the second batch whose front, three-quarter and side views he had accepted, instead of a table's middles. It spent one look past the ceiling, look 9, the first extension of the appetite. Three candidates stood beside her accepted armature and the head of look 8:

- **A:** every number the reference's front and side views give, 47 of the list's 86 proportions, and Saffi's for 34.
- **B:** A with her eyes and brows: 27 from the reference, 54 from her.
- **C:** the reference's front view alone, with what a front view cannot show from the table's middles where it gives a range.

**What Jon said at look 9:** B works well. He is not convinced it should replace her accepted front view. The side view was terrible in all cases.

**Decided by Jon, the same day: maquette is parked,** and it and its private store of references are archived. His reasons: for mudsong and later apps that need characters of wildly different types, beast-folk among them, gathering many references to build averages, or a character creator on them, was probably the wrong approach. He would rather give an app a few references he rates, have it take the relationships between their points, such as how far apart the inner corners of the eyes stand or how far the nose's base is from the upper lip, and allow variation between those references. The code and the data for capturing face or body points may serve that later.

**What a successor carries forward** (`PURP-1`), if it continues this premise:

- **Supported.** Two agents reading named points on a drawing, with Jon checking each character on a sheet, gave the table of both batches he accepted at look 8; four readings of five hand-made drawings stood within 1.5% of head height of their known positions. The generator draws a set of proportions as named guides in three views that VectorCraft 0.24.1 reads back exactly. In front, one reference's structure with Saffi's own eyes and brows worked for her (B).
- **Failed.** A side view drawn by projecting the front view's widths and the side view's depths through one head model: Jon judged it terrible for all three candidates at look 9, rejected the first at look 2, and found problems at the back of the jaw at look 8. Only Saffi's own side view, carrying a named correction, was accepted (look 3). A table's middles as a base never gave a head Jon judged right (looks 5, 6 and 8).
- **Not established.** Any character but Saffi; a head that is not human; drawing each view from its own reference rather than projecting; blending a few references; the handoff to mudsong, which never imported the files (#7); and that an armature improves what is painted over it (`SEARCH-4`, `SIM-2`).
- **Spent.** The first increment's eight looks and the trial's one. Parking is the terminal decision; none is outstanding.

**A waiver of `PUB-1`, granted by Jon** under `ADOPT-2` on 2026-11-18: the trial's code reaches `main` although its method was never adopted. **Scope:** the four commits of #22 (`c69b7c7`, `d7d2bd5`, `99a7a4f`, `d9b385f`), merged with this record. **Reason:** salvage for a later app that captures face or body points. **Risk accepted:** code that no accepted increment rests on stands on the default branch of an archived repository. **It ends** with the archive. The command, `maquette set-from-structure-reference`, is a trial's code, not a product.

**Where things are.** Saffi's accepted armature is at the tag `saffi-armature-1`. The table of both batches is in `tables/women-and-teenage-girls/`. The reference images, readings, records of Jon's checks and the trial's sets, drawings, faces and sheet are in the private store of references, archived with this repository. Issues #3, #7 and #22 are closed with a pointer to this record, and mudsong's import ticket is closed there.

## The GOV-1 count

Five foundations and this index. The count was about 4,100 words at founding, about 5,500 after the reorientation, about 6,700 with the SVG contract, about 6,900 after look 1, about 7,500 after look 3, about 8,000 with looks 4 to 6, about 8,800 with looks 7 and 8, and is about 9,500 with look 9 and the parking, over the 5,000 at which `GOV-1` asks for a recheck. The recheck: every addition records a ruling Jon made between 2026-11-16 and 2026-11-18 or a fact the first increment needs, and none describes work beyond that increment except in `03-direction.md`, which authorizes nothing. The contract's consumer is issue #5, which builds the generator's output on it, and its detail stays in `design/svg-contract/`. The research under `docs/research/` is reference-only source evidence and is not counted. A later addition to this directory needs an immediate decision or execution consumer, named in the pull request that adds it. The parking record's consumer is the next session that opens this repository, and any successor under `PURP-1`: it states the terminal decision and the evidence they inherit.
