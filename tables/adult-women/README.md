# The first table: adult women

`table.toml` is the first proportion table, built from the first batch of references. **It is a proposal.** Jon has not accepted it: it is put to him at look 5, and the pull request that carries it merges only if he does.

- **What it was measured on:** one adult woman from each of six works by six character designers, as `[[work]]` in the file lists them. A front view of all six, and a side view of three. Jon checked the readings on a reference check sheet at look 4, on 2026-11-17, and accepted all six characters.
- **What it gives:** 27 proportions with a valid range, each with the lowest, the middle and the highest value among the five or six works that give it.
- **What it lists without a number:** 26 proportions that one to four works give, which is too thin; 17 that no work gives; and the skull's 3 conventions, which are never measured. Each has its count of works and why the others do not give it. The values of a few works are those works' own numbers, so none is stated.
- **What it says of itself:** that its ranges are separate and do not rule on which combinations of proportions go together, and that a lowest and a highest value are each one reference's number, tied to no work.

## Why 46 proportions have no range

In the program's words, the file says for each proportion which landmark the works did not place. Read against the readings, the reasons are these.

| Reason | Proportions |
|---|---|
| A side view was needed, and three works have one | 11 depths: the chin, the mouth, the nose mark, the iris, the eyes' wrap, the hair, one depth of the jaw line and one of the ear |
| Hair covers it in the front view | The face outline's top end and its width at the eye line in every front; its width at the nose and mouth levels, the lid line's outer end and the brows in some |
| Hair covers the ears, and Jon marked one work's ears as off | All 12 ear proportions: none has more than two works |
| A side view cannot show it | The depths of the cheek points and of the jaw point: 5 |
| Hair covers it in the side view | The jaw line's depth at the nose level, the neck's depth |
| A collar, the hair or the picture's edge | The neck's width and base; the shoulders, which every drawing cuts off |
| No input for the rule | The skull's 3 conventions: the rule needs the ear's base and the temple |

So depth is not a table, no three-quarter view was measured, and no skull is built.

## How it was made

```sh
maquette table-from-readings --check <the record of Jon's check> --out <table>
```

The command reads the readings Jon accepted, in the reference folder, which no clean clone has. This file is what it wrote on 2026-11-17, copied here unchanged. Where each picture was taken from is in its readings and not here.

`cargo test` checks what can be checked without the readings (`tests/first_table.rs`): that a number is stated only where five works give a range, that the works are listed by work and designer and nothing else, and that Saffi's set against it is inside 19 ranges and outside 8. Saffi is the comparison case, and none of her numbers is in the table.

`maquette set-against-table --proportions <set> --table tables/adult-women/table.toml` sets any set of proportions against it.
