# Domain docs

How the engineering skills use this repository's domain documentation. Single-context layout.

## Before exploring, read these

- **`docs/foundation/`**: the repository's decision record. Its `README.md` says which foundation to read before which work. These outrank any ADR.
- **`GLOSSARY.md`** at the repo root, if it exists.
- **`docs/adr/`**, if it exists: ADRs that touch the area you are working in.

If `GLOSSARY.md` or `docs/adr/` is missing, carry on without saying so, and don't propose creating it upfront. `/domain-modeling` creates each one when a term or decision is actually settled.

## Layout

```
/
├── GLOSSARY.md            ← created lazily
├── docs/foundation/       ← founding decisions (exists)
└── docs/adr/              ← created lazily, for decisions the foundations don't settle
```

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a proposal, a hypothesis, a test name), use the term as `GLOSSARY.md` defines it, and as the foundations use it (armature, landmark, mannequin, view). Don't switch to synonyms the glossary avoids. If the concept isn't defined yet, either you are inventing language the project doesn't use, or there is a real gap to note for `/domain-modeling`.

## Don't duplicate the foundations

A decision the foundations already record is not written again as an ADR. A decision that changes a foundation is a foundation revision under the constitution, not an ADR. Write an ADR only for a decision with a current consumer that no foundation settles.

## Flag conflicts

If your output contradicts a foundation or an ADR, say so explicitly. Don't silently override it:

> _Contradicts ADR-0003 (…), but worth reopening because…_
