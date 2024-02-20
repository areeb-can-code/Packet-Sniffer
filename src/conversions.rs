use crate::modes;
use core::fmt::Write;

use std::env;




pub fn get_filename() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    let error_missing = "Error: missing filename value".to_string();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-o" {
            let next: usize = i + 1;
            return match args.iter().nth(next) {
                None => Err(error_missing),
                Some(value) => {
                    let filename = value.to_string();
                    Ok(filename)
                }
            };
        }
    }

     return Err(error_missing);
}

pub fn get_count() -> Result<u32, String> {
    let args: Vec<String> = env::args().collect();
    let error_missing = "Error: missing count value".to_string();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-c" {
            let next: usize = i + 1;
            return match args.iter().nth(next) {
                None => Err(error_missing),
                Some(c) => {
                    let count: u32 = c
                        .to_string()
                        .trim()
                        .parse()
                        .expect("Count has to be a number");
                    Ok(count)
                }
            };
        }
    }

    return Err(error_missing);
}

pub fn get_mode() -> Result<&'static modes::Modes, String> {
    let args: Vec<String> = env::args().collect();
    let error_missing = "Error: missing mode value".to_string();
    let modes = modes::get_modes_map();

    // iterating through the args
    for (i, arg) in args.iter().enumerate() {
        if arg == "-m" {
            let next: usize = i + 1;
            return match args.iter().nth(next) {
                None => Err(error_missing),
                Some(value, ) => {
                    let mode: u8 = value.to_string().trim().parse()
                        .expect("Mode has to be a number");
// extract the values of modes which is the value and give it the default value which is none
                    match modes.get(&mode) {
                        None => Ok(&modes::Modes::Default),
                        Some(m) => Ok(m)
                    }
                }
            }
        }
    }
    return Err(error_missing);
}

pub fn packet_data_to_hex(packet_data: &[u8]) -> String {
    let mut s = String::with_capacity(2 * packet_data.len());
    for byte in packet_data {
        write!(s, "{:02X} ", byte).expect("Byte conversion gone wrong");
    }

    return s;
}
