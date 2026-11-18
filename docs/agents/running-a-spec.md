# Running a spec here

What the `implement-spec` skill does not know about this repository. Read it before starting a spec's integration branch, and follow it where the skill says otherwise.

- **Choose each subagent's model with Jon before launch.** Implementers, reviewers and readers alike: name the model and the number of agents, and launch once he has agreed. A model he has not approved is not used.
- **Merge the integration branch to `main` at each of Jon's acceptances, and carry on from it.** A later ticket may need work he has accepted to be on `main`, and an acceptance is the point at which that work may be published. The integration branch is not held to the end of the spec.
- **Push nothing Jon has not accepted.** A pull request that holds an armature or a table he has not accepted is not pushed, and the branch that holds it stays in his checkout. The repository is public, and a table is measured on other people's drawings. The pending record of the look says where the branch is (`docs/agents/looks.md`).
- **Compose sheets in the main checkout.** VectorCraft's and FigCraft's servers write only under the main checkout's `design/`, so an implementer in a worktree cannot make a sheet. The agent in the main checkout composes it there, on the integration branch, and commits it.
- **Brief subagents with quotations, not with the constitution.** Paste the group of `docs/agents/rule-quotes.md` the subagent's job needs into its brief; the subagent then reads only what its own work triggers beyond it.
- **Readers run on briefs and the installed program.** A batch of drawings is read by `docs/agents/reader-brief.md`, with the program installed at a lasting path (README.md, "The reference folder").
- **Install a build only when no reader is running.** A new revision of the reading instructions reaches readers through the installed program, and a drawing's two readings must follow one revision: a reader that starts under the new one while its partner read under the old cannot be compared with it. Develop against the local build meanwhile.
- **Gather readings only after the last reader of a batch has finished.** A reader writes its reading as it works, so a copy taken earlier is a half-written reading. Compare every copy with its source byte for byte.
- **Every sheet put to Jon is a look,** by `docs/agents/looks.md`.
