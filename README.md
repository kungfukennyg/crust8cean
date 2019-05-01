## crust8cean

This is yet another™ CHIP-8 emulator built in Rust. I built this to practice both my Rust skills and my low-ish level programming skills. 

# Usage

After cloning this repo run:
```
cargo build --release && ./target/crust8cean /path/to/rom
```

Public domain roms can be found [here](https://github.com/dmatlack/chip8/tree/master/roms/games).

# Controls

Controls are rom specific. Note that keypresses are likely to be dropped occasionally. This is a limitation of the original CHIP-8 interpreter, as keys are only checked periodically (when an actual instruction is called that checks them) and wiped each frame. I may end up implementing a "sticky" key feature to make games more playable. 

Keymap layout: 

1	2	3	C
4	5	6	D
7	8	9	E
A	0	B	F

Physical keyboard layout:

1	2	3	4
Q	W	E	R
A	S	D	F
Z	X	C	V

Emulator specific keys:
- ESC -> exit the emulator
- P -> swap the color of pixels, 12 colors total

## TODO
- Scale graphics up, at the moment it renders at exactly the CHIP-8's resolution, 64x32 ✔️
- Finish keyboard input ✔️
- implement sound/delay timers ✔️
- Log all the things! better debug logging, display state of memory, graphics memory, sound/delay timer states ✔️
- toy with rendering methods/filters ala VBA (visual boy advance)
- implement sound when sound timer == 0
- make more things configurable (fps?, render mode?)
- add menus at top of window
- separate non-chip8 logic into distinct modules ✔️
- log to a file
- record stats (# of instructions executed, most expensive operations, etc.)
