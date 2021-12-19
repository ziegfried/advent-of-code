fn count_digits(numbers: &Vec<&str>, pos: usize) -> (usize, usize) {
    let chars = numbers
        .iter()
        .map(|line| line[pos..pos + 1].to_string())
        .collect::<Vec<_>>();

    (
        chars.iter().filter(|&c| c == "0").count(),
        chars.iter().filter(|&c| c == "1").count(),
    )
}

fn part1(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let len = lines[0].len();

    let digit_counts = (0..len)
        .map(|i| count_digits(&lines, i))
        .collect::<Vec<_>>();

    let epsilon = u32::from_str_radix(
        digit_counts
            .iter()
            .map(|(z, o)| if o > z { '1' } else { '0' })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    let gamma = u32::from_str_radix(
        digit_counts
            .iter()
            .map(|(z, o)| if o > z { '0' } else { '1' })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    gamma * epsilon
}

fn part2(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let len = lines[0].len();

    let mut o2_gen_candidates = lines.clone();
    for i in 0..len {
        let (z, o) = count_digits(&o2_gen_candidates.clone(), i);
        let ch = if z > o { "0" } else { "1" };
        o2_gen_candidates = o2_gen_candidates
            .iter()
            .filter(|&c| c[i..i + 1] == *ch)
            .map(|v| v.clone())
            .collect::<Vec<&str>>();
        if o2_gen_candidates.len() == 1 {
            break;
        }
    }
    let o2_gen_rating = u32::from_str_radix(o2_gen_candidates[0], 2).unwrap();

    let mut co2_scrub_candidates = lines.clone();
    for i in 0..len {
        let (z, o) = count_digits(&co2_scrub_candidates.clone(), i);
        let ch = if z > o { "1" } else { "0" };
        co2_scrub_candidates = co2_scrub_candidates
            .iter()
            .filter(|&c| c[i..i + 1] == *ch)
            .map(|v| v.clone())
            .collect::<Vec<&str>>();
        if co2_scrub_candidates.len() == 1 {
            break;
        }
    }
    let co2_scrub_rating = u32::from_str_radix(co2_scrub_candidates[0], 2).unwrap();

    o2_gen_rating * co2_scrub_rating
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 198);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 230);
}
