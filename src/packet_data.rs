use crate::conversions;
use etherparse::{InternetSlice, SlicedPacket};
use libc::timeval;
use pcap::Packet;


pub struct PacketData {
    pub timestamp: timeval,
    pub source_ip: String,
    pub destination_ip: String,
    pub protocol: String,
    pub length: u32,
    pub data: String,
}

impl PacketData {
    pub fn new(packet: &Packet) -> Self {
        let mut source_ip: String = String::new();
        let mut destination_ip: String = String::new();
        let mut protocol: String = String::new();

        match SlicedPacket::from_ethernet(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => match value.ip.unwrap() {
                InternetSlice::Ipv4(val, _) => {
                    source_ip = val.source_addr().to_string();
                    destination_ip = val.destination_addr().to_string();
                    protocol = get_protocol(val.protocol());
                }
                InternetSlice::Ipv6(val, _) => {
                    source_ip = val.source_addr().to_string();
                    destination_ip = val.destination_addr().to_string();
                }
            },
        }
        let data: String = conversions::packet_data_to_hex(&packet.data);
        let packet_data: Self = Self {
            timestamp: packet.header.ts,
            source_ip,
            destination_ip,
            protocol,
            length: packet.header.caplen,
            data,
        };
        return packet_data;
    }

    pub fn to_string(&self) -> String {
        return format!(
            "Time: {}.{:06} Source: {} Destination: {} Protocol: {} Length: {}",
            self.timestamp.tv_sec,
            self.timestamp.tv_usec,
            self.source_ip,
            self.destination_ip,
            self.protocol,
            self.length
        );
    }

    pub fn get_ts(&self) -> String {
        return format!("{}.{:06}", self.timestamp.tv_sec, self.timestamp.tv_usec);
    }
}
fn get_protocol(prot: u8) -> String {
    return match prot {
        6 => "TCP".to_string(),
        _ => prot.to_string(),
    }
}
