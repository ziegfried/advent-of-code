#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Operator(usize, usize, Box<Vec<Packet>>),
    LiteralValue(usize, usize),
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

fn read_value(bits: &mut Vec<u8>) -> Packet {
    let version = parse_num(&take_bits(bits, 3));
    let type_id = parse_num(&take_bits(bits, 3));
    assert_eq!(type_id, 4);
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
    Packet::LiteralValue(version, parse_num(&content))
}

fn read_operator(bits: &mut Vec<u8>) -> Packet {
    let version = parse_num(&take_bits(bits, 3));
    let type_id = parse_num(&take_bits(bits, 3));
    assert!(type_id != 4);
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
    Packet::Operator(version, type_id, Box::new(sub))
}

fn read_packet(bits: &mut Vec<u8>) -> Option<Packet> {
    if bits.len() <= 8 && bits.iter().all(|b| b == &0) {
        return None;
    }
    let type_id = parse_num(&bits[3..6]);
    if type_id == 4 {
        Some(read_value(bits))
    } else {
        Some(read_operator(bits))
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
        Packet::LiteralValue(version, _) => *version,
        Packet::Operator(version, _, children) => {
            let child_sum: usize = children.iter().map(sum_version).sum();
            version + child_sum
        }
    }
}

fn part1(input: &str) -> usize {
    let mut bits = hex_to_binary(input);
    let packet = read_packet(&mut bits).unwrap();
    sum_version(&packet)
}

fn compute_operator_result(type_id: usize, children: &Vec<Packet>) -> usize {
    match type_id {
        0 => children.iter().map(|child| compute_result(child)).sum(),
        1 => children
            .iter()
            .map(|child| compute_result(child))
            .fold(1, |a, b| a * b),
        2 => children
            .iter()
            .map(|child| compute_result(child))
            .min()
            .unwrap(),
        3 => children
            .iter()
            .map(|child| compute_result(child))
            .max()
            .unwrap(),
        5 => {
            let mut res = children.iter().map(|child| compute_result(child));
            let first = res.next().unwrap();
            let second = res.next().unwrap();
            if first > second {
                1
            } else {
                0
            }
        }
        6 => {
            let mut res = children.iter().map(|child| compute_result(child));
            let first = res.next().unwrap();
            let second = res.next().unwrap();
            if first < second {
                1
            } else {
                0
            }
        }
        7 => {
            let mut res = children.iter().map(|child| compute_result(child));
            let first = res.next().unwrap();
            let second = res.next().unwrap();
            if first == second {
                1
            } else {
                0
            }
        }
        _ => panic!(),
    }
}

fn compute_result(packet: &Packet) -> usize {
    match packet {
        Packet::LiteralValue(_, v) => *v,
        Packet::Operator(_, type_id, children) => compute_operator_result(*type_id, children),
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
