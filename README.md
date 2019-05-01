## crust8cean

![Screenshot](/screenshot_1.png?raw=true "The emulator running Brix'")

This is yet another™ CHIP-8 emulator built in Rust. I built this to practice both my Rust skills and my low-ish level programming skills. 

# Building
crust8cean uses the [ears](https://crates.io/crates/ears) crate to play audio. You need to install OpenAL and libsndfile on your system

Linux (Debian and Ubuntu):
```sudo apt install libopenal-dev libsndfile1-dev```

Linux (Fedora):
```sudo dnf install openal-soft-devel libsndfile-devel```

Mac:
```brew install openal-soft libsndfile```

Windows:
Install [MSYS2](http://www.msys2.org/) according to the instructions. Be sure to use the default installation folder (i.e. C:\msys32 or C:\msys64), otherwise compiling won't work. Then, run the following in the MSYS2 shell:
```pacman -S mingw-w64-x86_64-libsndfile mingw-w64-x86_64-openal```

Then finally, run 
```cargo build --release```

# Usage

After cloning and building this repo run:
```
./target/crust8cean /path/to/rom
```

Public domain roms can be found in the roms/

# Controls

Controls are rom specific. Note that keypresses are likely to be dropped occasionally. This is a limitation of the original CHIP-8 interpreter, as keys are only checked periodically (when an actual instruction is called that checks them) and wiped each frame. I may end up implementing a "sticky" key feature to make games more playable. 

Keymap layout: 

1	2	3	C<br>
4	5	6	D<br>
7	8	9	E<br>
A	0	B	F<br>

Physical keyboard layout:

1	2	3	4<br>
Q	W	E	R<br>
A	S	D	F<br>
Z	X	C	V<br>

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
