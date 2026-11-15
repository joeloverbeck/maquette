# 03 · Direction

What Jon said he wants from maquette beyond the first armature, on 2026-11-15 and 2026-11-16, with what is already known about each. It is recorded so that later work neither reinvents it nor contradicts it.

None of it is authorized work or a backlog (`GOV-1`). Each part becomes work only through a ticket Jon selects, when a real character needs it (`EVID-5`, `EVID-6`). Mechanisms named here are candidates, and the architecture in the research is a hypothesis, not a design (`PURP-2`).

## 1. Controls that cannot make a head look wrong

**Jon.** Head proportions should be adjustable "in a way that wouldn't make them look wrong", grounded in competent anime and manga. Either an "animeness" can be calculated from the measured sources, or there are styles. He prefers one flexible creator that draws on every accepted source.

**Known.** Mudsong measured four style families and their proportions differ; Jon accepted all four and chose one. In the research, the spread of eye size inside one style was half its average, a pooled average would centre on a teenage girl, and averaging more faces did not make them more attractive.

**Candidate.** One set of controls for every source. Each measured source is a point in that space and a family is a region of it. A setting is plausible when it lies near measured points, not when it lies near the overall average. Controls that vary together in the sources move together.

**Open, and Jon's to judge by eye.** Whether a setting halfway between two families looks right. That is the answer to "one animeness or several styles", and it can only be asked once three families are measured the same way.

A number that gates or ranks settings falls under `EVAL-1` and `SEARCH-5`.

## 2. Types of eyes, noses and mouths

**Jon.** Many selectable types, "from shifty to innocent".

**Known.** In mudsong the eyes were the fault Jon named once the proportions were right; he wanted them hooded. The research found no parametric anime eye to copy. Japanese drawing vocabulary names eye types (tsurime, tareme, jitome and others); that is a starting list to verify, not a finding.

**Scope.** Maquette supplies the guide for a part. For an eye that is the lid line, the tilt, the opening and the iris's share of it. The painted eye belongs to the painter. Each type needs a shape for each view.

**Earned by.** A second real character whose eyes differ from Saffi's.

## 3. Non-human characters

**Jon.** Beast-folk and others, with their ears and "possible other body parts".

**Known from mudsong's world.** Cat, beaver, badger, hyena, crane and newt folk, an oni and a goblin, with ears, tails, horns, tusks and a crown patch. Jon leans to human features with furred ears, markings and tails. Where each character sits stays open until drawn. Saffi is settled as human-faced.

**Two different things.** Parts attached to a human head or body (ears, horns, tails) are placed on the skull or the spine, so they turn with it. A different head (a muzzle, a newt's head) is another head shape altogether.

**Earned.** Saffi's ears put attached parts in the first armature. A different head waits for a character Jon rules to have one.

## 4. Views

**Jon.** Front, three-quarter and side; a full turn "if it could".

**Known.** Drawings of one character from different sides do not describe one solid shape, so an outline needs a shape per view. Positions can come from one set of points in space, and simple body volumes project correctly from any side. A full turn is therefore plausible for the positions and volumes of an armature, and harder for its outlines. Nobody yet needs a back view.

**Head tilt.** The first armatures are level heads only. Tilt matters first for references (`04-evidence-and-references.md`).

## 5. Measured references

**Jon.** Claude researches valid references, and Jon lends his eyes where they are needed. The rules are in `04-evidence-and-references.md`.

**Ruled on 2026-11-16.** No automatic landmark detector. Jon does not trust the ones the research found, and they exist only in Python. References are a wide breadth of competent anime and manga pictures, measured by an agent and checked by Jon's eyes.

## 6. Also said

- **Bodies**, for characters of all ages, under the mannequin rule (`01-mission.md`). Mudsong's playable cast runs from 17 to 58, so a child or an elder waits for a real one.
- **An interface** with controls Jon can move himself. Jon ruled on 2026-11-16 that it is a web app (`05-code-and-design-tools.md`). The first armature needs none.

## Questions nobody has answered

- Which characters come second and third. `EVID-6` asks for three materially different real characters and a held-out fourth before maquette claims to serve characters in general. Mudsong's cast offers real contrasts: a man, a goblin of about 4 ft, an oni of about 7.5 ft, a woman of 58.
- Whether a saved character must survive changes to the generator.
- Whether the patent on 2.5D cartoon models (`04-evidence-and-references.md`) touches the method chosen for views.
