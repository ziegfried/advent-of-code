mod bits;
use bits::Bits;

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

fn parse_num(bits: &[bool]) -> usize {
    usize::from_str_radix(
        bits.iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}

fn read_literal_value(bits: &mut Bits) -> usize {
    let mut result = 0;
    loop {
        let next = bits.take(5);
        result = result * 16 + parse_num(&next[1..]);
        if next[0] == false {
            break;
        }
    }
    result
}

fn read_operator_contents(bits: &mut Bits) -> Box<Vec<Packet>> {
    let mut sub = vec![];
    let length_bit = parse_num(bits.take(1));
    if length_bit == 0 {
        let content_length = parse_num(bits.take(15));
        let mut content = Bits::new(bits.take(content_length).to_vec());
        while let Some(packet) = read_packet(&mut content) {
            sub.push(packet);
        }
    } else if length_bit == 1 {
        let packet_count = parse_num(&bits.take(11));
        for _ in 0..packet_count {
            sub.push(read_packet(bits).unwrap());
        }
    } else {
        panic!();
    }
    Box::new(sub)
}

fn read_packet(bits: &mut Bits) -> Option<Packet> {
    if bits.empty() {
        return None;
    }
    let version = parse_num(bits.take(3));
    let type_id = parse_num(bits.take(3));
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
    sum_version(&read_packet(&mut Bits::from_hex_str(input)).unwrap())
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
    compute_result(&read_packet(&mut Bits::from_hex_str(input)).unwrap())
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_read_packet() {
    assert_eq!(
        read_packet(&mut Bits::from_hex_str("D2FE28")),
        Some(Packet::LiteralValue {
            version: 6,
            value: 2021
        })
    );
    assert_eq!(
        read_packet(&mut Bits::from_hex_str("38006F45291200")),
        Some(Packet::Operator {
            version: 1,
            op_type: OperatorType::Lt,
            sub_packets: Box::new(vec![
                Packet::LiteralValue {
                    version: 6,
                    value: 10,
                },
                Packet::LiteralValue {
                    version: 2,
                    value: 20,
                },
            ]),
        },)
    );
}

#[test]
fn test_part1() {
    assert_eq!(part1("8A004A801A8002F478"), 16);
    assert_eq!(part1("620080001611562C8802118E34"), 12);
    assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
    assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
}

#[test]
fn test_part2() {
    assert_eq!(part2("C200B40A82"), 3);
    assert_eq!(part2("04005AC33890"), 54);
    assert_eq!(part2("880086C3E88112"), 7);
    assert_eq!(part2("CE00C43D881120"), 9);
    assert_eq!(part2("D8005AC2A8F0"), 1);
    assert_eq!(part2("F600BC2D8F"), 0);
    assert_eq!(part2("9C005AC2F8F0"), 0);
    assert_eq!(part2("9C0141080250320F1802104A08"), 1);
}
