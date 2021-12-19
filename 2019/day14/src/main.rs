use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    amount: u64,
}

#[derive(Debug, Clone)]
struct Reaction {
    result: Ingredient,
    ingredients: Vec<Ingredient>,
}

fn parse_reactions(input: &str) -> HashMap<String, Reaction> {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, newline, u64},
        multi::separated_list1,
        IResult,
    };
    fn ingredient(input: &str) -> IResult<&str, Ingredient> {
        let (input, amount) = u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, name) = alpha1(input)?;
        Ok((
            input,
            Ingredient {
                amount,
                name: name.to_string(),
            },
        ))
    }
    fn reaction(input: &str) -> IResult<&str, Reaction> {
        let (input, ingredients) = separated_list1(tag(", "), ingredient)(input)?;
        let (input, _) = tag(" => ")(input)?;
        let (input, result) = ingredient(input)?;
        Ok((
            input,
            Reaction {
                result,
                ingredients,
            },
        ))
    }
    let (input, reactions) = separated_list1(newline, reaction)(input).unwrap();
    assert_eq!(input, "");
    reactions
        .iter()
        .map(|r| (r.result.name.clone(), r.clone()))
        .collect()
}

fn ceiling_div(a: u64, b: u64) -> u64 {
    a / b + if a % b > 0 { 1 } else { 0 }
}

fn reduce(
    orders: &mut HashMap<String, u64>,
    overflow: &mut HashMap<String, u64>,
    reactions: &HashMap<String, Reaction>,
) {
    let ro_orders = orders.clone();
    let (order_chem, &order_amount) = ro_orders.iter().find(|(chem, _)| **chem != "ORE").unwrap();
    let overflow_amount = *overflow.get(order_chem).unwrap_or(&0);
    if overflow_amount >= order_amount {
        overflow
            .entry(order_chem.clone())
            .and_modify(|v| *v -= order_amount);
        orders.remove(order_chem);
        return;
    }
    let order_amount = order_amount - overflow_amount;
    orders.remove(order_chem);
    overflow.remove(order_chem);
    let reaction = reactions.get(order_chem).unwrap();
    let reaction_times = ceiling_div(order_amount, reaction.result.amount);
    overflow
        .entry(order_chem.clone())
        .and_modify(|v| *v += (reaction.result.amount * reaction_times) - order_amount)
        .or_insert((reaction.result.amount * reaction_times) - order_amount);
    for ingredient in reaction.ingredients.iter() {
        orders
            .entry(ingredient.name.clone())
            .and_modify(|v| *v += ingredient.amount * reaction_times)
            .or_insert(ingredient.amount * reaction_times);
    }
}

fn required_ore(reactions: &HashMap<String, Reaction>, fuel_amount: u64) -> u64 {
    let mut orders = HashMap::new();
    orders.insert("FUEL".to_string(), fuel_amount);
    let mut overflow = HashMap::new();
    loop {
        reduce(&mut orders, &mut overflow, reactions);
        if orders.len() == 1 && orders.contains_key("ORE") {
            break;
        }
    }
    *orders.get("ORE").unwrap()
}

fn part1(input: &str) -> u64 {
    required_ore(&parse_reactions(input), 1)
}

fn part2(input: &str) -> u64 {
    let reactions = parse_reactions(input);
    let holding: u64 = 1000000000000;
    let mut low = 1;
    let mut high = 2;
    while required_ore(&reactions, high) < holding {
        low = high;
        high *= 2;
    }
    loop {
        let cur = low + (high - low) / 2;
        let ore = required_ore(&reactions, cur);
        match ore.partial_cmp(&holding).unwrap() {
            Ordering::Greater => {
                high = cur;
            }
            Ordering::Less => {
                if cur - low <= 1 {
                    return cur;
                }
                low = cur;
            }
            Ordering::Equal => {
                return cur;
            }
        };
    }
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test1.txt")), 31);
    assert_eq!(part1(include_str!("test2.txt")), 165);
    assert_eq!(part1(include_str!("test3.txt")), 2210736);
    assert_eq!(part1(include_str!("test4.txt")), 13312);
    assert_eq!(part1(include_str!("test5.txt")), 180697);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test3.txt")), 460664);
    assert_eq!(part2(include_str!("test4.txt")), 82892753);
    assert_eq!(part2(include_str!("test5.txt")), 5586022);
}
