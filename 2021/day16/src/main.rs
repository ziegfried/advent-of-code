#[derive(Debug, PartialEq, Clone, Copy)]
enum OperatorType {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Operator {
        version: usize,
        op_type: OperatorType,
        sub_packets: Box<Vec<Packet>>,
    },
    LiteralValue {
        version: usize,
        value: usize,
    },
}

fn parse_num(bits: &[u8]) -> usize {
    usize::from_str_radix(
        bits.iter()
            .map(|b| format!("{}", b))
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}

fn take_bits(bits: &mut Vec<u8>, count: usize) -> Vec<u8> {
    let mut res = vec![];
    for _ in 0..count {
        res.push(bits.remove(0));
    }
    res
}

fn read_literal_value(bits: &mut Vec<u8>) -> usize {
    let mut content = vec![];
    loop {
        let next = take_bits(bits, 5);
        for b in &next[1..] {
            content.push(*b);
        }
        if next[0] == 0 {
            break;
        }
    }
    parse_num(&content)
}

fn read_operator_contents(bits: &mut Vec<u8>) -> Box<Vec<Packet>> {
    let mut sub = vec![];
    let length_bit = parse_num(&take_bits(bits, 1));
    if length_bit == 0 {
        let content_length = parse_num(&take_bits(bits, 15));
        let mut content = take_bits(bits, content_length);
        while let Some(packet) = read_packet(&mut content) {
            sub.push(packet);
        }
    } else if length_bit == 1 {
        let packet_count = parse_num(&take_bits(bits, 11));
        for _ in 0..packet_count {
            sub.push(read_packet(bits).unwrap());
        }
    } else {
        panic!();
    }
    Box::new(sub)
}

fn read_packet(bits: &mut Vec<u8>) -> Option<Packet> {
    if bits.len() <= 8 && bits.iter().all(|b| b == &0) {
        return None;
    }
    let version = parse_num(&take_bits(bits, 3));
    let type_id = parse_num(&take_bits(bits, 3));
    if type_id == 4 {
        Some(Packet::LiteralValue {
            version,
            value: read_literal_value(bits),
        })
    } else {
        Some(Packet::Operator {
            version,
            op_type: match type_id {
                0 => OperatorType::Sum,
                1 => OperatorType::Product,
                2 => OperatorType::Min,
                3 => OperatorType::Max,
                5 => OperatorType::Gt,
                6 => OperatorType::Lt,
                7 => OperatorType::Eq,
                _ => panic!("invalid operator type_id {}", type_id),
            },
            sub_packets: read_operator_contents(bits),
        })
    }
}

fn hex_to_binary(hex_string: &str) -> Vec<u8> {
    hex_string
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|c| format!("{:04b}", c))
        .collect::<String>()
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect()
}

fn sum_version(packet: &Packet) -> usize {
    match packet {
        Packet::LiteralValue { version, value: _ } => *version,
        Packet::Operator {
            version,
            op_type: _,
            sub_packets,
        } => *version + sub_packets.iter().map(sum_version).sum::<usize>(),
    }
}

fn part1(input: &str) -> usize {
    let mut bits = hex_to_binary(input);
    let packet = read_packet(&mut bits).unwrap();
    sum_version(&packet)
}

fn compute_operator_result(op_type: OperatorType, sub: &Vec<Packet>) -> usize {
    use OperatorType::*;
    match op_type {
        Sum => sub.iter().map(|child| compute_result(child)).sum(),
        Product => sub
            .iter()
            .map(|child| compute_result(child))
            .fold(1, |a, b| a * b),
        Min => sub.iter().map(|child| compute_result(child)).min().unwrap(),
        Max => sub.iter().map(|child| compute_result(child)).max().unwrap(),
        Gt => {
            let mut res = sub.iter().map(|child| compute_result(child));
            let first = res.next().unwrap();
            let second = res.next().unwrap();
            if first > second {
                1
            } else {
                0
            }
        }
        Lt => {
            let mut res = sub.iter().map(|child| compute_result(child));
            let first = res.next().unwrap();
            let second = res.next().unwrap();
            if first < second {
                1
            } else {
                0
            }
        }
        Eq => {
            let mut res = sub.iter().map(|child| compute_result(child));
            let first = res.next().unwrap();
            let second = res.next().unwrap();
            if first == second {
                1
            } else {
                0
            }
        }
    }
}

fn compute_result(packet: &Packet) -> usize {
    match &packet {
        Packet::LiteralValue { version: _, value } => *value,
        Packet::Operator {
            version: _,
            op_type,
            sub_packets,
        } => compute_operator_result(*op_type, &sub_packets),
    }
}

fn part2(input: &str) -> usize {
    let mut bits = hex_to_binary(input);
    let packet = read_packet(&mut bits).unwrap();
    compute_result(&packet)
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("in.txt")));
    println!("Part 2: {:?}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1("8A004A801A8002F478"), 16);
    assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
}

#[test]
fn test_part2() {
    assert_eq!(part2("C200B40A82"), 3);
    assert_eq!(part2("04005AC33890"), 54);
    assert_eq!(part2("9C0141080250320F1802104A08"), 1);
}
