# Penplot

An assembly-like vector graphics toy, inspired by [Atari arcade vector hardware](https://www.youtube.com/watch?v=smStEPSRKBs).

## Building / Usage

Build with `cargo build`. The program takes two command line arguments. The first is the
filename of the code you want to run; the second is the filename you want the resulting image to be
saved to.

## Instruction Set

| Opcode         | Description                                                  |
|----------------|--------------------------------------------------------------|
| `NOOP`         | Do nothing.                                                  |
| `MOVE x y`     | Move the pen head to (x, y).                                 |
| `SHFT dx dy`   | Move the pen head over by (dx, dy).                          |
| `WALK d`       | Move the pen head forward d pixels.                          |
| `FACE t`       | Set current heading to t degrees.                            |
| `TURN t`       | Turn counterclockwise t degrees.                             |
| `RGBA r g b a` | Set current pen color to (r, g, b, a).                       |
| `RGB r g b`   | Set current pen color to (r, g, b, 255).                      |
| `BLNK`         | Set current pen color to (0, 0, 0, 0).                       |
| `BLOT`         | Set current pixel to pen color.                              |
| `GOTO add`     | Go to specified address.                                     |
| `JUMP n`       | Jump ahead n instructions.                                   |
| `CALL add`     | Call the subroutine at specified address.                    |
| `RTRN`         | Return from subroutine. Does nothing if not in a subroutine. |
| `LOOP add n`   | Repeat subroutine at specified address n times.              |
| `HALT`         | Finish executing.                                            |
| `; text`       | Comment. This is its own instruction for L-system purposes.  |

## Labels

A line can be followed by `@ text`, where `text` becomes the label for that line. Any address can be
replaced with a label. For example, the following code loops infinitely:

```
NOOP @ start
GOTO start
```
