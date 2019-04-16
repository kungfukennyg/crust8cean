## crust8cean

This is yet another™ CHIP-8 emulator built in Rust. I built this to practice both my Rust skills and my low-ish level programming skills. 

# Usage

After cloning this repo run:
```
cargo run -- /path/to/rom
```

Public domain roms can be found [here](https://github.com/dmatlack/chip8/tree/master/roms/games).

## TODO
- Scale graphics up, at the moment it renders at exactly the CHIP-8's resolution, 64x32. 
- Finish keyboard input
- implement sound/delay timers
- Log all the things! better debug logging, display state of memory, graphics memory, sound/delay timer states
- toy with rendering methods/filters ala VBA (visual boy advance)
- implement sound when sound timer == 0
- make more things configurable (fps?, render mode?)
- add menus at top of window
