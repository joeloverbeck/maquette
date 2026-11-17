# Rule quotations for briefs

Exact quotations of the constitution's rules, grouped by the job a subagent is briefed for. `CLAUDE.md` says that an exact quotation of a current-revision rule in a brief naming that revision satisfies the recipient's reading of that rule. So paste the whole group the job needs into the brief, with the revision line below, and the subagent need not open the constitution for those rules. A paraphrase, a summary or a rule quoted from memory does not count: the recipient must then read the rule itself.

The groups cover only these rules. The recipient still reads anything else its own work triggers, by the routes in `CLAUDE.md`.

Each rule below is its whole text, byte for byte, from `docs/principles/principles-for-repos.md`. `tests/rule_quotes.rs` fails when a quotation differs from it or the revision below is not the constitution's. When the constitution's revision changes, quote the rules again from the new text.

Constitution revision: 2026-11-15.1

## An implementer accepting bounded work

### PURP-3 — One selected stream and path

**Trigger:** Selecting, reprioritizing or challenging work.

**Rule:** Distinguish outcomes, enabling work, stewardship and deferred possibilities. Humans select stream/bottleneck; work MUST follow its primary path. Elegance, usefulness, sunk work and testability grant no adjacent priority. Agents MAY challenge with evidence, never redefine for architecture. Collectively leaving the path violates `WORK-3`.

Across repositories, name next actual consumer use and unresolved handoff risk. Local criteria MUST NOT exclude/discharge it. Producer value does not prove downstream promises (`EVID-3`, `INT-1`); repeatedly reporting missing consumers does not remove the need.

### WORK-1 — Governed work selection

**Trigger:** Activation/readiness, scope expansion, adjacent work, or foundational, shared, governance, migration, substrate or dependency-bearing work.

**Rule:** Before activation, name selected outcome, bottleneck/earliest missing prerequisite and smallest acceptance-bearing primary-path increment. Existing authority MAY cover direct work. Enabling, research, governance, migration, substrate or adjacent work MUST record in its issue/decision the bottleneck, primary-path consequence, current consumer/blocked human decision, displaced direct work and stop/return condition.

If these cannot be named, defer or park. Correctness, defects, testability, code, labels and detailed issues alone do not authorize selection.

### WORK-2 — Bounded, removable speculation

**Trigger:** An unproven premise, temporary bootstrap, candidate architecture or experiment.

**Rule:** Bound appetite by decision, semantic surface and human burden; artifact counts do not bound expanding obligations. Apply `EVID-4` before experiments look like primary progress/readiness. Constraining temporary bootstraps MUST name owner, purpose, allowed consumers, prohibited dependents, appetite, replacement/removal and expiry; inertia MUST NOT grant consumers/authority.

Implementation permits experiment, not adoption. External reliance requires human inspection of real end-to-end results and explicit adoption of the exact premise/scope. Human doubt permits only cheapest bounded falsification, without fan-out. Failure, abandonment or unadopted expiry removes the premise from active context, dependencies and default publication; independently justify survivors. Salvage is optional.

Before its defining effect is accepted, provisional behavior MUST NOT acquire product authority through broad golden/snapshot locks, mutation campaigns, certification, exhaustive conformance or preservation governance. Narrow tests, replay, diagnostics and invariants for safe, inspectable, reproducible decisions are allowed, as are proportionate security/privacy/corruption, irreversible-effect and authority protections. Later hardening follows demand, burden, primary path and governed lifetime.

### WORK-3 — Repository allocation tripwire

**Trigger:** Completed work accumulates without an acceptance-bearing primary-path increment.

**Rule:** A repository MAY accept its own human-observable unit and consecutive limit; otherwise **three completed work items** applies. New issues, lanes, cleanup, enabling success or crossing repositories with the same unresolved handoff/premise do not reset it or grant allowance.

Intermediate acceptance or acknowledged limitations do not reset an unexercised consumer effect. Retain the unmet premise; exclusions, deferred integration and “non-blocking” deficits do not discharge it. A producer can earn independent acceptance without proving its consumer.

For experiential, interactive or authoring outcomes, reset requires human-inspected representative use through the affected path, yielding the named effect or resolving its earliest blocking premise. It resets only what it exercises under `PURP-2`/`EVID-3`, not adjacent enjoyment, presentation, available possibilities or unrelated success. Isolated actions, components, checks and refactors do not establish unexercised loops, voluntary recurrence, return, recovery or assurance.

