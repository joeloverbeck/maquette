# Animation readiness and game use of a generated anime-style SVG character (state as of 2026-11-15)

Scope after the coordinator's mid-task change: the generator does not animate or export rig formats. Animation happens downstream in AniCraft (third-party repo `storytold/anicraft`, local checkout `~/src/anicraft`, version 0.3.0, commit `d439e3e`, read only, not built or run). The main question is therefore what the SVG must look like for a Flash-style animator. Rig formats are kept only as a short context table (section 6).

Evidence labels used in every finding:

- **[V-local]** read directly in the AniCraft checkout (file path and line given). Nothing was executed, so these are code and doc readings, not observed behaviour.
- **[V-web]** read on the primary page itself.
- **[S]** seen only in a search-result summary; the page was not opened or refused the fetch. Treat as a lead, not a fact.

## 1. What must the generated SVG look like so that AniCraft can animate it without redrawing?

### Takeaway
AniCraft's SVG importer flattens: it keeps leaf geometry, each leaf's `id` as the element name, fills, strokes, gradients, transforms and clip paths (as mask layers), but drops group nesting, group ids, `<use>`/`<symbol>` reuse, hidden elements and style sheets, and puts a whole file on one layer or into one symbol. A single well-grouped SVG therefore arrives as a flat pile of named shapes, so the rig structure has to travel another way: one SVG per part (its viewport centred on the joint, which becomes the symbol's registration point) plus a sidecar that an AniCraft command script uses to assemble layers, placement, bones and mouth charts.

### Cited Findings

Import behaviour (what survives)

- Supported elements: `svg`, `g`, `a`, `switch`, `use`, `path`, `rect` (rounded), `circle`, `ellipse`, `line`, `polyline`, `polygon`, read in document order; styling from presentation attributes and inline `style=""` with inheritance; `opacity`, `fill-opacity`, `stroke-opacity`, `fill-rule`, `stroke-width` with units, caps, joins, miter limit, dash array and offset; the full transform list; `viewBox` with `preserveAspectRatio`; linear and radial gradients in both unit systems with `gradientTransform` and `href`-inherited stops. **[V-local]** — `~/src/anicraft/crates/svg/src/lib.rs` L1-13
- Groups are traversed, not kept: `"svg" | "g" | "a" | "switch" => self.children(...)`. The import result type holds only `name`, `transform`, `alpha` and `kind` per element, with no parent or group field, so nesting and group ids are lost. **[V-local]** — `crates/svg/src/lib.rs` L99-109, L816
- Names: an element's name is its own `id`, cut at 128 characters; only leaves (shapes, text) become elements, so only leaf ids survive. **[V-local]** — `crates/svg/src/lib.rs` L201-204, L857-858
- `<use>` is expanded in place (with its `x`/`y` offset); a `<symbol>` target has its children inlined. No Library symbol or shared instance is created, and the copy takes the name of the *target's* id, not the `<use>` element's id, so repeated uses produce duplicate names. **[V-local]** — `crates/svg/src/lib.rs` L817-837
- Hidden content is dropped, not imported hidden: an element with `display:none` returns before anything is read (its whole subtree goes); a leaf with `visibility: hidden|collapse` is skipped. **[V-local]** — `crates/svg/src/lib.rs` L206-209, L765-768, L848
- `<style>` sheets are not read ("style sheets (inline styles are read)"); `<image>`, `<foreignObject>`, filters and patterns are skipped and counted in a report; gradient strokes are drawn in one colour. **[V-local]** — `crates/svg/src/lib.rs` L772-786, L714-716, L934-939; user-facing statement in `~/src/anicraft/docs/guide/images-and-sound.md` L88-93
- Transforms: each leaf gets the accumulated matrix of all its ancestors (`m = parent.mul(own transform)`) while its path stays in the leaf's own coordinates. **[V-local]** — `crates/svg/src/lib.rs` L790, L857-858
- Fill and stroke stay separate properties of one shape (`Shape { path, fill, stroke, fill_rule, ... }`); a stroke is a solid colour with width, cap, join, miter and dash. **[V-local]** — `crates/svg/src/lib.rs` L851-858, L930-939
- Clip paths and masks become AniCraft mask layers: artwork under a clip is a masked layer under a mask layer holding the clip's shapes; a clip that cuts nothing is dropped; opaque masks become shape masks and faded ones soft masks. **[V-local]** — `crates/svg/src/lib.rs` L15-22; `docs/guide/images-and-sound.md` L77-81
- Limits: 200,000 shapes per file, nesting depth 64, `<use>` depth 16. **[V-local]** — `crates/svg/src/lib.rs` L48-54

Where the artwork lands

- Import to Stage (`file.import`): all shapes become editable elements "on a new layer named after the file, in a keyframe at the playhead"; one layer per file. A drawing with clips or masks instead becomes a layer folder named after the file, containing mask layers and artwork layers named "Layer N". **[V-local]** — `~/src/anicraft/crates/engine/src/cmds_file.rs` L269-272, L343-351, L353-401
- Import to Library (`file.import_to_library`): one SVG becomes one graphic symbol named after the file. Children are offset by `translate(-width/2, -height/2)`, so the registration point is the centre of the SVG's `width` x `height` viewport, not the centre of the artwork's bounds. Its inner layer is named "Layer 1". **[V-local]** — `crates/engine/src/cmds_file.rs` L445-490; decision recorded in `~/src/anicraft/docs/adr/0065-templates-svg-into-the-library-and-drops-on-the-timeline.md` L54-58
- Convert to Symbol (F8, `modify.convert_to_symbol`) puts the registration point at the centre of the selection's bounds. **[V-local]** — `~/src/anicraft/crates/engine/src/cmds_draw.rs` L800, L829
- `file.import` accepts `layer: new | active`; `file.drop` accepts `to: stage | library | timeline` with a layer id and frame; `library.place` puts a symbol "centred on x, y" in the active layer's keyframe. **[V-local]** — `~/src/anicraft/docs/guide/reference/commands.md` L25 (params in `cmds_file.rs` L36), L306-307, L518-519

What AniCraft can then do with the parts

- Symbols: graphic symbols with nested timelines and loop modes Loop, Play once and Single frame plus a "First frame" setting, described as the way "to hold one pose, such as one mouth shape from a lip-sync chart"; Swap Symbol keeps transform, colour effect and looping. Headless equivalents: `element.set_loop {loop, first_frame}`, `element.swap_symbol {symbol}`. **[V-local]** — `~/src/anicraft/docs/guide/symbols-and-library.md` L99-116; `docs/guide/reference/commands.md` L510-511, L548-549
- Tweens: classic tweens interpolate a decomposed transform (translation, rotation, skew, scale) and the colour effect; shape tweens morph "between arbitrary vector contours" with shape hints (`timeline.create_shape_tween`, `tween.add_hint`). **[V-local]** — `~/src/anicraft/crates/doc/src/eval.rs` L397-407; `~/src/anicraft/README.md` L145; `docs/guide/reference/commands.md` L134, L175-176
- Bones: armatures join symbol instances with CCD inverse kinematics, joint limits and pins; `bone.add {from, to, head?: [x, y], tail?: [x, y]}` takes joint positions in stage pixels and defaults to the instances' registration points. Bones can also live inside one shape, with automatic binding, a Bind tool and painted weights. **[V-local]** — `~/src/anicraft/docs/guide/masks-guides-camera-bones.md` L168-205, L242-300; `docs/guide/reference/commands.md` L360-363
- Secondary motion: per-bone Spring and Damping (0 to 100) "like a tail, an antenna or a ponytail", evaluated deterministically. Asset Warp bends a shape or bitmap with pins and can be tweened. **[V-local]** — `docs/guide/masks-guides-camera-bones.md` L222-240, L305-337
- Stated gaps in the bone system: "symbols and shapes in one armature, reshaping at a posed frame, armatures that scripts pose while the movie plays, and armatures from Animate files"; Asset Warp cannot warp symbols or text. **[V-local]** — `docs/guide/masks-guides-camera-bones.md` L302-303, L339-340
- Variable-width strokes exist natively (a `WidthProfile` of up to 64 width points per stroke, command `stroke.set_profile`), but SVG cannot express them; on SVG and Lottie export AniCraft replaces each variable stroke with a filled polygon. The importer reads only a single `stroke-width`. **[V-local]** — `~/src/anicraft/docs/adr/0018-variable-width-strokes.md` L17-26, L44-48; `crates/svg/src/lib.rs` L638-640
- Built-in templates show the structure AniCraft expects of character work: a "Character turnaround" with five views (Front, Three-quarter, Side, Three-quarter back, Back) on guides eight heads tall, and a "Lip-sync mouth chart": a Mouth graphic symbol with ten labelled keyframes `Rest, AI, E, O, U, CDGKNRSThYZ, FV, L, MBP, WQ`, placed at Single Frame. **[V-local]** — `~/src/anicraft/crates/doc/src/templates.rs` L26-27, L176, L189-222
- Automation: every action is an engine command, reachable from the CLI (`anicraft-cli exec`, `anicraft-cli run doc.anicraft script.json --save out.anicraft`, where a script is a JSON array of `{cmd, params}` steps) and from an MCP server (`anicraft_commands`, `anicraft_run`, `anicraft_run_script`, `anicraft_document`, `anicraft_render`). **[V-local]** — `~/src/anicraft/docs/guide/working-headless.md` L72-122; `~/src/anicraft/docs/mcp.md` L32-40
- Outputs available after animating: PNG frames and sequences, animated GIF, APNG, WebP, SVG frames, sprite sheets, Lottie, HTML canvas, and WebM/MP4 through a user-installed ffmpeg. **[V-local]** — `README.md` L126-128; `~/src/anicraft/docs/guide/exporting-and-publish.md` L15-52
- Status and licence: version 0.3.0; "Early alpha: not yet for production animation work"; dual MIT or Apache-2.0. **[V-local]** — `~/src/anicraft/Cargo.toml` L9; `README.md` L192-194, L334

### Inferences

Rules for the emitted SVG, derived from the code above:

1. **Semantic names go on leaves.** Give every path a unique `id` of at most 128 characters, with the hierarchy encoded in the name (for example `head.eye_l.iris`), because group ids do not survive.
2. **Inline styling only.** Use presentation attributes or `style=""`; never `<style>` blocks or classes. Avoid filters, patterns, embedded images and gradient strokes. Solid fills and linear or radial gradient fills are safe.
3. **Do not hide alternates.** Mouth shapes, blink states and alternate views hidden with `display:none` are discarded on import. Emit them as separate files (or as visible artwork laid out on a model sheet).
4. **Do not rely on `<use>` for identity.** It imports as an unlinked copy named after its target. Reuse belongs in AniCraft's Library (one symbol placed many times).
5. **One file is one layer or one symbol.** The practical unit of rig structure is the file. The cleanest hand-off is one SVG per animatable part, imported to the Library, where the file name becomes the symbol name.
6. **Pivot by viewport.** Since the registration point is the centre of the SVG viewport and classic tweens and `bone.add` work from that point, each part file should have its `viewBox` centred on the part's joint (shoulder for the upper arm, base of the neck for the head). This is the one mechanism found that carries a pivot through the import. For a whole-character SVG imported to stage, no per-part pivot comes across: the shape's local origin is whatever origin its leaf coordinates use.
7. **Keep clips local.** A clip path is useful (iris clipped by the eye white is the classic Flash mask), but each clipped run adds a mask layer, so put clips inside the part file that needs them rather than around the whole character.
8. **Line weight.** Tapered anime line art must be emitted as filled outline shapes, or as uniform strokes with a width profile applied afterwards by command. Filled outlines look right everywhere but are harder to shape-tween than a centre-line stroke.
9. **Paint order is document order**, bottom first, within a file; across part files, order comes from the sidecar.
10. **Ship an assembly script.** Because AniCraft is fully command-driven, the generator can emit a JSON command script (import each part to the Library, create named layers, place instances, add bones at sidecar joint coordinates, build the Mouth symbol's keyframes) so the animator opens a ready rig. This needs no rig file format, only the sidecar data.

### Gaps
- Nothing was run. Whether rotation in a classic tween visibly pivots on the registration point is inferred from the transform decomposition in `eval.rs`, not observed.
- Whether AniCraft has a per-instance transformation point separate from the registration point (as Flash does): the `Element` struct has no pivot field (`crates/doc/src/lib.rs` L941-950) and the guide mentions a pivot only for multi-frame transforms, but the Free Transform tool code under `src/` was not read.
- Whether `library.place`'s "centred on x, y" positions the registration point or the bounds centre was not determined; `element.set_transform` can set an exact transform afterwards.
- No "Distribute to Layers" command was found in `commands.md`; whether another command moves existing elements between layers was not determined.
- No headless command was confirmed that maps an imported element's name to its numeric id. `find.search` is documented as searching layer and instance names (`docs/roadmap.md` L21) but was not examined. Per-part import avoids the problem because each import returns its ids.
- The header comment of `crates/svg/src/lib.rs` (L24-26) lists reflected and repeated gradients, soft masks and text on a path as not representable, while the user guide (`images-and-sound.md` L75-84) says they come across. The gradient code (L965-1043) was not read to settle which is current; pad-spread gradients are the safe choice.
- The `.anicraft` JSON format (v3) is described as documented, which would allow emitting a document directly, but its specification was not read and an early-alpha format may change.
- The guide's phrase "Groups, reused parts (`use` and symbols) ... come across" (`images-and-sound.md` L73) is true of their content only; the code shows the structure is flattened.

## 2. Structural requirements on the art: layers, over-painting, pivots, naming, mouth, eye and expression sets

### Takeaway
Every source that animates 2D characters by parts asks for the same things: each independently moving feature on its own layer, hidden areas painted in under overlaps, joints that overlap with pivots at the joint, and small sets of swappable drawings for mouth and eyes. For a generator this is cheap, because the parts already exist as separate parametric objects; what must be added is the over-painted geometry and the alternate drawings.

### Cited Findings

Part separation and over-painting (Live2D's guidance, the most explicit for anime faces)

- Hair is split into bangs, sides and back; hair extending to one side (sideburns, pigtails) gets a layer per side, and movable strands get their own layers. Eyes are split into eyelashes, eyeballs and whites. Eyebrows and nose are separate. The mouth is split into upper lip, lower lip and the inside of the mouth. Ears are separated from the face outline. Arms are split into upper arm, forearm and hand; legs into skirt, left leg and right leg. **[V-web]** — [Live2D tutorial, illustration processing](https://docs.live2d.com/en/cubism-editor-tutorials/psd/)
- Hidden areas must be drawn: hair parts hidden in the source image, the white of the eye behind the eyeball, the face outline under the hair (skin colour only), the inside of the mouth "one size larger", and a neck drawn large enough to reach the mouth area so it does not look cut off when the face moves. **[V-web]** — [Live2D tutorial, illustration processing](https://docs.live2d.com/en/cubism-editor-tutorials/psd/)
- More detailed division improves model quality; parts that move a lot, and parts that become visible when others move, should be drawn first. **[V-web]** — [Live2D manual, material separation](https://docs.live2d.com/en/cubism-editor-manual/divide-the-material/)
- Line art and fill are merged into one layer per part, every layer name unique. **[V-web]** — [Live2D manual, PSD precautions](https://docs.live2d.com/en/cubism-editor-manual/precautions-for-psd-data/)
- The standard parameter set shows which features an anime face is expected to move independently: eye open/close and smile per eye, eyeball X/Y, brow up/down, left/right, angle and form per brow, mouth form and mouth open, cheek, breath, hair sway front, side and back, plus optional arms, hands and shoulders. **[V-web]** — [Live2D standard parameter list](https://docs.live2d.com/en/cubism-editor-manual/standard-parameter-list/)

Cut-out joints and hierarchy

- A cut-out rig is a template in which all movable parts are broken down into layers and arranged in a hierarchy; a breakdown artist chooses the separated parts and sets up joints and views. **[S]** — [Toon Boom Harmony 24, how to rig a cut-out character](https://docs.toonboom.com/help/harmony-24/premium/getting-started/character-building.html); [Toon Boom cut-out workflow](https://docs.toonboom.com/help/harmony-12-2/paint/animation-workflow/cut-out_animation_workflow.html)
- Joints need articulations: the classical method erases part of the line so the colour overlaps the piece below; patch articulations cover the joint with a patch. **[S]** — [Toon Boom, patch articulation](https://docs.toonboom.com/help/harmony-11/workflow-network/Content/_CORE/_Workflow/020_Character_Building/071_H2_Patch_Articulation.html)
- In AniCraft, a bone's joint defaults to an instance's registration point and the chain's joints sit at bone ends, so the pivot of each limb segment must be at its proximal joint. **[V-local]** — `~/src/anicraft/docs/guide/reference/commands.md` L361

Mouth-shape sets

- Anime dialogue ("serifu") uses three mouth drawings: closed, half-open (drawn by the in-betweener) and open, arranged to look irregular rather than matched to phonemes. **[V-web, secondary]** (an AI summary page of a tutorial video, no channel or date given) — [Video Highlight summary of "What is SERIFU?"](https://videohighlight.com/v/SMVSJCcmBDU)
- In limited animation the mouth can snap between key shapes without in-betweens; the simplest approach times only open and closed, and more convincing results use roughly eight to ten shapes. **[S]** — [Animator Island, lip-sync for limited animation](https://www.animatorisland.com/a-lip-sync-must-know-for-limited-style-animation/)
- AniCraft's own chart uses ten shapes: Rest, AI, E, O, U, CDGKNRSThYZ, FV, L, MBP, WQ (the Preston Blair-style grouping). **[V-local]** — `~/src/anicraft/crates/doc/src/templates.rs` L189
- Rhubarb Lip Sync uses six basic shapes A-F (A closed for P/B/M, B clenched teeth, C open, D wide open, E rounded, F puckered) plus optional G (F/V), H (L) and X (rest). **[V-web]** — [Rhubarb Lip Sync README](https://github.com/DanielSWolf/rhubarb-lip-sync)
- Live2D drives the mouth with two continuous parameters, Mouth Open (0 to 1) and Mouth Form (-1 to 1), rather than discrete drawings. **[V-web]** — [Live2D standard parameter list](https://docs.live2d.com/en/cubism-editor-manual/standard-parameter-list/)
- Adobe Character Animator's viseme layer names are fixed; the help page lists Ah, D, Ee, F, L, M, Oh, R among them, with Neutral, Smile and Surprised driven by the camera instead of audio. **[S]** (the Adobe help page returned HTTP 403) — [Adobe, prepare artwork](https://helpx.adobe.com/adobe-character-animator/desktop/creating-and-controlling-puppets/prepare-artwork.html)

Expression sets

- Ren'Py layered images compose a sprite from `always`, `attribute` and mutually exclusive `group` layers; the documentation's examples use an outfit group and a face group (neutral, happy, angry), with files matched by `name_group_attribute` underscores. **[V-web]** — [Ren'Py 8.5.4, layered images](https://www.renpy.org/doc/html/layeredimage.html)
- Commercial visual-novel sprite packs commonly ship six expressions per character, with larger sets (one of 33) also sold; one pack names files `<character>_<expression>` and keeps all expressions on the same framing and baseline. **[S]** — [itch.io listing, 33-expression collection](https://prometheus-pictures.itch.io/luna-emotional-sprite-collection-33-expressions-for-visual-novels-rpgs-romance)

### Inferences

- **Minimum separate named parts for a bust that can blink, talk, emote and sway:** hair back, hair side left, hair side right, hair front (bangs), any strand that should swing; face base (skin, painted under the hair), ear left, ear right, nose; per eye: white, iris with pupil, highlight, upper lash line, lower lash line, and eyelid skin for the closed state; per brow: one shape; mouth: line or upper lip, lower lip, interior, teeth, tongue; cheek blush; neck (extended up under the chin and down under the collar); torso, with collar or ribbon separate. For a full body add upper arm, forearm, hand, thigh, shin, foot per side, pelvis and skirt or coat tails. This is a synthesis of the Live2D split and the cut-out limb split, roughly 30 parts for a bust and 45 for a body.
- **Over-painting rule for a generator:** every part's fill should continue under whatever covers it by a margin at least as large as the largest intended motion. Limb segments should end in a round cap centred on the joint so that rotation never opens a gap; this is the vector equivalent of the classical articulation. A parametric generator can produce this exactly, since it knows the joint position and limb radius.
- **Mouth set to emit:** a three-shape anime set (closed, half, open) is the stylistic default and maps onto any system by subsetting. Emitting the ten AniCraft chart shapes costs little for a parametric mouth and lets the downstream chart template be filled directly; Rhubarb's A-F plus X is a subset by meaning. Each shape should be multiplied by a small number of mouth forms (neutral, smile, frown) only if expressions need it.
- **Eye states:** open, half, closed, and smiling-closed (the arc used for happy expressions) per eye, with the iris as a separate part so gaze can move under a clip. Live2D's eye open and eye smile parameters correspond to these four states.
- **Expression set (hyojoshu):** defining an expression as a tuple (brow shape, eye state, mouth form, extras such as blush or sweat) lets six to eight named expressions (neutral, happy, angry, sad, surprised, embarrassed, plus one or two character-specific) be assembled from the same parts, the way layered images do.
- **How many drawings:** with parts and tuples, a talking, blinking, emoting bust needs on the order of 3 to 10 mouths, 4 eye states and 3 to 5 brow shapes per view, not a drawing per expression. Multiplied by five head views this is still under roughly 100 small part drawings, all of which a parametric generator derives rather than draws. No source gives a canonical count; this figure is arithmetic on the sets above.

### Gaps
- No primary Japanese-language source was found for the three-mouth convention (closed, middle, open); the only source is an AI summary of an English tutorial video.
- The Toon Boom and Adobe documentation pages could not be opened (404 and 403), so the cut-out joint guidance and the full Character Animator viseme list (S, Uh, W-Oo unconfirmed) rest on search summaries.
- No authoritative source was found for a standard expression list or count in visual novels; the six-expression figure comes from asset-store listings.
- Recommended numeric pivot positions (for example how far inside the limb the joint centre sits) were not found in any source.

## 3. Head and body turns: key views, view swapping, and what the generator must emit

### Takeaway
Part-based 2D systems turn a head smoothly only within about 30 degrees by sliding and deforming layered features; beyond that, productions swap to a separately drawn view. A parametric generator should therefore emit a small set of discrete key views with identical part names and pivots (front, three-quarter, side, three-quarter back, back, mirrored for the other side), and can use a per-part 3D anchor internally to place features consistently in every view.

### Cited Findings
- Live2D's standard head parameters Angle X, Y and Z run from -30 to 30, and body rotation X, Y, Z from -10 to 10. **[V-web]** — [Live2D standard parameter list](https://docs.live2d.com/en/cubism-editor-manual/standard-parameter-list/)
- Live2D builds the turn by giving the contour, eyes, nose, mouth, eyebrows, ears and hair each their own warp deformer "to create parallax and deform it three-dimensionally", then auto-generating the four diagonal corners from the X and Y forms. **[V-web]** — [Live2D tutorial, face XY movement](https://docs.live2d.com/en/cubism-editor-tutorials/xy/)
- 2.5D cartoon models (Rivers, Igarashi, Durand, SIGGRAPH 2010) take vector drawings of a cartoon in a few key views, give each stroke a single 3D anchor position, and render a new view by rotating the anchors while interpolating each stroke's 2D shape; Z-order follows from the anchors. The paper states that "just three or four key views" suffice and that top, front and side views are usually sufficient. **[V-web]** — [Rivers et al., 2.5D Cartoon Models (PDF)](https://www-ui.is.s.u-tokyo.ac.jp/~takeo/papers/alec_siggraph2010_cartoon.pdf)
- Limits stated in that paper: interior contour or detail lines are hard to show because strokes are treated independently; sharp or highly concave shapes interpolate poorly; hair "is a difficult case"; overlapping strokes can pop when their relative Z-order changes. **[V-web]** — [Rivers et al., 2.5D Cartoon Models (PDF)](https://www-ui.is.s.u-tokyo.ac.jp/~takeo/papers/alec_siggraph2010_cartoon.pdf)
- Flash/Animate practice for multiple head angles, from a 2020 Adobe community thread: (1) build each head angle as its own symbol "with the same registration at the base of the head where neck comes in" and swap at keyframes; (2) wrap the angle symbols in a graphic symbol with one angle per frame and pick the frame; (3) for smooth transitions keep elements on one timeline and tween or redraw. A combination is described as common. **[V-web]** — [Adobe community, rig with multiple head angles](https://community.adobe.com/questions-540/does-anyone-know-how-to-create-a-rig-with-multiple-head-angles-107406)
- A TV animator on the Moho forum describes smooth turning setups between three-quarter views and a Switch layer to "pop" to the side views, which are a different drawing and setup. **[S]** (the forum page returned only a redirect notice) — [Lost Marble forum post](https://www.lostmarble.com/forum/viewtopic.php?p=217899)
- Adobe Character Animator's Head and Body Turner uses the view layers Frontal, Left Quarter, Left Profile, Right Quarter, Right Profile. **[S]** — [Adobe, prepare artwork](https://helpx.adobe.com/adobe-character-animator/desktop/creating-and-controlling-puppets/prepare-artwork.html)
- AniCraft's turnaround template has five views: Front, Three-quarter, Side, Three-quarter back, Back. **[V-local]** — `~/src/anicraft/crates/doc/src/templates.rs` L176
- For eight-direction sprites, one tutorial series draws five directions and mirrors the right-facing ones to obtain the left. **[S]** — [gamedev.net, Spine 8-direction character](https://gamedev.net/tutorials/visual-arts/spine-8-direction-character-part-1-photoshop-r5408)

### Inferences
- **Emit key views, not a deformation rig.** AniCraft has no Live2D-style warp-deformer parameter space; it has symbols, frame picking, classic and shape tweens, bones and Asset Warp. The fitting hand-off is the Flash one: a Head symbol whose frames are the views, all sharing a registration point at the base of the neck, with the same part names in every view.
- **Five drawn views cover eight directions** by mirroring, provided the design is symmetric or the generator handles asymmetric details (hair parting, accessories) explicitly per side.
- **Small turns can still be tweened.** Within one view, a few degrees of head yaw or pitch is achievable in AniCraft with classic tweens that slide separate feature symbols by different amounts (the parallax Live2D uses), plus a shape tween or Asset Warp on the face contour. This is why features must be separate parts.
- **Give each part a 3D anchor in the generator.** If the parametric model stores an approximate 3D position per feature (the 2.5D idea), it can place features consistently in every key view and can also emit intermediate views (for example 15 and 30 degrees) as extra frames, which gives a stepped but coherent turn without any runtime deformation. Hair and other concave shapes should be generated per view rather than interpolated, given the paper's stated failure cases.
- **Point correspondence is optional but valuable.** If the same part has the same number and order of path points in adjacent views, AniCraft's shape tween can morph between them with few or no shape hints. Whether this produces acceptable anime faces was not tested.

### Gaps
- No primary Toon Boom or Moho documentation on master controllers for turnarounds was opened; only a search summary mentions a rig with master controllers for head angles and full turnaround.
- No source gives a measured limit for how far a parallax-only turn holds up in classic Flash rigs; the 30-degree figure is Live2D's parameter range, not a general law.
- The quality of AniCraft shape tweens between two generated head views is unknown.

## 4. RPG character needs: sprites, portraits, equipment layering, rasterising at small sizes

### Takeaway
RPG use splits into two products with different proportions: small map and battle sprites on fixed grids (RPG Maker: 48 px, 4 directions x 3 patterns; LPC: 64 px, 4 directions, up to 13 frames per row) and large portrait or bust art with swappable expressions. The first needs a dedicated chibi proportion set and layer-ordered paper-doll parts; the second is the same layered bust as section 2.

### Cited Findings

RPG Maker MZ

- Character images are "normally 48x48 pixels", with 4 directions and 3 patterns each (12 per character); a sheet holds 8 characters in 4 columns by 2 rows; a `$` prefix makes one file one character, and `!` turns off the 6-pixel upward shift. **[V-web]** — [RPG Maker MZ help, asset standards](https://rpgmakerofficial.com/product/MZ_help-en/01_11_01.html)
- Faces are 144x144 pixels, 8 per sheet (4 columns by 2 rows); enemies and pictures may be any size, with even widths recommended. **[V-web]** — [RPG Maker MZ help, asset standards](https://rpgmakerofficial.com/product/MZ_help-en/01_11_01.html)
- Side-view battlers use a 576x384 sheet in a 9x6 grid of 54 patterns, three patterns per motion; standby motions loop 1-2-3-2 and one-shot motions play 1-2-3. **[V-web]** — [RPG Maker MZ help, side-view character standards](https://rpgmakerofficial.com/product/MZ_help-en/01_11_02.html)

Liberated Pixel Cup and the Universal LPC generator

- Frame size 64x64, 13 frames per standard row, directions in the order up, left, down, right, body types male, female, teen, child, muscular, pregnant. **[V-web]** — [ULPC generator, sources/state/constants.ts](https://raw.githubusercontent.com/LiberatedPixelCup/Universal-LPC-Spritesheet-Character-Generator/master/sources/state/constants.ts)
- Frame cycles per animation: spellcast 7, thrust 8, walk 8 (frames 1-8 of a 9-frame row whose first frame is standing), slash 6, shoot 13, hurt 6 (one direction), climb 6 (one direction), idle 2 distinct frames, jump 5, sit 3, emote 3, run 8, combat idle 2. **[V-web]** — [ULPC generator, sources/state/constants.ts](https://raw.githubusercontent.com/LiberatedPixelCup/Universal-LPC-Spritesheet-Character-Generator/master/sources/state/constants.ts)
- Oversize weapon animations use 128 px and 192 px frames. **[V-web]** — [ULPC generator README](https://github.com/LiberatedPixelCup/Universal-LPC-Spritesheet-Character-Generator)
- How parts combine: each part is a sheet definition with a `layer_1` object holding a numeric `zPos` and one path per body type, an `animations` list naming the animations it is drawn for, `match_body_color`, recolour palettes, and per-file `credits` (authors, licences, URLs). The body is `zPos: 10`. **[V-web]** — [ULPC generator, sheet_definitions/body/body.json](https://raw.githubusercontent.com/LiberatedPixelCup/Universal-LPC-Spritesheet-Character-Generator/master/sheet_definitions/body/body.json)
- Licence model: each asset carries one or more of CC0, CC-BY, CC-BY-SA, OGA-BY, GPL; CC-BY-SA is the most restrictive; the generator exports a credits sheet for the selected parts and the repository ships `CREDITS.csv`; credits must be easy for users to find. The repository's code is GPL-3.0 and it was last pushed on 2026-11-11. **[V-web]** — [ULPC generator README](https://github.com/LiberatedPixelCup/Universal-LPC-Spritesheet-Character-Generator); repository metadata via the GitHub API

Portraits

- Ren'Py (8.5.4) layered images select layers by attributes at show time and crop each layer to its non-transparent bounds. **[V-web]** — [Ren'Py, layered images](https://www.renpy.org/doc/html/layeredimage.html)

Eight directions and small sizes

- Commercial eight-direction top-down packs use walk cycles of 3, 4, 6 or 8 frames per direction; there is no single convention. **[S]** — [Slynyrd Pixelblog 22, top-down character sprites](https://www.slynyrd.com/blog/2019/10/21/pixelblog-22-top-down-character-sprites)
- Pixel artists hand-rasterise vector shapes to avoid low-resolution artefacts, and an algorithm (Superpixelator, Inglis, Vogel and Kaplan, 2013) was published to automate rasterising vector line art in pixel-art style. **[S]** — [Eurographics Digital Library record](https://diglib.eg.org/items/d3a51944-0bfd-4af6-b5e2-e0923fdd14ee)
- Practitioner advice at small sizes: silhouette and contrast over detail; one-pixel strokes are a problem when downscaling; naive downscaling reads as a low-resolution photo and needs palette reduction and clean-up. **[S]** — [bugnet.io, pixel art that reads clearly](https://bugnet.io/blog/how-to-make-pixel-art-that-reads-clearly); [sorceress.games, image to pixel art](https://sorceress.games/blog/how-to-convert-an-image-to-pixel-art-free-game-ready)

### Inferences
- **A separate chibi proportion set is needed for map sprites.** AniCraft's turnaround guides assume eight heads; at eight heads a character filling a 64 px frame has a head about 8 px tall, and at 48 px about 6 px, which cannot carry anime eyes. At two to three heads the head is 16 to 32 px. The generator should therefore treat proportions (head-to-body ratio, limb thickness, eye size, line weight) as a named preset switched together, not as one model scaled down.
- **Rasterising vectors is practical for high-resolution or smooth-scaled sprites and for portraits, and only partly for true pixel art.** For a pixel-art look the pipeline needs grid-aligned geometry, thick outlines in pixel units, a fixed palette and no anti-aliasing; a general SVG rasteriser will not produce that by itself. Rendering at a few fixed target sizes with size-specific stroke widths and detail levels (dropping lashes, highlights and mouth detail below a threshold) is the realistic approach.
- **Adopt the LPC composition model for clothing and equipment.** Every wearable is a part with a z-position (possibly different per direction, since a cape is behind the body from the front and in front from the back), a path or generator per body type, and a list of the poses it supports. In a parametric vector generator the "per body type" art becomes a function of the body's parameters, which removes LPC's main scaling problem (each asset drawn for every body type and animation).
- **Sprite frames can come from the generator's skeleton.** A walk cycle on a chibi skeleton with four or five drawn directions is a small set of joint angles per frame; a generator that poses parts from a skeleton can emit RPG Maker's 3-pattern or LPC's 8-frame walk directly, or hand the parts to AniCraft and use its sprite-sheet export.
- **Target grids worth supporting first:** RPG Maker MZ (48x48, 4x3, `$` single-character files, 144x144 faces) and LPC-compatible 64x64 rows, because both are documented and widely consumed. LPC compatibility of layout does not require LPC's art or its share-alike licences.
- **Portraits:** emit the bust as aligned layers (base, outfit, brows, eyes, mouth, extras) on a common canvas with `name_group_attribute` file names, so the same output serves Ren'Py layered images, RPG Maker faces (cropped to 144x144) and the card illustration.

### Gaps
- The LPC style guide page (perspective, base proportions, palette rules) returned HTTP 403, so LPC's head-to-body ratio and camera angle are not cited here.
- The ULPC sheet height and the full per-direction z-position CSV were not read; only one sheet definition was inspected.
- No source was found that measures legibility of rasterised anime-style vector characters at 32 to 64 px, or that prescribes chibi ratios for specific sprite sizes. The head-size figures above are arithmetic.
- Battle-sprite conventions outside RPG Maker's side view (for example front-view battlers in other engines) were not researched.

## 5. Automation aids and precedents: lip-sync, physics, auto-rigging, generated Live2D

### Takeaway
Automatic 2D rigging research (2023 to 2026) spends most of its effort recovering what a parametric generator already has: separated layers, hidden regions, draw order and joints. That is the strongest argument that emitting structure from the start is the right design. Lip-sync timing and hair secondary motion are already available as free tools downstream.

### Cited Findings
- Rhubarb Lip Sync: MIT licence; latest release v1.14.0 (published 2025-04-03), repository last pushed 2026-06-16, not archived; reads WAV and Ogg Vorbis; writes TSV, XML, JSON and DAT (Moho and OpenToonz switch data); PocketSphinx recogniser is English only, with a language-independent phonetic recogniser for other languages; integrations for After Effects, Moho, OpenToonz, Spine and Vegas Pro. **[V-web]** — [Rhubarb Lip Sync README](https://github.com/DanielSWolf/rhubarb-lip-sync); the licence text (the repository's `LICENSE.md`) and the release data were read through the GitHub API for the same repository
- AniCraft has stream-synced sound for dialogue, audio scrubbing, and the mouth-chart template, but no automatic lip-sync feature was found in its guide or roadmap. **[V-local]** — `~/src/anicraft/docs/guide/images-and-sound.md` L155, L215; `~/src/anicraft/docs/guide/getting-started.md` L77
- AniCraft bone springs give hair and tail secondary motion without simulation state: the swing is computed from the armature's motion up to each frame and fades within six seconds. **[V-local]** — `~/src/anicraft/docs/guide/masks-guides-camera-bones.md` L222-240
- Meta's Animated Drawings (Smith et al., ACM Transactions on Graphics 42(3), 2023) detects a drawn humanoid, segments it, estimates joints, rigs it, deforms it with as-rigid-as-possible shape manipulation and retargets BVH motion; it writes `char_cfg.yaml`, `mask.png` and `texture.png`; code and weights are MIT; the repository was archived on 2025-09-03. **[V-web]** — [facebookresearch/AnimatedDrawings](https://github.com/facebookresearch/AnimatedDrawings)
- Textoon (arXiv 2501.10020, January 2025) generates a Live2D character from text in about one minute by filling a fixed template: predefined component categories (5 back hair, 3 mid hair, 3 front hair, 5 tops, 6 sleeves, 5 pants, 5 skirts, 6 shoes), SDXL texture generation guided by component contours, mask-based extraction with occluded areas pre-filled, and ARKit-style mouth blendshapes added to the usual Mouth Open and Mouth Form. Its stated limits are the component layout of the original Live2D model and limited variety of component styles. **[V-web]** — [Textoon, arXiv HTML](https://arxiv.org/html/2501.10020); [abstract](https://arxiv.org/abs/2501.10020)
- See-through (arXiv 2602.03749, February 2026) decomposes one anime illustration into fully inpainted, semantically distinct layers with inferred drawing order, trained on supervision generated from commercial Live2D models. **[V-web]** — [arXiv 2602.03749](https://arxiv.org/abs/2602.03749)
- Bunraku (arXiv 2607.27348, July 2026, a preprint) takes one illustration and outputs ordered RGBA layers with hidden regions completed, a triangle mesh per layer, and parameter-driven keypose vertex displacements; it reports a per-vertex direction cosine of 0.768 on 50 held-out characters, introduces an 8,884-model Live2D corpus, and states that no earlier generative method produced a complete Live2D asset. **[V-web]** — [arXiv 2607.27348](https://arxiv.org/abs/2607.27348)

### Inferences
- **The generator's structural output is the valuable part.** Textoon needs a template to know where parts are; See-through and Bunraku train large models to recover layers, occluded regions and draw order from a flat image. A parametric generator knows all three exactly and for free, and can also supply joints, which Animated Drawings estimates with a pose model.
- **Skeleton-first generation makes animation reusable across characters.** If every character is generated from the same named joint hierarchy with varying bone lengths, an animation stored as joint angles (a walk, a wave, a nod) applies to every character without retargeting error, because angles are independent of limb length; only root translation (stride length) needs scaling by leg length. This is an argument from the structure, not from a cited measurement.
- **Lip-sync path:** Rhubarb's output (time, shape letter) maps onto frame picks in an AniCraft Mouth symbol through `element.set_loop {single_frame, first_frame}` keyframes; a small script can do this. Japanese dialogue would use the phonetic recogniser or simple amplitude-driven three-shape flapping, which matches anime convention anyway.
- **Sidecar joints make AniCraft bones scriptable:** `bone.add` takes head and tail coordinates, so limb chains and hair chains with spring values can be created from generator data.

### Gaps
- None of the research systems was run; their quality claims are the authors' own. Bunraku is an unreviewed preprint and its page does not name an export file format.
- CharacterGen and VRoid-to-2D pipelines, named in the original brief, were not researched after the scope change.
- No published procedural walk-cycle method for 2D cut-out characters was reviewed; the retargeting statement above is reasoning, not a cited result.
- Whether Rhubarb's phonetic recogniser gives usable results on Japanese speech was not checked.

## 6. Rig formats and runtimes (context only: why an SVG hand-off is enough, and optional later targets)

### Takeaway
Every rig runtime surveyed either deforms raster meshes, needs a paid or closed editor in the loop, or is not stable enough to target; none accepts a layered vector character more directly than a Flash-style editor does. Since AniCraft already exports Lottie, sprite sheets, video and SVG frames, the generator can reach those outputs through it and need not write any rig format itself.

### Cited Findings

| Format | Art model | Can a program write it? | Licence and cost (as read) | Evidence |
|---|---|---|---|---|
| Spine JSON | Raster regions and meshes on bones | Yes in practice: documented JSON (bones, slots, skins, attachments incl. weighted meshes, constraints, animations) | Editor: Essential $99 (shown at $69), Professional $449 (shown at $379), for businesses under $500,000 revenue; Essential cannot save or export meshes or IK. Integrating the runtimes into a product requires a Spine editor licence | [V-web] [format](https://esotericsoftware.com/spine-json-format), [prices](https://esotericsoftware.com/spine-purchase), [runtimes licence, updated 2026-11-14](https://esotericsoftware.com/spine-runtimes-license) |
| Rive `.riv` | Vector-native | Binary format documented (major version 7); an XML text format, RML, is compiled to `.riv` by the Rive CLI and is described as suited to coding agents | Runtimes MIT, no runtime fees; editor Free (exports carry a splash screen), Cadet $9, Voyager $32, Enterprise $120 per seat per month | [V-web] [format](https://rive.app/docs/runtimes/advanced-topic/format.md), [RML](https://rive.app/docs/runtimes/advanced-topic/rml.md), [pricing](https://rive.app/pricing) |
| Lottie JSON | Vector-native | Yes: open JSON specification by the Lottie Animation Community, described as still a work in progress | Open specification; AniCraft already exports it | [V-web] [Lottie spec](https://lottie.github.io/lottie-spec/latest/); [V-local] `README.md` L126-128 |
| Live2D Cubism | Raster layers from PSD, mesh-deformed by parameters | Not determined; a 2026 preprint says no earlier generative method produced a complete Live2D asset | SDK release licence waived for individuals and small-scale enterprises (annual sales under 10 million yen), except "expandable applications"; PSD must be RGB, 8-bit, one merged layer per part | [V-web] [SDK licence](https://www.live2d.com/en/sdk/license/), [threshold](https://help.live2d.com/en/?p=2181), [PSD rules](https://docs.live2d.com/en/cubism-editor-manual/precautions-for-psd-data/), [Bunraku](https://arxiv.org/abs/2607.27348) |
| Inochi2D | Raster puppet (`.inp`, `.inx`) | Not determined | BSD-2-Clause; the SDK repository says it "is purely for the standard and is not useful if you're an end user"; Inochi Creator is early access, 0.8.5 on Flathub | [V-web] [Inochi2D repo](https://github.com/Inochi2D/inochi2d); [S] [Flathub](https://flathub.org/apps/com.inochi2d.inochi-creator) |
| Godot Skeleton2D | Textured `Polygon2D` meshes weighted to `Bone2D` | Scene files are text, but the documented workflow is editor-based | Engine is free; the documentation page (4.7) is marked as possibly outdated | [V-web] [Godot 2D skeletons](https://docs.godotengine.org/en/stable/tutorials/animation/2d_skeletons.html) |

- Live2D's PSD import requires merging line art, fills and clipping masks into a single layer per part and applying layer masks, so a vector character must be rasterised per part to enter that pipeline. **[V-web]** — [Live2D manual, PSD precautions](https://docs.live2d.com/en/cubism-editor-manual/precautions-for-psd-data/)

### Inferences
- **Why SVG is enough now:** the two vector-native runtimes (Rive, Lottie) are reachable later, Lottie already through AniCraft's exporter; the raster-mesh systems (Spine, Live2D, Inochi2D, Godot) would all consume per-part PNGs plus the same joints and pivots the sidecar holds. Nothing in any of them requires a decision in the generator beyond "named parts, over-painted, with pivots and a skeleton".
- **Most plausible later direct targets, in order:** per-part PNG atlas plus Spine-style or Godot scene data for game engines (the data is the sidecar re-serialised); Rive RML if interactive vector characters are wanted, since it is text and built for agent authoring.
- Spine's runtime licence makes it a poor default for an open-source project's users, since each product integrating the runtime needs an editor licence.

### Gaps
- Not researched after the scope change, so no claim is made: DragonBones' status, Unity 2D Animation, Moho, Adobe Character Animator's licence, OpenToonz, Blender Grease Pencil, SVG with SMIL, CSS, Web Animations or GSAP, dotLottie, and whether `.moc3` or `.cmo3` can be written by a program.
- The Lottie specification's current version number and its limits for character rigs (no bones or mesh deformation in the core spec, as far as is commonly stated) were not confirmed on the page read.
- Rive RML's release date, maturity and whether it covers bones were not stated on the page.
- Live2D Cubism Editor's current version and PRO prices were not on the page read.
- Inochi2D's actual 2026 release state could not be confirmed beyond the repository README and a Flathub listing.

## 7. Design implications: intermediate representation, emitted files, and first targets

### Takeaway
The generator should build one named, layered scene description with a skeleton, joints, anchors and per-part alternates, and emit from it (a) a flat illustration SVG, (b) per-part SVGs whose viewports are centred on their pivots, and (c) a sidecar JSON; an AniCraft assembly script is then a thin, optional fourth output. For short animations the first payoff is that AniCraft hand-off; for an RPG it is rasterised sprite sheets on RPG Maker and LPC grids plus layered portraits.

### Cited Findings
- AniCraft keeps only leaf ids, flattens groups, drops hidden alternates, and makes the SVG viewport centre the registration point of an imported symbol. **[V-local]** — `~/src/anicraft/crates/svg/src/lib.rs` L201-204, L765-768, L816; `~/src/anicraft/crates/engine/src/cmds_file.rs` L445-456
- AniCraft is scriptable end to end (import, layers, symbols, frame picking, bones with explicit joint coordinates). **[V-local]** — `~/src/anicraft/docs/guide/working-headless.md` L101-122; `~/src/anicraft/docs/guide/reference/commands.md` L51-52, L138, L360-361, L510-511
- AniCraft's chart and turnaround templates define ten mouth labels and five views. **[V-local]** — `~/src/anicraft/crates/doc/src/templates.rs` L176, L189
- LPC combines parts through per-part z-positions, per-body-type variants and per-part lists of supported animations, with per-asset licence records. **[V-web]** — [ULPC generator, body.json](https://raw.githubusercontent.com/LiberatedPixelCup/Universal-LPC-Spritesheet-Character-Generator/master/sheet_definitions/body/body.json)
- Ren'Py composes portraits from attribute-selected layers named with underscores. **[V-web]** — [Ren'Py, layered images](https://www.renpy.org/doc/html/layeredimage.html)
- Live2D, cut-out practice and the 2.5D paper all need separated parts with hidden areas filled; the 2.5D paper adds one 3D anchor per stroke. **[V-web]** — [Live2D tutorial](https://docs.live2d.com/en/cubism-editor-tutorials/psd/); [Rivers et al.](https://www-ui.is.s.u-tokyo.ac.jp/~takeo/papers/alec_siggraph2010_cartoon.pdf)

### Inferences

Intermediate representation (all of this is a recommendation built on the findings above)

- **Skeleton:** a fixed, named joint hierarchy (root, pelvis, spine, chest, neck, head, shoulder, elbow, wrist, hip, knee, ankle per side, plus optional hair and accessory chains) with bone lengths and rest angles as parameters. Parts are generated in bone-local coordinates, so the pivot is the origin by construction.
- **Parts:** each part has a stable dotted name, a parent bone, a pivot, a draw order per view, closed fill paths (extended under overlaps), optional line paths, a paint reference, and an optional clip (for example iris inside eye white).
- **Alternates:** per part, a named set of states (mouth: rest, AI, E, O, U, consonant, FV, L, MBP, WQ, with closed, half and open aliases; eye: open, half, closed, smile; brow: neutral, raised, lowered, angled). Expressions are named tuples over these sets.
- **Views:** front, three-quarter, side, three-quarter back, back, each holding the same part names, with a per-part 3D anchor used to place features; the mirrored side is derived.
- **Proportion presets:** at least "illustration" (tall) and "chibi" (two to three heads), each a full parameter set rather than a scale.
- **Wearables:** parts with a body-parameter-driven shape, a z-position per view, and a list of poses supported.

Emitted files

- **Illustration SVG:** one file, groups allowed for human editing, but every leaf uniquely named and styled inline, with no filters, patterns, style sheets or hidden content, so that it also imports cleanly as a single AniCraft layer.
- **Part SVGs:** one file per part and state, file name equal to the part name (for example `arm_l_upper.svg`, `mouth_AI.svg`, `head_front.svg`), `viewBox` centred on the pivot, artwork in pivot-relative coordinates.
- **Sidecar JSON:** units and canvas size; skeleton (joint names, parents, rest positions); per part: file, parent bone, pivot in stage coordinates, z-order per view, state set; expression tuples; view list; suggested spring values for hair chains; palette. This one file is what later exporters (sprite sheets, Spine-style data, a Godot scene, Rive RML) would also read.
- **Optional AniCraft script:** a JSON array of commands generated from the sidecar that imports the parts to the Library, creates named layers, places instances, builds the Mouth and Head symbols' labelled frames, and adds bones.

First targets

- **(a) Short animations:** per-part SVG plus sidecar into AniCraft. It is the only route examined that keeps the art as editable vectors, and it yields GIF, WebP, video, Lottie and sprite sheets from one tool. The risk is AniCraft's early-alpha status, which argues for keeping the sidecar tool-neutral and the script thin.
- **(b) RPG:** rasterised sheets written by the generator itself from posed skeleton frames, on RPG Maker MZ's 48x48 4x3 grid and an LPC-layout 64x64 grid, using the chibi preset, plus layered portrait PNGs or SVGs with expression attributes. This needs no animation tool for walk cycles and gives immediate use in existing engines.
- **Static card illustration** needs nothing beyond the illustration SVG, but benefits from the same expression tuples and proportion presets.

### Gaps
- The proposed pipeline has not been tried: no part SVG was imported into AniCraft, so the pivot-by-viewport mechanism and the assembly script are untested readings of the code.
- AniCraft is at version 0.3.0 and its import behaviour may change; the roadmap lists images and style sheets as next for SVG (`~/src/anicraft/docs/roadmap.md` L125) but says nothing about preserving groups, ids of groups, or symbols from `<use>`.
- Whether AniCraft's sprite-sheet exporter can write the exact RPG Maker or LPC grid layouts was not checked.
- No evidence was gathered on how much of this structure a card-game illustration workflow would actually use; the cost of emitting alternates and views that are never animated was not weighed.
