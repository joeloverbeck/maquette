# maquette

Maquette generates armatures for anime-style characters from proportions measured on published drawings: the construction guide for a head and body, as SVG with named landmarks, for an agent to draw and paint over. It does not draw the finished character.

In an animation studio a maquette is the sculpted reference model that keeps a character on model from every angle.

## Status

Founded on 2026-11-16, and reoriented by Jon the same day toward measured proportions; [`docs/foundation/README.md`](docs/foundation/README.md) records both. Work starts from the issue tracker.

The generator draws a set of proportions as a head in three views: front, three-quarter and side. The program lists how far such a head stands from Saffi's hand-made views, reads an armature back in VectorCraft, and reads her hand-made drawings into numbers. Jon accepted her front and three-quarter views at look 1, and her ears and lids have changed since, by his ruling. At look 2 he found that those two views read as one head and did not accept the side view: from the nose down the face stood too far forward. The first named correction now sets her mouth and chin back in the side view only. At look 3, on 2026-11-17, Jon accepted her armature in three views with that correction at the smaller of the two amounts he was shown. Those six files are in [`armatures/saffi/`](armatures/saffi/README.md), and a test pins their bytes. His acceptance covers her armature and nothing else: it does not make her proportions a standard, and it accepts no table. Nothing is exported to mudsong yet. The measuring of references and the proportion tables are not built.

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

The first prints the list of proportions: the unit, and each proportion's name, whether it is measured or a convention, and how it is taken in each view. The second writes `front.svg`, `three-quarter.svg` and `side.svg`, each with a landmark file beside it, into a folder Git ignores. `cargo run -- help` lists every command. The same input gives the same bytes.

Every landmark is one point in space and each view is a flat projection of the same points, so a landmark has one height in all three views. The side view is the three-quarter view's turn carried to 90°, facing the viewer's right: a landmark's sideways position there is its depth. It shows the near eye, brow and ear only, its face outline runs through the centre-line landmarks, and the list of proportions states how each of its guides is built.

A departure from flat projection is a named correction with an amount. The list of proportions defines each, a set of proportions carries it in a `[corrections.<name>]` table, and the review sheet shows it. There is one, `lower_face_set_back`: in the side view only, the mouth and the chin are drawn further back than their depths, and the landmark file gives each moved landmark's depth in space beside the depth it is drawn at. The front and three-quarter views are the same bytes with it and without it, and a set that carries no correction is drawn flat in every view.

It moves no landmark above the mouth. It does change guides there: the face outline is one smooth curve, so it bends differently between the eye line and the nose mark, and behind the mouth up to the nose level, and the two view points read off it there shift. In Saffi's side view, where the nose level line meets the outline's front stands 1.0 unit further forward, and the neck's front line starts 1.1 units higher. The list of proportions says this where it defines the correction.

```sh
cargo run -- proportions-from-landmarks \
  --front target/saffi/front.landmarks.toml \
  --three-quarter target/saffi/three-quarter.landmarks.toml \
  --side target/saffi/side.landmarks.toml
```

This takes an armature's landmark files back into the set of proportions it was drawn from: the proportions from the front and three-quarter views, and a correction from the side view, whose landmark file is the only one that states it. Without `--side` no correction is taken back, and the set written says so.

Saffi's set carries it because all her depths come from her three-quarter view, which puts the mouth almost straight below the nose mark and the chin close behind, and a side view drawn from that has a muzzle. Her set is written by:

```sh
cargo run -- proportions-from-hand-made-views \
  --record data/saffi/hand-made-views.toml \
  --final-record data/saffi/final-drawings.toml \
  --lower-face-set-back 0.5 \
  --out data/saffi/proportions.toml
```

The share is how much of the chin's full amount is taken, where the full amount draws the chin straight below the iris's centre; the mouth then goes onto the line from the nose mark's bottom to the chin as drawn. Both rules come from drawing lessons, which give no distances. No rule gives the share. Jon was shown 0.5 and 1 at look 3 and accepted her armature at 0.5; `design/look-3-saffi-side-view/` holds the sheet he saw, and the other candidate's input as the record of what was shown.

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

This writes `front.hand-made-overlay.svg` and `three-quarter.hand-made-overlay.svg`: in each view's own units, every position the records give as an orange dot, a dashed ring where the head covers it in her drawing, and every difference above 1% as a line from the generated position to the hand-made one with the number the differences file gives it. Placed over the view's SVG at the same place and size, an overlay lies over it exactly. `design/look-2-saffi-three-views/README.md` says how the review sheet is composed from these files in FigCraft, and `design/look-3-saffi-side-view/README.md` how the sheet with the two candidate side views is.

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
| `data/saffi/` | The hand-made front and three-quarter views of the comparison case, as landmark positions: her S2 underdrawings, and the ears and lids of her final drawings that replace theirs. Also her set of proportions worked out from them, and the list of her drawings in mudsong that these numbers rest on. Numbers only; they never enter a proportion table |
| `armatures/saffi/` | Saffi's accepted armature: the six files `maquette draw` writes from her set of proportions, as Jon accepted them at look 3. Generated output, not a design and not a table. A test pins their bytes |
| `proportions/list.toml` | The list of proportions: the unit, and every proportion's name, definition for each view, and mark as measured or a convention, how each guide is built in each view, and each named correction with the rule of its amounts. The program reads this same file |
| `Cargo.toml`, `src/` | The Rust package: the list and sets of proportions, the head model, the three views, the guides, the armature writer, the reading of a hand-made drawing, the differences from a hand-made armature and the read-back in VectorCraft, under a thin command line |
| `tests/` | Tests that drive the command line, files in and files out. `tests/fixtures/` holds made-up proportions, made-up views and a made-up drawing that say so, and recordings of VectorCraft's replies that say so |
| `.github/workflows/checks.yml` | Formatting, lints and tests on every pull request |
| `.gitattributes` | Keeps Git from converting line endings in `armatures/`, so that a checkout cannot change the pinned bytes |
| `GLOSSARY.md` | The words maquette uses for armatures, views and proportions |
| `docs/research/` | The research behind the founding and the reorientation. Evidence only |

## Working here

GitHub Issues and pull requests in `joeloverbeck/maquette` own active work. Agents start from `CLAUDE.md`. Jon is the acceptance authority.

Maquette's first consumer is mudsong, a private repository. Nothing here needs it to be present.

## Licence

MIT; see [`LICENSE`](LICENSE). The constitution in `docs/principles/` is Jon's own document, copied from its master repository.
