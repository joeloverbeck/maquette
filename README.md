# maquette

Maquette generates armatures for anime-style characters from proportions measured on published drawings: the construction guide for a head and body, as SVG with named landmarks, for an agent to draw and paint over. It does not draw the finished character.

In an animation studio a maquette is the sculpted reference model that keeps a character on model from every angle.

## Status

Founded on 2026-11-16, and reoriented by Jon the same day toward measured proportions; [`docs/foundation/README.md`](docs/foundation/README.md) records both. Work starts from the issue tracker.

The generator draws a set of proportions as a head in three views: front, three-quarter and side. The program lists how far such a head stands from Saffi's hand-made views, reads an armature back in VectorCraft, and reads her hand-made drawings into numbers. Jon accepted her front and three-quarter views at look 1; her ears and lids have changed since, by his ruling, and her side view has not been looked at, so no armature is pinned yet.

The measuring path is built: the reading instructions, the record of a reading, the commands that set readings against each other, and the reference check sheet. It has been run on made-up readings and on Saffi's recorded positions. [`docs/research/2026-11-16-views-and-measuring/notes/measuring_path_dry_run.md`](docs/research/2026-11-16-views-and-measuring/notes/measuring_path_dry_run.md) records what that showed, and the dry run on pictures of her hand-made drawings. No published drawing has been read, and the proportion tables are not built.

## Building, checking and drawing

The code is one Rust package: a library with a thin command-line program over it. `rust-toolchain.toml` pins the toolchain, and `cargo` fetches the crates it needs; `Cargo.toml` says what each is for.

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

The first prints the list of proportions: the unit, and each proportion's name, whether it is measured or a convention, and how it is taken in each view. The second writes `front.svg`, `three-quarter.svg` and `side.svg`, each with a landmark file beside it, into a folder Git ignores. `cargo run -- help` lists every command. The same input gives the same bytes.

Every landmark is one point in space and each view is a flat projection of the same points, so a landmark has one height in all three views. The side view is the three-quarter view's turn carried to 90°, facing the viewer's right: a landmark's sideways position there is its depth. It shows the near eye, brow and ear only, its face outline runs through the centre-line landmarks, and the list of proportions states how each of its guides is built. No view carries a correction: a departure from flat projection would be named, given an amount and shown on the review sheet.

### The differences from the hand-made views

```sh
cargo run -- differences-from-hand-made-views \
  --front target/saffi/front.landmarks.toml \
  --three-quarter target/saffi/three-quarter.landmarks.toml \
  --record data/saffi/hand-made-views.toml \
  --final-record data/saffi/final-drawings.toml \
  --out target/saffi/differences.toml
```

This lays the two generated views over Saffi's hand-made ones and writes, for every landmark both give, how far apart they put it as a share of head height. It prints the differences above 1%, numbered; the armature review sheet puts those numbers on the drawing. The file states how the views are laid over each other, which is one scale and one shift for a view and fitted to no landmark, and what has no counterpart on either side. It lists differences and judges none: whether one matters is Jon's look.

`--final-record` names the record of her final drawings: each of its landmarks, which are her ears and her lids, replaces the record's landmark of the same name in the same view, and a difference against one is marked. Without it the comparison is with the S2 underdrawings alone. No hand-made side view exists, so a generated side view has nothing to be laid over.

### The overlays for a review sheet

```sh
cargo run -- overlay-hand-made-views \
  --front target/saffi/front.landmarks.toml \
  --three-quarter target/saffi/three-quarter.landmarks.toml \
  --record data/saffi/hand-made-views.toml \
  --final-record data/saffi/final-drawings.toml \
  --out target/saffi
```

This writes `front.hand-made-overlay.svg` and `three-quarter.hand-made-overlay.svg`: in each view's own units, every position the records give as an orange dot, a dashed ring where the head covers it in her drawing, and every difference above 1% as a line from the generated position to the hand-made one with the number the differences file gives it. Placed over the view's SVG at the same place and size, an overlay lies over it exactly. `design/look-2-saffi-three-views/README.md` says how the review sheet is composed from these files in FigCraft.

### Reading a hand-made drawing

```sh
cargo run -- list-drawing --svg <a drawing's SVG>
cargo run -- record-hand-made-drawings \
  --underdrawings <the folder of the S2 underdrawings> \
  --final-drawings <the folder of the final drawings> \
  --out <a folder>
```

`list-drawing` prints what any hand-made drawing holds: every group, and every shape under its address, with its nodes numbered in path order and how far it reaches. It is what a reading of a new drawing is written from, in place of searching the file by hand. A drawing is read as written: one that moves its coordinates with a transform, or draws a path with anything but the absolute `M`, `L`, `C` and `Z`, is refused.

`record-hand-made-drawings` reads Saffi's four hand-made drawings and writes the three records of them in `data/saffi/`. The drawings are private and no clean clone has them: [`data/saffi/README.md`](data/saffi/README.md) lists each with its digest and where it is kept. Each is read only when its SHA-256 digest is the recorded one. Where a folder or a drawing is absent, or a digest differs, the command says so and writes nothing, and the committed numbers stand as recorded. Nothing else in the repository needs the drawings.

