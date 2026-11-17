# Reading a drawing

Revision: 2026-11-17.1

A reading is one reader's placement of the landmarks on one drawing of a head, with the reader's judgement of the view, the turn and whether the head is level. Maquette takes proportions from readings. Every drawing is read twice, by two readers who do not see each other's reading, and Jon checks the result. You are one of the two readers.

You work from the picture you were given and from nothing else: no other copy of the drawing, no vector file it was made from, no record of its positions, no earlier reading, and nothing you remember about the character. What you cannot see in the picture, you state as not seen.

`maquette reading-instructions` prints this file. When it changes, its revision changes, and readings are tested again on drawings whose positions are known.

## Steps

Each drawing is one reading file. The paths you give `maquette` are taken from the reference folder.

1. **Look at the whole picture as it is.** Find the head, and note roughly where its hair top and its chin are.
2. **Judge the drawing before you place anything:** its view, which way the head faces, how far it is turned, and whether it is level ("Judging the drawing" below). Each judgement needs its reason: what in the drawing shows it.
3. **Start the reading** in the view you judged:
   `maquette start-reading --image <picture> --view front|three-quarter|side [--faces right|left] --reader <your label> --out <reading file>`
   `--faces` is for a three-quarter or a side view: the way the head faces, from the viewer's side. The file it writes holds the picture's digest and size, the questions of the judgement, and one line for every landmark the view asks for.
4. **Write the judgement into the file,** with its reasons. If the drawing is to be set aside, say why, place nothing, and go to step 7.
5. **Place every landmark the file lists, or state it as not seen.** Enlarge wherever it helps ("Enlarging" below). For each landmark, remove the `# ` before its line and give its position, or add it under `[not_seen]` with the reason. Work feature by feature, and place both sides of a pair from what each side shows.
6. **Fill in `manner` and `aids`:** the manner the drawing is drawn in, in a few plain words and never a name, and how you looked at it: as it is, or which parts you enlarged and by how much.
7. **Check the reading:** `maquette check-reading --reading <reading file>`. It says what is missing or wrong. Correct the file and run it again until it says the reading is complete.

You are done when every drawing you were given has a reading that passes its check.

## Positions

- A position is in pixels of the picture itself: `x` from its left edge, `y` from its top edge. Give decimals where you can tell, such as `x = 412.5`.
- A position read off an enlargement is taken back to the picture before it is written. A grid's labels are already the picture's positions.
- Left and right in a name are the character's own. Seen from the front, the character's left is toward the viewer's right. In a turned view the near side is the character's right when the head faces the viewer's right, and the character's left when it faces the viewer's left.
- A landmark on a drawn line is on the middle of the line's thickness. A landmark on the edge of a filled shape is on the edge.
- An end, a corner or a tip is the last point of the shape in that direction. A top, a bottom or a side extent is the point that reaches furthest that way.
- Place what is drawn. A point under the hair, behind the head or outside the picture is not seen: say which. Do not put a point where it ought to be, mirror it from the other side, or carry it over from another view.

## The three levels

Three heights are worked out from landmarks, and the face outline is read where it crosses each:

- **The eye line** is midway between the top and the bottom of the eye opening. Where a view shows two eyes, it is the mean of the two.
- **The nose level** is midway between the nose mark's corners and its bottom. Where the mark has no corners, it is the height of the mark's middle.
- **The mouth level** is the height of the mouth line's centre.

Place the eyes, the nose mark and the mouth line first, work out the three heights, and then read the face outline at them.

A drawing may show construction lines of its own, as an underdrawing does: a circle for the skull, level lines, a centre line. They are not read, and a drawn level line is not the level: work each level out from the features.

## The landmarks

`<side>` is `left` or `right`. A front view and a three-quarter view ask for both sides; a side view asks for the near side only.