At the limit, select no further work until the human explicitly continues, reprioritizes, reframes or parks. Continuation MUST justify the direction and renew scope, appetite and allocation bound. Sessions are not an alternative count.

### WORK-5 — Terminal decisions freeze the repository

**Trigger:** Appetite ends, or accepted work reaches a terminal-decision gate.

**Rule:** All discretionary repository work MUST stop until the exact decision is durably recorded; changing lanes, issues, cleanup or refactoring cannot evade it. Only necessary decision evidence, exact-state preservation/quarantine or minimal imminent data-loss, credential-exposure or immediately exploitable security containment MAY continue. Record emergency containment separately; the pending decision remains next. Extension requires fresh human reprioritization with scope and appetite.

Count extensions of the same pending decision, including successor repositories (`PURP-1`). Each MUST name missing evidence, why accumulated evidence cannot decide, and the exact scope to obtain it. At most **two consecutive extensions** follow the original appetite. After the second, no third: the human MUST adopt, reject, narrow, park or reframe on existing evidence. Only a materially different primary claim or terminal decision resets the count; new repositories, lanes, mechanisms, presentation, cosmetic narrowing or local success do not.

### EVID-5 — Real inputs earn local paths

**Trigger:** Substantive architecture or implementation.

**Rule:** Start from accepted real input at consumed fidelity. Synthetic or hypothetical cases MAY test mechanics/falsify, but MUST NOT authorize canonical representations, general subsystems, architecture or backlogs.

One input earns only a local replaceable path; broader promises need `EVID-6`. Bounded, correct, safe, visible special cases, explicit branches and duplication MAY precede abstraction when cheaper to replace. Agents MUST NOT generalize merely to remove repetition before different real pressure earns it. This excuses neither inconsistent authority, hidden coupling, security/correctness defects nor duplicated logic whose current divergence is the bottleneck. Local clarity and repair remain required.

### OPS-1 — Feasibility before readiness/TDD

**Trigger:** Risk-bearing TDD, implementation, exact review, execution readiness or unattended work.

**Rule:** Establish adequate input information for promised outputs; identify finite representation/oracle where available, semantic authority, uncertainty behavior, smallest acceptance-bearing increment, appetite, stops and publication boundary. Repeated human judgment also requires `EVID-8`'s cumulative unit/ceiling.

Producer-consumer readiness MUST separate candidate-construction authority from proved handoff. Name the execution owner and actual source-to-consumer check (`INT-1` where cross-repository); generalized evaluators/universal representations are not prerequisites for product-specific code. A `PURP-5` joint probe can resolve uncertainty, but its plan is not evidence.

Recurrent, reusable, configurable, engine-like or separable experiential claims also need `PURP-2`'s loop/value mode, `EVID-8`'s setup/return burden, and planned first, different second and applicable return use (`EVID-3`).

Material external dependencies owning primary-path state/decisions require a minimal real lifecycle probe and independent semantic oracle. Exercise applicable update/rebuild, IDs, score/distance meanings, persistence/reload, failure/recovery and replacement. Docs, types, example-repeating wrappers and plausible output do not prove correct use. Straightforward replaceable dependencies unable to alter accepted meaning, authority or consequential action need only ordinary proportionate checks.

Apply this gate to unrestricted prose, semantic gating/scoring, decision-bearing retrieval, relied-on lifecycle dependencies, external fidelity, shared abstractions, parsers/DSLs/models/frameworks/controllers, migrations, cross-cutting validators, generated scale, session-spanning work and default publication. Missing answers mean not ready; labels/detail confer no feasibility.

### OPS-4 — Execution stops

**Trigger:** Nontrivial implementation or review.

**Rule:** Unless a stricter or explicitly accepted larger limit applies, stop at the earliest of:

- two active hours without inspectable acceptance-bearing increment;
- three failed implementation/review repair cycles at one acceptance boundary;
- three consecutive review passes retaining cases but adding members of an open semantic/paraphrase family;
- an unapproved capability class or independent pipeline;
- approximate doubling of expected production, test, evidence or semantic surface;
- exhaustion of token, compute, financial or human-burden limits;
- a context/session boundary requiring material reasoning reconstruction; or
- unattended continuation lacking `AGENT-4` state.

