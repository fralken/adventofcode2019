use std::fs;
use std::cmp::Ordering;
use regex::Regex;
use num_integer::lcm;

#[derive(Clone, Debug, PartialEq)]
struct Moon {
    position: [i32; 3],
    velocity: [i32; 3]
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day12.txt")
        .expect("Something went wrong reading the file");

    let c = impl_first_star(&contents, 1000);
    println!("day 12.1 - total energy after 1000 steps: {}", c);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day12.txt")
        .expect("Something went wrong reading the file");

    let steps = impl_second_star(&contents);
    println!("day 12.2 - steps to reach first state: {}", steps);
}

fn impl_first_star(contents: &str, steps: usize) -> i32 {
    let mut moons = extract_coordinates(contents);
    for _ in 0..steps {
        update_velocity(&mut moons);
        update_position(&mut moons);
    }
    compute_energy(&moons)
}

fn impl_second_star(contents: &str) -> i64 {
    let mut moons = extract_coordinates(contents);
    let initial_state = moons.clone();
    let mut steps = 0;
    let mut axis_steps= [0_i64; 3];
    while axis_steps.iter().any(|a| *a == 0) {
        update_velocity(&mut moons);
        update_position(&mut moons);
        steps += 1;
        for (k, axis_step) in axis_steps.iter_mut().enumerate().filter(|(_, &mut a)| a == 0) {
            if moons.iter()
                .zip(&initial_state)
                .all(|(m, s)| m.position[k] == s.position[k] && m.velocity[k] == s.velocity[k]) {
                *axis_step = steps;
            }
        }
    }
    lcm(axis_steps[0], lcm(axis_steps[1], axis_steps[2]))
}

fn extract_coordinates(contents: &str) -> Vec<Moon>{
    let re = Regex::new(r"<x=([-]?\d+),\sy=([-]?\d+),\sz=([-]?\d+)>").unwrap();
    re.captures_iter(contents)
        .map(|cap|
            Moon {
                position: [cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()],
                velocity: [0; 3]
            }
        )
        .collect()
}

fn update_velocity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i+1..moons.len() {
            for k in 0..3 {
                let pi = moons[i].position[k];
                let pj = moons[j].position[k];
                match pi.cmp(&pj) {
                    Ordering::Greater => {
                        moons[i].velocity[k] -= 1;
                        moons[j].velocity[k] += 1;
                    },
                    Ordering::Less => {
                        moons[i].velocity[k] += 1;
                        moons[j].velocity[k] -= 1;
                    },
                    Ordering::Equal => ()
                }
            }
        }
    }
}

fn update_position(moons: &mut [Moon]) {
    for moon in moons {
        for k in 0..3 {
            moon.position[k] += moon.velocity[k];
        }
    }
}

fn compute_energy(moons: &[Moon]) -> i32 {
    moons.iter().map(|moon| {
        let pot: i32 = moon.position.iter().map(|p| p.abs()).sum();
        let kin: i32 = moon.velocity.iter().map(|v| v.abs()).sum();
        pot * kin
    }).sum()
}

#[test]
fn test0_first_star() {
    let positions = "\
        <x=-1, y=0, z=2>\
        <x=2, y=-10, z=-7>\
        <x=4, y=-8, z=8>\
        <x=3, y=5, z=-1>";
    assert_eq!(impl_first_star(positions, 10), 179);
}

#[test]
fn test1_first_star() {
    let positions = "\
        <x=-8, y=-10, z=0>\
        <x=5, y=5, z=10>\
        <x=2, y=-7, z=3>\
        <x=9, y=-8, z=-3>";
    assert_eq!(impl_first_star(positions, 100), 1940);
}

#[test]
fn test0_second_star() {
    let positions = "\
        <x=-1, y=0, z=2>\
        <x=2, y=-10, z=-7>\
        <x=4, y=-8, z=8>\
        <x=3, y=5, z=-1>";
    assert_eq!(impl_second_star(positions), 2772);
}

#[test]
fn test1_second_star() {
    let positions = "\
        <x=-8, y=-10, z=0>\
        <x=5, y=5, z=10>\
        <x=2, y=-7, z=3>\
        <x=9, y=-8, z=-3>";
    assert_eq!(impl_second_star(positions), 4686774924);
}