| Landmark | What to place |
|---|---|
| `hair-top` | The highest point of the hair. Hair only: an ear or an ornament that stands higher does not count. |
| `hair-<side>-extent` | The point of the hair furthest out on that side. |
| `chin` | The lowest point of the face outline. |
| `face-outline-<side>-top` | The temple: the top end of the face outline on that side, where the hair leaves it showing. Where the outline runs up under the hair with no visible end, it is not seen. |
| `face-outline-<side>-at-eye-level` | Where the face outline on that side crosses the eye line. |
| `face-outline-<side>-at-nose-level` | Where it crosses the nose level. |
| `face-outline-<side>-at-mouth-level` | Where it crosses the mouth level. |
| `face-outline-<side>-jaw` | The jaw point: between the mouth level and the chin, where the outline turns most sharply from the side of the jaw into the chin. |
| `eye-<side>-outer-corner` | The point of the eye opening furthest from the centre line. The opening is the shape the lids leave open, not the lashes or the lid line. |
| `eye-<side>-inner-corner` | The point of the eye opening nearest the centre line. |
| `eye-<side>-opening-top` | The highest point of the eye opening, under the upper lid. |
| `eye-<side>-opening-bottom` | The lowest point of the eye opening. |
| `iris-<side>-top`, `iris-<side>-bottom` | The highest and the lowest point of the iris as drawn. Where a lid cuts the iris, it is where the lid cuts it. |
| `iris-<side>-outer-extent`, `iris-<side>-inner-extent` | The points of the iris furthest from and nearest the centre line. |
| `lid-line-<side>-inner-end` | The upper lid line is the dark line or band along the upper lid. This is the middle of its thickness at its end nearest the centre line. |
| `lid-line-<side>-crest` | The middle of its thickness where the line is highest. |
| `lid-line-<side>-outer-end` | The middle of its thickness at its end furthest from the centre line. |
| `brow-line-<side>-inner-end`, `brow-line-<side>-outer-end` | The middle of the drawn brow's thickness at its end nearest the centre line, and at its end furthest from it. |
| `nose-mark-<side>-corner` | The nose mark is whatever is drawn for the nose. This is its point furthest toward that side. Where the mark is one dot or one stroke with no width, it has no corners: state both as not seen. |
| `nose-mark-bottom` | The lowest point of the nose mark. |
| `mouth-line-<side>-end` | The end of the mouth line on that side. |
| `mouth-line-centre` | The point of the mouth line where the centre line of the face crosses it. In a front view it is midway between the two ends. |
| `ear-<side>-base-outer`, `ear-<side>-base-inner` | The ear's base is the stretch along which it stands on the head, and its two base points are the ends of that stretch: the two points where the ear's outline meets the head. On an ear set on top of the head they lie side by side: the outer one is further from the centre line. On an ear at the side of the head, as a human ear is, they lie one above the other: the upper one is `base-outer` and the lower one `base-inner`. |
| `ear-<side>-top` | The highest point of the ear. |
| `ear-<side>-outer-extent`, `ear-<side>-inner-extent` | The points of the ear furthest from and nearest the centre line. |
| `neck-<side>-base` | Where the neck ends on that side: where its edge turns into the shoulder. |
| `shoulder-<side>-point` | The outer end of the shoulder on that side, where its top turns down into the arm. |

**The skull is not read.** It lies under the hair, so maquette builds it by rule from the ears and the temples: the ear's two base points and `face-outline-<side>-top`. Read those with particular care, and never guess one that the hair covers.

Also not read, because maquette builds them: the middle of an eye opening, the centre of an iris, the pupil, the centre line and the level lines.

### In a three-quarter view

- Both sides are asked for. What the head, the nose or the hair hides on the far side is not seen.
- The far side's outline is the cheek. The near side's outline is the side of the jaw, and it is asked for only at the nose level and the mouth level, and at the jaw point. It keeps the name `face-outline-<side>-…`.
- An ear behind the head is placed only as far as it shows. A base point the head covers is not seen.

### In a side view

