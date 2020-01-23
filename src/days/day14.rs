use std::fs;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Chemical<'a> {
    name: &'a str,
    quantity: u64
}

#[derive(Debug)]
struct Reaction<'a> {
    input: Vec<Chemical<'a>>,
    output: Chemical<'a>
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day14.txt")
        .expect("Something went wrong reading the file");

    let ore = impl_first_star(&contents, "ORE","FUEL", 1);
    println!("day 14.1 - minimum amount of ORE required to produce exactly 1 FUEL: {}", ore);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day14.txt")
        .expect("Something went wrong reading the file");

    let fuel = impl_second_star(&contents, "ORE","FUEL", 1_000_000_000_000);
    println!("day 14.2 - amount of FUEL produced with 1 trillion of ORE: {}", fuel);
}

fn impl_first_star(contents: &str, start: &str, end: &str, quantity: u64) -> u64 {
    let reactions = extract_reactions(&contents);
    compute_quantity(&reactions, &start, &end, quantity, &mut HashMap::new()).unwrap()
}

fn impl_second_star(contents: &str, start: &str, end: &str, ore: u64) -> u64 {
    let reactions = extract_reactions(&contents);
    let mut extra = HashMap::new();
    let mut ore_left = ore as i64;
    let mut fuel_produced = 0;
    let ore_needed_for_one_fuel = compute_quantity(&reactions, &start, &end, 1, &mut extra).unwrap() as i64;
    if ore_needed_for_one_fuel <= ore_left {
        fuel_produced += 1;
        ore_left -= ore_needed_for_one_fuel;
        while ore_left >= 0 {
            let mut estimated_fuel = (ore_left / ore_needed_for_one_fuel) as u64;
            if estimated_fuel == 0 { estimated_fuel = 1; }
            let ore_needed = compute_quantity(&reactions, &start, &end, estimated_fuel, &mut extra).unwrap() as i64;
            if ore_needed <= ore_left { fuel_produced += estimated_fuel; }
            ore_left -= ore_needed;
        }
    }
    fuel_produced
}

fn compute_quantity<'a>(reactions: &[Reaction<'a>], start: &str, end: &'a str, quantity: u64, extra: &mut HashMap<&'a str, u64>) -> Option<u64> {
    if let Some(reaction) = reactions.iter().find(|r| r.output.name == end) {
        let extra_quantity = extra.entry(end).or_insert(0);
        if quantity <= *extra_quantity {
            *extra_quantity -= quantity;
            None
        } else {
            let multiplier = find_multiplier(reaction.output.quantity, quantity - *extra_quantity);
            *extra_quantity = reaction.output.quantity * multiplier - (quantity - *extra_quantity);
            let reactors = reaction.input.iter().map(|c| Chemical { name: c.name, quantity: c.quantity * multiplier }).collect::<Vec<_>>();
            let start_quantity = reactors
                .iter()
                .flat_map(|c|
                    if c.name == start {
                        Some(c.quantity)
                    } else {
                        compute_quantity(&reactions, &start, &c.name, c.quantity, extra)
                    }
                ).sum();
            Some(start_quantity)
        }
    } else {
        None
    }
}

fn find_multiplier(quantity: u64, expected: u64) -> u64 {
    let m = expected / quantity;
    if expected % quantity == 0 { m } else { m + 1 }
}

fn extract_reactions(contents: &str) -> Vec<Reaction> {
    contents
        .lines()
        .map(|l| {
            let v = l.split(" => ")
                .map(|a|
                    a.split(", ").map(|b| {
                        let c = b.split(' ').collect::<Vec<_>>();
                        Chemical { name: c[1], quantity: c[0].parse::<u64>().unwrap() }
                    }).collect::<Vec<_>>()
                ).collect::<Vec<_>>();
            Reaction { input: v[0].to_owned(), output: *v[1].first().unwrap() }
        }).collect()
}

#[test]
fn test0_first_star() {
    let reactions = "\
        10 ORE => 10 A\n\
        1 ORE => 1 B\n\
        7 A, 1 B => 1 C\n\
        7 A, 1 C => 1 D\n\
        7 A, 1 D => 1 E\n\
        7 A, 1 E => 1 FUEL";
    assert_eq!(impl_first_star(reactions, "ORE", "FUEL", 1), 31);
}

