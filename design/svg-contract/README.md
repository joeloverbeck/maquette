# The SVG contract: probes and read-backs

The evidence behind the SVG contract in [`docs/foundation/02-first-table.md`](../../docs/foundation/02-first-table.md), gathered for issue #4. It records what VectorCraft keeps of an SVG it opens, and what it writes when a shape authored inside it is exported.

Everything here was observed by running VectorCraft's MCP server on 2026-11-16:

- `vectorcraft-cli` 0.24.1, headless, started as `.mcp.json` starts it (`vectorcraft-cli mcp --automation-read-root . --automation-write-root design`).
- The binary's SHA-256 is `5ade309aea03d8987073d59f56741ec03b4e9d87ac6b5ee98a8a598e75ffa5c1`. It was installed on 2026-11-14 from the VectorCraft checkout.
- The preference `identifyObjectsBy` was at its default, `objectName`.
- The documentation compared against is `docs/mcp.md` in the VectorCraft checkout at commit `9ddbf25bef7711c3404549d95b730f7dec3210e7`, where that file is Git blob `fc03d070b6ce5c31751a97e3a9762b69f81fd301`.
- "SVG 1.1", here and in the contract, is *Scalable Vector Graphics (SVG) 1.1 (Second Edition)*, W3C Recommendation, 16 August 2011, `https://www.w3.org/TR/2011/REC-SVG11-20110816/`. One statement below rests on it: an element with no paint stated has a black fill and no stroke, which is the initial value of `fill` in its section 11.3 and of `stroke` in its section 11.4, read on 2026-11-16.

The names in the probes are neutral guide names. They are not the generator's naming scheme, which the next ticket chooses inside the contract.

## The files

`in/` holds the hand-written inputs. `out/` holds only what VectorCraft produced.

| File | What it is |
|---|---|
| `in/p1-names.svg` | Each attribute that might name an element |
| `in/p2-structure.svg`, `in/p2b-layers.svg` | Groups, nesting, layers, stacking order |
| `in/p3-transforms.svg` | Transforms on elements and on groups, and `use` |
| `in/p4-paint.svg` | Strokes without fill, and the ways of stating paint |
| `in/p5-hidden.svg` | Each way of hiding an element |
| `in/p6-curves.svg` | Path commands, basic shapes, number precision, degenerate shapes |
| `in/p7a` to `p7e-units-*.svg` | Sizes in mm, px and pt, with and without a `viewBox` |
| `in/hard-case.svg` | The hard case: a head in a three-quarter view. It is not generator output. Its numbers are those of `design/first-table-sheets/stand-in-armature-three-quarter.svg` with decimals added by hand, and that stand-in follows Saffi's hand-made three-quarter view. "The hard case" below says how closely |
| `out/<probe>.readback.txt` | What the server read back after `open_file`: the tree from `inspect_document`, then the anchors from `document.node` |
| `out/<probe>.export.svg` | VectorCraft's default SVG export of the imported probe |
| `out/<probe>.export-hidden-7dp.svg` | The same with `hiddenLayers: true` and `decimals: 7` |
| `out/authored.vectorcraft` | A document authored inside VectorCraft through the server: named shapes, a group, three layers, one hidden object, one locked object |
| `out/authored.default.svg`, `out/authored.hidden-7dp.svg` | Its SVG exports |
| `out/authored.readback.txt`, `out/authored.default.reimport.readback.txt` | The authored document as VectorCraft holds it, and its default export imported again |
| `out/hard-case.inspect.json`, `out/hard-case.node.json` | The server's replies for the hard case, whole, as starting material for the next ticket's oracle. Only the absolute path of the opened file was cut down to the repository's own |
| `out/hard-case.export.svg` | The hard case exported after `eye-opening-left` was hidden by name |
| `out/hard-case.reimport.readback.txt` | `out/hard-case.export-hidden-7dp.svg` imported again |

## Rerunning a read-back

Three calls to the `vectorcraft` server, from an agent session in this repository or by piping JSON-RPC lines into the command above:

1. `open_file {"path": "design/svg-contract/in/p1-names.svg"}`. The reply's `warnings` should be empty.
2. `inspect_document {}`: the tree with ids, kinds, names, visibility, bounds and paint.
3. `run_command {"command": "document.node", "params": {"id": N}}` for each top-level layer id `N` from step 2: every object below it with its anchors.

