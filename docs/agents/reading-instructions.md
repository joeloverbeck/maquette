# Reading a drawing

Revision: 2026-11-17.4

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
- An end, a corner or a tip is the last point of the shape in that direction. Where a band, such as a brow or a lid line, ends in a cut across its width, its end is the middle of the cut.
- A top, a bottom or a side extent is the point that reaches furthest that way. Where the shape runs level along its top or its bottom, or upright along its side, it is the middle of that run.
- **A landmark is a point of the feature itself.** Where another shape lies over part of a feature (hair over the face, an ear or a brow; the head over the far ear), the place where that shape's edge crosses the feature is not a landmark. An end, a corner or an extent that is covered is not seen, however near to it the feature comes back into view. A point that shows through a gap in the hair is seen, and is placed.
- Place what is drawn. A point under the hair, behind the head or outside the picture is not seen: say which. Do not put a point where it ought to be, mirror it from the other side, or carry it over from another view.

## The three levels

Three heights are worked out from landmarks, and the face outline is read where it crosses each:

- **The eye line** is midway between the top and the bottom of the eye opening. Where a view shows two eyes, it is the mean of the two.
- **The nose level** is midway between the nose mark's corners and its bottom. Where the mark has no corners, it is the height of the mark's middle.
- **The mouth level** is the height of the mouth line's centre.

Place the eyes, the nose mark and the mouth line first, work out the three heights, and then read the face outline at them.

A drawing may show construction lines of its own, as an underdrawing does: a circle for the skull, level lines, a centre line. They are not read, and a drawn level line is not the level: work each level out from the features.

## The face outline

The face outline is the edge of the face itself: one unbroken curve from the temple down the cheek and the jaw to the chin. Two other edges are easily taken for it.

- **The edge of hair that lies over the face.** Follow the outline up from the chin. Where hair crosses it, the edge beside the skin from there on is the hair's: it runs on straight past the face, or ends in the point of a lock, where the face's edge would curve with the cheek. A level that crosses a hair edge has no crossing with the face outline: that landmark is not seen.
- **A construction line that runs beside the outline.** A drawn circle or guide may lie within a few pixels of the outline for a stretch. The outline is the edge that continues the curve of the cheek from below. Read that edge, and where the two cannot be told apart, state the landmark as not seen.

## The landmarks

`<side>` is `left` or `right`. A front view and a three-quarter view ask for both sides; a side view asks for the near side only.

| Landmark | What to place |
|---|---|
| `hair-top` | The highest point of the hair. Hair only: an ear or an ornament that stands higher does not count. |
| `hair-<side>-extent` | The point of the hair furthest out on that side. |
| `chin` | The lowest point of the face outline. |
| `face-outline-<side>-top` | The temple: the top end of the face outline on that side, where the outline meets the hairline. It is seen only where that end itself shows, as with hair drawn back from the face. Where hair hangs over the outline, the outline runs up under it and its top end is not seen: the place where it goes under the hair is not the temple. |
| `face-outline-<side>-at-eye-level` | Where the face outline on that side crosses the eye line. |
| `face-outline-<side>-at-nose-level` | Where it crosses the nose level. |
| `face-outline-<side>-at-mouth-level` | Where it crosses the mouth level. |
| `face-outline-<side>-jaw` | The jaw point: where the jaw turns into the chin. Below the cheek the outline may turn twice: first where the cheek turns into the slanted run of the jaw, and then, close to the chin, where that run turns into the chin's nearly level bottom. The jaw point is the second, lower turn: the last turn before the chin's lowest point. Where the outline is one even curve from the mouth level to the chin, with no place where it bends most, the jaw point is not seen. |
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
| `mouth-line-centre` | The point of the mouth line where the centre line of the face crosses it. In a front view it is midway between the two ends. In a turned view the centre line is not drawn: take it as the straight line from the nose mark's bottom to the chin, and place the point where the mouth line crosses that line. |
| `ear-<side>-base-outer`, `ear-<side>-base-inner` | The ear's base is the stretch along which it stands on the head, and its two base points are the ends of that stretch: the two corners of the ear's foot, where each side of the ear's own outline ends at the head. On an ear set on top of the head they lie side by side: the outer one is further from the centre line. On an ear at the side of the head, as a human ear is, they lie one above the other: the upper one is `base-outer` and the lower one `base-inner`. Where hair lies over the ear, a base point is the corner of the ear and never the place where the hair's edge crosses the ear: a corner that shows through the hair is placed, and one the hair covers is not seen. |
| `ear-<side>-top` | The highest point of the ear. |
| `ear-<side>-outer-extent`, `ear-<side>-inner-extent` | The points of the ear furthest from and nearest the centre line. |
| `neck-<side>-base` | Where the neck ends on that side. Going down the edge of the neck, it is the point where the edge leaves its straight run and begins to curve outward into the shoulder: the start of that curve, not its middle. |
| `shoulder-<side>-point` | The outer end of the shoulder on that side. Going outward along the top of the shoulder, it is the point where the outline stops running outward and starts to run down the arm: on a rounded shoulder, the point of the round furthest to that side. Where the picture ends before the shoulder does, it is not seen. |

