#![warn(clippy::all, clippy::pedantic)]
use std::{error::Error, fs, path::PathBuf};

#[derive(Debug)]
struct LightDevice {
    // Name of the device.
    pub name: String,

    // The full system path to the device.
    pub device_full_path: PathBuf,

    // Current device brightness.
    pub brightness: i64,

    // Maximum brightness for device.
    pub max_brightness: i64,
}

impl LightDevice {
    pub fn new(
        name: &str,
        device_full_path: PathBuf,
        brightness: i64,
        max_brightness: i64,
    ) -> Self {
        Self {
            name: name.to_owned(),
            device_full_path,
            brightness,
            max_brightness,
        }
    }
}

fn load_light_devices(paths: &[&str]) -> Result<Vec<LightDevice>, Box<dyn Error>> {
    paths
        .iter()
        .map(fs::read_dir)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|x| x.path())
        .filter(|x| x.is_dir())
        .map(|path| {
            let device_path_utf8 = path.to_str().ok_or(
                "Could not get path for device, it might have been removed in the meantime.",
            )?;

            let file_name = path.file_name().ok_or(format!(
                "Could not get file_name for path: '{device_path_utf8}'"
            ))?;

            let device_name = file_name.to_str().ok_or(format!(
                "Could not get device name from OS string from path: '{device_path_utf8}'"
            ))?;

            let brightness_path: PathBuf = [&path, &PathBuf::from("brightness")].iter().collect();
            let brightness = fs::read_to_string(brightness_path)?.trim().parse::<i64>()?;

            let max_brightness_path: PathBuf =
                [&path, &PathBuf::from("max_brightness")].iter().collect();

            let max_brightness = fs::read_to_string(max_brightness_path)?
                .trim()
                .parse::<i64>()?;

            Ok(LightDevice::new(
                device_name,
                path.to_owned(),
                brightness,
                max_brightness,
            ))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let device_paths: [&str; 2] = ["/sys/class/leds", "/sys/class/backlight"];
    for device in load_light_devices(&device_paths)? {
        println!(
            "The device consists of the following: {} {} {} {}",
            device.name,
            device.device_full_path.to_str().unwrap(),
            device.brightness,
            device.max_brightness
        );
    }

    Ok(())
}