### The read-back in VectorCraft

An armature is put to Jon or exported only after its SVG has been read back in VectorCraft ([`docs/foundation/02-first-table.md`](docs/foundation/02-first-table.md), "The SVG contract"). That is run locally, for each view, where `vectorcraft-cli` is on the PATH:

```sh
cargo run -- read-back --landmarks target/saffi/front.landmarks.toml
cargo run -- read-back --landmarks target/saffi/three-quarter.landmarks.toml
cargo run -- read-back --landmarks target/saffi/side.landmarks.toml
cargo run -- read-back-hard-case --landmarks target/saffi/three-quarter.landmarks.toml
```

`read-back` starts `vectorcraft-cli mcp` itself, allowed to read the SVG's folder and to write nothing, opens the SVG the landmark file names, and compares what VectorCraft holds with the landmark file: no import warning, the same names each once, every anchor within 0.001 unit, every guide visible, stroked and unfilled. It prints the read-back and the VectorCraft version it ran against, which is whichever is installed. It exits 0 on a pass and 1 on a failure. Where `vectorcraft-cli` is not installed it says so and exits 2: nothing was checked, which is neither. A server that stops, or leaves a request unanswered for a minute, also ends the run with 1; the message says which request it was and passes on what the server wrote to its error output. `read-back-hard-case` runs the contract's hard case on a three-quarter view, with the two spoiled copies the oracle must fail. It refuses a front view and a side view: the hard case needs left and right pairs that stand in different places.

A pass says VectorCraft keeps what the landmark file says. It does not say the landmark file is right or that the armature looks right.

`cargo test` does not start VectorCraft. It tests the comparison on recordings of VectorCraft's replies in `tests/fixtures/read-back/`, which say they are recordings and of which version. The same checks run live with:

```sh
cargo test --test read_back_in_vectorcraft -- --ignored
```

`tests/vectorcraft_stand_in.rs` runs the read-back against a stand-in for the server, which is that test's own program under the name `vectorcraft-cli`: one that stops at once, in `cargo test`, and one that never answers, which waits out the minute and so runs only with `cargo test --test vectorcraft_stand_in -- --ignored`.

## Measuring

Measuring takes proportions from drawings. A reader, which is an agent, judges a drawing and places its landmarks; every drawing is read twice by readers who do not see each other's reading; and Jon checks each character on a reference check sheet. [`docs/agents/reading-instructions.md`](docs/agents/reading-instructions.md) is what a reader follows, and `cargo run -- reading-instructions` prints it.

### The reference folder

Reference images, the readings of them, everything worked out from one reading and the reference check sheets are kept in one folder outside the repository, and nowhere else: the repository is public and the drawings are other people's.

- **Where it is configured.** The environment variable `MAQUETTE_REFERENCES` names the folder. Nothing committed names it, and there is no default.
- **What is inside it.** Whatever the work needs: pictures, readings, results, sheets. Every path a measuring command is given for one of these is taken from the folder, and a path that leaves it is refused, so nothing can be read into a reading from the repository or written into it by accident.
- **When it is absent.** Every measuring command stops and says that the folder is not configured, or that the folder named does not exist, and where it is configured. Drawing an armature, its checks and `cargo test` need no reference folder: the tests make scratch folders of their own with made-up pictures.
- **What is never committed.** A reference image or a crop of one, a reading, anything worked out from the readings of one reference, and a reference check sheet ([`docs/foundation/04-evidence-and-references.md`](docs/foundation/04-evidence-and-references.md)).

### Reading a drawing

```sh
export MAQUETTE_REFERENCES=<the reference folder>
cargo run -- start-reading --image <picture> --view front --reader <label> --out <reading>
cargo run -- enlarge-image --image <picture> --left 400 --top 300 --width 200 --height 120 --factor 6 --grid 10 --out <enlargement>
cargo run -- check-reading --reading <reading>
```

`start-reading` writes a reading to fill in: the picture's SHA-256 digest and size, the questions of the judgement (the view, which way a turned head faces, the turn, whether the head is level, each with its reason), and a line for every landmark the view asks for. A landmark is placed, in pixels of the picture, or stated as not seen with the reason. The skull is never read: it lies under the hair. `enlarge-image` enlarges a part of a picture, each pixel as a block, with a grid that is labelled in the picture's own pixels. `check-reading` says what a reading lacks or gets wrong.

A reading says the head is level, or sets the drawing aside. A drawing that is set aside or not called level is not measured by any command.

### What readings say

```sh
cargo run -- agreement-of-readings --first <reading> --second <reading> --out <file>
cargo run -- turn-from-readings --front <readings> --three-quarter <readings> --out <file>
cargo run -- fit-of-views --front <readings> --three-quarter <readings> --out <file>
cargo run -- proportions-from-readings --front <readings> --three-quarter <readings> --out <file>
cargo run -- reference-check-sheet --sheet <description> --out <sheet.svg>
```

