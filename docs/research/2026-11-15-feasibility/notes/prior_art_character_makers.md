# Prior art: parametric / slider-driven anime and cartoon character creators (2D, vector/SVG emphasis), as of 2026-11-15

Legend used on every finding:
- **[P]** = read in this session on a primary source (official site or docs, the repository's own README / LICENSE / docs, or the GitHub REST API). All GitHub star counts, push dates and licence fields are from the GitHub REST API, fetched anonymously on 2026-11-15.
- **[S]** = taken from a search-result summary or a secondary page; the primary page was not opened or was blocked (HTTP 403). Treat as a lead, not as settled.
- Anything in an "Inferences" list is my reasoning, not a sourced fact.

---

## 1. Consumer avatar makers (Picrew, CHARAT, Neka, VRoid Studio, Live2D-based makers, Kisekae-style tools, "anime character creator" web apps): how do they represent a face, and are any truly parametric in proportions?

### Takeaway
The 2D consumer makers (Picrew, CHARAT, Neka, Kisekae-style dollmakers) are layered-image part swappers with at most per-part position / rotation / colour nudges; none found models proportions as continuous, interpretable parameters. The only mainstream consumer tool that is genuinely slider-parametric for anime proportions is VRoid Studio, which is 3D, GUI-only, and whose guidelines forbid distributing a generator application built on its meshes and textures. One paid 2D tool, Mannequin Character Generator, does export anime characters as SVG.

### Cited Findings

**Picrew**
- [P/S] Internal model is strictly layered images: an image maker is built from *parts* (eye, mouth, hair ...), each part is composed of one or more *layers*, and creators upload images for each *item* of a part; there is a left-right paired part type. — [Picrew support: basic knowledge on image makers](https://support.picrew.me/en/?p=1642)
- [S] Creator limits: up to 50 layers and parts, and up to 750 items in total per image maker. — [Picrew support: parts register](https://support.picrew.me/en/creator_functions/parts_register)
- [S] Usage rights are set per maker by its artist, in tiers (personal, non-commercial, commercial, processing); "commercial" is defined broadly as any direct or indirect activity for financial gain, and the creator's own description takes precedence over Picrew's general examples. — [Picrew support: what we want you to protect](https://support.picrew.me/en/?p=3051)
- [P] Community tools exist only to scrape a maker's layer images (e.g. `mincerafter42/picrew-downloader`, 18 stars, last push 2022-08-10; `Martinz64/picrew-player-reimplementation`, 5 stars, MIT, last push 2025-10-19), which confirms the layered-raster representation and that there is no official API. — [picrew-downloader](https://github.com/mincerafter42/picrew-downloader); [picrew-player-reimplementation](https://github.com/Martinz64/picrew-player-reimplementation)

**CHARAT**
- [S] A web service where the user combines parts into an original avatar; known for roughly 2.5-head-tall avatars; mainly used for SNS icons; the official FAQ is reported to recommend SNS-icon type use (not a commercial licence). — [pixiv dictionary: CHARAT](https://dic.pixiv.net/a/CHARAT)
- [S] Per-part controls are position, rotation and colour (some categories add transparency and hue) rather than proportion sliders. — [fone.tips review](https://fone.tips/best-anime-character-creator/)
- [P] At least CHARAT's chibi maker serves its parts as SVG: a third-party tool exists whose stated purpose is "download pure SVG from Charat chibi creator" (`archanaberry/charatchibidl`, 4 stars, MIT, last push 2026-02-16). — [charatchibidl](https://github.com/archanaberry/charatchibidl)

**Neka**
- [S] Parts-based: the user switches the widget for each part, with a colour linkage that keeps linked parts in the same tone. No proportion parameters described. — [Neka docs: make a neka](https://docs.neka.cc/en-US/for-user/make-a-neka)

**VRoid Studio (pixiv) - 3D, slider-based**
- [S] Face features, body shape and outfits are edited with preset items plus parameter sliders; the stable release unified the face parameters that had been split male/female and added eye/nose shape, contour and neck-thickness options. — [Steam listing](https://store.steampowered.com/app/1486350/VRoid_Studio/); [VRoid news: stable version](https://vroid.com/en/news/1SVF4jUBUAGiUiO0NeM1Fy)
- [S] Still maintained: latest update seen in results was v2.12.0, dated 2026-03-26 (20 new side-hair presets). — [Steam listing](https://store.steampowered.com/app/1486350/VRoid_Studio/)
- [P] Licence, from the official guidelines (page marked "Edited December 21st, 2023"): preset meshes, clothing and base models are pixiv content and are explicitly **not CC0**; pixiv retains copyright and licenses use. Absent special clauses, any individual or corporation may use them commercially, sell exported model data, edit it in other software, and profit from "images and videos of models" in "videos, books, printed materials such as posters and cards, goods ..., games, applications, software". — [VRoid Studio Guidelines](https://vroid.com/en/studio/guidelines)
- [P] Critical restriction for this project: "You cannot create an application that can generate or output 3D models, avatars, or items consisting of deformed or combined meshes and textures that were created in VRoid Studio" without a separate pixiv licence (a program for the author's own use only is exempt). Reverse engineering, decompiling and disassembling the program are prohibited. — [VRoid Studio Guidelines](https://vroid.com/en/studio/guidelines)
- [S] VRoid Hub models carry per-model licence fields (separate corporate / individual commercial-use settings). — [VRoid developer conditions of use](https://developer.vroid.com/en/guidelines/conditions_of_use.html)

**Mannequin Character Generator (AR14) - paid 2D anime tool with SVG export**
- [P] A "2D Character Portrait Generator" for games, visual novels and VTuber streams, tagged Anime / JRPG / Visual Novel, art made in Adobe Illustrator; exports PNG and **SVG** ("theoretically unlimited resolution") and Inochi2D VTuber models; multiple face and clothing variations per file with batch export; Windows / macOS / Linux; minimum price USD 29.99 with a free demo; status "In development" (early access), custom templates "coming soon"; page showed "Updated 4 days ago" on 2026-11-15; itch.io asset-licence field reads CC0 1.0; no CLI, API or scripting is mentioned. — [itch.io page](https://ar14.itch.io/mannequin)
- [P] Steam page: full-body portraits with "various poses, clothing styles, hairstyles and expressions"; "You are free to use any of your exported character files in your personal and commercial projects"; Steam Workshop support. Neither page states whether shapes are adjusted by continuous sliders or only by choosing templates/parts. — [Steam page](https://store.steampowered.com/app/1280420/)
- [S] Inochi2D export arrived in version 0.3.17 with a redrawn female template; the free demo's Inochi2D export is unrestricted. — [devlog 0.3.17](https://ar14.itch.io/mannequin/devlog/485723/0317-is-here-redrawn-female-template-and-vtuber-export-via-inochi2d)

**Kisekae-style tools**
- [S, low authority - fan posts] "Kisekae 2" is a dollmaker that began as a Flash game by a single developer ("pochi"); since Flash's end it runs as an offline Adobe AIR build. Characters and scenes are exchanged as long text "export codes" that can be imported whole or per part (clothes, face, eyes ...); sliders can be pushed past their bounds; a community mod ("KKL") extends it and its codes are not accepted by the vanilla build. — [fan tutorial on export/import codes](https://art.cleberg.net/post/sheepy-drackzahn/Sheepy-s-Kisekae-Hints-Export-Import-Codes-and-716209611); [fan post on KKL codes](https://art.cleberg.net/post/seadude54/Request-Pyra-KKL-Codes-in-Description-969256848)
- [P] The older KiSS ("Kisekae Set System") paper-doll format has open implementations: `kisekae/UltraKiss` (27 stars, GPL-3.0, pushed 2026-11-14) and `emhoracek/smooch` (49 stars, GPL-3.0, last push 2022-12-06). These are layered-cel viewers, not parametric. — [UltraKiss](https://github.com/kisekae/UltraKiss); [smooch](https://github.com/emhoracek/smooch)
- [P] The Character Creator (charactercreator.org) describes itself as kisekae-like: pick items to dress a character, download as SVG or PNG (see section 4). — [charactercreator.org](https://charactercreator.org)

**Live2D-based makers and GAN "waifu" makers**
- [S] No app was found that assembles a Live2D model from continuous identity parameters. Closest consumer items: "vTuber Kit" (itch.io, Windows; mix-and-match parts) and tools that split a user's own illustration into parts with local AI models ("VTuberPro Workstation" on Steam). — [vTuber Kit](https://kyuppin.itch.io/vtuber-kit); [Steam app 5354360](https://store.steampowered.com/app/5354360)
- [S] GAN-era anime makers (MakeGirlsMoe, Crypko, Waifu Labs) expose attribute choices such as hair and eye colour over a learned raster generator; MakeGirlsMoe corresponds to the paper "Towards the Automatic Anime Characters Creation with Generative Adversarial Networks" (arXiv 1708.05509). — [arXiv 1708.05509 mirror](https://www.arxiv-vanity.com/papers/1708.05509/); [Gizmodo Japan on MakeGirlsMoe](https://www.gizmodo.jp/2017/08/make-girls-moe.html)

### Inferences
- Picrew, CHARAT and Neka cannot serve as a base: no API, per-artist licences, and a representation (stacked pre-drawn images) that has no notion of jaw shape or eye-line height. Their value is as evidence of demand and of how users expect parts to be categorised.
- CHARAT's chibi maker using SVG parts shows vector part libraries are practical at consumer scale, but it is still swap-and-nudge, not morphing.
- VRoid Studio is the best existing reference for "what sliders an anime creator should have", but it is unusable as a base: no headless mode was found, and the generator-application clause blocks redistributing anything built on its meshes. Using it privately as a measurement source (rendering fronts and sides to study proportions) is a different question from shipping its assets and should be checked against the Terms of Use.
- Mannequin is the nearest shipped product in output terms (anime, 2D vector, SVG). It differs in every other respect that matters here: closed source, GUI-driven, template-based as far as its pages show, portrait-oriented.
- Kisekae 2's text export code is a de facto serialised parameter vector for a 2D anime-style figure, which makes it a useful precedent for a compact, shareable character encoding even though the tool itself is closed and Flash/AIR-bound.

### Gaps
- Whether Mannequin exposes any continuous shape sliders, and whether its SVG export keeps parts on named layers, could not be confirmed from its itch.io or Steam pages. The demo would have to be run.
- No authoritative documentation was found for Kisekae 2's slider set (height, head size, limb lengths) or its export-code grammar; only fan posts. I could not verify the commonly repeated claim that body proportions are slider-driven.
- VRoid Studio's full face/body parameter list and count were not found on an official page (the help centre was not reached).
- The exact text of VRoid Studio Terms of Use Article 11 was not read; only the guidelines page was.
- Whether "vTuber Kit" outputs Live2D models specifically was not confirmed.

---

## 2. 3D parametric systems that could be a base (VRoid/VRM, MakeHuman/MPFB2, Blender anime base meshes, Character Creator, Daz, Ready Player Me, MetaHuman): does any expose anime-style morph targets under an open licence?

### Takeaway
No mature, openly licensed set of anime-proportion morph targets was found. MakeHuman/MPFB2 is the strongest open parametric base (GPLv3 code, CC0 targets) but is realistic-human; MB-Lab shipped anime base models but is archived and its base character is AGPL; the one open project aimed squarely at headless, agent-driven parametric anime characters (Hamr) is months old, 3D/VRM-only, and leans on the MB-Lab mesh.

### Cited Findings

**VRM ecosystem (format, not a generator)**
- [S] VRM 1.0 became the stable specification in September 2022; the spec is described as MIT-licensed by a secondary source. The GitHub API reports no detected licence file on `vrm-c/vrm-specification` (348 stars, pushed 2026-10-02). — [VIVERSE overview](https://news.viverse.com/post/vrm-models-explained); [vrm-specification](https://github.com/vrm-c/vrm-specification)
- [P] Open tooling is healthy: `pixiv/three-vrm` (2,290 stars, MIT, pushed 2026-10-22); `saturday06/VRM-Addon-for-Blender` (1,741 stars, MIT, pushed 2026-11-14, "Blender 2.93 to 5.3"). — [three-vrm](https://github.com/pixiv/three-vrm); [VRM-Addon-for-Blender](https://github.com/saturday06/VRM-Addon-for-Blender)

**MakeHuman / MPFB2**
- [P] `makehumancommunity/mpfb2`: 704 stars, pushed 2026-11-12 (active). The standalone `makehumancommunity/makehuman` app: 1,653 stars, last push 2024-08-19 (stale). — [mpfb2](https://github.com/makehumancommunity/mpfb2); [makehuman](https://github.com/makehumancommunity/makehuman)
- [S] MPFB2 code is GPLv3; bundled assets are CC0; works with Blender 4.2+. — [CG Channel, March 2025](https://www.cgchannel.com/2025/03/check-out-open-source-blender-character-generation-plugin-mpfb-2)
- [S] Since the 2020 licence change, targets, proxies and the base mesh are treated as CC0, "regardless of how they were obtained". — [MakeHuman FAQ: 2020 licence change](https://static.makehumancommunity.org/oldsite/faq/what_changed_regarding_the_license_in_2020.html)
- [S] Structure: every slider is backed by one or more artist-made targets (per-vertex offsets); macro targets cover gender, age, height, weight, ethnicity, muscle and proportion, with a macro definition deciding which underlying targets to blend at what intensity. A third-party port states the target folder holds 1,258 morph targets. — [MPFB docs: targets](https://static.makehumancommunity.org/mpfb/docs/assets/targets.html); [makehuman.js listing](https://awesome.ecosyste.ms/projects/github.com%2Fmarkandre13%2Fmakehuman.js)
- [S] Community asset packs are not uniformly CC0 (some target packs are CC-BY). No anime- or toon-proportion target pack was found in the catalogue. — [MakeHuman asset packs](https://static.makehumancommunity.org/assets/assetpacks.html)

**MB-Lab and CharMorph**
- [P] `animate1978/MB-Lab`: 2,034 stars, last push 2024-07-21, **archived**. Its README mentions "Anime models" (a cel shader was added to them), so anime base characters did ship with it. — [MB-Lab](https://github.com/animate1978/MB-Lab)
- [P/S] `Upliner/CharMorph` (581 stars, last push 2025-05-29) is a rewrite of MB-Lab; code GPLv3. Per its docs, the MB-Lab base character is AGPL, while alternative bases are Antonia (CC-BY) and Vitruvian (CC0). Its 0.4 release (March 2025) was the first major one. No mention of anime bases in its README. — [CharMorph](https://github.com/Upliner/CharMorph); [CharMorph docs](https://charmorph-docs.readthedocs.io/en/main/Introduction.html); [CG Channel](https://www.cgchannel.com/2025/03/check-out-new-open-source-3d-character-generator-charmorph/)

**Hamr - open, headless, agent-oriented parametric anime (3D)**
- [P] `hrabanazviking/Hamr`: 12 stars, created 2026-05-08, pushed 2026-10-22. Self-description: "Open-source parametric 3D anime character forge. Linux-native, headless-first, agent-orchestrated, VRM 1.0"; "the open-source alternative to VRoid Studio ... driven by YAML specs and agent commands"; "No GUI". Runs through `blender --background`. LICENSE file is MIT (the GitHub API classifies it as NOASSERTION). Acknowledges "MB-Lab - Open-source character base mesh". README claims 2,233 tests. — [Hamr](https://github.com/hrabanazviking/Hamr)
- [S] Spec fields listed in a directory copy of the README: body (`height_cm`, `build`, skin, `shoulder_width`, `bust`, `waist`, `hip_width`, `leg_length`), hair (style, length, volume, colours), face (eyes shape/size, eyebrow thickness/shape, nose bridge/tip/nostril, mouth width/fullness, cheeks, ears incl. `elf_factor`, chin width/protrusion/jaw width); 8 body presets; CLI `hamr build | validate | check-env | list-presets | inspect`; needs Python 3.11+ and Blender 3.4+. — [openclawdir listing](https://openclawdir.com/plugins/hamr-ir7jve)

**Other open or academic 3D**
- [S] RaBit (CVPR 2023): a parametric model for 3D biped cartoon characters learned from 3DBiCar, a dataset of 1,500 textured cartoon models; covers shape, pose and texture. — [RaBit paper](https://openaccess.thecvf.com/content/CVPR2023/papers/Luo_RaBit_Parametric_Modeling_of_3D_Biped_Cartoon_Characters_With_a_CVPR_2023_paper.pdf)
- [S] Blender anime base meshes on Gumroad / BOOTH / itch.io: none found under CC0. One free Genshin-style female base with shape keys is listed CC Attribution; others use custom terms (no resale of the base; BOOTH listings banning AI-dataset use, etc.). — [free Genshin-style base mesh](https://downloadforfree.gumroad.com/l/free-genshin-style-anime-female-base-mesh-for-blender-3d-model); [BOOTH anime base mesh](https://booth.pm/en/items/8196484)

**Proprietary, for contrast**
- [S] Ready Player Me is **discontinued**: Netflix's acquisition was announced 2025-12-19 and public services (avatar creator, developer APIs) shut down 2026-01-31. — [TechCrunch](https://techcrunch.com/2025/12/19/netflix-acquires-gaming-avatar-maker-ready-player-me/); [Avatar SDK status roundup](https://avatarsdk.com/blog/2026/08/31/avatar-platforms-2026-whos-alive-whos-gone/)
- [S] MetaHuman (realistic, not anime): from Unreal Engine 5.6 its use falls under the UE licence, and mid-2025 coverage says MetaHumans may be sold or used in other engines such as Unity or Godot; free below USD 1M revenue. — [CG Channel, June 2025](https://www.cgchannel.com/2025/06/you-can-now-sell-metahumans-or-use-them-in-unity-or-godot/); [MetaHuman licence page](https://www.metahuman.com/license); a 2026 commercial blog disputes the cross-engine reading — [StraySpark](https://www.strayspark.studio/blog/metahuman-2026-web-app-shutdown-migration-guide)

### Inferences
- A 3D base buys consistent front / three-quarter / side views for free, which no 2D system surveyed here gets. The cost is that clean SVG then has to come from a line-extraction step (for example Blender's Freestyle or Line Art SVG output), which tends to produce noisy strokes rather than hand-quality vector shapes. That trade-off is my reasoning; no source was gathered on line-extraction quality.
- MPFB2 is the only base whose data licence (CC0 targets) is unambiguously compatible with any open-source licence. Getting anime proportions from it would mean authoring new targets against the MakeHuman base mesh.
- Hamr's MIT claim sits awkwardly with an MB-Lab base mesh if that mesh is the AGPL one described in CharMorph's docs. I did not inspect which mesh Hamr actually ships, so treat this as a risk to check, not a finding.
- VRM is a sensible interchange target if 3D is ever added, because both the format tooling and the Blender add-on are MIT and active.

### Gaps
- Reallusion Character Creator and Daz Studio/Genesis were not researched to source level (both proprietary; licence terms for redistributing generated characters or building tools on them not verified).
- Whether MB-Lab's anime base models and morphs survive in CharMorph, and under what licence, was not determined.
- Hamr's actual output quality, its real number of working sliders, and which base mesh it bundles were not verified (only README claims were read).
- VRM specification licence text was not read at the primary source.
- No statistically derived 3D anime head model (a "3DMM for anime") was found; searches returned only realistic-head models and RaBit.

---

## 3. 2D puppet/rig ecosystems (Live2D Cubism, Inochi2D, Spine, Rive, Moho, Adobe Character Animator): do any generate characters parametrically rather than rig a hand-drawn one?

### Takeaway
No. All of these animate artwork someone else drew; their "parameters" drive motion and part swaps, not identity or proportions. They matter as *output targets* (Inochi2D is BSD-licensed and already has a generator exporting to it) and as evidence that recent research generates Live2D rigs from an image or text with neural models, not from sliders.

### Cited Findings
- [S] Live2D Cubism SDK release licence: individuals and small-scale enterprises (latest annual sales under 10 million yen, counted for the whole company) are exempt from the publication licence and payment, **except for "Expandable Applications"**; the obligation arises at release, not during development. — [Live2D SDK licence](https://www.live2d.com/en/sdk/license/); [Live2D help](https://help.live2d.com/en/?p=2181)
- [S] Live2D characters consist of layered parts saved as a Photoshop file, deformed by the rig. — [VIVERSE VTuber essentials](https://news.viverse.com/?p=2416)
- [S] Research on generating Live2D models is image- or text-driven and neural: CartoonAlive (arXiv 2507.17327) maps facial landmarks from one portrait to Live2D parameters in under 30 seconds; Textoon (arXiv 2501.10020) generates Live2D-format characters from text via an LLM plus text-to-image components; Bunraku (arXiv 2607.27348, listed July 2026) predicts ordered RGBA layers from a single illustration and builds meshes from them. — [CartoonAlive](https://arxiv.org/pdf/2507.17327); [Textoon summary](https://www.emergentmind.com/papers/2501.10020); [Bunraku listing](https://www.alphaxiv.org/abs/2607.27348)
- [P] Inochi2D: `Inochi2D/inochi2d` SDK 1,871 stars, BSD-2-Clause, pushed 2026-11-10 (active); `Inochi2D/inochi-creator` (the rigging app) 1,267 stars, BSD-2-Clause, last push 2025-06-16 (slower). — [inochi2d](https://github.com/Inochi2D/inochi2d); [inochi-creator](https://github.com/Inochi2D/inochi-creator)
- [P] A generator already targets Inochi2D: Mannequin exports "VTuber models with face tracking ... via Inochi2D". — [Mannequin on Steam](https://store.steampowered.com/app/1280420/)
- [S] Spine supports runtime "mix-and-match": a skin is prepared per item in the editor and skins are combined at runtime into a new skin; new empty skins can be created and populated in code; the 3.8 runtimes added skin-specific bones and constraints. This is discrete part swapping. — [Spine: mix and match](https://en.esotericsoftware.com/spine-unity-mix-and-match); [Spine runtime skins](https://ar.esotericsoftware.com/spine-runtime-skins)
- [S] Rive: runtimes are open source under MIT with no royalties; the editor is a commercial product with a free tier; Data Binding is promoted for swapping images and components "for outfits and swaps". — [Rive runtimes](https://rive.app/runtimes); [Rive vs Spine](https://rive.app/rive-vs-spine)
- [S] Adobe Character Animator's Puppet Maker: choose a style, pick features (skin colour, clothing ...), generate an optimised puppet. Styles are `.maker` template files; template authors can weight random choices. Parts-based, with a small set of built-in styles; users asked for more face shapes and custom artwork support. — [Adobe help: Puppet Maker](https://helpx.adobe.com/cz/adobe-character-animator/using/puppet-maker.html); [Adobe community: Feature Focus Puppet Maker](https://community.adobe.com/t5/character-animator-beta-discussions/feature-focus-puppet-maker/td-p/12146893)

### Inferences
- If animation is a goal, emitting an Inochi2D puppet (BSD-2-Clause, open format) alongside SVG is the route with the least licence friction. Emitting Live2D would require care: a tool that produces or loads arbitrary models may fall under Live2D's "Expandable Application" category, which is excluded from the small-business exemption. I did not read the definition of that category, so this needs checking before any Live2D export is planned.
- Spine skins, Rive data binding and Puppet Maker templates are all the same idea as Picrew (pick one item per slot), moved into an animation runtime. None interpolates shape.
- The 2025-2026 Live2D-generation papers show the field has gone to learned image-to-rig pipelines; a transparent, slider-level parametric generator is not what that literature is building.

### Gaps
- Moho was not researched (no sourced finding).
- Reallusion Cartoon Animator's "custom 2D character" composer surfaced in search results but was not read: [lead](https://cdn-www.reallusion.com/cartoon-animator/custom-2d-character.html).
- The definition and practical scope of Live2D "Expandable Application" was not read.
- Spine's editor/runtime licence terms were not researched.
- The Bunraku paper was seen only as a listing; its claims were not checked against the paper.

---

## 4. Open-source avatar generators that emit SVG (DiceBear, Avataaars, Open Peeps, Big Heads, Multiavatar, Boring Avatars, Notion-style), and anime-specific SVG or canvas generators on GitHub

### Takeaway
Mature SVG avatar generators exist and are headless and permissively licensed, but they are seed-driven part pickers for small western-cartoon busts. Anime-specific, code-drawn generators on GitHub are extremely sparse: repeated searches found one real match, `rhedak/anime-character-creator` (MIT, created August 2026, 5 stars), which draws parametric anime characters as SVG from a headless CLI. It is the closest prior art to the proposed project and should be read before any code is written.

### Cited Findings

**GitHub search results (anonymous REST search, 2026-11-15, sorted by stars)**
- [P] Query counts: "parametric anime face" 0 repositories; "anime avatar generator svg" 0; "chibi generator svg" 0; "anime character creator svg" 0; "anime svg parametric" 0; "svg character generator parametric" 0; "anime portrait generator procedural" 0; "procedural anime face" 1 (a Godot cel-shader kit, unrelated); "anime character generator svg" 3 (OmniSVG, a fork of it, and a 0-star trading-card renderer); "anime character creator" 39, of which the only code-drawn generator in the top results is the one below. — [GitHub repository search](https://github.com/search?q=anime+character+creator&type=repositories)

**The closest match: `rhedak/anime-character-creator`**
- [P] 5 stars, 1 fork, MIT, TypeScript, created 2026-08-05, pushed 2026-11-05. README: "Anime-style characters, drawn entirely in code. Every hair lock, eye, fold and button is a parametric vector shape (SVG) placed against a small skeleton of proportion anchors, so a whole cast stays on-model by construction ... no image generator runs when a character is drawn." Browser tool plus headless CLI: `node ts/bin/render.ts --out out/risa --preset risa` writes an SVG; no runtime dependencies; each SVG embeds a link in `<metadata>` that reproduces the exact character. — [anime-character-creator](https://github.com/rhedak/anime-character-creator)
- [P] Scope today: 21 preset characters; 19 are a "tall chibi" house figure whose `height` runs 0.8 to 1.3, and 2 are an adult woman's body "about 6.4 heads tall and traced from reference images"; two finishes (flat, soft); poses limited to arm swing and elbow bend; expressions, blink and gaze. — [README](https://github.com/rhedak/anime-character-creator/blob/main/README.md)
- [P] Parameter surface (docs/api.md): `CharacterParams` holds colours, `hairstyle` (named), `hair_length`, `body` profile (named), `body_build`, `frame` (-1 to 1), `bust` (0 to 1), `height`, `chest`, `waist_shift`, arm angles, hand style, plus an `Outfit` of garments switched on by giving them a colour. `FaceStyle` has 22 fields, mostly continuous: `eye_size`, `eye_width`, `eye_openness`, `eye_lower_lid`, `eye_tilt`, `eye_corner`, `iris_size`, `iris_inset`, `iris_aspect`, `eye_gap`, `brow_tilt`, `brow_weight`, `brow_length`, `mouth_drop`, `mouth_curve`, `mouth_width`, `blush`, `lid_shadow`, and a few booleans. The skeleton is derived from one number, `heads` (2.4 for the chibi build). — [docs/api.md](https://github.com/rhedak/anime-character-creator/blob/main/docs/api.md)
- [P] Design limits stated in its own docs: "The eye is one almond aperture built from four quadratics, and these deform it; there is no second eye shape to choose"; defaults are tuned to the project's own reference art ("measured off the canon"; `eye_gap` default "keeps the cast's median"), not to a corpus of drawings. — [docs/api.md](https://github.com/rhedak/anime-character-creator/blob/main/docs/api.md); [docs/architecture.md](https://github.com/rhedak/anime-character-creator/blob/main/docs/architecture.md)
- [P] Process facts: began as a Python package, ported to TypeScript and "proved byte-identical on a corpus of 7,662 characters"; about a thousand tests that fail on any byte change in a character's drawing; "This project was built with AI coding agents ... mostly Claude Code"; ships `CLAUDE.md` and `AGENTS.md`. — [README](https://github.com/rhedak/anime-character-creator/blob/main/README.md)

**Other anime / code-drawn hits (all small)**
- [P] `claudetee/sakura-atelier`: 0 stars, no licence, pushed 2026-07-02. Browser "procedural anime character creator" with sliders for face shape, eye size, spacing and tilt; it is a 3D toon-shaded turntable whose face is painted on a canvas and projected onto the skull; PNG export only. — [sakura-atelier](https://github.com/claudetee/sakura-atelier)
- [P] `miladnia/MangaFace`: 16 stars, MIT, TypeScript, pushed 2026-10-21. "Highly customizable avatar generator", "Inspired by FaceYourManga". — [MangaFace](https://github.com/miladnia/MangaFace)
- [P] `fish2lab/touhou-code-video-template`: 4 stars, MIT, pushed 2026-10-08. Fourteen chibi paper-cut characters drawn purely with JavaScript and Canvas 2D, no external images; ships `AGENTS.md` and `llms.txt` for AI agents. Fan characters, canvas not SVG. — [touhou-code-video-template](https://github.com/fish2lab/touhou-code-video-template)
- [P] `upbrew-tech/svg-creator-skill`: 42 stars, Apache-2.0, pushed 2026-07-29. An agent skill that has an LLM write SVG illustrations and characters freehand with a render-feedback loop. Not parametric; no stable character model. — [svg-creator-skill](https://github.com/upbrew-tech/svg-creator-skill)
- [P] `hugoogb/avatar-generator` (`@avatar-generator/core`): MIT, v3.0.0; 11 styles including an "Anime" style ("anime/manga-inspired faces with expressive eyes"); `createAvatar` returns an SVG string; output is seed-deterministic and byte-identical across runtimes. — [avatar-generator docs](https://avatar-generator.hugoogb.dev/get-started/introduction)
- [P] `OmniSVG/OmniSVG`: 2,663 stars, Apache-2.0, pushed 2026-03-01; NeurIPS 2025 end-to-end multimodal SVG generators built on pretrained vision-language models. A learned image/text-to-SVG model, not a parametric character system. — [OmniSVG](https://github.com/OmniSVG/OmniSVG)

**General SVG avatar generators**
- [P] DiceBear: `dicebear/dicebear` 10,090 stars, MIT code, pushed 2026-11-15. 61 styles: 42 CC0 1.0, 14 CC BY 4.0, 1 MIT, 4 under the artist's own terms (Avataaars and Bottts, described as free for personal and commercial use). Open Peeps, Lorelei and Notionists are CC0. — [dicebear](https://github.com/dicebear/dicebear); [DiceBear licences](https://www.dicebear.com/licenses/)
- [S] DiceBear is usable as a JS library, an HTTP API (`api.dicebear.com/10.x/<style>/svg`, no auth) and an official CLI (`npx dicebear lorelei --seed "Felix" --format svg`). — [DiceBear HTTP API](https://next.dicebear.com/how-to-use/http-api); [DiceBear introduction](https://dicebear.com/introduction/)
- [P] faces.js: `zengm-games/facesjs` 1,369 stars, Apache-2.0, created 2012, pushed 2026-10-25. "Generates and displays cartoon faces, somewhat reminiscent of Nintendo's Miis"; a face is a small JSON object; `faceToSvgString` for Node; CLI (`npx facesjs -r white -o test.svg`, or `--input-json` with a full or partial face object, missing features randomised); overrides; "relative of an existing face" generation; each face is assembled from multiple hand-drawn SVG feature files on a 400x600 canvas. — [facesjs](https://github.com/zengm-games/facesjs)
- [P] Bean Heads (formerly Big Heads): `RobertBroersma/beanheads` 1,493 stars, MIT, pushed 2026-01-14. — [beanheads](https://github.com/RobertBroersma/beanheads)
- [P] Avataaars React component: `fangpenlin/avataaars` 847 stars, MIT, last push 2024-03-04 (inactive). — [avataaars](https://github.com/fangpenlin/avataaars)
- [P] Multiavatar: 1,980 stars, last push 2022-03-26 (**unmaintained**); custom "Multiavatar License v1.0" permitting free commercial use subject to listed restrictions. — [Multiavatar](https://github.com/multiavatar/Multiavatar)
- [P] Boring Avatars: 6,503 stars, MIT, pushed 2026-06-19; abstract SVG avatars from a name and palette. — [boring-avatars](https://github.com/boringdesigners/boring-avatars)
- [P] Notion-style: `Mayandev/notion-avatar` 3,233 stars, MIT, pushed 2026-10-30. — [notion-avatar](https://github.com/Mayandev/notion-avatar)
- [P] The Character Creator: `ubik23/charactercreator` 346 stars, last push 2025-11-21, licence not auto-detected; full-body characters dressed from item lists, downloadable as SVG or PNG, crops for full / upper body / close-up, "more angles and positions" for Patreon backers; character art is CC-BY-NC with commercial use via a Patreon pledge of USD 5/month or more. — [charactercreator.org](https://charactercreator.org); [repository](https://github.com/ubik23/charactercreator)
- [P] Universal LPC Spritesheet Character Generator: 2,014 stars, GPL-3.0, pushed 2026-11-11; parts-based pixel-art sprite sheets (raster). — [LPC generator](https://github.com/LiberatedPixelCup/Universal-LPC-Spritesheet-Character-Generator)
- [P] `alec-deason/face_generator`: 15 stars, MIT, last push 2021-12-08; template-based procedural faces for D&D-style NPCs (inactive). — [face_generator](https://github.com/alec-deason/face_generator)

### Inferences
- The SVG avatar ecosystem solved "headless, deterministic, JSON-in, SVG-out" long ago. What it never attempted is continuous proportion control, bodies, or anime drawing conventions (large eyes with layered highlights, hair as overlapping locks, minimal noses).
- faces.js is the best architectural reference among the mature projects: JSON face object, CLI, overrides, partial-spec completion, and a drop-in folder of SVG features. It is head-and-shoulders only and stylistically far from anime.
- `rhedak/anime-character-creator` overlaps the proposal on medium (SVG), method (code-drawn shapes on a proportion skeleton), driving mode (headless CLI, built and operated with coding agents) and licence (MIT). It does not overlap on: statistically derived defaults and ranges, a library of discrete part *types* per feature (it has one eye construction), jaw / face-outline parameters (none appear in `FaceStyle`), and view angles (no view or angle parameter appears in its API docs, so it appears front-only). It is three months old with one evident author, so its direction could change quickly.
- The appearance of several 2026 repositories that draw anime characters in code with agent-oriented docs (`AGENTS.md`, `CLAUDE.md`, `llms.txt`) suggests this niche is opening right now, driven by coding agents. The window in which "nothing exists" is closing, though nothing found is established.
- None of DiceBear's 61 style names indicates an anime or manga style; I did not view every style, so this is an inference from the list.

### Gaps
- GitHub search is keyword-bound; a relevant repository with an unrelated name or non-English description could have been missed. GitLab, Codeberg, npm and PyPI were not searched.
- Whether faces.js face objects include continuous fields (such as head fatness, feature size or angle) was not confirmed from the README lines read; I recall such fields from background knowledge but did not verify them.
- MangaFace's internal model (parts versus continuous) was not inspected.
- `hugoogb/avatar-generator` star count and whether its Anime style takes continuous options were not checked.
- `rhedak/anime-character-creator`: the "front-only" and "no jaw parameter" statements rest on reading its API reference, not its source; the licence status of the reference images it traces was not examined.

---

## 5. Game character creators as a design reference (FaceGen in Oblivion/Skyrim/Fallout; Code Vein, Black Desert, PSO2, Custom Maid/Koikatsu, Honey Select): how do the sliders work and how many parameters does a good creator expose?

### Takeaway
FaceGen is the direct precedent for "statistically grounded sliders": a PCA face space (50 symmetric plus 30 asymmetric shape modes, each scaled to one standard deviation) in which named sliders are linear directions and random faces are draws from a normal distribution. Anime-styled creators instead use artist-authored bone or morph sliders plus large preset libraries; the Mii system shows the cheapest workable hybrid, a discrete part type plus four or five continuous transform values per feature.

### Cited Findings

**FaceGen (Singular Inversions)**
- [P] Uses principal components analysis: a dataset is represented as one mean plus PCA modes (deltas from the mean), stored in decreasing order of magnitude so trailing modes can be dropped; each mode's magnitude corresponds to one standard deviation in the dataset and modes are statistically independent. — [FaceGen SDK manual: In Depth](https://facegen.com/dl/sdk/doc/manual/indepth.html)
- [P] Mode counts: "currently 50" symmetric shape modes and "currently 30" asymmetric shape modes; colour is modelled by separate texture modes. A face is a coordinate vector in "face space". — [FaceGen SDK manual: In Depth](https://facegen.com/dl/sdk/doc/manual/indepth.html)
- [P] Named sliders are *linear controls*: each is a direction vector in face space; the slider value is the projection of the face onto it, and because the directions are not orthogonal, moving one control moves others. Age (in years) and gender (-1 male, +1 female) are offset linear controls, solved jointly because they overlap. Race controls are differences between group mean positions. — [FaceGen SDK manual: In Depth](https://facegen.com/dl/sdk/doc/manual/indepth.html)
- [P] "Caricature" is the Mahalanobis distance from the group mean (1 = average distance for that group, 0 = the mean face); "asymmetry" scales the asymmetric sub-vector (0 = perfectly symmetric). — [FaceGen SDK manual: In Depth](https://facegen.com/dl/sdk/doc/manual/indepth.html)
- [P] Random faces are generated by drawing coefficients from a standard normal distribution; the manual notes its modelled gender distribution is unimodal although the real one should be bimodal. — [FaceGen SDK manual: In Depth](https://facegen.com/dl/sdk/doc/manual/indepth.html)
- [P] Asset split: base mesh (TRI/FGMESH) plus `.egm` shape modes and `.egt` texture modes per mesh, and a control file for distribution and controls. The shape model covers only the face, from mid-forehead to just below the chin, excluding ears. An `.fg` face file holds only the coordinate, no mesh data. — [FaceGen SDK manual: In Depth](https://facegen.com/dl/sdk/doc/manual/indepth.html); [FaceGen SDK manual: Overview](https://facegen.com/dl/sdk/doc/manual/overview.html)
- [S] Oblivion used FaceGen: its heads use `.egm` files for shape and `.egt` files for tone during character creation. Community reports say the in-game sliders behave differently from FaceGen Modeller's, so imported faces never match exactly (anecdotal). — [Nexus Mods: Head06 docs](https://www.nexusmods.com/oblivion/mods/20166?tab=docs); [Nexus forum thread](https://forums.nexusmods.com/topic/183195-how-do-i-use-facegen-to-create-a-character)
- [S] Skyrim's character sliders are driven by chargen morph files (e.g. `FemaleHeadCharGen.tri`, `EyesMaleCharGen.tri`), i.e. named artist morphs per slider; mods such as RaceMenu add many more. — [Nexus forum: vanilla facegen sliders for custom races](https://forums.nexusmods.com/topic/13507926-resource-for-creating-vanilla-facegen-sliders-for-custom-races)

**Mii (Nintendo) - discrete type plus continuous transform**
- [S] In the documented 3DS Mii format each feature is a part index plus a few small integer transforms. Eyes: style, colour, scale, y-scale, rotation, x-spacing, y-position. Nose: style, scale, y-position. Similar fields exist for eyebrows and mouth. — [3dbrew: Mii](https://3dbrew.org/wiki/Mii); [libctru MiiData](https://libctru.devkitpro.org/structMiiData.html)
- [S] Wii-era part counts: 48 eye types, 12 nose types, 24 mouth types. — [WiiBrew: rendering Miis](https://wiibrew.org/wiki/Libmii/Rendering_Miis)

**Anime-styled creators**
- [P] Koikatsu-family character cards are machine-readable: `great-majority/KoikatuCharaLoader` (62 stars, pushed 2026-09-17) deserialises and reserialises cards for Koikatu, EmotionCreators, Honeycome and later titles, exports JSON, and exposes face and body shape as flat float arrays (`Custom.face.shapeValueFace`, `Custom.body.shapeValueBody`) that a script can set, e.g. `k["Custom"]["body"]["shapeValueBody"][0] = 0.5`. — [KoikatuCharaLoader](https://github.com/great-majority/KoikatuCharaLoader)
- [S] Koikatsu's maker lets nearly all face and body settings (angles, sizes, feature shapes) be adjusted by hand; community plugins (ABMX, ExpandMaleMaker) add bone-scale sliders beyond the base set. — [Moegirl wiki](https://zh.moegirl.org.cn/en/%E6%81%8B%E6%B4%BB%EF%BC%81); [ABMX](https://github.com/manlymarco/abmx)
- [S] Code Vein leans on large preset libraries: Kotaku counted nearly 500 eyebrow options, 58 hairstyles and 66 eye-highlight options. — [Kotaku](https://kotaku.com/code-veins-character-creation-has-all-the-options-1838495058)
- [S] Black Desert Online uses direct manipulation of face and body regions with three slider types on height / width / depth axes, down to ear tips and mouth corners; reviewers note the jaw line, lips and eyes are finely controllable while other regions are harder. — [Gamepressure guide](https://www.gamepressure.com/blackdesert/character-creation/z18711); [MMOGames](https://www.mmogames.com/articles/archives/exploring-depths-black-deserts-character-creator)
- [S] VRoid Studio's guidance on anime faces: eye and mouth positions are centred on the nose, and moving them closer gives a younger impression, further an adult impression. — [STYLY VRoid face tutorial](https://styly.cc/tips/vroid-texture-kikiyu/)

**Parameter-count reference points gathered**
- [P] FaceGen: 80 shape dimensions (50 + 30) for the face region alone. — [FaceGen SDK manual: In Depth](https://facegen.com/dl/sdk/doc/manual/indepth.html)
- [S] MakeHuman: about 1,258 morph targets behind its sliders, grouped under a handful of macro sliders. — [makehuman.js listing](https://awesome.ecosyste.ms/projects/github.com%2Fmarkandre13%2Fmakehuman.js)
- [P] `rhedak/anime-character-creator`: 22 face fields and roughly ten body-shape fields. — [docs/api.md](https://github.com/rhedak/anime-character-creator/blob/main/docs/api.md)
- [S] Hamr: roughly twenty face fields and five body-proportion fields plus height and build. — [openclawdir listing](https://openclawdir.com/plugins/hamr-ir7jve)

### Inferences
- There are two distinct slider philosophies and the proposal mixes them, which is reasonable. (a) *Statistical* (FaceGen): axes come from data, sliders are directions in that space, ranges are expressed in standard deviations, randomisation is sampling. (b) *Authored* (Skyrim, Koikatsu, VRoid, Black Desert): an artist defines each slider as a bone scale or morph and picks the range by eye. For 2D anime, where there is no scan corpus and features are stylised symbols, the practical form of (a) is: authored, interpretable parameters whose *defaults and ranges* are fitted to measured landmark distributions, optionally with a PCA layer on top for "random plausible character" and "more/less typical" controls.
- FaceGen's design choices worth copying directly: store a character as a small coefficient vector separate from any geometry; express ranges in standard deviations from the mean; provide a "caricature" (distance from mean) control; accept that named sliders are correlated and solve overlapping ones jointly.
- FaceGen's own caveat about a unimodal model of a bimodal trait applies strongly to anime: proportions cluster by style family and by character age/type (chibi, shoujo, seinen), so a single mean and variance would blur distinct modes. Per-style or per-archetype statistics are likely needed.
- The Mii scheme (part type + scale + stretch + rotation + spacing + height) is the minimum parameterisation that makes swapped parts feel continuous, and it fits SVG well because each is an affine transform on a part group. True morphing (jaw curve control points) goes beyond it.
- As an order of magnitude, the surveyed creators sit at roughly 20 to 80 continuous face parameters and 10 to 50 body parameters; fewer than about 20 for the face tends to rely on preset libraries to supply variety. This range is assembled from heterogeneous sources and partly from my background knowledge, so treat it as indicative only.

### Gaps
- The number of scanned faces behind FaceGen's model is not stated in the SDK manual pages read. (I recall a figure of a few hundred laser scans from background knowledge but could not confirm it in this session.)
- Exact slider counts for vanilla Oblivion and Skyrim were not obtained (UESP returned HTTP 403).
- Koikatsu's exact array lengths were not confirmed. (My unverified recollection is 52 face values and 44 body values; do not rely on it without checking a card or the game's definitions.) Honey Select and Custom Maid parameter sets were not researched.
- Phantasy Star Online 2's 2D proportion morph pad was not sourced.
- No GDC talk or developer write-up on the design of an anime-styled character creator was found.
- No primary Bethesda source on how Oblivion mapped its UI sliders onto FaceGen modes was found.

---

## 6. Academic or hobby procedural-cartoon-face work: parametric cartoon face generators, "procedural anime face" papers or posts, generative-art projects that draw anime faces with code

### Takeaway
Parametric 2D face drawing has a long lineage (Brennan's 1985 caricature generator already exaggerated line drawings away from a mean face), and one 2023 paper produces vector caricatures from several hundred parameters, but no paper or substantial write-up was found that builds a parametric generator for *anime* faces or bodies with statistically derived ranges. The anime-specific literature is almost entirely neural and raster; what exists on the statistical side is small landmark datasets and proportion studies that could feed such a generator.

### Cited Findings
- [S] Brennan's Caricature Generator (MIT master's thesis 1982; Leonardo 18(3), 1985): "Exaggerating the Difference from the Mean" on a frontal face polyline whose topology matches a mean-face drawing, pushing vertices away from the mean by a constant factor. — [MIT DSpace record](https://dspace.mit.edu/handle/1721.1/15743); [SIGGRAPH 2004 talk citing it](https://history.siggraph.org/wp-content/uploads/2022/12/2004-Talks-Mo_Improved-Automatic-Caricature-by-Feature-Normalization-and-Exaggeration.pdf)
- [S] The rule is commonly written Q = P + b(P - S) (P input face, S mean face, b exaggeration rate), a formulation attributed to the PICASSO caricature system. — [Cyberworlds 2014 paper](https://www.design.yamanashi.ac.jp/docs/2014/Face_Cyberworlds14_02.pdf)
- [S] "Automatic Vector Caricature via Face Parametrization" (Madono, Hold-Geoffroy, Li, Ito et al., Pacific Graphics 2023 short paper): "Parametric Caricature, the first parametric-based caricature generation that yields vectorized and animatable caricatures"; several hundred parameters encode facial traits and are predicted directly instead of a raster image; parts are retrieved from an artist-made caricature database. — [Eurographics Digital Library](https://diglib.eg.org/handle/10.2312/pg20231271)
- [P] Red Blob Games "Procedural Face Generator" (started 2011, updated 2015 and 2018): simple cartoon faces with a mouth shape defined in a five-dimensional space (m, p, q, r, s, constrained m+p+q >= 0) plus skew and rotate, and eyebrows by `browLift` and `browAngle`; named expression presets are points in that space; cites Scott McCloud's *Making Comics* on interpolating expressions. No licence stated. — [Red Blob Games](https://www.redblobgames.com/x/1845-face-generator/)
- [S] A manga facial-landmark dataset built on Manga109 uses a 60-landmark representation; the public set has 1,446 annotated images (arXiv 1811.03214). — [paper overview](https://alphaxiv.org/overview/1811.03214v1)
- [P] `hysts/anime-face-detector`: 547 stars, MIT, pushed 2026-07-05; anime face detection and landmarks using mmdet and mmpose. — [anime-face-detector](https://github.com/hysts/anime-face-detector)
- [S] An anthropometric study measured facial landmarks on 173 frontal images of anime characters from 90 popular series (characters aged roughly 10 to 18) to test age estimation from proportions (Anthropological Review 85(2), 2022). — [Anthropological Review](https://czasopisma.uni.lodz.pl/ar/article/download/13199/15130?inline=1)
- [S] A 2024 study in the Journal of Japanese Academy of Facial Studies composited (averaged) faces of lead animated characters and measured how averaging affects perceived attractiveness and suitability for a main role. — [J-STAGE](https://www.jstage.jst.go.jp/article/kaogaku/24/1/24_3/_article/-char/en)
- [S] 3D counterpart: RaBit (CVPR 2023) learns a parametric shape space for cartoon bipeds from 1,500 models. — [RaBit paper](https://openaccess.thecvf.com/content/CVPR2023/papers/Luo_RaBit_Parametric_Modeling_of_3D_Biped_Cartoon_Characters_With_a_CVPR_2023_paper.pdf)
- [S] Neural anime generation is plentiful and raster or rig-oriented: MakeGirlsMoe's GAN paper (arXiv 1708.05509), CartoonAlive (arXiv 2507.17327), Textoon (arXiv 2501.10020). — [arXiv 1708.05509 mirror](https://www.arxiv-vanity.com/papers/1708.05509/); [CartoonAlive](https://arxiv.org/pdf/2507.17327); [Textoon summary](https://www.emergentmind.com/papers/2501.10020)
- [S] Hobby and teaching material on code-drawn faces is mostly p5.js coursework ("variable face" assignments using Bezier curves for chins and hair), not anime generators. — [CMU 15-104 Variable Face](https://courses.ideate.cmu.edu/15-104/f2020/index.html%3Fp=56553.html)

### Inferences
- The idea "mean drawing plus scaled deviation" for 2D line faces is forty years old and is exactly FaceGen's idea in 2D. Applying it to anime needs only (i) a fixed-topology landmark or control-point template for the anime face and (ii) a measured population, and neither has been published as a ready-made resource.
- Existing manga/anime landmark sets (60 points on 1,446 Manga109 faces; detector landmarks from `anime-face-detector`) are large enough to estimate means and variances for coarse face proportions (eye size, eye-line height, inter-eye gap, face width-to-height, chin taper), but they are frontal-biased and say nothing about body proportions.
- The Pacific Graphics 2023 paper is the nearest academic design to the proposal (hundreds of interpretable parameters, vector output, animatable), but it targets caricatures of real people, predicts parameters from a photo with a network, and relies on a proprietary artist database; I found no public code.
- The 2022 anthropometric study shows that measuring anime faces with landmark ratios is feasible on a sample of under 200 images, which is a useful lower bound for how much data a first statistical pass needs.

### Gaps
- No paper or blog post titled or described as a procedural / parametric anime face generator was found. Searches were run in English only; Japanese-language work (for example on nigaoe/caricature systems or on "moe" face statistics) may exist and was not searched.
- The Pacific Graphics paper's page returned HTTP 403; details come from search summaries of the Eurographics record, and code availability was not confirmed.
- Whether `anime-face-detector`'s landmark scheme is 28 points (my recollection) was not verified.
- No dataset of anime full-body proportions (head-to-body ratio, limb lengths) with annotations was found in this pass.
- A US patent titled "Vectorized caricature avatar generator" (US 11854115) appeared in results but was not read: [lead](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11854115).
- Google Scholar and the ACM Digital Library were not queried directly; coverage came through general web search and arXiv links.

---

## 7. Is there a clear gap: does anything already do "statistically grounded, continuous-parameter anime character to clean SVG, headless"? Closest three

### Takeaway
Nothing found does all of it. No system combines (a) anime style, (b) continuous proportion parameters with a choice of part types, (c) defaults and ranges derived from measured drawings, (d) clean SVG output and (e) headless, agent-drivable operation. The three closest are `rhedak/anime-character-creator` (has a, d, e and part of b; lacks c and part variety), Mannequin Character Generator (has a and d; closed and GUI-bound), and Hamr (has a, b, e in 3D; no SVG, very early).

### Cited Findings

**Closest 1 - `rhedak/anime-character-creator` (2D SVG, parametric, headless, MIT)**
- [P] What it has: code-drawn parametric vector shapes on a proportion skeleton, SVG output from a dependency-free CLI, a parameter object with continuous face and body fields, reproducible characters via embedded metadata, MIT licence, an agent-oriented repository. — [repository](https://github.com/rhedak/anime-character-creator)
- [P] Where it falls short of the stated goal: one eye construction rather than selectable eye types; defaults tuned to its own reference art, not a corpus; a 2.4-head "tall chibi" house figure for 19 of 21 characters, with the adult body traced from reference images; created 2026-08-05 with 5 stars. — [docs/api.md](https://github.com/rhedak/anime-character-creator/blob/main/docs/api.md); [README](https://github.com/rhedak/anime-character-creator/blob/main/README.md)

**Closest 2 - Mannequin Character Generator (2D anime, SVG export, commercial)**
- [P] What it has: anime-style 2D character portraits, full-body poses, clothing, hairstyles and expressions, PNG and SVG export, Inochi2D puppet export, commercial use of exported files allowed, active as of November 2026. — [itch.io](https://ar14.itch.io/mannequin); [Steam](https://store.steampowered.com/app/1280420/)
- [P] Where it falls short: paid closed-source desktop application (USD 29.99), still labelled in development, no CLI / API / scripting mentioned, template-and-variation model with custom templates "coming soon"; no statement of continuous proportion sliders or statistical basis. — [itch.io](https://ar14.itch.io/mannequin)

**Closest 3 - Hamr (headless parametric anime, 3D)**
- [P] What it has: YAML character specs with face and body proportion fields, CLI build / validate, "headless-first, agent-orchestrated", MIT licence file, VRM 1.0 export. — [repository](https://github.com/hrabanazviking/Hamr)
- [P] Where it falls short: output is a 3D VRM avatar through headless Blender, not 2D vector; 12 stars, created 2026-05-08; depends on an MB-Lab base mesh; all capability claims are from its README. — [repository](https://github.com/hrabanazviking/Hamr)

**Runners-up, each strong on one axis**
- [P] faces.js: best mature "JSON in, SVG out, CLI" architecture (1,369 stars, Apache-2.0, active since 2012), but western-cartoon busts with no bodies. — [facesjs](https://github.com/zengm-games/facesjs)
- [P] VRoid Studio: the reference anime slider set, but GUI-only 3D, and its guidelines prohibit distributing an application that generates models from its meshes and textures. — [VRoid Studio Guidelines](https://vroid.com/en/studio/guidelines)
- [P] FaceGen: the reference statistical method (PCA modes in standard-deviation units, sliders as directions, normal-distribution sampling), but realistic 3D heads and proprietary. — [FaceGen SDK manual: In Depth](https://facegen.com/dl/sdk/doc/manual/indepth.html)
- [S] MPFB2 / MakeHuman: the only open parametric base with CC0 morph data, but realistic human proportions and 3D. — [CG Channel on MPFB2](https://www.cgchannel.com/2025/03/check-out-open-source-blender-character-generation-plugin-mpfb-2)
- [P] GitHub keyword searches for the combined idea returned zero repositories ("parametric anime face", "anime svg parametric", "anime avatar generator svg", "chibi generator svg", "svg character generator parametric"). — [GitHub repository search](https://github.com/search?q=parametric+anime+face&type=repositories)

### Inferences
- Synthesis matrix (my reading of the findings above; "?" = not confirmed):

| System | Anime | Continuous proportions | Part-type choice | Stat. grounded | SVG out | Headless | Open licence | Multi-view |
|---|---|---|---|---|---|---|---|---|
| rhedak/anime-character-creator | yes | yes (face ~22, body ~10) | hair/outfit yes, eyes no | no | yes | yes | MIT | appears front-only |
| Mannequin | yes | ? | yes (templates) | no | yes | no | no (closed) | poses; angles ? |
| Hamr | yes | yes | presets | no | no (VRM) | yes | MIT (mesh ?) | yes (3D) |
| VRoid Studio | yes | yes | yes | no | no | no | no | yes (3D) |
| faces.js | no | limited ? | yes | no | yes | yes | Apache-2.0 | no |
| DiceBear | no | no | yes (seeded) | no | yes | yes | MIT + per-style | no |
| Picrew / CHARAT / Neka | yes | no | yes | no | CHARAT chibi only | no | no | no |
| FaceGen | no | yes | no | **yes** | no | SDK | no | yes (3D) |
| MPFB2 / MakeHuman | no | yes | yes | no (artist targets) | no | via Blender | GPLv3 + CC0 | yes (3D) |
| Koikatsu family | yes | yes | yes | no | no | card files scriptable | no | yes (3D) |

- The genuinely unoccupied ground is the *statistical grounding* column for anything anime, and the combination of *part-type choice with continuous morphing* in 2D vector. Headless anime SVG by itself is no longer unoccupied as of August 2026.
- Consistent multi-view output (front, three-quarter, side) is unsolved in every 2D system found; only 3D systems provide it. Any 2D parametric design should decide early whether views are separate hand-built templates sharing parameters or projections of an underlying 3D proportion model, because nothing existing can be borrowed for it.
- Practical bases to build on rather than rebuild: faces.js for the JSON/CLI/override architecture; DiceBear's CC0 styles and Open Peeps as examples of licence-clean vector part sets (not anime); Inochi2D (BSD-2-Clause) as an animation target; FaceGen's manual as the specification of the statistical layer; `rhedak/anime-character-creator` as either a collaborator, a fork point or a detailed comparison.
- The strongest reason not to build is the existence of `rhedak/anime-character-creator`; the strongest reason it does not settle the matter is that it is a young single-author house-style project with no statistical layer, no part-type library for faces and no evident multi-view support.

### Gaps
- "Nothing does" is a negative result bounded by the searches run: English-language web search and GitHub keyword search on 2026-11-15. Japanese- and Chinese-language communities (BOOTH, pixiv, Bilibili, Gitee), commercial asset stores, and closed tools without web presence were not systematically covered.
- I did not run any of the three closest systems, so output quality, real parameter behaviour and SVG cleanliness (layering, path count, editability) are unverified for all of them.
- Whether `rhedak/anime-character-creator`'s author intends statistical grounding, more part types or more views (its roadmap documents `PLAN.md` and `STATUS.md` were only skimmed) was not established.
- No licence review was done on reusing parts or code from the candidates beyond reading licence fields and headline terms.
