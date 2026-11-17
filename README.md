# maquette

Maquette generates armatures for anime-style characters from proportions measured on published drawings: the construction guide for a head and body, as SVG with named landmarks, for an agent to draw and paint over. It does not draw the finished character.

In an animation studio a maquette is the sculpted reference model that keeps a character on model from every angle.

## Status

Founded on 2026-11-16, and reoriented by Jon the same day toward measured proportions; [`docs/foundation/README.md`](docs/foundation/README.md) records both. Work starts from the issue tracker.

The generator draws a set of proportions as a head in three views: front, three-quarter and side. The program lists how far such a head stands from Saffi's hand-made views, reads an armature back in VectorCraft, and reads her hand-made drawings into numbers. Jon accepted her front and three-quarter views at look 1, and her ears and lids have changed since, by his ruling. At look 2 he found that those two views read as one head and did not accept the side view: from the nose down the face stood too far forward. The first named correction now sets her mouth and chin back in the side view only. At look 3, on 2026-11-17, Jon accepted her armature in three views with that correction at the smaller of the two amounts he was shown. Those six files are in [`armatures/saffi/`](armatures/saffi/README.md), and a test pins their bytes. His acceptance covers her armature and nothing else: it does not make her proportions a standard, and it accepts no table. Nothing is exported to mudsong yet.

