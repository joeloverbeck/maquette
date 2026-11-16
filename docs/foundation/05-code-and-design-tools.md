# 05 · Code and design tools

Jon's rulings of 2026-11-16.

## Code

All code is Rust, tooling included. The one exception is the thin layer of TypeScript that the web interface needs: what must run in the browser to show an armature and carry the controls to the Rust code. Proportions, geometry, parts, checks and SVG output are Rust, so the command line and the interface give the same armature from the same values. No Python. A shell script only for a job Rust cannot do by itself.

The interface is a web app. When it is built is not settled here; the first increment needs none (`02-first-table.md`).

## Design

Every design made by hand is made in one of two tools, through its MCP server.

| Tool | Makes | Examples here |
|---|---|---|
| VectorCraft | Single vector elements | A guide shape authored for one view, such as an ear or an eye's lid line; an icon |
| FigCraft | Mock-ups, layouts and compositions | The review sheet for one of Jon's looks; a mock-up of the web interface |

An armature that the generator emits is output, not a design. A shape that a person or an agent authors for the generator to use is a design, and is drawn in VectorCraft. It reaches the generator as VectorCraft's SVG export, which gives the compatibility promise in `02-first-table.md` a second direction (`FID-1`).

A review sheet that shows only generated armatures is composed in FigCraft. A reference check sheet shows published drawings, which the servers cannot read and the repository never holds. Its layout is designed in FigCraft, and a Rust tool renders each real sheet outside the repository from that layout.

## The servers

`.mcp.json` registers both for Claude Code, as `vectorcraft` and `figcraft`.

- Each may read anywhere in the repository and write only under `design/`.
- The commands are `vectorcraft-cli` and `figcraft-cli`, installed on the machine from the open-source ArtCraft checkouts. Neither is Jon's project, and both are alpha. At founding they were 0.24.1 and 0.18.1, and both started from this folder and listed their tools.
- A clean clone without them loses design work only. The generator and its checks do not need them (`OPS-7`).

## Where designs live

`design/` holds each tool's source file beside its SVG or PNG export. Reference images are not designs and never go there (`04-evidence-and-references.md`).
