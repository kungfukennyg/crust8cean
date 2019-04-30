use minifb::Window;

const SCREEN_WIDTH: u16 = 64;
const SCREEN_HEIGHT: u16 = 32;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;

pub struct MiniFbDisplay {
    color: u32,
    redraw: bool,
    screen: [u8; SCREEN_SIZE],
    times_rendered: u64,
}

impl MiniFbDisplay {
    pub fn new() -> Self {
        MiniFbDisplay {
            color: 0x0000_FFFF,
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
        window.update_with_buffer(&buf);
    }

    pub fn get_times_rendered(&self) -> u64 {
        self.times_rendered
    }

}