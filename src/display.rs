use crate::packet_data::PacketData;
use libc::{exit};
use online::{self, check};

pub fn individual_display(num: u32, packet: &PacketData, incl_data: bool) {
    clear_display();
    print_header();
    print_packet(num, &packet, incl_data);
}

pub fn print_header() {
    println!(
        // columns
        "{:<10} | {:<20} | {:<40} | {:<40} | {:<10} | {:<10}",
        "Num:", "Time:", "Source:", "Destination:", "Protocol:", "Length:"
    );
    println!("{}", "=".repeat(145));
}

pub fn print_packet(num: u32, packet: &PacketData, incl_data: bool) {
    println!(
        "{:<10} | {:<20} | {:<40} | {:<40} | {:<10} | {:<10}",
        num,
        packet.get_ts(),
        packet.source_ip,
        packet.destination_ip,
        packet.protocol,
        packet.length
    );
    // we should probably catch exceptions here if something bad came in
    if incl_data {
        println!("In the packet is : \t");
        println!("{}", packet.data);
    }
}

pub fn print_help() {
    println!("\n Smelly Packs - This is a terminal utility to capture packets, display them to the screen with \
    opportunities to resolve certain IP addresses and more!.\n");
    println!("Usage:\n");
    println!("smelly_packs <args>\n");
    println!("Arguments:");
    println!("\t-m -- display mode (0, 1, 2, 3) [REQUIRED]");
    println!("\t-o -- name of the output file [REQUIRED]");
    println!("\t-c -- number of packets to be captured and displayed\n");
    println!("Any questions on the usage of the program can be found in the README.md.");
}
pub fn welcome_msg()
{
    println!(" Hello welcome to Smelly Pack, \n \t  a utility to capture packets. Like any projects, they can always improve\
     and I want to make sure I can make it pretty cool.\
     First things first, let's make sure you are connected to the internet!\n \n ");
 
      if check(None).is_ok() == false
      {
      println!("You're not connected to the internet");
      unsafe {exit(1);}
      }
      else {
        println!("Online? {}:, yup you are ", check(Some(5)).is_ok());
      }
      
}
fn clear_display() {
    print!("\x1B[2J\x1B[1;1H");
}
