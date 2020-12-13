fn parse_input(s: &str) -> (u32, Vec<u32>) {
    let lines = s.split('\n').collect::<Vec<_>>();
    let arrival_time = lines.get(0).unwrap().parse::<u32>().unwrap();
    let buses = lines
        .get(1)
        .unwrap()
        .split(',')
        .filter(|c| *c != "x")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (arrival_time, buses)
}

fn parse_schedule(s: &str) -> Vec<(usize, u32)> {
    s.split(',')
        .enumerate()
        .filter(|(_, c)| *c != "x")
        .map(|(n, s)| (n, s.parse::<u32>().unwrap()))
        .collect::<Vec<(usize, u32)>>()
}

fn parse_input2(s: &str) -> Vec<(usize, u32)> {
    let lines = s.split('\n').collect::<Vec<_>>();
    parse_schedule(lines.get(1).unwrap())
}

fn part1((arrival_time, buses): (u32, Vec<u32>)) -> u32 {
    let (min_bus, min_time) = &buses
        .into_iter()
        .map(|bus| {
            let mut m = bus;
            while m < arrival_time {
                m += bus;
            }
            (bus, m)
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    (min_time - arrival_time) * min_bus
}

/* Fairly inefficient brute-force approach. Took ~30 mins on my machine to find the solution */
fn part2(input: Vec<(usize, u32)>, start_timestamp: u64) -> u64 {
    let schedule = &input
        .iter()
        .map(|(a, b)| (*a as u64, *b as u64))
        .collect::<Vec<_>>();
    let (max_bus_offset, max_bus) = &schedule
        .clone()
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    let mut t: u64 = start_timestamp;
    while (t + *max_bus_offset) % max_bus != 0 {
        t += 1;
    }
    let inc = *max_bus;
    loop {
        if (&schedule)
            .iter()
            .all(|(offset, bus)| (t + *offset) % *bus == 0)
        {
            break;
        }
        t += inc;
    }
    t
}

fn main() {
    println!("Part 1: {:?}", part1(parse_input(include_str!("in.txt"))));
    println!(
        "Part 2: {:?}",
        part2(parse_input2(include_str!("in.txt")), 100000000000000)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(include_str!("test.txt"))), 295);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input2(include_str!("test.txt")), 0), 1068781);
        assert_eq!(part2(parse_schedule("17,x,13,19"), 0), 3417);
        assert_eq!(part2(parse_schedule("67,7,59,61"), 0), 754018);
        assert_eq!(part2(parse_schedule("67,x,7,59,61"), 0), 779210);
        assert_eq!(part2(parse_schedule("67,7,x,59,61"), 0), 1261476);
        assert_eq!(part2(parse_schedule("1789,37,47,1889"), 0), 1202161486);
    }
}