The `.readback.txt` files are those replies cut down to one line per object with `jq`, a one-off done by hand for this evidence. That and the comparisons of numbers reported under "The hard case" were one-off checks run outside the repository with `jq` and `awk`: nothing that is not Rust is committed, and the repeatable check belongs to the Rust package the next ticket brings. Numbers are as the server printed them. The object ids are VectorCraft's own, given in document order with a group after its children. They are not stable names and no check should rest on them.

Opening the same file twice gave the same tree, and exporting it twice gave the same bytes.

## Into VectorCraft: what survives an import

No import reported a warning, including the ones that dropped elements.

| Checked | Result |
|---|---|
| `id` | Kept unchanged as the object's name: hyphens, underscores, dots, capitals, a leading digit, a space, an accented letter. `_x5F_` is not decoded |
| `data-name`, `inkscape:label`, `aria-label` beside an `id` | Each replaces the `id` as the name. The `id` cannot be read back |
| `<title>` child, `<desc>` child, a `name` attribute | Ignored |
| `data-name` with no `id` | Ignored. The object is unnamed |
| No `id` | Unnamed. `inspect_document` labels it `<Path>` or `<Group>`, and `document.node` gives no name |
| Two elements with one `id` | Both kept under the same name |
| Other `data-*` attributes | Kept as the object's data (`attrs.data`) and written back on export |
| A left and right pair | Two objects, each under its own name |
| `g` with an `id` | A group of that name. Nesting kept four deep |
| `g` without an `id` | With two children, an unnamed group. With one child, the group is removed and the child kept |
| Empty `g` | Dropped |
| Top-level elements | If every top-level element is a `g`, each becomes a layer named after it. Otherwise everything goes inside one layer, `Layer 1`, and a top-level `g` is a group |
| `inkscape:groupmode="layer"` | A layer inside `Layer 1`, named by `inkscape:label` |
| Stacking order | Kept |
| Positions | 1 unit of the file is 1 unit of the document when `width` and `height` are plain numbers equal to the `viewBox` size and the `viewBox` starts at `0 0`, or when one of the two is absent |
| `width` and `height` in mm | Every coordinate and stroke width multiplied by 72/25.4, with rounding |
| `width` and `height` in pt | Coordinates a little off: 10 came back as 9.999999404 |
| A scaled or shifted `viewBox` | Every coordinate and stroke width rescaled and shifted |
| Number precision | Every number is read at single precision: 223.4 came back as 223.39999389648438. Below 1024 that is at most 0.000031 off. In the hard case the largest difference was 0.000024 on typed coordinates and 0.000037 on the outer points of a circle or ellipse |
| Art outside the page | Kept |
| `transform` on an element or a group | Applied to the coordinates. None is kept. `translate`, `scale` and `matrix` mirrors came back exact. `rotate` came back up to 0.000013 off. `scale` multiplies the stroke width unless `vector-effect="non-scaling-stroke"` is set |
| `use` | A group named after the `use`, holding an unnamed copy |
| `line`, `polyline`, `polygon`, `rect` | Paths with the same corners as anchors |
| `circle`, `ellipse` | A closed path of four anchors at the right, bottom, left and top. Not a circle any more |
| `path` with `M L C Z`, relative commands including `h` and `v`, and `S` | Same anchors and handles. A closing segment that returns to the start is merged into it |
| `Q`, `T` | Converted to cubic curves |
| `A` | Approximated by cubic curves. A half circle gained an anchor at its middle |
| Several subpaths in one `path` | A compound path of that name whose pieces are unnamed |
| A zero-length line | Kept |
| A `path` with only `M`, a circle of radius 0 | Dropped |
| `fill="none"` with a stroke | Kept, whether stated as attributes, in `style`, by a CSS class or on a group above. Closed shapes stay unfilled |
| No paint stated | Black fill and no stroke, as SVG says |
| Dashes, caps, joins, `opacity`, `stroke-opacity`, named colours | Kept |
| `fill="none"` with `stroke="none"`, or `fill="none"` with `stroke-width="0"` | Dropped. Each was a `line`. Neither was tried with a fill |
| `display="none"`, as an attribute or in `style` | Kept, hidden. On a group or layer, the group or layer is hidden and its children are kept |
| `visibility="hidden"`, as an attribute or in `style` | Dropped. In a hidden group, only a child with `visibility="visible"` is kept |
| `opacity="0"`, `stroke-opacity="0"` | Kept and visible, at that opacity |
| An element inside `defs` | Dropped |

