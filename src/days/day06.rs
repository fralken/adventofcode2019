#![allow(clippy::clone_double_ref)]
use std::fs;
use std::collections::BTreeMap;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day06.txt")
        .expect("Something went wrong reading the file");

    let count = impl_first_star(&contents);

    println!("day  6.1 - total number of direct and indirect orbits: {}", count);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day06.txt")
        .expect("Something went wrong reading the file");

    let count = impl_second_star(&contents, "YOU", "SAN");

    println!("day  6.2 - minimum number of orbital transfers: {}", count);
}

fn extract_orbits(contents: &str) -> Vec<(&str, &str)> {
    contents
        .lines()
        .map(|s| {
            let v: Vec<&str> = s.split(')').collect();
            (v[0], v[1])
        }).collect()
}

fn impl_first_star(contents: &str) -> u32 {
    let mut orbits = extract_orbits(contents);
    let mut orbits_count = BTreeMap::new();
    while !orbits.is_empty() {
        let position = orbits.iter().position(|(k, _)| orbits.iter().all(|(_, v)| k != v));
        if let Some(p) = position {
            let (k, v) = orbits.get(p).unwrap();
            let count = orbits_count.get(k).map_or_else(|| 1, |c| c + 1);
            orbits_count.insert(v.clone(), count);
            orbits.remove(p);
        }
    }
    orbits_count.values().sum()
}

fn impl_second_star(contents: &str, start: &str, end: &str) -> usize {
    let orbits = extract_orbits(contents);
    let mut start_orbits = Vec::new();
    let mut position = orbits.iter().position(|(_, v)| *v == start);
    while let Some(p) = position {
        let planet = orbits.get(p).unwrap().0;
        start_orbits.push(planet);
        position = orbits.iter().position(|(_, v)| *v == planet)
    }
    let mut end_orbits = Vec::new();
    position = orbits.iter().position(|(_, v)| *v == end);
    while let Some(p) = position {
        let planet = orbits.get(p).unwrap().0;
        end_orbits.push(planet);
        position = orbits.iter().position(|(_, v)| *v == planet)
    }
    if let Some(start_pos) = start_orbits.iter().position(|s| end_orbits.iter().any(|e| s == e)) {
        let end_pos = end_orbits.iter().position(|e| e == start_orbits.get(start_pos).unwrap()).unwrap();
        start_pos + end_pos
    } else {
        panic!("orbit path not found");
    }
}

#[test]
fn test0_first_star() {
    assert_eq!(impl_first_star("\
        COM)B\n\
        B)C\n\
        C)D\n\
        D)E\n\
        E)F\n\
        B)G\n\
        G)H\n\
        D)I\n\
        E)J\n\
        J)K\n\
        K)L"), 42);
}

#[test]
fn test0_second_star() {
    assert_eq!(impl_second_star("\
        COM)B\n\
        B)C\n\
        C)D\n\
        D)E\n\
        E)F\n\
        B)G\n\
        G)H\n\
        D)I\n\
        E)J\n\
        J)K\n\
        K)L\n\
        K)YOU\n\
        I)SAN", "YOU", "SAN"), 4);
}
