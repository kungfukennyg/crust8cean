use minifb::Window;

const SCREEN_WIDTH: u16 = 64;
const SCREEN_HEIGHT: u16 = 32;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;

// sprites
pub const BYTES_PER_CHARACTER: u8 = 5;
pub const NUM_FONT_CHARACTERS: u8 = 16;
pub const FONT_ARRAY_SIZE: usize = (BYTES_PER_CHARACTER * NUM_FONT_CHARACTERS) as usize;

pub const FONT_SPRITES: [u8; FONT_ARRAY_SIZE] = [
    // 0
    0b11110000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11110000,
    // 1
    0b00100000,
    0b01100000,
    0b00100000,
    0b00100000,
    0b01110000,
    // 2
    0b11110000,
    0b00010000,
    0b11110000,
    0b10000000,
    0b11110000,
    // 3
    0b11110000,
    0b00010000,
    0b11110000,
    0b00010000,
    0b11110000,
    // 4
    0b10100000,
    0b10100000,
    0b11100000,
    0b00100000,
    0b00100000,
    // 5
    0b11110000,
    0b10000000,
    0b11110000,
    0b00010000,
    0b11110000,
    // 6
    0b11110000,
    0b10000000,
    0b11110000,
    0b10010000,
    0b11110000,
    // 7
    0b11110000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b01000000,
    // 8
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b11110000,
    // 9
    0b11110000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b11110000,
    // A
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b10010000,
    // B
    0b11100000,
    0b10000000,
    0b11100000,
    0b10010000,
    0b11100000,
    // C
    0b11110000,
    0b10000000,
    0b10000000,
    0b10000000,
    0b11110000,
    // D
    0b11100000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11100000,
    // E
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b11110000,
    // F
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b10000000];

pub const COLORS: [u32; 14] = [
                            // white
                            0xFFFFFF,
                            // red
                            0xFF0000,
                            // orange
                            0xFF8000,
                            // yellow
                            0xFFFF00,
                            // light green
                            0x80FF00,
                            // green
                            0x00FF00,
                            // dark green
                            0x00FF80,
                            // teal
                            0x00FFFF,
                            // blue
                            0x0080FF,
                            // dark blue
                            0x0000FF,
                            // purple
                            0x7F00FF,
                            // pink
                            0xFF00FF,
                            // magenta
                            0xFF007F,
                            // gray
                            0x808080
                        ];

pub struct MiniFbDisplay {
    color: u32,
    redraw: bool,
    screen: [u8; SCREEN_SIZE],
    times_rendered: u64,
}

impl MiniFbDisplay {
    pub fn new() -> Self {
        MiniFbDisplay {
            color: COLORS[0],
            redraw: false,
            screen: [0; SCREEN_SIZE],
            times_rendered: 0,
        }
    }

    pub fn read(&self, pos: usize) -> u8 {
        self.screen[pos]
    }

    pub fn write(&mut self, pos: usize, value: u8) {
        self.screen[pos] = value;
    }

    pub fn set_redraw(&mut self, redraw: bool) {
        self.redraw = redraw;
    }

    pub fn change_color(&mut self) {
        for (i, color) in COLORS.iter().enumerate() {
            if self.color == *color {
                let index = if i + 1 < COLORS.len() {
                    i + 1
                } else {
                    0
                };

                self.color = COLORS[index];
                break;
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.screen.iter_mut() {
            if *pixel != 0 {
                self.redraw = true;
                *pixel = 0;
            }
        }
    }

    pub fn render(&mut self, window: &mut Window) {
        if !self.redraw {
            return;
        }

        let mut buf: [u32; SCREEN_SIZE] = [0; SCREEN_SIZE];
        for (i, pixel) in self.screen.iter_mut().enumerate() {
            if *pixel != 0 {
                buf[i] = self.color;
            }
        }

        self.redraw = false;
        self.times_rendered += 1;
        // unwrap, we want to know if this fails
        window.update_with_buffer(&buf).unwrap();
    }

    pub fn get_times_rendered(&self) -> u64 {
        self.times_rendered
    }

}