After import, `document.find {"name": ...}` matches by substring and ignores case: `neck` also finds `neck-left`. An exact comparison of the names it returns finds one guide. `object.setProps {"ids": [id], "visible": false}` then hides that guide alone.

## Out of VectorCraft: what an export contains

From the document authored through the server with `draw_shape`, `draw_path`, `object.group`, `object.setProps`, `object.hide`, `object.lock`, `layer.new` and `transform`.

| Checked | Result |
|---|---|
| A name that is a valid `id` (`eye_line`, `nose.mark`, `mouth-line`) | Written as the `id`, with no `data-name` |
| A name with spaces | `id` with underscores, and the name in `data-name`: `id="ear_left" data-name="ear left"` |
| A leading digit, `<`, `&`, quotes | `id` rewritten (`_1_digit_first`, `a_b__c_`), the name in `data-name` |
| Two objects of one name, or two names that give one `id` | The second gets `-2` on its `id` and keeps its name in `data-name` |
| No name | No `id` |
| Groups | `g` with the same naming. Nesting kept |
| Layers | A top-level `g` each |
| Positions | Written as they are, rounded to 3 decimals by default. `decimals` takes 1 to 7 |
| A rotated shape | Rotation applied to the coordinates. No `transform` is written |
| Element chosen | By geometry: a straight path of two anchors is a `line`, an open path of straight segments a `polyline`, a closed one a `polygon` or an upright `rect`, an ellipse a `circle` or `ellipse`, anything curved a `path`. At 7 decimals a circle that came from an import is written as an `ellipse` whose radii differ in the fifth decimal |
| Stroke without fill | `fill="none"` with the stroke, as attributes. `stroke-miterlimit="10"` is added to shapes drawn in VectorCraft |
| Hidden object or layer | Left out by default. With `hiddenLayers: true`, written with `display="none"` |
| Locked object | Written like any other. The lock is not in the file |
| Object data | Written as `data-*` attributes |
| A path of several subpaths | One `path` |
| Art outside the page | Written |
| `objectIds: "minimal"` | No `id` and no `data-name` at all |
| `objectIds: "unique"` | Every `id` prefixed with a hash, and the name always in `data-name` |
| Paint left out of `draw_path` | The paint last used in the session, not a default |

## Round trips

- **Import, export with hidden kept and 7 decimals, import again:** the same names and tree for every probe. Coordinates that were exact stay exact. The others move by the same single-precision rounding.
- **Import, default export, import again:** the same, less every hidden object and layer, and with coordinates at 3 decimals. For the hard case every geometry attribute of the default export is the same text as in the hand-written input, apart from the one guide hidden beforehand, which is left out.
- **Authored, exported, imported:** every name comes back, including the ones whose `id` was rewritten, because `data-name` wins. Ellipses and rectangles come back as plain paths. The lock is gone.
- **Saved as `.vectorcraft` and reopened:** identical, whether the document was authored or imported. Coordinates authored inside VectorCraft are held at double precision.
- **`save_file` with no path on an imported SVG:** overwrites that SVG with VectorCraft's own export, hidden objects kept, and warns that SVG does not keep everything.

## Where the documentation and the behaviour disagree

1. The server's instructions say coordinates are points. Observed: an SVG sized in plain numbers comes in one unit for one unit and the document's units are `Pixels`. A size in mm is converted at 72 units to the inch.
2. `docs/mcp.md` says the SVG of an artboard leaves out art outside it. Observed: the default export wrote a line that lay wholly outside the page.

Observed behaviour wins in both. The documentation agrees with the behaviour on `display:none` coming back hidden, on `hiddenLayers` and on `data-*` attributes. The server's own `document.formats` listing gives `decimals` a default of 3, as observed. It says nothing about `visibility`, about unpainted elements, or about which attribute gives an object its name.

## Not observed

- Version 0.24.0, which mudsong's foundation records.
- The desktop app. Only the headless server was run.
- The preference `identifyObjectsBy` set to `xmlId`.
- Text, gradients, patterns, clipping paths, masks, images, symbols and filters. The armature uses none.
- Coordinates above 1050.
- Whether any probe is valid SVG 1.1. No run through a validator is recorded.
- FigCraft's reading of the same files.
- Mudsong's own import. The handoff stays unproven until mudsong has read the names and positions back itself.

## The hard case

