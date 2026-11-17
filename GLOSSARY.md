# Maquette

The words for armatures of anime-style characters: what an armature is made of, how its views are named, and where its proportions come from. Decisions are in `docs/foundation/`; this file only fixes the language.

## Language

### The armature

**Armature**:
The construction guide a character is drawn and painted over: the named guides of one figure. It is hidden in the finished art.
_Avoid_: skeleton, base, underdrawing

**Guide**:
One named element of an armature that the painter draws against, such as the skull, the face outline or a brow line.
_Avoid_: part, shape, layer

**Landmark**:
A named point on the figure whose position is recorded, such as the chin or an eye's outer corner. Every view shows the same landmarks.
_Avoid_: anchor, keypoint

**View point**:
A named point that belongs to one view only: the side of a round form, or where two guides cross. Under the same name another view shows a different point of the head, so it is not a landmark.
_Avoid_: silhouette landmark

**Landmark file**:
The file beside a view's SVG that gives the position of every landmark and view point, and which of them each guide passes through.

**Hair volume**:
The plain envelope the hair fills over the skull, with no fringe and no locks. The hair's shape is the painter's.
_Avoid_: hair, hairstyle, hair outline

**Shoulder line**:
The line across the tops of the shoulders, where the neck ends.

**Underdrawing**:
A grey drawing with features, hair and torso placed over construction lines. Mudsong's hand-made armature is one; a generated armature is not.

**Mannequin**:
A body with a smooth surface and no anatomical detail, at every age.

### Views

**View**:
The direction a figure is seen from, with the head level.

**Front view**:
The figure seen straight on.

**Three-quarter view**:
The head turned part of the way toward the viewer's right. Its angle is the turn of the hand-made armature Jon accepted, which is shallower than the 45° the term often means.
_Avoid_: 3/4, angled view

**Side view**:
The same turn carried to a full profile, facing the viewer's right.
_Avoid_: profile view

**Turn**:
How far a level head is turned about its upright axis, toward the viewer's right, in degrees.
_Avoid_: rotation, yaw, angle

**Near side**, **far side**:
In a turned view, the side of the head toward the viewer and the side turned away.

**Left**, **right**:
In the name of a guide or landmark, always the character's own left and right. Only the direction a view faces is given from the viewer's side.

### Proportions

**Proportion**:
A named ratio between two spans of the figure, with a definition that says how it is taken in each view.
_Avoid_: measurement, parameter, slider value

**Unit**:
The one span every proportion is divided by: eye line to chin.
_Avoid_: scale, module, head unit

**Eye line**:
The level midway between the top and the bottom of the eye opening. The eye level line is the guide drawn on it.

**Depth**:
How far a point stands in front of the eyes, toward where the face points. A front view shows none.

**Set of proportions**:
One value for every proportion of the list, with the amounts of any correction it carries: what the generator draws one head from.
_Avoid_: preset, parameters, settings

**Convention**:
A proportion or a guide built by a stated rule because nothing is drawn behind it, such as the skull under the hair. It is never a measurement and has no valid range.

**Correction**:
A named departure from flat projection, with an amount, that a set of proportions carries and one view draws. The list of proportions defines each, and the landmark file of the view it changes gives every moved landmark's depth in space beside the depth it is drawn at.
_Avoid_: tweak, cheat, adjustment, fix

**Head height**:
The distance from the hair's top to the chin of a level head. Differences between positions are stated as shares of it.

**Reference**:
A published drawing whose proportions are measured.

**Reading**:
One agent's placement of the landmarks on one drawing, with its judgement of the view, the turn and whether the head is level. The drawing is a reference, or a hand-made drawing with known positions.

**Reader**:
The agent that makes a reading. Every drawing has two, and neither sees the other's reading.

**Reading instructions**:
What a reader follows: which landmarks a view asks for, and how the view, the turn and the level are judged. A reading names the revision it followed.

**Picture**:
The image file of one drawing that a reader is given and reads: its landmarks are placed in the picture's pixels, and the reading records the picture's digest. The drawing is the work; the picture is the file of it. A reference image is the picture of a reference. Commands and a reading's file call it `image`.
_Avoid_: scan, screenshot

**Reference folder**:
The one folder, outside every repository, that holds reference images, readings and reference check sheets. Configuration that is not committed names it.

**Set aside**:
What a reader does with a drawing that should not be measured: the head is not level, it is in none of the three views, the expression pulls the features out of place, or too little of the head shows to read. It is not measured. Jon can overrule it.

**Agreement**:
How far apart two readings of one drawing put a landmark, as a share of head height. A disagreement above 1% is shown to Jon.

**Fit**:
As one figure, how far a character's views are from being drawings of one level, symmetric head: the largest misfit between them, as a share of head height. It is not a lay-over.
_Avoid_: consistency, match

**Known positions**:
The positions of a drawing's landmarks that are known without reading it: those of a hand-made drawing, taken from the drawing's own file by arithmetic, and those of an accepted armature's view, taken from its landmark file. A reading's error is its distance from them. An armature's view is a drawing of guides, so it tests how well guides are read and not a painted head.

**Proportion table**:
For one group of characters, each proportion's valid range, measured on references from several works by several hands. Measurements from one series or one artist are not a table.
_Avoid_: family table (for anything narrower)

**Valid range**:
For one proportion, the lowest and the highest value measured in a table. It says nothing about which combinations of proportions go together.

**Comparison case**:
A character whose accepted armature is kept out of every table, so that what the tables allow can be tested against it. Saffi is the first.

**Style family**:
A manner of drawing with its own proportions, named by a letter and a plain description, never after a work, studio or artist.

### Judging

**Look**:
One review sheet Jon is asked to judge. Accepted, rejected and inconclusive looks all count.
_Avoid_: round, review

**Review sheet**:
The layout shown to Jon at a look. An armature review sheet shows generated views; a reference check sheet shows references with the points read from them.

**Record of Jon's check**:
What Jon decided at a reference check sheet, written down by an agent from his words: a verdict on each character, a ruling where he overrules a drawing's readers, his marks as off, and the readings he checked, each by the digest of its file. It is kept in the reference folder. A measuring command given it measures only what it accepts, and nothing while it is not complete.
_Avoid_: approval, sign-off

**Mark as off**:
Jon's word that a landmark is not on the right feature in a drawing. It leaves that landmark out of the drawing, which is still measured. A mark is on every reading of the drawing, or on one reader's alone where he rules that one placed it rightly and the other did not: the landmark is then measured from the other reader's point. A proportion can be marked as off for a character too, and is then not given for that work.
_Avoid_: reject (that is for a whole character)

**Difference**:
How far a generated view puts a landmark from where the hand-made view puts it, once the one is laid over the other, as a share of head height. A list of differences judges none of them.
_Avoid_: error (that is a reading's distance from known positions), deviation, mismatch

**Lay over**:
To put a generated view on a hand-made one by one scale and one shift for the whole view, so that their units, eye lines and turn axes coincide. It is fitted to no single landmark.
_Avoid_: align, register, fit

**Overlay**:
An SVG in a generated view's own units that shows a hand-made view as its recorded landmarks, with every difference above 1% numbered, so that it lies exactly over the generated SVG on a review sheet. It is the lay-over undone, not a second lay-over.

**Read-back**:
What VectorCraft holds of an armature's SVG after opening it, compared with the view's landmark file: names, the anchors of each guide, visibility and paint. It checks that VectorCraft keeps what the landmark file says, not that the landmark file is right.
_Avoid_: import test, round trip
