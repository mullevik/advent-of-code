pub fn first_part(input: &str) -> i32 {
    let (packet, _) = interpret_packet(&parse(input));
    sum_versions(&packet)
}

pub fn second_part(input: &str) -> i64 {
    let (packet, _) = interpret_packet(&parse(input));
    packet.evaluate()
}
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

struct LiteralPacket {
    version: u8,
    value: Vec<bool>,
}

struct OperatorPacket {
    version: u8,
    operation: Operation,
    children: Vec<Packet>,
}

trait Evaluable {
    fn evaluate(&self) -> i64;
}
impl Evaluable for Packet {
    fn evaluate(&self) -> i64 {
        match self {
            Packet::Literal(l) => l.evaluate(),
            Packet::Operator(o) => o.evaluate(),
        }
    }
}

impl Evaluable for LiteralPacket {
    fn evaluate(&self) -> i64 {
        array_to_num(&self.value)
    }
}

impl Evaluable for OperatorPacket {
    fn evaluate(&self) -> i64 {
        match self.operation {
            Operation::Sum => self.children.iter().map(|ch| ch.evaluate()).sum::<i64>(),
            Operation::Product => self
                .children
                .iter()
                .map(|ch| ch.evaluate())
                .product::<i64>(),
            Operation::Minimum => self.children.iter().map(|ch| ch.evaluate()).min().unwrap(),
            Operation::Maximum => self.children.iter().map(|ch| ch.evaluate()).max().unwrap(),
            Operation::GreaterThan => {
                (self.children[0].evaluate() > self.children[1].evaluate()) as i64
            }
            Operation::LessThan => {
                (self.children[0].evaluate() < self.children[1].evaluate()) as i64
            }
            Operation::EqualTo => {
                (self.children[0].evaluate() == self.children[1].evaluate()) as i64
            }
        }
    }
}

fn sum_versions(packet: &Packet) -> i32 {
    match packet {
        Packet::Literal(l) => l.version as i32,
        Packet::Operator(o) => {
            o.version as i32 + o.children.iter().map(|ch| sum_versions(ch)).sum::<i32>()
        }
    }
}

fn interpret_packet(sequence: &[bool]) -> (Packet, usize) {
    let version = array_to_num(&sequence[..3]) as u8;

    let _type = array_to_num(&sequence[3..6]);

    if _type == 0b100 {
        let (packet, eof) = interpret_literal(version, &sequence[6..]);
        (Packet::Literal(packet), 6 + eof)
    } else {
        let operation: Operation = {
            match _type {
                0 => Operation::Sum,
                1 => Operation::Product,
                2 => Operation::Minimum,
                3 => Operation::Maximum,
                5 => Operation::GreaterThan,
                6 => Operation::LessThan,
                7 => Operation::EqualTo,
                _ => panic!("Unknown operation type"),
            }
        };
        let (packet, eof) = interpret_operator(version, operation, &sequence[6..]);
        (Packet::Operator(packet), 6 + eof)
    }
}

fn interpret_multiple_packets(sequence: &[bool]) -> (Vec<Packet>, usize) {
    let mut packets = vec![];
    let mut start_at = 0;

    while start_at < sequence.len() {
        let (packet, eof) = interpret_packet(&sequence[start_at..]);
        start_at += eof;
        packets.push(packet);
    }

    (packets, start_at)
}

fn interpret_literal(version: u8, sequence: &[bool]) -> (LiteralPacket, usize) {
    let mut data: Vec<bool> = vec![];
    let mut start_at: usize = 0;
    loop {
        let should_continue_flag = sequence[start_at];

        data.extend::<Vec<bool>>(
            sequence[(start_at + 1)..(start_at + 5)]
                .iter()
                .cloned()
                .collect(),
        );

        start_at += 5;
        if !should_continue_flag {
            break;
        }
    }
    (
        LiteralPacket {
            version,
            value: data,
        },
        start_at,
    )
}

