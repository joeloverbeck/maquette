# Artificial evaluators and composite scores

The core constitution's requirements language, definitions and applicable rules govern this module. Its Modules table determines any additional applicable modules. One adopted revision binds the core and its modules.

### EVAL-1 — Composite scores preserve non-compensable requirements

**Trigger:** A decision-bearing score aggregates materially different properties.

**Rule:** When a decision-bearing score aggregates materially different properties, its component values, directions, normalizations, weights, uncertainty where relevant, and aggregation or compensation policy MUST remain inspectable. For the stated decision, the acceptance authority MUST distinguish properties that may trade off from requirements or invariants that are non-compensable. Failure of a non-compensable requirement MUST remain a gate, veto, abstention, or explicit failure; improvement elsewhere MUST NOT conceal it. A composite score MAY guide search, ranking, triage, or summary, and MAY be the acceptance oracle only when the exact aggregation and trade-offs are themselves the complete accepted claim. When unresolved but plausible choices of weights, normalization, or aggregation can materially reverse a decision, expose that sensitivity rather than present one scalar as determinate.

When a guiding score is offered as acceptance evidence, also apply `SEARCH-5`.

### EVAL-2 — Evaluator competence bounds the claim

**Trigger:** Artificial evaluators materially bear a claim whose interpretation depends on their competence or behavioral coverage.

**Rule:** When simulation, self-play, synthetic users, model judges, surrogate models, or other artificial evaluators materially bear a claim whose interpretation depends on their competence or behavioral coverage, independently establish that relevant competence or coverage on known, planted, held-out, or otherwise independently adjudicated cases at the level the claim requires. A weak, deliberately restricted, or uncalibrated evaluator MAY provide a baseline, conditional result, diagnostic, or valid counterexample; its failure to find a behavior or defect MUST NOT establish absence beyond its demonstrated coverage. If no adequate competence or coverage oracle exists, report behavior conditional on the named evaluator population, configuration, and validity limits rather than attributing the property generally to the evaluated system.

Causal or robustness claims also trigger `SIM-2`; improvement attributed to adaptation or search also triggers `SEARCH-3`.

