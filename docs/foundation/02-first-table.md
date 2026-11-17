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

Maquette exports Saffi's accepted armature at a tagged commit. Mudsong copies the files into its own study folder, records the commit and imports them into VectorCraft itself. Maquette does not depend on mudsong's working tree. Jon approved one exception on 2026-11-16: the hand-made drawings in mudsong's study folder are read once, pinned by digest, and only numbers taken from them are committed, with where they came from (`INT-4`). He ruled the same day that her comparison case takes the ears and the lids of her two final drawings there in place of the S2 underdrawing's; those are read the same way. `data/saffi/README.md` lists every drawing maquette relies on, with its digest and its standing.

The compatibility promise is narrow: the exported SVG, as read by the VectorCraft version mudsong uses, keeps every guide's name and position. The SVG contract below says what that takes. The handoff is proven only when mudsong has imported the files and read the names and positions back itself.

## The SVG contract (`FID-1`, `FID-2`, `FID-3`)

Proposed by the agent on 2026-11-16 and accepted by Jon the same day, in pull request #11. Every statement about VectorCraft here was observed that day by running its MCP server. The probes, the read-backs and every result are in `design/svg-contract/`.

**Version.** Observed on `vectorcraft-cli` 0.24.1, headless: the binary on Jon's machine that day, whose digest is in the probe folder. Mudsong starts the same command on the same machine, so it runs the same binary. Its foundation records 0.24.0, which was not tested. VectorCraft is updated almost daily, so the contract is not pinned to this version (Jon, 2026-11-16). The oracle is what holds it to the installed one: it runs each time an armature is put to Jon or exported, against whichever version is installed, and its read-back names that version. The probes are rerun when the oracle fails, or when the armature needs something outside the list below.

**Sources, in order.** What this version does when run; then VectorCraft's `docs/mcp.md`; then SVG 1.1, which the armature must still be valid as. The earlier wins. The probe folder names the exact revision of each and records each disagreement. Jon rules on what they leave open.

**What the armature uses.**

- *Size:* `width` and `height` as plain numbers equal to a `viewBox` that starts at `0 0`. Other units or another `viewBox` change every coordinate.
- *Names:* the `id` alone, in lower-case letters, digits and hyphens, starting with a letter, once per file. It arrives unchanged as the object's name. `data-name`, `inkscape:label` and `aria-label` would replace it.
- *Guides:* `path` with one subpath of `M`, `L`, `C` and `Z`; `line`; `polyline`; `circle`; `ellipse`. Each arrives as a path whose anchors can be read. Arcs, transforms, `use` and paths of several subpaths arrive approximated, rounded or as pieces without names, and are left out.
- *Paint:* a stroke with `fill="none"`, as attributes on the element or a group above it.
- *Structure:* every top-level element is a `g` with an `id`, which arrives as a layer. Groups nest, each with an `id` and at least one guide.

Nothing is hidden in the file. VectorCraft drops an element with neither fill nor stroke, an unfilled element whose stroke is of width 0, an element with `visibility="hidden"` and an empty group, and reports no warning.

**The oracle.** Open the SVG with the server's `open_file`, read the tree with `inspect_document` and each path's points, which VectorCraft calls anchors, with `document.node`, and compare with the landmark file. It passes when the import reports no warning; the names read back are the landmark file's names for its layer, its groups and its guides, each once; every point the landmark file says a guide passes through is within 0.001 unit of the anchor read back, and a position that no guide passes through is not in the SVG, so it is named in the read-back and not compared; and every guide is visible, stroked and unfilled. A circle or an ellipse comes back as four anchors at its right, bottom, left and top, with none at its centre, so its centre and radii are checked through those four points. Coordinates come back at single precision, at most 0.00005 unit off in the generated views of 2026-11-16. Expected values come from the landmark file, never from the SVG. The oracle is a mechanical validator (`AUTH-3`): it shows that VectorCraft keeps what the landmark file says, not that the landmark file is right (`EVID-4`). It runs locally, and its read-back is shown in the pull request.

**When a guide does not survive.** The check fails, and the armature is neither put to Jon nor exported. Nothing is renamed, loosened or passed with a note. The generator's output returns to the list above, or the contract is revised from new probes and Jon accepts the revision.

**The hard case** the generator's output must pass first (`FID-2`) was chosen from what the probes showed to be fragile, not from any design of the generator. It is a three-quarter view whose positions are not round numbers:

- every guide at once, in nested groups, with one name contained in another;
- left and right pairs whose two members stand in different places, and differ in shape wherever the head model draws the two sides differently;
- closed guides, which must arrive unfilled;
- after import, one member of a pair hidden by its name, and only it hidden;
- two spoiled copies that the oracle must fail: one with a guide given `visibility="hidden"`, one with a pair's names exchanged. The second reads back with the right set of names, and only positions show it.

`design/svg-contract/in/hard-case.svg` is a hand-written stand-in that passes. Its numbers are those of the stand-in armature in `design/first-table-sheets/`, which follows Saffi's hand-made three-quarter view, with decimals added. It proves nothing about the generator.

**The other direction.** VectorCraft exports an authored shape with its name in `data-name` whenever the name is not already a valid `id`, with positions rounded to three decimals unless seven are asked for, and without hidden objects unless they are asked for. A reader takes the name from `data-name` first. The hard case for this direction is chosen when a ticket first uses an authored shape.

**Not tested.** Any version but 0.24.1, the desktop app, mudsong's own import, and anything the armature does not use.

## What this cannot show

Other groups of characters, ages or species. Bodies. A control's safe range. Which combinations of proportions go together. Whether a wider batch moves the ranges. Whether the side view is right beyond Jon's look: nothing hand-made exists to check it against.

## Stops

The limits in `01-mission.md`, and `OPS-4`.
