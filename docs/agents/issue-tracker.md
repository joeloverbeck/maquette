# Issue tracker: GitHub

Issues and specifications live in the public repository [joeloverbeck/maquette](https://github.com/joeloverbeck/maquette/issues). Use the `gh` CLI from this clone, or `--repo joeloverbeck/maquette` from a session opened in another repository.

GitHub Issues and PRs are the sole active work-control plane. Local documents may support an issue; they do not form a second queue. Publishing a ticket or specification does not activate implementation. Follow the repository constitution and the accepted issue for scope, effort limits, acceptance and publication.

The repository is public. A ticket restates the facts it needs from mudsong, which is private, and names where they came from; it does not paste mudsong's text or images.

## Conventions

Before the first write of a session, verify that the target resolves to `joeloverbeck/maquette`. Work on what mudsong does with an armature is filed in mudsong.

- **Create:** `gh issue create --title "..." --body-file <file>`.
- **Read:** `gh issue view <number> --json number,title,body,state,labels,comments`.
- **List:** `gh issue list --state open --json number,title,body,labels,assignees`, with appropriate label or state filters.
- **Parent/child:** create with `gh issue create --parent <parent> ...`, or link an existing child with `gh issue edit <parent> --add-sub-issue <child>`. If native sub-issues are unavailable, put `Part of #<parent>` in the child body and link the child in the parent's task list.
- **Blocking:** `gh issue create --blocked-by <number> ...` or `gh issue edit <child> --add-blocked-by <blocker>`. If unavailable, put `Blocked by: #<number>` at the top of the child body.
- **Comment:** `gh issue comment <number> --body-file <file>`.
- **Labels:** `gh issue edit <number> --add-label <label>` or `--remove-label <label>`; use the mapping and readiness rules in `docs/agents/triage-labels.md`.
- **Close:** post the explanation as a separate comment, then run `gh issue close <number>`. A pull request whose body says `Closes #<number>` closes that issue at its merge, so post the issue's closing comment before merging.

For multiline bodies and comments, write the exact text to a temporary file and use `--body-file`. Read back the changed issue, comment or labels after a write.

Every number in a comment (a count, a total, a share, a cost) comes from a command run for it in the session, and is checked against that command's output in the read-back before the next step. Comments are the project's record and are public; three figures written from memory in one session of #18 were wrong and had to be corrected.

## Publication

A pull request lists exactly what it publishes. It merges once its checks pass and Jon has accepted every armature and every table in it; the agent then merges it. The checks are `.github/workflows/checks.yml`: formatting, lints and tests, on every pull request and on `main`. A run takes some seconds to appear after a push or `gh pr create`; until then `gh pr checks --watch` exits with a failure, and `gh run list` shows no run. To wait for them, take the id of the run whose `headSha` is the pushed commit from `gh run list --branch <branch> --json databaseId,headSha`, once it is listed, and run `gh run watch <id> --exit-status`.

**PRs as a request surface: no.**

## Skill instructions

When a skill says "publish to the issue tracker", create a GitHub issue here within the authorized scope. When it says "fetch the relevant ticket", read that issue and its comments.
