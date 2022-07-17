//! AOC Day xx
// use aoc_2021;

const PTYPE_LITERAL_VALUE: usize = 4;

const EXAMPLE_PART_ONE: [(&str, usize); 4] = [
    ("8A004A801A8002F478", 16),
    ("620080001611562C8802118E34", 12),
    ("C0015000016115A2E0802F182340", 23),
    ("A0016C880162017C3686B18A3D4780", 31),
];

const EXAMPLE_PART_TWO: [(&str, usize); 8] = [
    ("C200B40A82", 3),
    ("04005AC33890", 54),
    ("880086C3E88112", 7),
    ("CE00C43D881120", 9),
    ("D8005AC2A8F0", 1),
    ("F600BC2D8F", 0),
    ("9C005AC2F8F0", 0),
    ("9C0141080250320F1802104A08", 1),
];

const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_16.txt"));

#[derive(Debug, Eq, PartialEq)]
enum PacketType {
    Literal,
    OperatorTotalLength(usize),
    OperatorSubPackets(usize),
    Unknown,
}

#[derive(Debug)]
struct Packet {
    version: usize,
    #[allow(dead_code)]
    ptype: PacketType,
    value: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn calculate(&self) -> usize {
        // if we're a literal, all we do is return our associated value
        if self.ptype == PacketType::Literal {
            return self.value;
        }

        // if we're not a literal, we need to do some calculation
        let mut values = vec![];

        // calculate all of our sub-packets
        for p in self.sub_packets.iter() {
            values.push(p.calculate());
        }

        let op = match self.ptype {
            PacketType::OperatorSubPackets(v) => v,
            PacketType::OperatorTotalLength(v) => v,
            _ => panic!("invalid packet type"),
        };

        match op {
            0 => values.iter().sum(),
            1 => values.iter().product(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => {
                // greater than
                assert_eq!(2, values.len());
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                // less than
                assert_eq!(2, values.len());
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                // equal to
                assert_eq!(2, values.len());
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!("invalid operator"),
        }
    }
}

/// Parses the header for our Packet
///
/// # Returns
/// (bits_used, (version, type))
fn parse_header(input: &str) -> (usize, (usize, usize)) {
    let packet_version = usize::from_str_radix(&input[0..3], 2).unwrap();
    let packet_type = usize::from_str_radix(&input[3..6], 2).unwrap();

    (6, (packet_version, packet_type))
}

fn parse_literal_value(mut input: &str) -> (usize, usize) {
    let mut final_value = 0;
    let mut bits_used = 0;
    loop {
        let value = usize::from_str_radix(&input[1..5], 2).unwrap();
        final_value <<= 4;
        final_value |= value;
        bits_used += 5;
        if input.chars().nth(0).unwrap() == '0' {
            break;
        }
        input = &input[5..];
    }

    (bits_used, final_value)
}

fn parse_op_total_length(mut input: &str) -> (usize, Vec<Packet>) {
    let mut bits_used = 0;
    let mut packets = vec![];
    let mut total_length = isize::from_str_radix(&input[0..15], 2).unwrap();
    input = &input[15..];
    bits_used += 15;

    while total_length > 0 {
        let (bits, packet) = parse_packet(input);
        input = &input[bits..];
        bits_used += bits;
        total_length -= bits as isize;
        packets.push(packet);
    }

    (bits_used, packets)
}

fn parse_op_num_sub(mut input: &str) -> (usize, Vec<Packet>) {
    let mut bits_used = 0;
    let mut packets = vec![];
    let num_packets = usize::from_str_radix(&input[0..11], 2).unwrap();
    input = &input[11..];
    bits_used += 11;

    for _ in 0..num_packets {
        let (bits, packet) = parse_packet(input);
        input = &input[bits..];
        bits_used += bits;
        packets.push(packet);
    }

    (bits_used, packets)
}

fn hex_str_to_bin_str(input: &str) -> String {
    let mut out = String::new();

    for char in input.chars() {
        let digit = char.to_digit(16).unwrap();
        out.push_str(&format!("{:04b}", digit));
    }

    out
}

fn parse_packet(mut input: &str) -> (usize, Packet) {
    let mut bits_used = 0;
    let (bits, (p_version, p_type)) = parse_header(input);
    bits_used += bits;
    input = &input[bits..];
    let mut packet = Packet {
        version: p_version,
        ptype: PacketType::Unknown,
        value: 0,
        sub_packets: vec![],
    };

    if p_type == PTYPE_LITERAL_VALUE {
        packet.ptype = PacketType::Literal;
        let (bits, value) = parse_literal_value(input);
        packet.value = value;
        bits_used += bits;
    } else {
        // we have a operator packet
        let (bits, sub_packets) = match input.chars().nth(0).unwrap() {
            '0' => {
                packet.ptype = PacketType::OperatorTotalLength(p_type);
                parse_op_total_length(&input[1..])
            }
            '1' => {
                packet.ptype = PacketType::OperatorSubPackets(p_type);
                parse_op_num_sub(&input[1..])
            }
            _ => panic!("invalid operator packet"),
        };
        packet.sub_packets = sub_packets;
        bits_used += bits + 1; // + 1 for the length type ID
    }

    (bits_used, packet)
}

fn sum_versions(packet: &Packet) -> usize {
    let mut v_sum = 0;
    v_sum += packet.version;
    for p in packet.sub_packets.iter() {
        v_sum += sum_versions(p);
    }

    v_sum
}

fn part_one(input: &str) -> usize {
    let bin_str: &str = &hex_str_to_bin_str(input);
    // println!("{} -> {}", input, bin_str);

    let (_bits, packet) = parse_packet(bin_str);
    // println!("Parsed packet {:#?}", packet);

    sum_versions(&packet)
}

fn part_two(input: &str) -> usize {
    let bin_str: &str = &hex_str_to_bin_str(input);
    // println!("{} -> {}", input, bin_str);

    let (_bits, packet) = parse_packet(bin_str);

    packet.calculate()
}

#[test]
fn example_part_one() {
    for (input, answer) in EXAMPLE_PART_ONE {
        let result = part_one(input);
        println!("example result: {}", result);
        assert_eq!(result, answer);
    }
}

#[test]
fn example_part_two() {
    for (input, answer) in EXAMPLE_PART_TWO {
        let result = part_two(input);
        println!("example result: {}", result);
        assert_eq!(result, answer);
    }
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(974, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(180616437720, result);
}
