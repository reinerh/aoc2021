fn main() {
    let packet = parse_input(advent::read_file(16).trim_end());
    println!("16a: {}", sum_versions(&packet));
    println!("16b: {}", packet.evaluate());
}

struct PacketLiteral {
    value: usize,
    bit_length: usize, // number of bits taken by encoding of literal value
}

impl PacketLiteral {
    fn from(bits: &[usize]) -> PacketLiteral {
        let mut value = 0;
        let mut i = 0;
        loop {
            let last = bits[i*5];
            let val = bits_to_int(&bits[i*5 + 1 .. i*5 + 5]);
            value <<= 4;
            value += val;
            i += 1;
            if last == 0 {
                break;
            }
        }
        PacketLiteral { value, bit_length: i * 5 }
    }
}

struct PacketOperator {
    length_type_id: usize,
    sub_packets: Vec<Packet>,
}

impl PacketOperator {
    fn from(bits: &[usize]) -> PacketOperator {
        let length_type_id = bits[0];
        let length = match length_type_id {
            0 => bits_to_int(&bits[1..16]),
            1 => bits_to_int(&bits[1..12]),
            _ => unreachable!(),
        };
        let mut sub_packets = Vec::new();

        let mut bit_length = 0;
        match length_type_id {
            0 => {
                let mut start = 16;
                loop {
                    let packet = Packet::from(&bits[start..]);
                    let packet_len = packet.total_length();
                    start += packet_len;
                    bit_length += packet_len;
                    sub_packets.push(packet);
                    if bit_length == length as usize {
                        break;
                    }
                }
            },
            1 => {
                let mut start = 12;
                for _ in 0 .. length {
                    let packet = Packet::from(&bits[start..]);
                    let packet_len = packet.total_length();
                    start += packet_len;
                    sub_packets.push(packet);
                }
            }
            _ => unreachable!(),
        }

        PacketOperator { length_type_id, sub_packets }
    }
}

enum PacketData {
    Literal(PacketLiteral),
    Operator(PacketOperator)
}

struct Packet {
    version: usize,
    type_id: usize,
    data: PacketData,
}

impl Packet {
    fn from(bits: &[usize]) -> Packet {
        let version = bits_to_int(&bits[0..3]);
        let type_id = bits_to_int(&bits[3..6]);
        let data = match type_id {
            4 => PacketData::Literal ( PacketLiteral::from(&bits[6..]) ),
            _ => PacketData::Operator ( PacketOperator::from(&bits[6..]) ),
        };

        Packet { version, type_id, data }
    }

    fn total_length(&self) -> usize {
        let length = match &self.data {
            PacketData::Literal(literal) => literal.bit_length,
            PacketData::Operator(operator) => {
                let header = 1 + match operator.length_type_id {
                    0 => 15,
                    1 => 11,
                    _ => unreachable!(),
                };
                let body : usize = operator.sub_packets.iter().map(|pkt| pkt.total_length()).sum();
                header + body
            }
        };
        3 + 3 + length
    }

    fn evaluate(&self) -> usize {
        match &self.data {
            PacketData::Literal(literal) => {
                literal.value
            },
            PacketData::Operator(operator) => {
                match self.type_id {
                    0 => {
                        operator.sub_packets.iter()
                                            .map(|pkt| pkt.evaluate())
                                            .sum()
                    },
                    1 => {
                        operator.sub_packets.iter()
                                            .map(|pkt| pkt.evaluate())
                                            .product()
                    },
                    2 => {
                        operator.sub_packets.iter()
                                            .map(|pkt| pkt.evaluate())
                                            .min()
                                            .unwrap()
                    },
                    3 => {
                        operator.sub_packets.iter()
                                            .map(|pkt| pkt.evaluate())
                                            .max()
                                            .unwrap()
                    },
                    4 => panic!("type id 4 is not an operation"),
                    5 => {
                        let (val1, val2) = (operator.sub_packets[0].evaluate(), operator.sub_packets[1].evaluate());
                        if val1 > val2 { 1 } else { 0 }
                    },
                    6 => {
                        let (val1, val2) = (operator.sub_packets[0].evaluate(), operator.sub_packets[1].evaluate());
                        if val1 < val2 { 1 } else { 0 }
                    },
                    7 => {
                        let (val1, val2) = (operator.sub_packets[0].evaluate(), operator.sub_packets[1].evaluate());
                        if val1 == val2 { 1 } else { 0 }
                    },
                    _ => panic!("invalid operation"),
                }
            }
        }
    }
}