Chosen from what the tables above show to be fragile, before any generator exists: names that survive only in one form, elements dropped without a warning, coordinates that come back rounded, and a tree whose shape depends on what sits at the top level. A front view with round numbers would pass an exact comparison of coordinates, which real proportions will not. The hard case is a three-quarter view with positions that are not round, every guide at once in nested groups, left and right pairs that differ, one name inside another (`neck`, `neck-left`), and closed guides.

"Every guide" is the list in issue #3 and in `docs/foundation/02-first-table.md`: the skull, the face outline, the centre line, the level lines for the eyes, the nose and the mouth, each eye's opening, lid line and iris, the two brow lines, the nose mark, the mouth line, the two ears, the neck, the hair volume and the shoulder line. That is 22 drawn elements. With the layer and the three groups, the file has 26 `id`s.

**Where its numbers came from.** They are not made up.

- Every geometry number of `in/hard-case.svg` is the matching number of `design/first-table-sheets/stand-in-armature-three-quarter.svg` moved by at most 0.9 unit, so that none is round: 293 numbers in 21 elements. The shoulder line, the 22nd, keeps only the long stroke of the stand-in's three, its four numbers moved the same way. The names are new and the paint is one stroke for all.
- That stand-in follows the hand-made three-quarter view of Saffi, Jon's own character, moved 5 units toward the viewer's right. Of its 66 distinct points (path nodes, line ends, and the centres of the circle and the two ellipses), 34 are landmarks of `data/saffi/hand-made-views.toml` once 5 is added to x. The skull's radius is hers, and the three level lines are at her three levels. No other whole shift from −30 to 30 matches more than 3 points.
- The 32 points that do not match are the hair volume, the ends of the level lines, the brows, the nose mark, the centres of the irises, the neck, the shoulder line, the top of the face outline's near side and the bottom of the far eye's opening.

The hard case is a test input for the importer. It is not a record of Saffi and not a measurement, and it never enters a proportion table. Her record is `data/saffi/`. These counts come from a comparison of the three committed files.

`in/hard-case.svg` was put through it by hand:

| Step | Result |
|---|---|
| `open_file` | No warning |
| Names read back | The 26 `id`s of the file, each once, under one layer `armature`: 26 objects |
| Positions | 282 typed coordinates within 0.000024 of the file's numbers: every anchor and handle of the 19 paths, lines and polylines. The 24 coordinates of the outer points of the circle and the two ellipses within 0.000037 |
| Paint | All 22 guides visible, stroked and unfilled, the 5 closed ones included |
| `document.find {"name": "neck"}` | Three matches: `neck`, `neck-left`, `neck-right` |
| `eye-opening-left` hidden by name | That guide alone is hidden |
| Spoiled copy: `visibility="hidden"` on `brow-line-left` | 25 objects, `brow-line-left` missing, no warning |
| Spoiled copy: the `id`s of the two eye openings exchanged | 26 objects, the same set of names, no warning. Only the positions show it: `eye-opening-left` reads back at 458.2, 369.4 where the right one was, and all 24 anchors and handles of the two openings are more than 0.001 from the file's |

The two spoiled copies were made, read back and deleted. They are not in `in/`.

**How positions were compared.** The expected numbers were taken from the text of `in/hard-case.svg`, because no landmark file exists yet.

- *Paths, lines and polylines:* each anchor and each handle read back was set against the number typed in the file. A closed path's last point repeats its first and is merged into it, so it is compared once: 282 coordinates where the file types 286.
- *The circle and the two ellipses:* they arrive as four anchors and none is at the centre. The four outer points were worked out from the file's attributes, as `cx + rx, cy`, then `cx, cy + ry`, `cx − rx, cy` and `cx, cy − ry`, with `r` for both radii of the circle, and set against the four anchors in the order they are read back: right, bottom, left, top. The handles between them are VectorCraft's own and were not compared.
- *A centre read back:* the top and bottom anchors carry `cx` as their x, and the right and left anchors carry `cy` as their y, each within 0.000013 of the file's. Half the distance between opposite anchors gives a radius within 0.000025.

The hard case was run again on 2026-11-16 after the nose and mouth level lines were added and `eye-line` was renamed `eye-level-line` to match them. Every `out/hard-case.*` file is from that run. Before the change the same check gave 24 objects and 274 typed coordinates, with the same largest differences.

This is one hand-written file read by an agent. It shows the subset can pass. It shows nothing about the generator, whose own three-quarter view has to pass with a landmark file as the source of expected values.
