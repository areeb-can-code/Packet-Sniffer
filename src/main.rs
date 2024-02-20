mod display;
mod modes;
mod packet_data;
mod conversions;

use pcap::{Capture, Device};
use std::env;
use std::process::exit;

fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        if arg == "-h" || arg == "--help" {
            display::print_help();
            exit(0);
        }
    }
    // display welcome message
display::welcome_msg();
    let mut limit = 100;

    let device = Device::lookup()
        .unwrap()
        .expect("Expected a device but recieved nothing instead");
    println!("Using device {}", device.name);
    let mut cap = Capture::from_device(device).unwrap().open().unwrap();

    let filename = match conversions::get_filename() {
        Err(err) => panic!("{}", err),
        Ok(filename) => filename,
    };

    let mode =
        match conversions::get_mode() {
        Err(err) => panic!("{}", err),
        Ok(mode) => mode,
    };

    match conversions::get_count() {
        Err(_) => (),
        Ok(count) => {
            limit = count;
        }
    };

    let mut output_file = cap.savefile(format!("{}.pcap", filename)).unwrap(); 
    /*.output_file(format!("{}.pcap", filename)).unwrap();*/

    display::print_header();

    let mut i: u32 = 1;
    while let Ok(packet) = cap.next_packet() {
        let packet_data = packet_data::PacketData::new(&packet);

        match mode {
            modes::Modes::IndividualPrint => display::individual_display(i, &packet_data, false),
            modes::Modes::Default => display::print_packet(i, &packet_data, false),
            modes::Modes::IndividualPrintWithData => {
                display::individual_display(i, &packet_data, true)
            }
            modes::Modes::DefaultWithData => display::print_packet(i, &packet_data, true),
        }

        output_file.write(&packet);

        i += 1;
        if i > limit {
            break;
        }
    }
}