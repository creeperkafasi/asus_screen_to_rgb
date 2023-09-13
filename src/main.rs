use std::{env::args, error::Error, process, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut loop_delay = 50u64;
    let mut ignore_black = false;
    let mut brightness_factor = 1;
    let mut mode = "static";

    let mut args = args();
    args.next(); // Ignore the first arg
    while let Some(arg) = args.next() {
        match arg.to_string().as_str() {
            "-h" | "--help" => {
                println!(
                    "
Command Options:

-d / --delay <milliseconds>   : Time between changing colors in milliseconds (default: 50)
-i / --ignore-black <bool>    : Ignore black pixels when calculating average (default: false)
-b / --brightness <number>    : Brightness multiplier (default: 1)
-m / --mode <mode>            : Led mode; static, breathe, pulse (default: static)
"
                );
                return Ok(());
            }
            "-d" | "--delay" => {
                loop_delay = args
                    .next()
                    .ok_or("Missing delay argument")?
                    .parse::<u64>()?;
            }
            "-i" | "--ignore-black" => {
                ignore_black = args
                    .next()
                    .ok_or("Missing ignore-black argument")?
                    .parse::<bool>()?;
            }
            "-b" | "--brightness" => {
                brightness_factor = args
                    .next()
                    .ok_or("Missing brightness argument")?
                    .parse::<u32>()?;
            }
            "-m" | "--mode" => {
                mode = match args.next().ok_or("Missing mode argument")?.as_str() {
                    "static" => Ok("static"),
                    "breathe" => Ok("breathe"),
                    "pulse" => Ok("pulse"),
                    _ => Err("Invalid mode specified"),
                }?;
            }
            _ => {
                println!("Unknown argument: {}", arg.to_string())
            }
        }
    }
    loop {
        set_color(
            get_screen_average(ignore_black)
                .unwrap()
                .map(|val| val * brightness_factor),
            mode,
        );
        std::thread::sleep(Duration::from_millis(loop_delay));
    }
}

fn set_color(color: [u32; 3], mode: &str) {
    process::Command::new("asusctl")
        .arg("led-mode")
        .arg(mode)
        .arg("-c")
        .arg(color.map(|val| format!("{:02x}", val)).join(""))
        .spawn()
        .unwrap();
}

fn get_screen_average(ignore_black: bool) -> Result<[u32; 3], Box<dyn Error>> {
    let pixel_values = screenshots::Screen::all()?[0]
        .capture()
        .unwrap()
        .pixels()
        .map(|p| p.0.map(|v| v as u32))
        .collect::<Vec<[u32; 4]>>();
    let mut sum = [0, 0, 0];
    let mut pixel_count = 0;
    for pix in pixel_values {
        if !ignore_black || !pix.starts_with(&[0, 0, 0]) {
            pixel_count += 1;
            for i in 0..3 {
                sum[i] += pix[i];
            }
        }
    }

    if pixel_count == 0 {
        return Ok([0, 0, 0]);
    }

    return Ok([
        sum[0] / pixel_count,
        sum[1] / pixel_count,
        sum[2] / pixel_count,
    ]);
}