Open issues, AFK, sunk cost, passing legacy tests or reports override no stop. Extension needs fresh human priority, scope and appetite; `WORK-3`/`WORK-4` counts persist.

### AGENT-5 — Delegated judgment, evidenced escalation

**Trigger:** Uncertainty, alternatives, apparent ambiguity or a contemplated human request before the current acceptance surface.

**Rule:** Bounded activation delegates implementation judgment. Agents MUST proportionately investigate applicable work, foundations, sources, code, tests, history, conventions and authoritative external evidence, then choose/proceed with the best-supported option. Low confidence, novelty, complexity, alternatives, availability or responsibility transfer create no human gate. Do not ask technical questions the investigated agent can answer better.

A request requires a residual reserved decision currently blocking safe/proportionate progress. It MUST meet `AUTH-3`, identify the human consequence and why authority/investigation cannot decide, lead with a recommendation, explain why the strongest alternative loses, and state delay/error cost and safe return. Translate technical detail into human consequences.

For technical uncertainty, choose a local reversible consistent option, contain/test/proceed; record only materially relevant assumptions. For reserved uncertainty without an immediate stop requirement, agents MUST continue explicit local reversible proposals when isolated from false claims, adoption, dependents, irreversible effects and disproportionate waste; expose them at the next coherent acceptance. Stop at authority/risk/appetite/capability/repository/publication/irreversible boundaries, material user-semantic lock-in, disproportionate waste or other stop rules.

For delegated creative/domain proposals, agents SHOULD resolve ordinary gaps coherently and explain choices together. Proposals remain unaccepted; `PURP-5` may permit reversible demonstrations. Do not silently change accepted behavior/identity or reserve every provisional rule.

Batch genuine independent decisions when safe. Agents MUST NOT create serial micro-approvals, speculative pre-investigation questions or technical subgates. IDs do not reserve choices; interactive work follows the same rule.

Generic judgment/investigation/advice requests authorize technical judgment, not reserved choices. Silence, confusion, fatigue or advice requests MUST NOT count as acceptance. Clear authorization after an `AUTH-3` complete recommendation MAY accept it; MUST NOT reconfirm without materially changed facts, consequences or scope. Agent judgment remains a proposal, not canonical human authority.

## Using a test or other evidence for completion

### EVID-3 — Artifact before system

**Trigger:** General systems, workflows, frameworks, catalogs, large generated networks, semantic scale-up or external fidelity claims.

**Rule:** First produce/materially revise a real primary artifact or substantial risk-bearing slice through the proposed path. It MUST exercise relevant authoring, semantics, tests, runtime and presentation with fidelity adequate for material visual, motion, latency, readability and interaction judgments; not misleading debug abstractions.

Experiential, interactive and authoring slices MUST allow bounded actual task completion/intentional termination; isolated operations are probes unless selected outcomes. Each `PURP-2` loop justifying material machinery/burden MUST run before expansion. Observed options and adjacent enjoyment prove only experienced effects.

A diagnostic run MAY create credible rare/optional/unfamiliar opportunity. Recurrence requires later ordinary use with informed voluntary repetition/return; forced repetition is insufficient. Non-use/rejection is evidence against demand, not repair authority. For selected recovery, assurance, accessibility, availability or possibility value, name/expose the condition; humans MUST use it or, for availability/assurance, inspect sufficient real consequences/burden. This accepts that mode, not recurrence or broader persistence.

Supported effects remain evidence when others fail/remain unproved. Unsupported machinery MUST stop under `WORK-5`; agents MAY recommend simpler preservation without silently changing outcomes.

Before recurrent adoption/material expansion of experiential, interactive, authoring, configurable, reusable or engine-like paths, materially different second use MUST pass through the same accepted authoring/configuration, runtime and presentation. It MAY add ordinary content/configuration, bounded consumer adapters at existing seams or in-scope repairs. New capability classes, core ontology levels, case-only shared schemas, required services or synchronized unrelated changes do not prove recurrent fitness. Completion-blocking/burden-hiding defects require reruns after repair; lesser defects still count against fitness. Experiential recurrence requires voluntary election.

