# maquette

Maquette generates armatures for anime-style characters from proportions measured on published drawings: the construction guide for a head and body, as SVG with named landmarks, for an agent to draw and paint over. It does not draw the finished character.

In an animation studio a maquette is the sculpted reference model that keeps a character on model from every angle.

## Status

Founded on 2026-11-16, and reoriented by Jon the same day toward measured proportions; [`docs/foundation/README.md`](docs/foundation/README.md) records both. Work starts from the issue tracker.

The generator draws a set of proportions as a head in the front view and the three-quarter view. The program lists how far such a head stands from Saffi's hand-made views, and reads an armature back in VectorCraft. No armature it draws has been accepted yet. The side view, the measuring of references and the proportion tables are not built.

## Building, checking and drawing

The code is one Rust package: a library with a thin command-line program over it. `rust-toolchain.toml` pins the toolchain, and `cargo` fetches the three crates it needs.

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

### The differences from the hand-made views

```sh
cargo run -- differences-from-hand-made-views \
  --front target/saffi/front.landmarks.toml \
  --three-quarter target/saffi/three-quarter.landmarks.toml \
  --record data/saffi/hand-made-views.toml \
  --out target/saffi/differences.toml
```

This lays the two generated views over Saffi's hand-made ones and writes, for every landmark both give, how far apart they put it as a share of head height. It prints the differences above 1%, numbered; the armature review sheet puts those numbers on the drawing. The file states how the views are laid over each other, which is one scale and one shift for a view and fitted to no landmark, and what has no counterpart on either side. It lists differences and judges none: whether one matters is Jon's look.

### The read-back in VectorCraft

An armature is put to Jon or exported only after its SVG has been read back in VectorCraft ([`docs/foundation/02-first-table.md`](docs/foundation/02-first-table.md), "The SVG contract"). That is run locally, for each view, where `vectorcraft-cli` is on the PATH:

```sh
cargo run -- read-back --landmarks target/saffi/front.landmarks.toml
cargo run -- read-back --landmarks target/saffi/three-quarter.landmarks.toml
cargo run -- read-back-hard-case --landmarks target/saffi/three-quarter.landmarks.toml
```

`read-back` starts `vectorcraft-cli mcp` itself, allowed to read the SVG's folder and to write nothing, opens the SVG the landmark file names, and compares what VectorCraft holds with the landmark file: no import warning, the same names each once, every anchor within 0.001 unit, every guide visible, stroked and unfilled. It prints the read-back and the VectorCraft version it ran against, which is whichever is installed. It exits 0 on a pass and 1 on a failure. Where `vectorcraft-cli` is not installed it says so and exits 2: nothing was checked, which is neither. `read-back-hard-case` runs the contract's hard case on a three-quarter view, with the two spoiled copies the oracle must fail.

A pass says VectorCraft keeps what the landmark file says. It does not say the landmark file is right or that the armature looks right.

`cargo test` does not start VectorCraft. It tests the comparison on recordings of VectorCraft's replies in `tests/fixtures/read-back/`, which say they are recordings and of which version. The same checks run live with:

```sh
cargo test --test read_back_in_vectorcraft -- --ignored
```

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
| `Cargo.toml`, `src/` | The Rust package: the list and sets of proportions, the head model, the views, the guides, the armature writer, the differences from a hand-made armature and the read-back in VectorCraft, under a thin command line |
| `tests/` | Tests that drive the command line, files in and files out. `tests/fixtures/` holds made-up proportions and made-up views that say so, and recordings of VectorCraft's replies that say so |
| `.github/workflows/checks.yml` | Formatting, lints and tests on every pull request |
| `GLOSSARY.md` | The words maquette uses for armatures, views and proportions |
| `docs/research/` | The research behind the founding and the reorientation. Evidence only |

## Working here

GitHub Issues and pull requests in `joeloverbeck/maquette` own active work. Agents start from `CLAUDE.md`. Jon is the acceptance authority.

Maquette's first consumer is mudsong, a private repository. Nothing here needs it to be present.

## Licence

MIT; see [`LICENSE`](LICENSE). The constitution in `docs/principles/` is Jon's own document, copied from its master repository.
