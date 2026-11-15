# Search, adaptation and causal attribution

The core constitution's requirements language, definitions and applicable rules govern this module. Its Modules table determines any additional applicable modules. One adopted revision binds the core and its modules.

### SEARCH-1 — Retrieval fitness includes relevant and absent answers

**Trigger:** Memory, search, retrieval or ranking materially bears the acceptance-bearing slice.

**Rule:** When memory, search, retrieval, or ranking materially bears the slice, independently selected cases MUST distinguish a relevant result from a merely nearest or highest-scoring result and exercise confusable alternatives, absent-answer behavior, and changed or superseded information when those are part of the accepted claim. Judge the downstream decision or human effect as well as retrieval mechanics; a plausible neighbor list or aggregate hit metric alone does not prove downstream memory or retrieval fitness unless that metric is exactly the complete selected outcome.

### SEARCH-2 — Search needs a productive candidate neighborhood

**Trigger:** A search, evolutionary, repeated-generation, automated-variation or adaptive-operator mechanism claims cumulative improvement, learned operator effectiveness or efficient exploration.

**Rule:** When a search, evolutionary, repeated-generation, automated-variation, or adaptive-operator mechanism claims cumulative improvement, learned operator effectiveness, or efficient exploration rather than merely bounded sampling, the first acceptance-bearing slice MUST exercise the representation, candidate generation or variation, validation, material repair or post-processing, evaluation, and selection stages actually relied upon. Before material scale, that path MUST demonstrate a productive candidate neighborhood: enough valid, materially distinct, inspectable candidates reach the accepted evaluation surface at acceptable cost to support the claimed mode of search, and downstream transformation does not silently erase the lineage on which improvement or operator learning is attributed. The repository MUST state the observed limitations and leave unsupported operators, regions, or generality unproven. No universal yield threshold or search-space metric is required. Cheap exhaustive enumeration, fuzzing, random sampling, one-off creative generation, or another bounded search that makes no cumulative-improvement, operator-learning, or search-efficiency claim MAY proceed under the ordinary evidence and appetite rules.

Artificial evaluation and decision-bearing composite scores also trigger the applicable evaluators-scores rules.

### SEARCH-3 — Improvement claims isolate the mechanism's contribution

**Trigger:** Improvement is attributed to feedback, training, ranking, search, adaptation, optimization, repeated selection or another evaluation-responsive mechanism.

**Rule:** When the accepted claim attributes improvement to feedback, training, ranking, search, adaptation, optimization, repeated selection, or another mechanism that changes candidates in response to an evaluation signal, the evidence MUST distinguish that mechanism's contribution through a credible simpler, non-adaptive, null, ablated, or prior-state comparator, or a held-out evaluation, under materially comparable inputs, opportunity, and relevant resource budget. Use the cheapest matched comparison adequate to the claim. The `PURP-4` comparator MAY satisfy this obligation when it tests the same causal claim; no second tournament, universal random control, formal statistical study, or publication-grade protocol is required unless the accepted claim needs that strength. A mechanism selected as a learning exercise, implementation exercise, exploratory instrument, or recreational medium MAY be built and used without proving superiority, but any claim that its adaptation caused cumulative improvement remains experimental until this comparison bears it. If no meaningful comparison can be obtained, state the limitation rather than claiming the effect.

Causal tendencies and robustness across variations also trigger `SIM-2`; artificial evaluators bearing the claim trigger `EVAL-2`.

### SEARCH-4 — Transformations preserve lineage and causal credit

**Trigger:** An upstream stage receives causal credit after material downstream transformation.

**Rule:** When a generator, search operator, model, policy, or earlier stage receives causal credit for a final candidate that is materially changed by repair, retry, normalization, compiler or test feedback, filtering, reranking, human editing, or another downstream transformation, the evidence MUST preserve lineage and use an adequate ablation, comparator, or stage-specific measurement to isolate the credited contribution. A transformative stage is a separate mechanism and cost. Improvement by the combined pipeline MAY be claimed without isolation when that combined pipeline is exactly the claim, but the result MUST NOT be attributed to an upstream stage alone. Routine formatting or semantics-preserving normalization does not trigger this obligation.

### SEARCH-5 — Exposed objectives do not establish broader acceptance

**Trigger:** A guiding reward, fitness function, training objective, benchmark, optimization target, user-selection signal or repeatedly consulted evaluation set is offered as evidence.

**Rule:** A reward, fitness function, training objective, benchmark score, optimization target, user-selection signal, or repeatedly consulted evaluation set MAY guide candidate generation or selection. Improvement on that guiding signal establishes only the narrow declared objective on that exposed evaluation surface. It does not by itself establish generalization, causal improvement, consumer utility, superiority, interesting behavior, or acceptance beyond that narrow signal. A broader claim requires a held-out or otherwise independently constituted evaluation surface, representative case, or human comparison derived from the accepted effect rather than selected by what the candidate already optimizes. The guiding signal MAY serve as acceptance evidence when satisfying it is exactly the complete accepted claim and no broader effect is asserted.

Decision-bearing composite scores and artificial evaluators also trigger the applicable evaluators-scores rules.

