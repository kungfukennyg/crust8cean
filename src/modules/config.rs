use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub debug: bool,
    pub render_screen: bool,
    pub initial_color: u32,
    pub play_sound: bool,
}

impl Config {
    pub fn new(config_name: &str) -> Self {
        let mut config = config::Config::default();
        config.merge(config::File::with_name(config_name)).unwrap();
        let config = config.try_into::<HashMap<String, String>>()
                .unwrap();

        let debug = read_value("debug", false, &config)
            .expect("debug should be one of: true/false");
        let render_screen = read_value("render_screen", true, &config)
            .expect("render_screen should be one of: true/false");
        let initial_color = read_value("initial_color", 0xFFFF_FFFF, &config)
            .expect("initial_color should be a 32bit number");
        let play_sound = read_value("play_sound", true, &config)
            .expect("play_sound should be one of: true/false");

        Config {
            debug,
            render_screen,
            initial_color,
            play_sound
        }
    }
}

fn read_value<T: std::str::FromStr>(name: &str, default: T, config: &HashMap<String, String>) -> Result<T, T::Err> {
    return config.get(name)
        .map_or(Ok(default), |v| v.parse::<T>());
}