The measuring path is built: the reading instructions, the record of a reading, the commands that set readings against each other, and the reference check sheet. It has been run on made-up readings, on Saffi's recorded positions, and in a dry run on 2026-11-17: two agents read pictures of her two hand-made underdrawings, whose positions are known. They placed the features of the face to 0.2% of head height on average; the worst of them, the top of an eye opening, was 1.7% off. Their readings gave the recorded turn to within a degree, by a list of counted pairs that had been narrowed after seeing what Saffi's exact positions gave. Three landmarks on smooth curves (the jaw point, the neck's base and the shoulder point) were read up to 26% off, which was a fault of their definitions, and the reading instructions were revised once for it. Two new readers then read the same pictures under the revision: in the front view those three came down to under 1.5%, in the three-quarter view to between 0.1% and 7.5%, and the features held, at 0.2% on average and 1.6% at worst. The skull rule has no input from these pictures, because the hair covers both temples. [`docs/research/2026-11-16-views-and-measuring/notes/measuring_path_dry_run.md`](docs/research/2026-11-16-views-and-measuring/notes/measuring_path_dry_run.md) records the figures and their limits: they hold for clean hand-made drawings with known positions and no further. A first batch of published drawings has since been read by agents, in front and side views, and the side-view part of the reading instructions was revised once for what those readers reported, to 2026-11-17.3, and tested once on Saffi's generated side view. Jon checked the batch on a reference check sheet at look 4, on 2026-11-17, and accepted all six characters: a front view of each of six works and a side view of three. So the front view has the six works a table needs, and depth from a front and a side view has three, which is too thin. Of the eighteen readings he checked, thirteen were made by Fable 5.2 and five by Opus 5.5, whose reading accuracy no test here has measured. His acceptance says the heads are level and the points sit on the right features as he sees them; two readers agreeing is no proof that a published head is read rightly, and nothing in the batch is set against known positions. The commands of a proportion table are built, and tested on made-up readings: one builds a table from the record of Jon's check, one sets a set of proportions against a table, and one makes a whole set of proportions from a table for a head to be drawn.

The first table has been built from that batch and is in [`tables/adult-women/`](tables/adult-women/README.md). **It is a proposal: Jon has not accepted it.** It gives 25 of the 74 proportions a valid range, each from five or six works. It lists 27 as too thin and 19 as given by no work, with counts and reasons and no numbers, and the skull's 3 conventions. Depth comes from three side views, so depth is not a table; no three-quarter view was measured; and no skull is built. The jaw point has no range: Jon ruled after look 5 that it is left out until the landmark is redefined, because the readers placed it at the corner of the chin. Set against the table, Saffi's proportions are inside 19 ranges and outside 6, each by under 0.6% of head height, and her other 49 have no range to be set against. That is a finding, and nothing was changed to bring her in.

The table is drawn as Saffi's own head with the table's values put in where the references give them, which Jon decided on 2026-11-17. **At look 5 he did not accept it,** because of the side view's nose: the head model drew the tip of the nose, whose depth a side view gives, at the height of the front view's nose mark, which is the base of the nose. Three things were repaired. The tip of the nose has a height of its own, `nose_tip_drop`, taken from the side view; the jaw point left the table; and the neck is borrowed whole. The head for look 6 has 25 proportions from the table's middle values, 16 averages of two to four works, marked as thin, and 33 borrowed from Saffi, marked as borrowed. Its side view carries no correction. The sheets are in [`design/look-5-first-table/`](design/look-5-first-table/README.md), as the record of what he saw, and [`design/look-6-first-table/`](design/look-6-first-table/README.md). Jon has not yet looked at the second.

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

Every landmark is one point in space and each view is a flat projection of the same points, so a landmark has one height in all three views. The side view is the three-quarter view's turn carried to 90°, facing the viewer's right: a landmark's sideways position there is its depth. It shows the near eye, brow and ear only, its face outline runs through the centre-line landmarks, and the list of proportions states how each of its guides is built.

A departure from flat projection is a named correction with an amount. The list of proportions defines each, a set of proportions carries it in a `[corrections.<name>]` table, and the review sheet shows it. There is one, `lower_face_set_back`: in the side view only, the mouth and the chin are drawn further back than their depths, and the landmark file gives each moved landmark's depth in space beside the depth it is drawn at. The front and three-quarter views are the same bytes with it and without it, and a set that carries no correction is drawn flat in every view.

**The tip of the nose and the nose mark are two points.** A front view marks the base of the nose, and a side view shows its tip, which stands higher. A set gives the tip its own height, `nose_tip_drop`, at the depth `nose_mark_bottom_depth`. Where the two heights differ, the landmark files name a `nose-tip`: the side view's face outline, the three-quarter view's centre line and the nose mark of both run through it, and the front view's nose mark runs through its own bottom as before. Where a set gives the tip the height of the nose mark's bottom, as one with no measured side view does by rule, the two are one point and no file names a tip. Saffi's set is such a one, and her armature is drawn as it was, byte for byte.

The correction moves no landmark above the mouth. It does change guides there: the face outline is one smooth curve, so it bends differently between the eye line and the nose mark, and behind the mouth up to the nose level, and the two view points read off it there shift. In Saffi's side view, where the nose level line meets the outline's front stands 1.0 unit further forward, and the neck's front line starts 1.1 units higher. The list of proportions says this where it defines the correction.

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

`list-drawing` prints what any hand-made drawing holds: every group, and every shape under its address, with its nodes numbered in path order and how far it reaches. It is what a reading of a new drawing is written from, in place of searching the file by hand. A drawing is read as written: one that moves its coordinates with a transform, draws a path with anything but the absolute `M`, `L`, `C` and `Z`, or draws on after `Z` without a new `M`, is refused. A shape is listed as filled or not as SVG has it: by its own `fill`, in its `style` or as an attribute, or else by its group's.

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

Reference images, the readings of them, everything worked out from one reading and the reference check sheets are kept in one folder outside every repository, and nowhere else: this repository is public and the drawings are other people's.

- **Where it is configured.** The environment variable `MAQUETTE_REFERENCES` names the folder. Nothing committed names it, and there is no default. A folder inside a Git repository, this one or any other, is refused: nothing kept there could then be committed.
- **What is inside it.** Whatever the work needs: pictures, readings, results, sheets. Every path a measuring command is given for one of these is taken from the folder. A path that leaves it is refused, whether by its own steps or through a link inside the folder that leads out of it, so nothing can be read into a reading from the repository or written into it by accident.
- **When it is absent.** Every measuring command stops and says that the folder is not configured, or that the folder named does not exist, and where it is configured. Drawing an armature, its checks and `cargo test` need no reference folder: the tests make scratch folders of their own with made-up pictures. They make them in the system's temporary folder, because `target/` is inside this repository.
- **What is never committed.** A reference image or a crop of one, a reading, anything worked out from the readings of one reference, and a reference check sheet ([`docs/foundation/04-evidence-and-references.md`](docs/foundation/04-evidence-and-references.md)).

### Reading a drawing

```sh
export MAQUETTE_REFERENCES=<the reference folder>
cargo run -- start-reading --image <picture> --view front --reader <label> --out <reading>
cargo run -- enlarge-image --image <picture> --left 400 --top 300 --width 200 --height 120 --factor 6 --grid 10 --out <enlargement>
cargo run -- check-reading --reading <reading>
```

`start-reading` writes a reading to fill in: the picture's SHA-256 digest and size, the questions of the judgement (the view, which way a turned head faces, the turn, whether the head is level, each with its reason), and a line for every landmark the view asks for. A landmark is placed, in pixels of the picture, or stated as not seen with the reason. The skull is never read: it lies under the hair. `enlarge-image` enlarges a part of a picture by a whole factor of at most 16, each pixel as a block, with a grid that is labelled in the picture's own pixels. `check-reading` says what a reading lacks or gets wrong.

A picture is a PNG or a JPEG, the two formats official sites publish heads in, and is told by its bytes, not by its name. It is read as it was published and never converted, because the digest a reading records is of the file itself. An enlargement is written as a PNG, and a reference check sheet holds each picture in the format it has. A head that is published cut out of its background is enlarged as it is shown on a white page: what a see-through pixel holds underneath, which may be black or more of the drawing than the picture shows, is not enlarged. A picture whose file says it is to be shown turned or mirrored, as a JPEG's EXIF orientation can, is refused: a browser, and so the sheet, would show it turned while its pixels are read as they are stored.

A reading says the head is level, or sets the drawing aside. A drawing that is set aside or not called level is not measured by any command.

### What readings say

```sh
cargo run -- agreement-of-readings --first <reading> --second <reading> --out <file>
cargo run -- turn-from-readings --front <readings> --three-quarter <readings> --out <file>
cargo run -- fit-of-views --front <readings> [--three-quarter <readings>] [--side <readings>] --out <file>
cargo run -- proportions-from-readings --front <readings> [--three-quarter <readings> | --side <readings>] --out <file>
cargo run -- reference-check-sheet --sheet <description> --out <sheet.svg>
cargo run -- start-jons-check --sheet <description> --out <check>
cargo run -- jons-check --check <check>
```

`<readings>` is one reading, or the two readings of the drawing with a comma between them: a landmark then stands midway between the two.

A reading is measured only when it passes what `check-reading` checks: it is complete, and its picture is still the picture that was read. The two readings of a drawing are two files by two readers, each under its own reader's label: one file named twice is refused, and so are two readings that carry one label. They follow one revision of the reading instructions. A reading made under an earlier revision is still measured, and whatever is written from it says which revision it followed.

Every file these write states the rules behind its numbers. In short:

- **The agreement** between two readings of one drawing: how far apart they put each landmark, as a share of head height. A disagreement above 1% is numbered. Most landmarks are compared as positions, by the straight distance. The hair's top and the top and bottom of an iris are compared in height only, and a side extent of the hair, of an iris or of an ear sideways only: each says how far a form reaches, and along a level or an upright stretch of its edge two readers may put the point anywhere. Where the face outline crosses the eye line, the nose level or the mouth level it is compared sideways only, because its height is the level's.
- **The turn** a three-quarter drawing shows, from how far the spacing of left and right pairs narrows against the front view. It is worked out from the positions; the readers' own estimates stand beside it. Where the front view shows a pair with no spacing, or the three-quarter view shows the pairs wider apart than the front view does, there is no turn: the command fails and writes nothing.
- **The fit** of a character's views to one head, as one figure: the largest misfit in level, in spacing against the turn, and from the front view's centre line. It is zero when the views are plain projections of one level, symmetric head. The front view is fitted with a three-quarter view, a side view or both. With a side view alone there is no turn: the two are set against each other in height, each in its own unit. A side view's heights of the nose mark's bottom and corner are not compared: the reading instructions put them under the tip of the nose and at the middle of the nostril in a profile, other points of the nose than a front view's.
- **The proportions** the readings give, by the head model's inverse direction, with the proportions they do not give and why. A published drawing shows no skull, so the skull is built by the rule the list of proportions states: its centre at the height and the depth of the middle of the ear's base, its radius the distance to the temple. `cargo run -- skull-by-rule --proportions <set>` sets the rule against a set whose skull is drawn.
  - *A front view alone* gives heights and widths and no depth.
  - *A front view and a three-quarter view* give depth at the turn the readings show.
  - *A front view and a side view* give depth as the side view shows it. Each view is measured in its own unit, eye line to chin, which is upright and so as long from the side as from the front: the two drawings may be of two sizes. Depth zero in a side view is midway between the near eye's two corners, and a side view may face either way. Heights and widths are the front view's, and the side view stands in for a height the front view does not show, except where a profile names another point: the nose mark's bottom and corner, and an ear's side extents. The tip of the nose is the one height that is the side view's own: where it places `nose-mark-bottom`, under the tip of the nose.
  - *What a side view cannot give.* It is asked for no temple, no cheek point and no jaw point, so the face outline's top end, the cheek points at the eye line, the nose level and the mouth level, and the jaw point get no depth. The skull's rule needs the temple's depth, so the skull is not built from a front and a side view. The list of proportions says this proportion by proportion.
- **The reference check sheet**, rendered from a description that lists each character's readings: each character's views side by side with the points read, where two readings disagree, what each reader judged and why, what was set aside and why, and the fit. Where two readings could not be compared, or are not the two readings of one drawing, the card of that drawing says so and the rest of the sheet stands. It is one SVG that holds its drawings. Its layout follows the mock-up in `design/first-table-sheets/`; `src/reference_check_sheet.rs` says where a real sheet departs from it and why.
  - One published picture often holds two heads of a character. The description then names each drawing by itself, with its own readings and, in words, which picture and head it is, and the sheet shows them as two drawings.
  - Each card says which picture it is and how high the head is in it, in pixels. The header says that a disagreement sets the two readings against each other and against nothing known, and that the reading of a side view has been tested once on known positions, on Saffi's generated side view, which is a drawing of guides, and never on a painted side view.
  - Where no character has a three-quarter view, the sheet says so once and the front and the side view stand beside each other.

Reading a side view by the instructions at 2026-11-17.3 has been tested once on a drawing whose positions are known: Saffi's generated side view, by two readers, both Fable 5.2. That drawing is guides, so the test says nothing of how a painted side view is read, and nothing has tested that. Depth from a side view is tested against known depths on made-up heads only.

### Jon's check

Jon checks every character on a reference check sheet and says his decisions in words. An agent writes them into a record, a small text file in the reference folder: `start-jons-check` writes one to fill in from the sheet's description, and `jons-check` says what a record does and what it still lacks.

- **A verdict for each character:** accepted or rejected. Only an accepted character's readings are measured.
- **A ruling on a drawing, where he overrules its readers:** `measure`, for a drawing a reader set aside, or `set aside`, for one the readers measured. A drawing ruled to be measured is measured from each reading that placed its landmarks; one that both readers set aside must be read again first.
- **Points marked as off:** each is left out of that drawing, and the drawing is kept. A mark is a landmark's name, or a name with `*` for every landmark it fits, such as `ear-*`. Where Jon rules that one reader placed a landmark wrongly and the other rightly, the mark names that reader, as `{ landmark = "brow-line-*", reader = "<the reader's label>" }`: the landmark is left out of that reading alone and measured from the other reading's point, and what is written says so.
- **Proportions marked as off for a character:** each is not given for that work, and its landmarks stay. The eye opening's height can be left out while the opening still gives the eye line.
- **The readings he checked** are named with the digests of their files. A reading that has changed since is refused.

`turn-from-readings`, `fit-of-views` and `proportions-from-readings` take the record as `--check <check>`. A record that `jons-check` does not call complete measures nothing, and the command says what it lacks. With a complete one they measure only what he accepted, as he ruled, with his marks left out, and each drawing from all the readings the record lists for it: naming only some of them measures nothing. Without it, what they write says that Jon has not checked the readings.

`jons-check` also counts the accepted works for each view, and for each route to depth. A view with fewer than five is reported as too thin (`docs/foundation/04-evidence-and-references.md`).

### The table

```sh
cargo run -- table-from-readings --check <check> --out <table>
cargo run -- set-against-table --proportions <set> --table <table> [--out <file>]
cargo run -- set-from-table --check <check> --borrow <set> --out <set>
cargo run -- overlay-sources --proportions <set> --armature <folder> --out <folder>
```

`table-from-readings` builds a proportion table from a record of Jon's check that is complete, and from nothing else. For each proportion of the list it gives the lowest, the middle and the highest value among the works whose readings he accepted, and the number of works behind it. The middle is the median. A drawing that is set aside or not called level is left out, and so is a rejected character. Each accepted character is one work: heights and widths are its front view's, and depth is taken from its side view, or else from its three-quarter view.

- **Given, too thin, no work.** A proportion with five works or more behind it is given, and its lowest and highest value are its valid range. One with fewer is too thin: it is listed with its count of works and why the others do not give it, and with no number, because the values of one to four works are those works' own numbers. The floor is held proportion by proportion: in a view that has five works, a proportion that hair or a mark of Jon's leaves with fewer is too thin all the same, because a range is backed only by the works that show that feature. A view with fewer than five works is reported as too thin and is not called a table.
- **Conventions.** A convention is built by rule and never measured, so it has no valid range.
- **What the table says of itself.** That its ranges are separate and do not rule on which combinations of proportions go together, and that a lowest and a highest value is one reference's number each, tied to no work. It lists the works it was measured on by work, designer and what the character is. Where each picture was taken from stays in its readings.
- **The comparison case stays out.** The command takes the record of Jon's check and no set of proportions, and reads only from the reference folder, which is never inside a repository. It refuses a record's positions written as a reading by `known-positions-as-reading`, which is the one way the program has of turning recorded positions, such as Saffi's, into something a record of Jon's check can name. An agent's reading of a picture of a comparison case is not recognised by the program: the list of works in the table, and Jon's check before it, are what keep that out.
- **A table is a proposal until Jon has accepted it.** It is written into the reference folder. One that is put to him is copied into `tables/` on the branch of its pull request, which merges only if he accepts it.

`set-against-table` sets a set of proportions against a table and writes, for each proportion the set gives, whether it is inside or outside its valid range, with how far beyond in units and as a share of the set's own head height, or that it has no range to be set against: the table has fewer than five works behind it, or none, or the proportion is a convention. A value on either end of a range is inside. The set need not be whole. It reads both files from where they are named and needs no reference folder. Outside is a finding and not a fault, and nothing in the set or the table is changed to bring a set inside.

`set-from-table` makes a whole set of proportions for one head to be drawn, since `maquette draw` needs all 74 and a table gives fewer. It says of every proportion which of three it is:

- **The table's middle value,** where the proportion has a range.
- **A thin average,** where two to four works give it: their average, and not their middle, because the middle of three works is one work's own number.
- **Borrowed** from the set named by `--borrow`, as it stands there: what one work alone gives, what no work gives, and a convention. An ear and the neck are each taken whole: unless the works give every one of a form's proportions, all are borrowed. A neck's width from one source on a depth from another set the head forward on a post at look 5.

The set carries no correction, and says what the list's rule for `lower_face_set_back` would set back on its own depths. It is no table and no part of one. `overlay-sources` then marks the views `maquette draw` wrote from it: for each view an SVG with a dot on every point a guide passes through, coloured by the least supported source among the proportions that place the point in that view. A front view shows no depth and a side view no width, so a point may be of one kind in one view and of another in the next.

### Testing a reader on known positions

An agent's reading is an artificial evaluation: it is trusted only as far as it has matched drawings whose positions are known (`EVAL-2`). Saffi's hand-made drawings are such drawings, and their positions are in `data/saffi/hand-made-views.toml`.

```sh
cargo run -- score-reading --reading <reading> --known-positions data/saffi/hand-made-views.toml --out <file>
cargo run -- known-positions-as-reading --known-positions data/saffi/hand-made-views.toml --view front --image <picture> --out <reading>
cargo run -- compare-proportions --first <set> --second <set>
```

`score-reading` gives a reader's error by kind of landmark, and keeps apart what cannot be seen in the picture. It scores a reading as it stands, unlike the commands above: a landmark the reading leaves out is listed, and its picture is not checked. `known-positions-as-reading` writes the record's own positions as a reading, which is not one: the measuring commands run on it show what they make of exact positions, so that a reader's error can be told from the method's. `compare-proportions` sets two sets of proportions against each other. The readers must not see the record or the drawings' vector files, and the reading is tested again whenever the reading instructions change. A side reading is scored against the landmark file of an armature's side view, such as `armatures/saffi/side.landmarks.toml`, at the positions that view draws: it is the only side drawing with known positions, and it is a drawing of guides, so the score says how well guides are read and not how well a painted profile is.

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
| `armatures/saffi/` | Saffi's accepted armature: the six files `maquette draw` writes from her set of proportions, as Jon accepted them at look 3. Generated output, not a design and not a table. A test pins their bytes |
| `tables/adult-women/` | The first proportion table, with the list of works it was measured on. A proposal until Jon has accepted it |
| `proportions/list.toml` | The list of proportions: the unit, and every proportion's name, definition for each view, and mark as measured or a convention, how each guide is built in each view, and each named correction with the rule of its amounts. The program reads this same file |
| `Cargo.toml`, `src/` | The Rust package: the list and sets of proportions, the head model, the three views, the guides, the armature writer, the reading of a hand-made drawing, the differences from a hand-made armature, the read-back in VectorCraft, and measuring: the reference folder, the record of a reading, what readings say when set against each other, the scoring of a reading against known positions, the enlarging of a picture, the reference check sheet, the record of Jon's check, the proportion table, the setting of a set of proportions against one, the making of a whole set from one and the marking of where a drawn head's numbers come from, under a thin command line |
| `tests/` | Tests that drive the command line, files in and files out. `tests/fixtures/` holds made-up proportions, made-up views, a made-up drawing and a made-up record of known positions that say so, and recordings of VectorCraft's replies that say so. The readings and pictures the measuring tests use are made up by the tests themselves, in the system's temporary folder |
| `.github/workflows/checks.yml` | Formatting, lints and tests on every pull request |
| `.gitattributes` | Keeps Git from converting line endings in `armatures/`, so that a checkout cannot change the pinned bytes |
| `GLOSSARY.md` | The words maquette uses for armatures, views and proportions |
| `docs/research/` | The research behind the founding and the reorientation. Evidence only |

## Working here

GitHub Issues and pull requests in `joeloverbeck/maquette` own active work. Agents start from `CLAUDE.md`. Jon is the acceptance authority.

Maquette's first consumer is mudsong, a private repository. Nothing here needs it to be present.

## Licence

MIT; see [`LICENSE`](LICENSE). The constitution in `docs/principles/` is Jon's own document, copied from its master repository.