**The skull is not read.** It lies under the hair, so maquette builds it by rule from the ears and the temples: the ear's two base points and `face-outline-<side>-top`. Read those with particular care, and never guess one that the hair covers.

Also not read, because maquette builds them: the middle of an eye opening, the centre of an iris, the pupil, the centre line and the level lines.

### In a three-quarter view

- Both sides are asked for. What the head, the nose or the hair hides on the far side is not seen.
- The far side's outline is the cheek. The near side's outline is the side of the jaw, and it is asked for only at the nose level and the mouth level, and at the jaw point. It keeps the name `face-outline-<side>-…`.
- An ear behind the head is placed only as far as it shows. A base point or an extent that the head or the hair covers is not seen: where the ear's outline goes out of view is neither.

### In a side view

A side view is a profile. Several landmarks are another point of the drawing there than the table above says for a face seen from the front. Where a sentence of this part differs from the table or from "Positions", this part is the one that holds in a side view.

**What is asked for.**

- Only the near side is asked for, under that side's name. The reading file lists every landmark a side view asks for.
- A side view asks for no temple, no jaw point, no crossing of the face outline with the eye line on the side of the face, no `hair-<side>-extent` and no `neck-<side>-base`. Do not place them, and do not state them as not seen.
- A side view asks for points of the profile that no other view does: the forehead, the bridge of the nose, the jaw's angle and the top of the neck's two lines ("The forehead, the bridge, the jaw's angle and the neck" below).
- Inner and outer: in every name of a side view, inner is toward the front of the face and outer is toward the back of the head. That holds for the eye's corners, the iris's extents, the lid line's and the brow line's ends, and the ear's extents.

**The front of the face.** It is read through three points of the outline: `nose-mark-bottom`, `mouth-line-centre` and `chin`. Each is how far forward the face stands at its height, so each is placed on the turn of the outline and never on a level run behind it.

- `chin` is the chin's own lowest point. Going down the front of the face, it is the point where the outline stops running down and turns to run back under the jaw. Where the underside of the jaw then runs level or almost level toward the neck, the chin is the front end of that run, at the turn, and not its middle: the sentence about the middle of a run does not hold for the chin in a side view. A point on the underside of the jaw is not the chin.
- `nose-mark-bottom` is under the tip of the nose. The nose mark in a side view is the underside of the nose: the edge that runs from under the tip back to where the nose meets the upper lip. `nose-mark-bottom` is the front end of that edge: going down the bridge of the nose and round its tip, the point where the outline turns to run back toward the face. On a rounded tip it is where the turn is sharpest. It is not where the nose meets the upper lip, which lies as far behind as the nose stands out.
- `nose-mark-<side>-corner` is the middle of the nostril, or of the curve drawn for the wing of the nose. Where neither is drawn it is not seen. A side view asks for the near corner only.
- The nose level in a side view is midway in height between that corner and `nose-mark-bottom`. Where the corner is not seen, it is the height of `nose-mark-bottom`.
- `mouth-line-centre` is where the line between the lips reaches the outline of the face. Where the outline has a notch between the two lips, it is the bottom of that notch, and it stays there where the upper lip overhangs the lower. Where the drawn mouth line stops short of the outline, it is the point of the outline level with the line's front end.
- `mouth-line-<side>-end` is the back end of the drawn mouth line: the corner of the mouth.

