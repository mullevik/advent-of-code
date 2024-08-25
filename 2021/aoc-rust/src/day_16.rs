pub fn first_part(input: &str) -> i32 {
    let (packet, _) = interpret_packet(&parse(input));
    sum_versions(&packet)
}

pub fn second_part(input: &str) -> i32 {
    unimplemented!()
}
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

struct LiteralPacket {
    version: u8,
    value: Vec<bool>,
}

struct OperatorPacket {
    version: u8,
    children: Vec<Packet>,
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
        let (packet, eof) = interpret_operator(version, &sequence[6..]);
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

fn interpret_operator(version: u8, sequence: &[bool]) -> (OperatorPacket, usize) {
    let _info_flag = sequence[0];

    if !_info_flag {
        let subpacket_width = array_to_num(&sequence[1..16]);

        let (children, eof) =
            interpret_multiple_packets(&sequence[16..(16 + subpacket_width as usize)]);
        (OperatorPacket { version, children }, 16 + eof)
    } else {
        let n_subpackets = array_to_num(&sequence[1..12]);

        let mut children = vec![];
        let mut start_at = 12;
        for _ in 0..n_subpackets {
            let (packet, eof) = interpret_packet(&sequence[start_at..]);

            start_at += eof;
            children.push(packet);
        }
        (OperatorPacket { version, children }, start_at)
    }
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

fn array_to_num(sequence: &[bool]) -> i32 {
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
    use crate::{day_16::{array_to_num, first_part, num_to_bool_array, parse}};

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
        assert_eq!(first_part(include_str!("../inputs/16_example")), 6);
        assert_eq!(first_part(include_str!("../inputs/16_example_01")), 16);
        assert_eq!(first_part(include_str!("../inputs/16_example_02")), 12);
        assert_eq!(first_part(include_str!("../inputs/16_example_03")), 23);
        assert_eq!(first_part(include_str!("../inputs/16_example_04")), 31);
        assert_eq!(first_part(include_str!("../inputs/16_example_05")), 9);
        assert_eq!(first_part(include_str!("../inputs/16_example_06")), 14);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/16.in")), -1);
    }

}