`<readings>` is one reading, or the two readings of the drawing with a comma between them: a landmark then stands midway between the two. Every file these write states the rules behind its numbers. In short:

- **The agreement** between two readings of one drawing: how far apart they put each landmark, as a share of head height. A disagreement above 1% is numbered.
- **The turn** a three-quarter drawing shows, from how far the spacing of left and right pairs narrows against the front view. It is worked out from the positions; the readers' own estimates stand beside it.
- **The fit** of a character's views to one head, as one figure: the largest misfit in level, in spacing against the turn, and from the front view's centre line. It is zero when the views are plain projections of one level, symmetric head.
- **The proportions** the readings give, by the head model's inverse direction, with the proportions they do not give and why. A published drawing shows no skull, so the skull is built by the rule the list of proportions states: its centre at the height and the depth of the middle of the ear's base, its radius the distance to the temple. `cargo run -- skull-by-rule --proportions <set>` sets the rule against a set whose skull is drawn.
- **The reference check sheet**, rendered from a description that lists each character's readings: each character's views side by side with the points read, where two readings disagree, what was set aside and why, and the fit. It is one SVG that holds its drawings. Its layout follows the mock-up in `design/first-table-sheets/`; `src/reference_check_sheet.rs` says where a real sheet departs from it and why.

A front view and a three-quarter view are taken back into proportions. A side view can be read, compared and shown, and its heights count in the fit; taking depth from it is not built, because no side drawing has been read yet.

### Testing a reader on known positions

An agent's reading is an artificial evaluation: it is trusted only as far as it has matched drawings whose positions are known (`EVAL-2`). Saffi's hand-made drawings are such drawings, and their positions are in `data/saffi/hand-made-views.toml`.

```sh
cargo run -- score-reading --reading <reading> --known-positions data/saffi/hand-made-views.toml --out <file>
cargo run -- known-positions-as-reading --known-positions data/saffi/hand-made-views.toml --view front --image <picture> --out <reading>
cargo run -- compare-proportions --first <set> --second <set>
```

`score-reading` gives a reader's error by kind of landmark, and keeps apart what cannot be seen in the picture. `known-positions-as-reading` writes the record's own positions as a reading, which is not one: the measuring commands run on it show what they make of exact positions, so that a reader's error can be told from the method's. `compare-proportions` sets two sets of proportions against each other. The readers must not see the record or the drawings' vector files, and the reading is tested again whenever the reading instructions change.

## Where things are

| Path | What it is |
|---|---|
| `CLAUDE.md` | Agent instructions; `AGENTS.md` is a symlink to it |
| `docs/principles/` | The constitution this repository adopts, and its five adopted modules, copied byte for byte |
| `docs/foundation/` | What maquette is for, the first increment, Jon's direction, the rules for evidence and references, and the rules for code and design tools |
| `.claude/skills/` | The engineering and productivity skills from `mattpocock/skills` (MIT), copied unchanged at its commit `5863806`; `.agents/skills/` holds a symlink to each |
| `.mcp.json` | Registers the VectorCraft and FigCraft MCP servers for Claude Code |
| `design/` | Designs made by hand in those two tools; the only folder their servers may write to |
| `docs/agents/` | How agents use the issue tracker and the triage labels, and the reading instructions a reader of a drawing follows |
| `data/saffi/` | The hand-made front and three-quarter views of the comparison case, as landmark positions: her S2 underdrawings, and the ears and lids of her final drawings that replace theirs. Also her set of proportions worked out from them, and the list of her drawings in mudsong that these numbers rest on. Numbers only; they never enter a proportion table |
| `proportions/list.toml` | The list of proportions: the unit, and every proportion's name, definition for each view, and mark as measured or a convention, and how each guide is built in each view. The program reads this same file |
| `Cargo.toml`, `src/` | The Rust package: the list and sets of proportions, the head model, the three views, the guides, the armature writer, the reading of a hand-made drawing, the differences from a hand-made armature, the read-back in VectorCraft, and measuring: the reference folder, the record of a reading, what readings say when set against each other, the scoring of a reading against known positions, the enlarging of a picture and the reference check sheet, under a thin command line |
| `tests/` | Tests that drive the command line, files in and files out. `tests/fixtures/` holds made-up proportions, made-up views, a made-up drawing and a made-up record of known positions that say so, and recordings of VectorCraft's replies that say so. The readings and pictures the measuring tests use are made up by the tests themselves |
| `.github/workflows/checks.yml` | Formatting, lints and tests on every pull request |
| `GLOSSARY.md` | The words maquette uses for armatures, views and proportions |
| `docs/research/` | The research behind the founding and the reorientation. Evidence only |

## Working here

GitHub Issues and pull requests in `joeloverbeck/maquette` own active work. Agents start from `CLAUDE.md`. Jon is the acceptance authority.

Maquette's first consumer is mudsong, a private repository. Nothing here needs it to be present.

## Licence

MIT; see [`LICENSE`](LICENSE). The constitution in `docs/principles/` is Jon's own document, copied from its master repository.
