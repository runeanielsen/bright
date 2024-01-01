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
    let leds = fs::read_dir("/sys/class/leds").expect("Failed to open LED controller directory.");
    let backlights = fs::read_dir("/sys/class/backlight")
        .expect("Failed to open backlight controller directory.");

    Ok(leds
        .chain(backlights)
        .map(|x| x.unwrap().path())
        .filter(|x| x.is_dir())
        .map(|x| {
            // TODO: Fix this, do not just unwrap.
            let device_name = x.file_name().unwrap().to_str().unwrap();

            let brightness_path: PathBuf =
                [x.to_owned(), PathBuf::from("brightness")].iter().collect();

            let brightness =
                i64::from_str_radix(&fs::read_to_string(&brightness_path).unwrap().trim(), 10)
                    .unwrap();

            let max_brightness_path: PathBuf = [x.to_owned(), PathBuf::from("max_brightness")]
                .iter()
                .collect();

            let max_brightness = i64::from_str_radix(
                &fs::read_to_string(&max_brightness_path).unwrap().trim(),
                10,
            )
            .unwrap();

            LightDevice::new(&device_name, brightness, max_brightness)
        })
        .collect::<Vec<_>>())
}

fn main() -> Result<(), Box<dyn Error>> {
    let devices = load_light_devices()?;
    for device in devices {
        println!("{:?}", device);
    }

    Ok(())
}
