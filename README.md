# maquette

Maquette generates armatures for anime-style characters from proportions measured on published drawings: the construction guide for a head and body, as SVG with named landmarks, for an agent to draw and paint over. It does not draw the finished character.

In an animation studio a maquette is the sculpted reference model that keeps a character on model from every angle.

## Status

Founded on 2026-11-16, and reoriented by Jon the same day toward measured proportions; [`docs/foundation/README.md`](docs/foundation/README.md) records both. Work starts from the issue tracker.

The generator draws a set of proportions as a head in the front view and the three-quarter view. No armature it draws has been accepted yet. The side view, the measuring of references and the proportion tables are not built.

## Building, checking and drawing

The code is one Rust package: a library with a thin command-line program over it. `rust-toolchain.toml` pins the toolchain, and `cargo` fetches the two crates it needs.

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Those three are what `.github/workflows/checks.yml` runs on every pull request. None of them needs VectorCraft, FigCraft, mudsong or a reference folder.

```sh
cargo run -- list-proportions
cargo run -- draw --proportions data/saffi/proportions.toml --out target/saffi
```

The first prints the list of proportions: the unit, and each proportion's name, whether it is measured or a convention, and how it is taken in each view. The second writes `front.svg` and `three-quarter.svg`, each with a landmark file beside it, into a folder Git ignores. `cargo run -- help` lists every command. The same input gives the same bytes.

An armature is put to Jon or exported only after its SVG has been read back in VectorCraft, which is run locally ([`docs/foundation/02-first-table.md`](docs/foundation/02-first-table.md), "The SVG contract").

## Where things are

| Path | What it is |
|---|---|
| `CLAUDE.md` | Agent instructions; `AGENTS.md` is a symlink to it |
| `docs/principles/` | The constitution this repository adopts, and its five adopted modules, copied byte for byte |
| `docs/foundation/` | What maquette is for, the first increment, Jon's direction, the rules for evidence and references, and the rules for code and design tools |
| `.claude/skills/` | The engineering and productivity skills from `mattpocock/skills` (MIT), copied unchanged at its commit `5863806`; `.agents/skills/` holds a symlink to each |
| `.mcp.json` | Registers the VectorCraft and FigCraft MCP servers for Claude Code |
| `design/` | Designs made by hand in those two tools; the only folder their servers may write to |
| `docs/agents/` | How agents use the issue tracker and the triage labels |
| `data/saffi/` | The hand-made front and three-quarter views of the comparison case, as landmark positions, and her set of proportions worked out from them. Numbers only; they never enter a proportion table |
| `proportions/list.toml` | The list of proportions: the unit, and every proportion's name, definition for each view, and mark as measured or a convention. The program reads this same file |
| `Cargo.toml`, `src/` | The Rust package: the list and sets of proportions, the head model, the views, the guides and the armature writer, under a thin command line |
| `tests/` | Tests that drive the command line, files in and files out. `tests/fixtures/` holds made-up proportions that say so |
| `.github/workflows/checks.yml` | Formatting, lints and tests on every pull request |
| `GLOSSARY.md` | The words maquette uses for armatures, views and proportions |
| `docs/research/` | The research behind the founding and the reorientation. Evidence only |

## Working here

GitHub Issues and pull requests in `joeloverbeck/maquette` own active work. Agents start from `CLAUDE.md`. Jon is the acceptance authority.

Maquette's first consumer is mudsong, a private repository. Nothing here needs it to be present.

## Licence

MIT; see [`LICENSE`](LICENSE). The constitution in `docs/principles/` is Jon's own document, copied from its master repository.
