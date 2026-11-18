# The reader's brief

A reader is an agent that reads the drawings of one work by the reading instructions. This is the brief it is given, as a template. Every brief in the first batch carried these rules; a brief written from this one carries them without rebuilding them.

## Filling it in

Copy the template below into a file inside the reader's own folder, one file per reader per work, and replace every `{{BLANK}}`:

- `{{PROGRAM}}`: the whole path of the installed program, which README.md, "The reference folder", says how to install. Written out in full, with no `~` and no variable.
- `{{REFERENCES}}`: the whole path of the reference folder.
- `{{FOLDER}}`: the reader's own folder, as a path from the reference folder. Each reader has its own; two readers of one drawing never share one.
- `{{LABEL}}`: the reader's label, which goes into every reading it starts. The two readers of a drawing have two labels.
- `{{REVISION}}`: the revision of the reading instructions the batch is read under.
- `{{PICTURES}}`: one line per drawing to read: the picture's path from the reference folder, which head of it to read where it holds more than one, and the name of the reading file to write in `{{FOLDER}}`, ending `.reading.toml`.
- `{{REFERENCE_VALUES}}`: the `[reference]` values of each picture, which whoever hands over the drawing says: `work`, `designer`, `source`, `age_band`, `sex` and `species`. `manner` is the reader's own to say, so it is left out.

The filled-in brief names a work and stays in the reference folder. The template names none.

## Running a batch

Read a batch with the workflow `.claude/workflows/read-batch.js`: call the Workflow tool with the name `read-batch` and `args` of `{ "model": "<the model Jon approved>", "briefs": ["<whole path of a brief>", ...] }`. It starts one fresh agent per brief, in parallel, each told only to read its brief and do what it says, and returns one compact entry per brief: the readings, whether each passed its check, the view and level judged, the head's height, whether a command was refused, and at most three lines on where the instructions failed. Run at most ten briefs at a time, both readers of a drawing in one run.

When a run ends, take two things from its folder of transcripts, which the run's result names (`.../subagents/workflows/wf_<id>/`):

- **The results in full.** The notification truncates them past a few readers. `journal.jsonl` holds every one: `jq -r 'select(.type=="result") | .result.readings[] | "\(.path) \(.view) \(.level) \(.head_height_px) \(.passed_check)"' journal.jsonl`.
- **Each reader's report.** The harness refuses a reader's own report file, so the report is the reader's last message. Save it as `report.md` in the reader's folder: `jq -rs '[.[] | select(.type=="assistant") | .message.content[]? | select(.type=="text") | .text] | last' agent-<id>.jsonl`. The journal's `started` lines pair each `agentId` with its `reader <n>` label, which is the brief's place in `briefs`.

## Retesting a revision

A revision of the reading instructions is read on drawings whose positions are known before any reference is read under it, with briefs written from this template. Score each reading with `maquette score-reading --known-positions <record>`, and set the pass mark before reading the scores. Every test drawing so far has been level, so how a reader judges level has never been scored, and a backwards sign of level went unseen through two revisions. A retest therefore includes, beside each drawing, a copy turned about 10° up and one turned about 10° down in VectorCraft: a reader passes on those when it calls them not level.

## The template

````markdown
# Your brief: reading drawings

You are a reader. You read the drawings named below by maquette's reading instructions, and nothing else decides what you do.

- **The program:** `{{PROGRAM}}`
- **The reference folder:** `{{REFERENCES}}`
- **Your folder:** `{{FOLDER}}`, in the reference folder.
- **Your label:** `{{LABEL}}`
- **The reading instructions:** revision `{{REVISION}}`.

## The drawings

{{PICTURES}}

## What each picture is

Write these into each reading's `[reference]` table as they are given here. `manner` is yours to say, as the instructions say.

{{REFERENCE_VALUES}}

## How you work

1. **Read the instructions first,** before you open any picture: run `{{PROGRAM}} reading-instructions` and read all of it. If its `Revision:` line is not `{{REVISION}}`, stop and report that.
2. **Read each drawing by the instructions,** one reading file for each, at the path given above in your folder.
3. **Write every file in your folder,** and only there: readings and enlargements. Look at a picture by opening it at its whole path in the reference folder.
4. **Give each command as one line that begins with `{{PROGRAM}}`,** and give every command but `reading-instructions` the option `--references {{REFERENCES}}`. One command per line: no `cd`, no variable, nothing before the program's path and nothing joined to it.
5. **If the permission system refuses a command, stop.** Report the command as you gave it. Do not try it in another form.
6. **Work from the pictures alone.** Keep to the pictures named above and your own folder: not the repository, not another reader's folder, not any other copy of a drawing, and not anything you remember of the work or its characters.

You are done when every drawing above has a reading that passes `check-reading`, or you have stopped for a reason this brief names.

## Your report

End with your report as your answer; the agent that runs the batch saves it in your folder. For each reading:

- its path, and that `check-reading` last said it is complete;
- the view, which way the head faces, the turn and whether it is level, each with its reason;
- the head's height in pixels of the picture, from the hair's top to the chin as you placed them.

Then say where the instructions were unclear or did not fit a drawing, each point with the landmark or the judgement it concerns. Say also if any command was refused.
````
