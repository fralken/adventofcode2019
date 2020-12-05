mod days;
use days::*;
use std::env;

fn main() {
    let days = [
        [day01::first_star, day01::second_star],
        [day02::first_star, day02::second_star],
        [day03::first_star, day03::second_star],
        [day04::first_star, day04::second_star],
        [day05::first_star, day05::second_star],
        [day06::first_star, day06::second_star],
        [day07::first_star, day07::second_star],
        [day08::first_star, day08::second_star],
        [day09::first_star, day09::second_star],
        [day10::first_star, day10::second_star],
        [day11::first_star, day11::second_star],
        [day12::first_star, day12::second_star],
        [day13::first_star, day13::second_star],
        [day14::first_star, day14::second_star],
        [day15::first_star, day15::second_star],
        [day16::first_star, day16::second_star],
        [day17::first_star, day17::second_star],
        [day18::first_star, day18::second_star],
        [day19::first_star, day19::second_star],
        [day20::first_star, day20::second_star],
        [day21::first_star, day21::second_star],
        [day22::first_star, day22::second_star],
        [day23::first_star, day23::second_star],
        [day24::first_star, day24::second_star],
        [day25::first_star, day25::second_star]
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day = args[1]
            .parse::<usize>()
            .unwrap_or_else(|_|
                panic!("you must enter a number between 1 and {}", days.len())) - 1;
        if day >= days.len() {
            panic!("you must enter a number between 1 and {}", days.len());
        } else if args.len() > 2 {
            let star = args[2].parse::<usize>().expect("you must enter a number between 1 and 2") - 1;
            if star > 1 {
                panic!("you must enter a number between 1 and 2");
            } else {
                days[day][star]();
            }
        } else {
            for star in days[day].iter() {
                star();
            }
        }
    } else {
        for day in days.iter() {
            for star in day.iter() {
                star();
            }
        }
    }
}