**The eye.** It is seen from its side, usually as a wedge that opens toward the front of the face.

- `eye-<side>-inner-corner` is the front end of the opening: its point furthest toward the front of the face. Where the front of the eye is drawn as a contour of its own, or as the front edge of the iris, that contour is the front of the opening.
- `eye-<side>-outer-corner` is the back end of the opening: where the upper lid and the lower lid meet. Where no lower lid is drawn and the opening is not closed, it is the back end of what is drawn of the opening: of the white, or of the lower edge of the eye.
- `eye-<side>-opening-bottom`: where no lower lid is drawn, it is the lowest point of the iris or of the white as drawn.
- `iris-<side>-inner-extent` is the iris's front edge and `iris-<side>-outer-extent` its back edge. Where the iris itself is the front of the eye, its inner extent and the eye's inner corner are one point: place both there.

**The ear.**

- An ear at the side of the head, as a human ear is, stands on the head along its front edge in a side view. `ear-<side>-base-outer` is the upper end of that edge: where the rim of the ear, coming forward over its top, meets the head. `ear-<side>-base-inner` is the lower end: where the lobe meets the head.
- On an ear set on top of the head, the two corners of its foot lie one before the other. The one toward the front of the head is `base-inner` and the one toward the back is `base-outer`.
- `ear-<side>-inner-extent` is the ear's point furthest toward the front of the face, and `ear-<side>-outer-extent` its point furthest toward the back of the head.
- Hair that runs down in front of the ear covers its base as in any view: a corner the hair covers is not seen.

**The jaw, the hair and the neck.**

- `face-outline-<side>-at-nose-level` and `face-outline-<side>-at-mouth-level` are where the back edge of the jaw crosses those levels: the edge that runs from under the chin back and up toward the ear. Where it is drawn as the edge of the shadow under the jaw and not as a line, that edge is read. Where hair covers it, it is not seen.
- `hair-back-extent` and `hair-front-extent` are the points of the hair furthest toward the back and toward the front of the head, read at or above the height of the chin. Hair that hangs below the chin is not counted, whatever it is tied into. Where the hair runs out of the picture on that side, the extent is not seen.
- `neck-back-base` and `neck-front-base` are where the back and the front of the neck end: where each edge turns into the shoulder or the chest.
- Head height is a difference in height only. In a side view the hair's top and the chin stand on different uprights, and how far apart they are sideways does not count.

**The forehead, the bridge, the jaw's angle and the neck.** These are points of the profile that a drawn side view needs and no other view shows. Place the eye and the brow first, and work out the eye line from the near eye.

- `centre-line-at-brow-level` is the forehead: where the front outline of the face crosses the height of `brow-line-<side>-inner-end`, the brow's front end. Where the brow is not seen, or hair or anything else lies over the front outline at that height, it is not seen.
- `centre-line-at-eye-level` is the bridge of the nose: where the front outline of the face crosses the eye line. Where hair or the rim of a pair of glasses lies over the outline at that height, it is not seen.
- `jaw-<side>-angle` is the angle of the jaw. Going back along the underside of the jaw from the chin, it is the point where the outline turns to run up toward the ear; on a rounded angle, where the turn is sharpest. Where the run up toward the ear is drawn as the edge of a shadow and not as a line, that edge is read, as for the back edge of the jaw. Where the underside runs on into the neck and no turn up toward the ear is drawn, or hair or a collar covers the turn, it is not seen. It is not the chin, and not where the neck's front line meets the jaw.
- `neck-front-top` is the top of the front of the neck: going up the neck's front edge, where it meets the underside of the jaw. Where the two meet in a corner, it is the corner; where the edge curves into the jaw, it is where the curve is sharpest.
- `neck-back-top` is the top of the back of the neck: going up the neck's back edge, where it meets the head. Where hair covers that end, it is not seen: the place where the hair's edge crosses the neck is not it.