fn bits_to_int(bits: &[usize]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, bit)| bit << i)
        .sum()
}

fn parse_input(input: &str) -> Packet {
    let mut bits = Vec::new();
    for c in input.chars().map(|c| c.to_digit(16).unwrap() as usize) {
        let mut tmp = Vec::new();
        let mut c = c;
        for _ in 0 .. 4 {
            tmp.push(c & 1);
            c >>= 1;
        }
        tmp.reverse();
        bits.append(&mut tmp);
    }
    Packet::from(&bits)
}

fn sum_versions(packet: &Packet) -> usize {
    let mut versions = 0;
    if let PacketData::Operator(operator) = &packet.data {
        for pkt in &operator.sub_packets {
            versions += sum_versions(pkt);
        }
    }
    versions + packet.version
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let input = "D2FE28";
        let packet = parse_input(&input);
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        if let PacketData::Literal(literal) = packet.data {
            assert_eq!(literal.value, 2021);
            assert_eq!(literal.bit_length, 15);
        } else {
            panic!("invalid packet");
        }
    }

    #[test]
    fn test_operator1() {
        let input = "38006F45291200";
        let packet = parse_input(&input);
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        if let PacketData::Operator(operator) = packet.data {
            assert_eq!(operator.length_type_id, 0);
            assert_eq!(operator.sub_packets.len(), 2);
            if let PacketData::Literal(literal) = &operator.sub_packets[0].data {
                assert_eq!(literal.value, 10);
            } else {
                panic!("invalid packet type");
            }
            if let PacketData::Literal(literal) = &operator.sub_packets[1].data {
                assert_eq!(literal.value, 20);
            } else {
                panic!("invalid packet type");
            }
        } else {
            panic!("invalid packet");
        }
    }

    #[test]
    fn test_operator2() {
        let input = "EE00D40C823060";
        let packet = parse_input(&input);
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, 3);
        if let PacketData::Operator(operator) = packet.data {
            assert_eq!(operator.length_type_id, 1);
            assert_eq!(operator.sub_packets.len(), 3);
            if let PacketData::Literal(literal) = &operator.sub_packets[0].data {
                assert_eq!(literal.value, 1);
            } else {
                panic!("invalid packet type");
            }
            if let PacketData::Literal(literal) = &operator.sub_packets[1].data {
                assert_eq!(literal.value, 2);
            } else {
                panic!("invalid packet type");
            }
            if let PacketData::Literal(literal) = &operator.sub_packets[2].data {
                assert_eq!(literal.value, 3);
            } else {
                panic!("invalid packet type");
            }
        } else {
            panic!("invalid packet");
        }
    }

    #[test]
    fn test_version_sum() {
        let packet = parse_input("8A004A801A8002F478");
        assert_eq!(sum_versions(&packet), 16);

        let packet = parse_input("620080001611562C8802118E34");
        assert_eq!(sum_versions(&packet), 12);

        let packet = parse_input("C0015000016115A2E0802F182340");
        assert_eq!(sum_versions(&packet), 23);

        let packet = parse_input("A0016C880162017C3686B18A3D4780");
        assert_eq!(sum_versions(&packet), 31);
    }

    #[test]
    fn test_evaluate() {
        let packet = parse_input("C200B40A82");
        assert_eq!(packet.evaluate(), 3);

        let packet = parse_input("04005AC33890");
        assert_eq!(packet.evaluate(), 54);

        let packet = parse_input("880086C3E88112");
        assert_eq!(packet.evaluate(), 7);

        let packet = parse_input("CE00C43D881120");
        assert_eq!(packet.evaluate(), 9);

        let packet = parse_input("D8005AC2A8F0");
        assert_eq!(packet.evaluate(), 1);

        let packet = parse_input("F600BC2D8F");
        assert_eq!(packet.evaluate(), 0);

        let packet = parse_input("9C005AC2F8F0");
        assert_eq!(packet.evaluate(), 0);

        let packet = parse_input("9C0141080250320F1802104A08");
        assert_eq!(packet.evaluate(), 1);
    }
}
