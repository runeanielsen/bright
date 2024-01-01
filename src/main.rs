#![warn(clippy::all, clippy::pedantic)]
use std::{error::Error, fs, path::PathBuf};

#[derive(Debug)]
struct LightDevice {
    // Name of the device.
    pub name: String,
    // Current device brightness.
    pub brightness: i64,
    // Maximum brightness for device.
    pub max_brightness: i64,
}

impl LightDevice {
    pub fn new(name: &str, brightness: i64, max_brightness: i64) -> Self {
        Self {
            name: name.to_string(),
            brightness,
            max_brightness,
        }
    }
}

fn load_light_devices() -> Result<Vec<LightDevice>, Box<dyn Error>> {
    let leds = fs::read_dir("/sys/class/leds")?;
    let backlights = fs::read_dir("/sys/class/backlight")?;

    Ok(leds
        .chain(backlights)
        .map(|x| x.unwrap().path())
        .filter(|x| x.is_dir())
        .map(|path| {
            // TODO: Fix this, do not just unwrap.
            let device_name = path.file_name().unwrap().to_str().unwrap();

            let brightness_path: PathBuf = [&path, &PathBuf::from("brightness")].iter().collect();

            let brightness = fs::read_to_string(brightness_path)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap();

            let max_brightness_path: PathBuf =
                [&path, &PathBuf::from("max_brightness")].iter().collect();

            let max_brightness = fs::read_to_string(max_brightness_path)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap();

            LightDevice::new(device_name, brightness, max_brightness)
        })
        .collect::<Vec<_>>())
}

fn main() -> Result<(), Box<dyn Error>> {
    let devices = load_light_devices()?;
    for device in devices {
        println!("{device:?}");
    }

    Ok(())
}
