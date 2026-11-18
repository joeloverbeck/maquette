# 01 · Mission

## What maquette is

Maquette generates armatures for anime-style characters from proportions measured on good drawings that Jon rates. An armature is the construction guide a drawing is built on: the skull, the face outline, guides for the eyes, brows, nose, mouth and ears, the neck, and later the volumes of the body. It comes as SVG, with a file giving the coordinates of every named landmark. An agent then draws and paints the character over it, and the armature is hidden in the finished art.

## Purpose (`PURP-1`)

- **Purpose class:** necessary support (Jon, 2026-11-16).
- **Benefit that justifies it:** a character's armature is built inside proportions measured on many good references, in views that agree, without measuring and drawing each character by hand.
- **Displaces:** sessions of mudsong's work other than its portrait run. Jon ruled on 2026-11-16 that Saffi's portrait continues in mudsong on the hand-made armature he accepted on 2026-11-15. `arkham-unfolded` and `netrunner-unfolded`, the other unfinished repositories that mudsong's mission names, continue beside it.
- **Appetite:** the increment in `02-first-table.md`, as its spec states it, and eight looks (Jon, 2026-11-16).
- **Expansion event:** Jon accepting the first table and Saffi's armature. It permits him to select what comes next: more references, another group of characters, or the controls. It permits nothing else.

## Primary outcome (`PURP-2`)

Jon builds a character's armature inside proportions measured on good drawings he rates, in front, three-quarter and side views that he accepts on sight, and the agent that paints over it produces a drawing whose proportions he does not send back.

Jon selected the form on 2026-11-16: a character creator, in which he or an agent moves controls for the face shape, the eyes, nose, mouth, ears and other parts. He ruled the same day that measured references are its base, and that Saffi's accepted armature tests what the creator allows and never feeds it. Those selections are part of the outcome. They waive the comparison of mechanisms in `PURP-4` and nothing else. Whether an armature improves what is painted over it is still to be shown (`SEARCH-4`, `SIM-2`), and everything inside the creator stays replaceable.

## Non-goals

Painting, colour, line style, hair design, clothing, expressions, rigging and animation. Finished characters. A general dataset of anime proportions. Anything in `03-direction.md` that has not been earned.

## The mannequin rule

Bodies are mannequins at every age: a smooth surface with no genitals, nipples or other anatomical detail, and nothing sexualised. A base-garment guide is on by default. Clothes are the painter's work, drawn over the body.

## Consumer and real input (`PURP-6`, `EVID-5`)

The one consumer is mudsong's character art. Its next use of an armature is the model sheet for Saffi Two-Tides (mudsong #72, stage S6), which needs a front view that agrees with her three-quarter view.

There are two real inputs.

- **References:** drawings Jon rates, whatever their source, by several hands, each checked for its view and tilt (`04-evidence-and-references.md`). They fill the proportion tables.
- **Saffi,** as mudsong has settled her: an adult woman of the cat-folk, human-faced, with round ears set on the skull. Her hand-made armature, a front view and a three-quarter view from mudsong's portrait study, is the comparison case.

Mudsong is private, so a ticket here restates the facts it needs and says where they came from.

## Inherited state (`PURP-1`, `EVID-1`)

Maquette extracts the armature step from mudsong's portrait run, so that run's evidence comes with it. The record is mudsong #69, read twice on 2026-11-16: at the founding, and again for the reorientation, which corrected two of the founding's statements.

**Supported by the record.**

- Jon, on the first busts: "the base outline is just wrong. The head is too big for the face."
- Building the armature first worked. An armature on life-drawing proportions was correct, and the face still felt "off".
- Armatures built from proportions measured on published faces were accepted: four three-quarter armatures, families A to D, in two looks. Ten faces were measured across the four families. Family A's three came from one series. Jon chose family A for Saffi.
- A hand-made front view of A was on the sheet Jon accepted. The question put to him covered the four three-quarter armatures only.

**Contradicted by the record.**

- That the armature was the only fault. The same issue names a modelled nose and lips where the style draws a mark and a line, and eyes that were "markedly wrong for her". The outcome above is therefore written to the narrower claim: maquette answers for proportions, not for whether a face reads as the character.
- That family A had a table. Three faces from one series are not one (`04-evidence-and-references.md`, rule 6).

**Not established.** A side view. A front view Jon accepted. A body. Any character but Saffi. That one family's proportions serve another character. That a generated armature is as good as the hand-made one. That Saffi's proportions are good ones: Jon accepted them as good enough for now (2026-11-16).

**Established since, in maquette.** This is maquette's own evidence and no part of mudsong's record. At look 3, on 2026-11-17, Jon accepted Saffi's generated armature in three views (issue #6; the files are in `armatures/saffi/`). The side view carries the named correction `lower_face_set_back` at the amounts in `data/saffi/proportions.toml`. So she has a front view and a side view Jon accepted. The rest of the list above stands: a body, any character but Saffi, that one family's proportions serve another character, that a generated armature is as good as the hand-made one, and that her proportions are good ones. At look 4, the same day, Jon accepted the readings of a first batch of references: six works, a front view of each and a side view of three. The first table built from them was not accepted at look 5, for the side view's nose, and had no verdict at look 6, where he sent the work to more references. He gathered a second batch himself and accepted most of its characters at look 7, on 2026-11-18. At look 8, the same day, he accepted the table built from both batches as measured, with what it counts: 20 characters of 12 works, adult women and teenage girls, in `tables/women-and-teenage-girls/`. That is the first accepted table. It accepts no armature of a character but Saffi: Jon found the side view of the head drawn from it better than before, and still wrong at the back of the jaw. Mudsong has not yet used the files.

The portrait run keeps its own appetite and state in mudsong. Nothing here discharges it.

## Two pipelines, one repository (`BOUND-1`, `BOUND-2`)

- **Measuring:** references → readings by an agent, checked by Jon → a proportion table, which Jon accepts.
- **Generating:** a set of proportions → the generator → armature SVG and landmark file → mechanical checks, then Jon's look → export at a tagged commit → mudsong.

Jon ruled on 2026-11-16 that both stay here. The tables have no consumer but the generator, the measuring tools exist only to fill them, and he would park or abandon both together. Reference images stay outside the repository. If a table gains another consumer, or measuring needs work that the generator never uses, the ruling is reopened (`BOUND-4`). Parking or abandoning maquette leaves mudsong on its hand method.

## Authority

Jon is the acceptance authority. Only he decides whether an armature looks right, which drawing counts as a reference, whether a reference is level and in the view stated, which style family a character is drawn in, where a character sits between human and animal, and what is published.

## Limits

- **Allocation (`WORK-3`):** the default, three completed tickets without an accepted increment.
- **Looks (`EVID-8`):** eight. A look is one review sheet Jon is asked to judge. Accepted, rejected and inconclusive looks all count.
- At either limit the next step is Jon's terminal decision (`WORK-5`).

## Work control (`GOV-2`)

GitHub Issues and pull requests in the public repository `joeloverbeck/maquette`.
