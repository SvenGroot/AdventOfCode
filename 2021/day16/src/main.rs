// https://adventofcode.com/2021/day/16

use std::{io::Seek, str::FromStr};

use aoc::{
    bitreader::BitReader,
    input::{AocInput, ParseHex},
};
use bitvec::vec::BitVec;

fn main() {
    println!("Part 1: {}", part1(AocInput::from_input()));
    println!("Part 2: {}", part2(AocInput::from_input()));
}

// Parse the packets and sum all their version fields.
fn part1(input: AocInput) -> usize {
    Packet::from_str(&input.single_line())
        .unwrap()
        .get_version_sum()
}

// Compute the result of the operation encoded in the packets.
fn part2(input: AocInput) -> usize {
    Packet::from_str(&input.single_line()).unwrap().get_value()
}

struct PacketHeader {
    version: u8,
    type_id: u8,
}

impl PacketHeader {
    fn read_from(reader: &mut BitReader) -> Self {
        Self {
            version: reader.read(3),
            type_id: reader.read(3),
        }
    }
}

enum PacketBody {
    Value(usize),
    Operator(Vec<Packet>),
}

impl PacketBody {
    fn read_value(reader: &mut BitReader) -> Self {
        let mut value = 0usize;
        let mut not_last = true;
        while not_last {
            not_last = reader.read_bool();
            value = (value << 4) | reader.read::<usize>(4);
        }

        Self::Value(value)
    }

    fn read_operator(reader: &mut BitReader) -> Self {
        let use_packet_count = reader.read_bool();
        let count = if use_packet_count {
            reader.read(11)
        } else {
            let count = reader.read::<u64>(15);
            reader.stream_position().unwrap() + count
        };

        let mut packet_count = 0;
        let mut packets = Vec::new();
        loop {
            packets.push(Packet::read_from(reader));
            if use_packet_count {
                packet_count += 1;
                if packet_count == count {
                    break;
                }
            } else if reader.stream_position().unwrap() == count {
                break;
            }
        }

        Self::Operator(packets)
    }
}

struct Packet {
    header: PacketHeader,
    body: PacketBody,
}

impl Packet {
    fn read_from(reader: &mut BitReader) -> Self {
        let header = PacketHeader::read_from(reader);
        let body = if header.type_id == 4 {
            PacketBody::read_value(reader)
        } else {
            PacketBody::read_operator(reader)
        };

        Self { header, body }
    }

    fn get_version_sum(&self) -> usize {
        let children = match &self.body {
            PacketBody::Value(_) => 0,
            PacketBody::Operator(packets) => packets.iter().map(|p| p.get_version_sum()).sum(),
        };

        self.header.version as usize + children
    }

    fn get_value(&self) -> usize {
        match &self.body {
            PacketBody::Value(value) => *value,
            PacketBody::Operator(packets) => self.evaluate_operator(packets),
        }
    }

    fn evaluate_operator(&self, packets: &[Packet]) -> usize {
        match self.header.type_id {
            0 => packets.iter().map(|p| p.get_value()).sum(),
            1 => packets.iter().map(|p| p.get_value()).product(),
            2 => packets.iter().map(|p| p.get_value()).min().unwrap(),
            3 => packets.iter().map(|p| p.get_value()).max().unwrap(),
            5 => (packets[0].get_value() > packets[1].get_value()).into(),
            6 => (packets[0].get_value() < packets[1].get_value()).into(),
            7 => (packets[0].get_value() == packets[1].get_value()).into(),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.parse_hex();
        let mut reader = BitReader::new(BitVec::from_vec(hex));
        Ok(Packet::read_from(&mut reader))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            16,
            Packet::from_str("8A004A801A8002F478")
                .unwrap()
                .get_version_sum()
        );

        assert_eq!(
            12,
            Packet::from_str("620080001611562C8802118E34")
                .unwrap()
                .get_version_sum()
        );

        assert_eq!(
            23,
            Packet::from_str("C0015000016115A2E0802F182340")
                .unwrap()
                .get_version_sum()
        );

        assert_eq!(
            31,
            Packet::from_str("A0016C880162017C3686B18A3D4780")
                .unwrap()
                .get_version_sum()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(3, Packet::from_str("C200B40A82").unwrap().get_value());
        assert_eq!(54, Packet::from_str("04005AC33890").unwrap().get_value());
        assert_eq!(7, Packet::from_str("880086C3E88112").unwrap().get_value());
        assert_eq!(9, Packet::from_str("CE00C43D881120").unwrap().get_value());
        assert_eq!(1, Packet::from_str("D8005AC2A8F0").unwrap().get_value());
        assert_eq!(0, Packet::from_str("F600BC2D8F").unwrap().get_value());
        assert_eq!(0, Packet::from_str("9C005AC2F8F0").unwrap().get_value());
        assert_eq!(
            1,
            Packet::from_str("9C0141080250320F1802104A08")
                .unwrap()
                .get_value()
        );
    }

    #[test]
    fn test_value_packet() {
        let packet = Packet::from_str("D2FE28").unwrap();
        assert_eq!(6, packet.header.version);
        assert_eq!(4, packet.header.type_id);
        assert!(matches!(packet.body, PacketBody::Value(2021)));
    }

    #[test]
    fn test_operator_packet() {
        let packet = Packet::from_str("38006F45291200").unwrap();
        assert_eq!(1, packet.header.version);
        assert_eq!(6, packet.header.type_id);
        let PacketBody::Operator(packets) = &packet.body else {
            panic!();
        };

        assert_eq!(2, packets.len());
        assert!(matches!(packets[0].body, PacketBody::Value(10)));
        assert!(matches!(packets[1].body, PacketBody::Value(20)));
    }

    #[test]
    fn test_operator_packet_count() {
        let packet = Packet::from_str("EE00D40C823060").unwrap();
        assert_eq!(7, packet.header.version);
        assert_eq!(3, packet.header.type_id);
        let PacketBody::Operator(packets) = &packet.body else {
            panic!();
        };

        assert_eq!(3, packets.len());
        assert!(matches!(packets[0].body, PacketBody::Value(1)));
        assert!(matches!(packets[1].body, PacketBody::Value(2)));
        assert!(matches!(packets[2].body, PacketBody::Value(3)));
    }
}