Periodic reuse, use beyond fresh memory, handoff or interruption recovery also needs return use from durable state without conversational/unauthored-memory reconstruction. Clean handoff MAY replace elapsed interruption with equivalent burden exposure. Single-use work without recurrence claims needs neither extra run and MUST NOT generalize merely to satisfy them. These runs prove only that path; generality needs `EVID-6`.

Authored specifications/configurations/packages promising downstream use MUST reach their intended consumer before bulk/repeated authoring or supporting machinery expands. The consumer MUST use actual pinned material as claimed, exercise difficult behavior against independent expectations and expose consequences. Interactive promises need real use; headless output proves only executed behavior. `INT-1` distinguishes specification implementation/runtime data; `EVID-7` allows bounded design exploration. Without consumers, next establish the path or obtain narrowing/parking/abandonment; independently selected producers may continue (`PURP-5`). Reviews/joins/models bypassing input or consumer prove no handoff.

Work MUST NOT scale to hundreds of interdependent records/rules/assets or equivalent semantic surface before representative effects at acceptable authoring/validation/maintenance cost. Legacy success proves its path only.

### EVID-4 — Evidence bears only its claim

**Trigger:** A test, prototype, benchmark, review, simulation, metric, component or probe is offered as evidence.

**Rule:** A **seam probe** proves real input crosses a boundary to output. An **acceptance-bearing vertical slice** proves expressibility, production, validation and operational sustainability of the smallest accepted demand, with required configurations, populations, controls, scale, provenance, refusal and verdict. Disconnected demos MUST NOT accumulate into wider fitness.

Source review, structure, coverage, expectations, consumer execution and human use each prove only their property. Joins are not entailment; models prove their behavior, not another consumer's; receipts record, not create evidence. Expectations MAY use source examples, calculations, invariants or justified models; implementation MUST NOT supply its own expected answer.

When priority/adoption depends on it, classify **product-bearing** versus **decision-only** in existing work before implementation or counting output as progress. Decision-only work MAY be disposable/high-fidelity; acceptance covers its named claim/decision only. Polish, extent, expense or resemblance MUST NOT imply downstream progress/readiness/reuse. Promotion separately requires real-input, authority, acceptance, boundary and publication compliance, not inertia.

Presentation/tangibility/enthusiasm/motivation/communication/emotional orientation proves only its selected effect. Proportionately bounded acceptance MAY resolve no product premise (`PURP-1`/`PURP-2`). Without separate design/acceptance for another claim, it MUST NOT imply upstream testing/stability/readiness or reusable machinery. State appetite, prohibited dependents and parking/disposal; promotion follows `PURP-4`/`PURP-5` too.

Replay proves pinned reproducibility, not causality/robustness/stability/diversity/generality; use proportionate controls, counterfactuals or independent variation (`SIM-2`, `EVID-6`). Distinguish desirable causal sensitivity from incidental variation. Statistics/simulation MAY diagnose/compare/falsify, never manufacture meaning, taste, fun, desirability or acceptance. Tests need valid oracles. Self-created evidence/consumers/obligations prove consistency at most, not need, demand, fitness, authority or causal credit.

### EVID-6 — Generality follows different pressure

**Trigger:** Cross-product/domain/repository/independent-consumer reuse or a shared constraining contract.

**Rule:** Adoption MUST use **three materially different real design cases and one further holdout not used in design**; count alone is insufficient. Cross-product claims need different named products/independent consumers; other generality needs independently selected product-bearing artifacts or current accepted consumers. Synthetic variants, future or manufactured products and decision-only probes do not qualify. Target-specific language, fields, defaults, fixtures, sources or assumptions block adoption until removed/quarantined and the holdout re-evaluated.

Repositories MUST NOT become homes for imagined product families merely for reuse. Keep product work local and independently abandonable unless `BOUND-2` allows co-location. Repeated real pressure MAY earn an explicitly versioned shared extraction under boundary/integration rules. Visibility, licensing or confidentiality affect custody, not demand, generality, co-location or case requirements; justified shared code may be public or private.

Fewer cases require an intrinsically finite problem, fully governing external formal standard or scoped waiver. Ordinary local refactoring, helpers and deduplication with mechanically evident behavior and no broader/published reuse claim are exempt; they stay local/reversible until wider pressure.

### REP-2 — Independent representation oracles