#[test]
fn test1_first_star() {
    let reactions = "\
        9 ORE => 2 A\n\
        8 ORE => 3 B\n\
        7 ORE => 5 C\n\
        3 A, 4 B => 1 AB\n\
        5 B, 7 C => 1 BC\n\
        4 C, 1 A => 1 CA\n\
        2 AB, 3 BC, 4 CA => 1 FUEL";
    assert_eq!(impl_first_star(reactions, "ORE", "FUEL", 1), 165);
}

#[test]
fn test2_first_star() {
    let reactions = "\
        157 ORE => 5 NZVS\n\
        165 ORE => 6 DCFZ\n\
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
        179 ORE => 7 PSHF\n\
        177 ORE => 5 HKGWZ\n\
        7 DCFZ, 7 PSHF => 2 XJWVT\n\
        165 ORE => 2 GPVTF\n\
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    assert_eq!(impl_first_star(reactions, "ORE", "FUEL", 1), 13_312);
}

#[test]
fn test3_first_star() {
    let reactions = "\
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
        17 NVRVD, 3 JNWZP => 8 VPVL\n\
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
        22 VJHF, 37 MNCFX => 5 FWMGM\n\
        139 ORE => 4 NVRVD\n\
        144 ORE => 7 JNWZP\n\
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
        145 ORE => 6 MNCFX\n\
        1 NVRVD => 8 CXFTF\n\
        1 VJHF, 6 MNCFX => 4 RFSQX\n\
        176 ORE => 6 VJHF";
    assert_eq!(impl_first_star(reactions, "ORE", "FUEL", 1), 180_697);
}

#[test]
fn test4_first_star() {
    let reactions = "\
        171 ORE => 8 CNZTR\n\
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
        114 ORE => 4 BHXH\n\
        14 VRPVC => 6 BMBT\n\
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
        5 BMBT => 4 WPTQ\n\
        189 ORE => 9 KTJDG\n\
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
        12 VRPVC, 27 CNZTR => 2 XDBXC\n\
        15 KTJDG, 12 BHXH => 5 XCVML\n\
        3 BHXH, 2 VRPVC => 7 MZWV\n\
        121 ORE => 7 VRPVC\n\
        7 XCVML => 6 RJRHP\n\
        5 BHXH, 4 VRPVC => 5 LTCX";
    assert_eq!(impl_first_star(reactions, "ORE", "FUEL", 1), 2_210_736);
}

#[test]
fn test0_second_star() {
    let reactions = "\
        157 ORE => 5 NZVS\n\
        165 ORE => 6 DCFZ\n\
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
        179 ORE => 7 PSHF\n\
        177 ORE => 5 HKGWZ\n\
        7 DCFZ, 7 PSHF => 2 XJWVT\n\
        165 ORE => 2 GPVTF\n\
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    assert_eq!(impl_second_star(reactions, "ORE", "FUEL", 1_000_000_000_000), 82_892_753);
}

#[test]
fn test1_second_star() {
    let reactions = "\
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
        17 NVRVD, 3 JNWZP => 8 VPVL\n\
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
        22 VJHF, 37 MNCFX => 5 FWMGM\n\
        139 ORE => 4 NVRVD\n\
        144 ORE => 7 JNWZP\n\
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
        145 ORE => 6 MNCFX\n\
        1 NVRVD => 8 CXFTF\n\
        1 VJHF, 6 MNCFX => 4 RFSQX\n\
        176 ORE => 6 VJHF";
    assert_eq!(impl_second_star(reactions, "ORE", "FUEL", 1_000_000_000_000), 5_586_022);
}

#[test]
fn test2_second_star() {
    let reactions = "\
        171 ORE => 8 CNZTR\n\
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
        114 ORE => 4 BHXH\n\
        14 VRPVC => 6 BMBT\n\
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
        5 BMBT => 4 WPTQ\n\
        189 ORE => 9 KTJDG\n\
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
        12 VRPVC, 27 CNZTR => 2 XDBXC\n\
        15 KTJDG, 12 BHXH => 5 XCVML\n\
        3 BHXH, 2 VRPVC => 7 MZWV\n\
        121 ORE => 7 VRPVC\n\
        7 XCVML => 6 RJRHP\n\
        5 BHXH, 4 VRPVC => 5 LTCX";
    assert_eq!(impl_second_star(reactions, "ORE", "FUEL", 1_000_000_000_000), 460_664);
}