This part was revised at revision 2026-11-17.3, after the first published side drawings were read, and at revision 2026-11-17.4, after look 6, to ask for the forehead, the bridge, the jaw's angle and the neck's top ends. Each revision is tested on a side drawing whose positions are known before a published drawing is read under it.

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

**In a side view.** The signs of level above are written for a face seen from the front. In a side view, use these.

- The lid line and the mouth line run level, or rise a little toward the back of the head.
- A human ear spans from about the brow to about the base of the nose, as in any view.
- The underside of the jaw rises from the chin toward the neck.
- Signs that it is not level: the jaw's underside runs level or falls toward the neck and the ear stands high (tilted down); the jaw's underside rises steeply, the nose points up and the ear stands low (tilted up).
- A lean toward the viewer or away does not show in a side view. Say so in `level_why` rather than claim it.
- A side view shows nothing of the far side of the face. Where the far brow, the far eye's lashes or the far cheek shows beyond the front of the face, the head is not in full profile: call it a three-quarter view, or set it aside.

**Set aside.** Set the drawing aside when the head is not level, when it is in none of the three views, when the expression pulls the features out of place, or when too little of the head shows to read. Write `set_aside = true` and the reason, and place nothing. When in doubt, set it aside and say what the doubt is: a tilted head measured as a level one spoils a table, and Jon can overrule a set-aside in a moment.

**A set-aside that was overruled.** The person who checks the readings can overrule a set-aside and have the drawing read. You are then told so with the drawing.

- Write `set_aside_overruled = true` under your judgement, and in `set_aside_overruled_by` who overruled it, as you were told.
- Judge the drawing as ever. Where you would have set it aside yourself, still write `set_aside = true` and your reason: your judgement stands beside the ruling and does not undo it.
- Then place every landmark that is drawn, and state the rest as not seen, as for any other drawing. "Place nothing" does not hold for an overruled drawing, and nothing is left unplaced because of the reason the drawing was set aside for.
- Where a feature is drawn in a way the table does not cover, as an open mouth has no one line, place what the table's words fit best and say in `aids` what you took for it, or state it as not seen with that reason.

## Enlarging

A head that is small in the picture is read less accurately. Readings were tested on heads about 480 pixels high from hair top to chin. Where the head is smaller than that, or a feature is too fine to place, enlarge the part you are reading:

`maquette enlarge-image --image <picture> --left <x> --top <y> --width <w> --height <h> --factor <n> [--grid <step>] --out <enlargement>`

- `--left`, `--top`, `--width` and `--height` are the part to enlarge, in pixels of the picture. `--factor` is a whole number: each pixel becomes a block that many pixels across.
- `--grid 10` draws a line where every tenth pixel of the picture begins and labels it with its position in the picture, across the top and down the left side. Read a position off the labels: a point a third of the way from the line labelled 410 to the line labelled 420 is at 413.3.
- Keep the grid's lines at least 50 pixels apart in the enlargement, or they crowd the drawing: `--grid 10` from a factor of 5 up, `--grid 20` at a factor of 3 or 4.
- An enlargement may be at most 1568 pixels on an edge, because a larger picture is shown to you resized. The part's width times the factor, and its height times the factor, must each stay under about 1500: at most 300 pixels of the picture at a factor of 5, 375 at 4, 500 at 3. Enlarge a smaller part instead: one eye, the nose and mouth, an ear.
- Without a grid, the command prints how a point in the enlargement is taken back to the picture.
- Look at the whole picture as well. The judgement, and which feature is which, are read from the whole.

An enlargement is a working copy. Write it beside your reading, and say in `aids` which parts you enlarged.