**Trigger:** Representation, specification, state/consequence model, migration or renderer constrains work/consumers, even with product-specific naming.

**Rule:** Before adoption, a risk-bearing slice MUST prove real authoring/revision → inspectable proposed record/state → human-accepted artifact/effect in consumer form: structured, rendered or executable. No extra representation without a consumer; claimed benefit MUST improve an actual task.

Explicit state/consequences require current accepted cases needing narration-independent persistence, coordination, deterministic or externally inspectable consequences with uncertainty/failure, recovery or another named consumer. Possible domain facts create no demand. Start with minimal local state/transition authority; generality needs `EVID-6`.

The candidate MUST NOT select/filter its validation inventory, scope, fixtures, hard cases or oracle. Derive these independently from accepted tasks, semantic/authority distinctions, preservation obligations and sources, regardless of expressibility. Before adoption, map every finite-target obligation/deficit; for open targets disclose challenged inventory/unproved remainder, including hard/breaking cases. Identity coverage/receipts prove no source acceptance, research, freedom from contamination or fitness.

A disposable local model MAY precede proof only without adequacy/reuse claims, adopted external dependents or supporting infrastructure; it stays replaceable and cannot choose its oracle/scope. `PURP-5` consumer probes gather evidence, not expansion authority. Representation-created synchronization, reconciliation, invalidation, migration, authoring and validation burden counts against fitness. Parsing old renders does not prove new authoring.

## Repairing a test or review failure

### OPS-3 — Adjudicate tests, reviews, and infrastructure before repair

**Trigger:** A test, CI run, or reviewer reports a failure.

**Rule:** First determine whether the expectation is still valid, whether the defect lies in the system under test, the test or review oracle, or execution infrastructure, and whether the finding belongs to accepted scope. Read the actual failure annotation before diagnosing code or architecture; billing, permissions, unavailable services, dependency drift, environment drift, and runner failure are infrastructure claims until shown otherwise.

Classify the finding as an implementation defect, specified missing case, invalid or obsolete test, specification ambiguity, authority or ownership defect, infrastructure failure, novel requirement, or non-convergence signal. Only implementation defects and specified missing cases enter automatic repair. Uncertainty about classification requires proportionate investigation and does not itself authorize repair or human escalation. If the residual choice is implementation judgment, the agent chooses the best-supported reversible classification and proceeds; if it is a reserved human decision, `AGENT-5` governs the request. Reviewer-generated examples are observations, not requirements until their interpretation and scope are accepted.

### OPS-5 — Exploration stabilizes the claim; exact review verifies a finite candidate

**Trigger:** Adversarial review, exact-SHA review, or repeated test expansion is proposed.

**Rule:** Exploratory review MAY discover and stabilize a draft contract, interface, and oracle before final implementation review. Once a finite candidate is declared, exact-candidate review verifies that candidate; it MUST NOT become an unlimited source of new requirements. Reviewer-generated examples remain in an evaluation corpus until ratified.

Permanent selectors, ledgers, red-green evidence, and reconciliation duties are reserved for stable decision-bearing behavior and MUST be proportional to its authority and expected lifetime. Three review passes that continue expanding an open semantic family trigger `OPS-4` rather than another repair cycle.

## Before merge or publication

### AUTH-1 — Human authority

**Trigger:** Interpreted, generated or selected content entering persistent/canonical state or external action.

**Rule:** Apply the bootstrap authority chain. LLM outputs/changes remain proposals until humans accept exact surface. A mechanism or prerequisite selected as outcome needs inspectable benefit for named real use, or a bounded experiment with consumer effect, stop and removal conditions.

Runtime LLM behavior MAY avoid per-output approval only under accepted policy covering permissions, available state, persistence, side effects, refusal, failure, publication and intervention. Emission/display/action grants no canonical authority.

### AUTH-2 — Granular, non-transitive acceptance

**Trigger:** A proposal, patch, migration, repair, batch or derived consequence may change authoritative state.

**Rule:** Expose each decision atom as **entailed** by accepted premises, **required repair** of an invariant/contradiction, or **discretionary**. Entailed atoms MAY form a visible batch naming its premise. Repairs MUST expose the invariant and valid alternatives, including narrowing/rejecting the parent when applicable. Discretionary choices stay deselectable. Acceptance MUST NOT cover omitted/later descendants; localize atoms to protect unrelated accepted state on rejection.