fn interpret_operator(
    version: u8,
    operation: Operation,
    sequence: &[bool],
) -> (OperatorPacket, usize) {
    let _info_flag = sequence[0];

    if !_info_flag {
        let size = array_to_num(&sequence[1..16]);
        let (packet, eof) =
            interpet_operator_with_size(version, operation, size as usize, &sequence[16..]);
        (packet, 16 + eof)
    } else {
        let count = array_to_num(&sequence[1..12]);
        let (packet, eof) =
            interpet_operator_with_count(version, operation, count as usize, &sequence[12..]);
        (packet, 12 + eof)
    }
}

fn interpet_operator_with_count(
    version: u8,
    operation: Operation,
    count: usize,
    sequence: &[bool],
) -> (OperatorPacket, usize) {
    let mut children = vec![];
    let mut start_at = 0;
    for _ in 0..count {
        let (packet, eof) = interpret_packet(&sequence[start_at..]);

        start_at += eof;
        children.push(packet);
    }
    (
        OperatorPacket {
            version,
            operation,
            children,
        },
        start_at,
    )
}

fn interpet_operator_with_size(
    version: u8,
    operation: Operation,
    size: usize,
    sequence: &[bool],
) -> (OperatorPacket, usize) {
    let (children, eof) = interpret_multiple_packets(&sequence[..size]);
    (
        OperatorPacket {
            version,
            operation,
            children,
        },
        eof,
    )
}

fn parse(input: &str) -> Vec<bool> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| num_to_bool_array(c.to_digit(16).unwrap() as u8))
        .flatten()
        .collect()
}

fn num_to_bool_array(num: u8) -> Vec<bool> {
    vec![
        num & 0b1000 != 0,
        num & 0b0100 != 0,
        num & 0b0010 != 0,
        num & 0b0001 != 0,
    ]
}

fn array_to_num(sequence: &[bool]) -> i64 {
    let mut num = 0;
    for (i, bit_value) in sequence.iter().rev().enumerate() {
        if *bit_value {
            num += 1 << i
        }
    }
    num
}

#[cfg(test)]
mod tests_day_16 {
    use crate::day_16::{array_to_num, first_part, num_to_bool_array, parse, second_part};

    #[test]
    fn test_parse() {
        let mut res = vec![];
        res.extend(num_to_bool_array(0b1101));
        res.extend(num_to_bool_array(0b0010));
        res.extend(num_to_bool_array(0b1111));
        res.extend(num_to_bool_array(0b1110));
        res.extend(num_to_bool_array(0b0010));
        res.extend(num_to_bool_array(0b1000));
        assert_eq!(parse("D2FE28"), res);
    }

    #[test]
    fn test_array_to_num() {
        assert_eq!(array_to_num(&vec![true, false, true]), 5);
    }

    #[test]
    fn test_example_part_one() {
        assert_eq!(first_part("D2FE28"), 6);
        assert_eq!(first_part("8A004A801A8002F478"), 16);
        assert_eq!(first_part("620080001611562C8802118E34"), 12);
        assert_eq!(first_part("C0015000016115A2E0802F182340"), 23);
        assert_eq!(first_part("A0016C880162017C3686B18A3D4780"), 31);
        assert_eq!(first_part("38006F45291200"), 9);
        assert_eq!(first_part("EE00D40C823060"), 14);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/16.in")), 854);
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part("C200B40A82"), 3);
        assert_eq!(second_part("04005AC33890"), 54);
        assert_eq!(second_part("880086C3E88112"), 7);
        assert_eq!(second_part("CE00C43D881120"), 9);
        assert_eq!(second_part("D8005AC2A8F0"), 1);
        assert_eq!(second_part("F600BC2D8F"), 0);
        assert_eq!(second_part("9C005AC2F8F0"), 0);
        assert_eq!(second_part("9C0141080250320F1802104A08"), 1);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/16.in")), 186189840660);
    }
}
