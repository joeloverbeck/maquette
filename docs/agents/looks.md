# Looks

A look is one review sheet Jon is asked to judge (`docs/foundation/01-mission.md`, "Limits"). His eyes are the oracle no check replaces, and the looks are counted against a ceiling, so each one is spent on a sheet that is ready. A question put to him in words, with no sheet, is not a look; the record says so when it happens.

## What a request for a look names

`EVID-8` asks every request for a human judgement to name four things. Each pending record below names them:

- **The decision:** the one question Jon answers, such as whether a table is accepted as measured. A question that binds nothing is marked so.
- **Why no check can make it:** what the mechanical checks prove and where they stop.
- **How many drawings:** the number on the sheet, and what each is.
- **When it stops:** what follows acceptance, what follows a rejection, and that at the ceiling the next step is Jon's decision (`WORK-5`).

Accepted, rejected and inconclusive looks all count against the ceiling in `01-mission.md`. A look that ends without a verdict counts.

## Before a sheet goes to Jon

1. **Repair every known mismatch first.** A fault already found, in a fit, a reader's report or an agent's own look, is repaired in what the sheet draws before a look is spent on it. Look 5 was spent on a long nose whose cause had been seen at look 4 and repaired only in the fit of views.
2. **Draw a face over a side or turned view of a head Jon has not accepted, and look at it.** An agent draws an original anime-style adult face in VectorCraft, exactly through the armature's points, such as the nose point, the mouth and the chin, hides the armature, and looks. Guides alone hide what a face shows: at look 5 such a face showed at once a long pointed nose that two sheets of guides had not. The face is a diagnostic study of the armature; maquette does not paint. If the face reads wrong, repair before the look.
3. **Make every number on the sheet say what it is measured against:** another reader, a known position, the table's range, or Saffi's accepted armature. A number set against nothing known says so.
4. **Tie each card to the picture it shows:** which picture, which head of it, its height in pixels. At the first batch Jon found a list that did not tie its numbers to the pictures hard to check.
5. **Run the checks the sheet depends on,** and name them in the pending record: formatting, lints and tests, and for an armature its read-back in VectorCraft (`02-first-table.md`, "The SVG contract").

An agent's own look at a render, and at the face, is a diagnostic. Finding nothing wrong establishes nothing; the record says what the agent saw.

## The records

Both are comments on the issue that owns the work, written by `docs/agents/issue-tracker.md`. The repository is public: no work, character or designer is named, and what is private is placed as "in the reference folder, under `<folder>/`". Replace each `<...>`.

### Pending

```markdown
**Pending: Jon's look <N> (of a ceiling of <C>), on <what the sheet is>.** Recorded here so the work can resume without the conversation (`AGENT-3`).

- **Question:** <the decision>. <Any question that binds nothing, marked "binding nothing".>
- **Input:** `<sheet path>` at `<commit>` on `<branch>`, in Jon's checkout. <Whether the branch is pushed, and if not why not.>
- **What the sheet shows:** <how many drawings, and what each is>.
- **Why no check can make it:** <what the checks prove and where they stop>.
- **What changed since look <N-1>:** <only when this sheet repeats one>.
- **Checks before the look:** <formatting, lints, tests; the read-back and its VectorCraft version; the face study for a side or turned view>.
- **An agent's own look, a diagnostic only:** <what it saw>.

**Safe return point:**
- **If Jon accepts:** <the next steps, to merge and report>.
- **If he says what is wrong:** repair and put another sheet to him, which is look <N+1>. <How many would then remain.>
- **If it cannot be brought to acceptance:** the next step is Jon's decision.

**Appetite:** looks used so far, <N-1> of <C>. Completed tickets without an accepted increment: <count>.

Rules consulted: <IDs>, at revision <constitution revision>.
```

### Outcome

```markdown
**Look <N> of <C>, <date>: <accepted | not accepted | no verdict>.** <What Jon said, in a sentence.>

**What was shown:** <the drawings, and the question>.

**What Jon said:**
- **Accepted:** <what>.
- **Rejected or overruled:** <what, or "nothing">.
- <Each ruling, as he gave it.>

**What follows:** <his decision on what is done next, and what it changes>.

**Not shown by this look:** <the claims the look does not carry>.

**Looks used:** <N> of <C> (<which were accepted, which not, which without a verdict>). Completed tickets without an accepted increment: <count>.

**Next:** <the next step>.
```