Visibility concerns consequences, not approval per field/sentence/file. Related choices MAY share a coherent revision-bound account of behavior, alternatives, consequences and uncertainty. Link detail; independent discretionary commitments stay identifiable/rejectable.

Bounded activation delegates necessary implementation judgment, reviewable through patch/evidence, without extra gates: e.g. internal reversible naming, placement, decomposition, algorithms, data/fixtures, tests, refactoring and consistency. Alternatives do not reserve choices.

Agents, reviewers and artifacts MUST NOT reserve choices by labels, alternatives or approval requests. Only an accepted human instruction naming concrete human consequence, risk or authority reason can do so; buried agent-authored detail cannot. Delegation creates proposals (`AUTH-1`, `AGENT-5`).

### AUTH-3 — Exposed authority

**Trigger:** Results govern acceptance, rejection, promotion, scoring, publication, canonical mutation or consequential action.

**Rule:** Causality/state transitions MUST be inspectable/testable at their authority boundary. Separate reasoning, authoritative state and workflow control. Expose owner, input revisions, oracle, authority, evidence, uncertainty, validity limits, failure and recheck conditions.

A **mechanical validator** checks formal properties; a **semantic reviewer** proposes interpretations; an **acceptance authority** decides canonical state. “Validator” MUST NOT conceal roles; envelope validity proves no contents. Verify cited sources' existence, identity/revision and production/deterministic derivation of attributed values; otherwise label manual assertions, not verified provenance.

Decision surfaces MUST be intelligible without reconstructing the agent's technical context; bind exact revision; explain changed behavior/claims/risks/authority, recommendation, material alternatives, evidence/uncertainty, reversibility and blocked work; and separate demonstrated results, reasoned expectations and unresolved conditions. Concise evidence-linked explanations/examples/demonstrations suffice; raw records alone do not. Humans need not decode technical files.

Agents own competent investigation/verification; acceptance supplies no missing evidence or unexamined facts. Deficits persist until resolved or humans change the promise. Titles/qualifications MUST NOT claim satisfaction excluding defining requirements. Recommend changed promises plainly; `ADOPT-2` governs relaxed safeguards, `AGENT-5` requests.

### OPS-6 — Bounded, fresh, recoverable effects

**Trigger:** External, destructive, irreversible, repeated or concurrency-sensitive effects.

**Rule:** Effects MUST have narrow accepted authority, expected-revision or equivalent freshness checks, idempotent or explicitly bounded retries, inspectable consequences, explicit failure behavior and recovery ownership. Local accepted edits need only ordinary version identity and atomic validated writes unless a real boundary proves extra machinery necessary. Do not build orchestration/receipt platforms before the smallest fault-tested slice proves need.

### OPS-7 — Clean-clone operational closure

**Trigger:** Activation, handoff, publication or reliance on a repository.

**Rule:** A clean clone MUST expose purpose/authority, work control, build/check and primary-outcome activation paths, external/private input contracts/locations, output ownership/location and absent-input behavior. Dependencies, services, stores, submodules and content roots need visible roles, access, version/pins and setup/recovery. Preserve root `AGENTS.md` → `CLAUDE.md` and the canonical constitution.

Ordinary use MUST NOT rely on remembered siblings, hidden gitlinks/mounts/services/topology or conversational setup. Missing private input MAY block its workflow but must fail early/honestly while preserving claimed input-independent checks. Public demo data is optional; synthetic fixtures MUST NOT claim real demand, representativeness or readiness.

### PUB-1 — Publication

**Trigger:** Merge, release, deployment, package/data publication or another authoritative-path change.

**Rule:** Terminally failed, rejected, parked or unaccepted production MUST NOT become newly reachable on authoritative paths, even solo/private. Quarantined failure reports, fixtures and adversarial corpora MAY publish separately. Independent prerequisite/evidence work SHOULD exclude failed candidates.

Before publication, list exact commits/decision-bearing files and reconcile each acceptance. Names/status/tests/mergeability/intent do not suffice. Archiving exact failed production requires scoped waiver. Issue approval permits work, not all resulting bytes; proposal labels/custody records do not evade this. `PURP-5` provisional consumers stay isolated pending exact source/behavior/publication acceptance.