- Only the near side is asked for, under that side's name.
- The inner corner of the eye and the inner extent of the iris are the ones toward the front of the face.
- `face-outline-<side>-at-nose-level` and `face-outline-<side>-at-mouth-level` are where the back edge of the jaw crosses those levels. The front of the face is read through `nose-mark-bottom`, `mouth-line-centre` and `chin`.
- `nose-mark-bottom` is where the underside of the nose meets the face, and `mouth-line-centre` is where the lips meet on the outline of the face.
- `hair-back-extent` and `hair-front-extent` are the points of the hair furthest toward the back and toward the front of the head.
- `neck-back-base` and `neck-front-base` are where the back and the front of the neck end: where each edge turns into the shoulder or the chest.

No drawing with known positions exists in a side view, so this part has not been tested.

## Judging the drawing

State each as you see it, with the reason. Jon checks these judgements; a wrong one costs more than a misplaced point, because every number taken from the drawing rests on it.

**View.** One of three.

- *Front:* the centre line of the face is midway between the two outlines at the eye line, the two eyes are as wide as each other, and both ears or both sides of the hair show equally.
- *Three-quarter:* the head is turned part of the way. The far eye is narrower than the near eye, and the nose, mouth and chin stand toward the far outline.
- *Side:* a full profile. One eye shows, and the nose and the chin are on the outline.

A head turned even a little is not a front view. Call it a three-quarter view, or set it aside.

**Faces.** For a turned head: toward the viewer's right or the viewer's left.

**Turn.** Your own estimate, in degrees, of how far the head is turned from facing the viewer: 0 is a front view and 90 a side view. Say what you judged it from, such as how much narrower the far eye is than the near one. Three-quarter is not one angle: drawings called that run from about 20 to about 50 degrees. Maquette works the turn out from your positions as well, and Jon sees both.

**Level.** A level head is neither tilted up or down nor leaning to a side. Signs that it is level:

- The line through the two eyes is horizontal, and so are the brows and the mouth.
- The eye, nose and mouth levels are straight across the face, not bowed.
- Nothing of the underside of the chin or of the nose shows, and no more of the top of the head than a level head shows.
- A human ear at the side of the head spans from about the brow to about the base of the nose.

Signs that it is not: the levels bow upward or downward; the ears stand above the eye line and the top of the head shows (tilted down), or they stand low and the underside of the chin shows (tilted up); one eye stands higher than the other (leaning). Ears set on top of the head, as an animal's are, say nothing about level: use the other signs.

**Set aside.** Set the drawing aside when the head is not level, when it is in none of the three views, when the expression pulls the features out of place, or when too little of the head shows to read. Write `set_aside = true` and the reason, and place nothing. When in doubt, set it aside and say what the doubt is: a tilted head measured as a level one spoils a table, and Jon can overrule a set-aside in a moment.

## Enlarging

A head that is small in the picture is read less accurately. Readings were tested on heads about 480 pixels high from hair top to chin. Where the head is smaller than that, or a feature is too fine to place, enlarge the part you are reading:

`maquette enlarge-image --image <picture> --left <x> --top <y> --width <w> --height <h> --factor <n> [--grid <step>] --out <enlargement>`

- `--left`, `--top`, `--width` and `--height` are the part to enlarge, in pixels of the picture. `--factor` is a whole number: each pixel becomes a block that many pixels across.
- `--grid 10` draws a line where every tenth pixel of the picture begins and labels it with its position in the picture, across the top and down the left side. Read a position off the labels: a point a third of the way from the line labelled 410 to the line labelled 420 is at 413.3.
- An enlargement may be at most 1568 pixels on an edge, because a larger picture is shown to you resized. Enlarge a smaller part instead: one eye, the nose and mouth, an ear.
- Without a grid, the command prints how a point in the enlargement is taken back to the picture.
- Look at the whole picture as well. The judgement, and which feature is which, are read from the whole.

An enlargement is a working copy. Write it beside your reading, and say in `aids` which parts you enlarged.
