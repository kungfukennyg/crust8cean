use minifb::{Window, Key, KeyRepeat};
use crate::modules::display::MiniFbDisplay;

const KEYBOARD_SIZE: usize = 16;

pub struct Keymap {
    keys_pressed: [bool; KEYBOARD_SIZE],
    awaiting_keypress: bool,
    awaiting_keypress_register: usize,
}

impl Keymap {
    pub fn new() -> Self {
        Keymap {
            keys_pressed: [false; KEYBOARD_SIZE],
            awaiting_keypress: false,
            awaiting_keypress_register: 0,
        }
    }

    pub fn update(&mut self, display: &mut MiniFbDisplay, window: &Window) {
        let keys_pressed: Option<Vec<Option<i32>>> = window.get_keys_pressed(KeyRepeat::Yes)
            .map(|keys| {
                keys.into_iter().map(|key| {
                    let key_pressed: Option<i32> = match key {
                        // chip-8 16 key keypad
                        // 1 2 3 4
                        // Q W E R
                        // A S D D
                        // Z X C V
                        Key::Key1 => Some(0),
                        Key::Key2 => Some(1),
                        Key::Key3 => Some(2),
                        Key::Key4 => Some(3),
                        Key::Q => Some(4),
                        Key::W => Some(5),
                        Key::E => Some(6),
                        Key::R => Some(7),
                        Key::A => Some(8),
                        Key::S => Some(9),
                        Key::D => Some(10),
                        Key::F => Some(11),
                        Key::Z => Some(12),
                        Key::X => Some(13),
                        Key::C => Some(14),
                        Key::V => Some(15),

                        // interpreter specific keys
                        // cycle color
                        Key::P => {
                            display.change_color();
                            None
                        }
                        _ => None
                    };
                    key_pressed
                }).collect::<Vec<Option<i32>>>()
            });

        if keys_pressed.is_some() {
            let keys_pressed = keys_pressed.unwrap();
            for (i, key) in self.keys_pressed.iter_mut().enumerate() {
                let pressed = keys_pressed.contains(&Some(i as i32));
                *key = pressed;
            }
        }
    }

    pub fn is_key_pressed(&self, value: usize) -> bool {
        self.keys_pressed[value]
    }

    pub fn set_awaiting_keypress(&mut self, value: bool) {
        self.awaiting_keypress = value;
    }

    pub fn set_awaiting_keypress_register(&mut self, value: usize) {
        self.awaiting_keypress_register = value;
    }

    pub fn get_awaiting_keypress_register(&self) -> usize {
        self.awaiting_keypress_register
    }

    pub fn is_awaiting_keypress(&self) -> bool {
        self.awaiting_keypress
    }

    pub fn await_keypress(&mut self) -> Option<u8> {
        for (i, key) in self.keys_pressed.iter_mut().enumerate() {
            if *key {
                self.awaiting_keypress = false;
                return Some(i as u8);
            }
        }
        None
    }

    pub fn map_keys_pressed_to_real_values(&self) -> Vec<Key> {
        let mut keys = Vec::new();
        for (i, key) in self.keys_pressed.iter().enumerate() {
            if *key {
                let pressed = match i {
                    0 => Some(Key::Key1),
                    1 => Some(Key::Key2),
                    2 => Some(Key::Key3),
                    3 => Some(Key::Key4),
                    4 => Some(Key::Q),
                    5 => Some(Key::W),
                    6 => Some(Key::E),
                    7 => Some(Key::R),
                    8 => Some(Key::A),
                    9 => Some(Key::S),
                    10 => Some(Key::D),
                    11 => Some(Key::F),
                    12 => Some(Key::Z),
                    13 => Some(Key::X),
                    14 => Some(Key::C),
                    15 => Some(Key::V),
                    _ => None
                };

                if pressed.is_some() {
                    keys.push(pressed.unwrap());
                }
            }
        }

        keys
    }
}