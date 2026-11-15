# 01 · Mission

## What maquette is

Maquette generates armatures for anime-style characters. An armature is the construction guide a drawing is built on: the skull, the face outline, guides for the eyes, brows, nose, mouth and ears, the neck, and later the volumes of the body. It comes as SVG, with a file giving the coordinates of every named landmark. An agent then draws and paints the character over it, and the armature is hidden in the finished art.

## Purpose (`PURP-1`)

- **Purpose class:** necessary support (Jon, 2026-11-16).
- **Benefit that justifies it:** a character's armature reaches Jon's acceptance in fewer looks than measuring and drawing each one by hand, and its views agree with each other.
- **Displaces:** sessions of mudsong's work other than its portrait run. Jon ruled on 2026-11-16 that Saffi's portrait continues in mudsong on the hand-made armature he accepted on 2026-11-15. `arkham-unfolded` and `netrunner-unfolded`, the other unfinished repositories that mudsong's mission names, continue beside it.
- **Initial appetite:** three completed tickets and six looks, to reach the first accepted armature (`02-first-armature.md`).
- **Expansion event:** Jon accepting that armature. It permits him to select the next real character, and nothing else.

## Primary outcome (`PURP-2`)

For a named character, Jon gets an armature in front, three-quarter and side views that he accepts on sight as that character's proportions, and the agent that paints over it produces a drawing whose proportions he does not send back.

Jon selected the form on 2026-11-16: a character creator, in which he or an agent moves controls for the face shape, the eyes, nose, mouth, ears and other parts. That selection is part of the outcome. It waives the comparison of mechanisms in `PURP-4` and nothing else. Whether an armature improves what is painted over it is still to be shown (`SEARCH-4`, `SIM-2`), and everything inside the creator stays replaceable.

## Non-goals

Painting, colour, line style, hair design, clothing, expressions, rigging and animation. Finished characters. A general dataset of anime proportions. Anything in `03-direction.md` that has not been earned.

## The mannequin rule

Bodies are mannequins at every age: a smooth surface with no genitals, nipples or other anatomical detail, and nothing sexualised. A base-garment guide is on by default. Clothes are the painter's work, drawn over the body.

## Consumer and real input (`PURP-6`, `EVID-5`)

The one consumer is mudsong's character art. Its next use of an armature is the model sheet for Saffi Two-Tides (mudsong #72, stage S6), which needs a front view that agrees with her three-quarter view.

The real input is Saffi as mudsong has settled her: an adult woman of the cat-folk, human-faced, with round ears set on the skull, drawn in style family A (modern television cel). Her working proportions for family A are in mudsong #69. Mudsong is private, so a ticket here restates the facts it needs and says where they came from.

## Inherited state (`PURP-1`, `EVID-1`)

Maquette extracts the armature step from mudsong's portrait run, so that run's evidence comes with it. The record is mudsong #69, read on 2026-11-16.

**Supported by the record.**

- Jon, on the first busts: "the base outline is just wrong. The head is too big for the face."
- Building the armature first worked. An armature on life-drawing proportions was correct, and the face still felt "off".
- Armatures built from proportions measured on about ten published faces per style family were accepted: four three-quarter armatures, families A to D, in two looks. Jon chose family A for Saffi.

**Contradicted by the record.** That the armature was the only fault. The same issue names a modelled nose and lips where the style draws a mark and a line, and eyes that were "markedly wrong for her". The outcome above is therefore written to the narrower claim: maquette answers for proportions, not for whether a face reads as the character.

**Not established.** A front or side view. A body. Any character but Saffi. That one family's proportions serve another character. That a generated armature is as good as the hand-made one.

The portrait run keeps its own appetite and state in mudsong. Nothing here discharges it.

## One pipeline (`BOUND-1`)

Values for a named character and a proportion table → the generator → armature SVG and landmark file → mechanical checks, then Jon's look → export at a tagged commit → mudsong. Jon maintains and accepts it. Parking or abandoning it leaves mudsong on its hand method.

Measuring reference drawings by hand to fill a proportion table is a step of this pipeline. A system that collects and measures a corpus at scale would be a second pipeline, and `BOUND-2` decides where it lives before any of it is built.

## Authority

Jon is the acceptance authority. Only he decides whether an armature looks right, which published work counts as a competent reference, which style family a character is drawn in, where a character sits between human and animal, and what is published.

## Limits

- **Allocation (`WORK-3`):** the default, three completed tickets without an accepted increment.
- **Looks (`EVID-8`):** six. A look is one review sheet Jon is asked to judge. Accepted, rejected and inconclusive looks all count.
- At either limit the next step is Jon's terminal decision (`WORK-5`).

## Work control (`GOV-2`)

GitHub Issues and pull requests in the public repository `joeloverbeck/maquette`.
