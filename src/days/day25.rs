use std::fs;
use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use crate::intcode::{ IntCode, Status, extract_codes };

pub fn first_star() {
    let last_message = impl_first_star(false);

    println!("day 25.1 - message containing password for the main airlock: {}", last_message);
}

pub fn second_star() {
    println!("day 25.2 - THE END ... Thanks for watching");
}

pub fn impl_first_star(verbose: bool) -> String {
    let contents = fs::read_to_string("./input/day25.txt")
        .expect("Something went wrong reading the file");

    let print = |s: &str| { if verbose { print!("{}", s)} };

    let mut droid = IntCode::new(extract_codes(&contents));
    let path_to_target = visit_and_collect(&mut droid, None, &print);
    let last_door = goto_target_location(&mut droid, &path_to_target, &print);
    pass_last_door(&mut droid, last_door, &|_| ()); // never print attempts to cross last door

    droid.read_string()
}

fn visit_and_collect(droid: &mut IntCode, from: Option<&str>, print: &impl Fn(&str)) -> Vec<String> {
    lazy_static! {
        static ref BAD_ITEMS: HashSet<&'static str> = vec![
            "infinite loop",
            "molten lava",
            "escape pod",
            "giant electromagnet",
            "photons",
        ]
        .into_iter()
        .collect();
    }

    droid.process();
    let output = droid.read_string();
    print(&output);

    // Automatically pick up "good" items
    for item in parse_items(&output) {
        if !BAD_ITEMS.contains(&item[..]) {
            let command = format!("take {}\n", item);
            print(&command);
            droid.write_string(&command);
            droid.process();
            print(&droid.read_string());
        }
    }

    let mut path_to_target = Vec::new();

    let location = parse_location(&output).unwrap();
    let doors = parse_doors(&output);
    if location == "Security Checkpoint" {
        // If we are here we are coming from somewhere, so 'from' is Some(dir).
        path_to_target.push(from.unwrap().to_string());
        // Next location is "Pressure-Sensitive Floor", stop visiting doors because
        // we need to collect all useful items first, but save door's direction
        doors.into_iter()
            .filter(|&d| Some(opposite_direction(&d)) != from)
            .for_each(|d| path_to_target.push(d.to_string()));
    } else {
        // Explore all doors
        for dir in doors {
            // Do not back track while exploring
            if Some(opposite_direction(&dir)) != from {
                // Go through door
                let command = format!("{}\n", &dir);
                print(&command);
                droid.write_string(&command);
                // Explore
                let path = visit_and_collect(droid, Some(dir), print);
                if !path.is_empty() {
                    if let Some(f) = from {
                        path_to_target.push(f.to_string());
                    }
                    path_to_target.extend_from_slice(&path);
                }
                // Go back
                let command = format!("{}\n", opposite_direction(&dir));
                print(&command);
                droid.write_string(&command);
                droid.process();
                print(&droid.read_string());
            }
        }
    }

    path_to_target
}

fn goto_target_location<'a>(droid: &mut IntCode, path_to_target: &'a [String], print: &impl Fn(&str)) -> &'a String {
    // do not cross the last door
    for dir in path_to_target.iter().take(path_to_target.len() - 1) {
        let command = format!("{}\n", dir);
        print(&command);
        droid.write_string(&command);
        droid.process();
        print(&droid.read_string());
    }

    path_to_target.last().unwrap()
}

fn pass_last_door(droid: &mut IntCode, last_door: &str, print: &impl Fn(&str)) {
    let command = "inv\n";
    print(&command);
    droid.write_string(&command);
    droid.process();
    let output = droid.read_string();
    print(&output);

    // true when item is taken by droid
    let mut items = parse_inventory(&output)
        .into_iter()
        .map(|s| (s, true))
        .collect::<Vec<_>>();

    // try enter last door
    let command = format!("{}\n", last_door);
    droid.write_string(&command);
    // try any combinations of items to have the correct weight
    let mut combinations = (1 << items.len()) - 1;
    // we finish when program ends
    while droid.process() != Status::End {
        print(&droid.read_string());
        combinations -= 1;
        for (i, item) in items.iter_mut().enumerate() {
            let take = combinations & 1 << i != 0;
            if take && !item.1 {
                let command = format!("take {}\n", item.0);
                print(&command);
                droid.write_string(&command);
                droid.process();
                print(&droid.read_string());
            } else if !take && item.1 {
                let command = format!("drop {}\n", item.0);
                print(&command);
                droid.write_string(&command);
                droid.process();
                print(&droid.read_string());
            }
            item.1 = take;
        }
        // try enter last door again
        let command = format!("{}\n", last_door);
        print(&command);
        droid.write_string(&command);
    }
}

fn opposite_direction(dir: &str) -> &str {
    match dir {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => panic!("unknown direction {}", dir)
    }
}

fn parse_location(s: &str) -> Option<&str> {
    lazy_static! {
        static ref LOCATION_RE: Regex = Regex::new(r"== ([^\n]+) ==").unwrap();
    }
    LOCATION_RE
        .captures_iter(s)
        .last()
        .map(|c| c.get(1).unwrap().as_str())
}

fn parse_doors(s: &str) -> Vec<&str> {
    lazy_static! {
        static ref DOORS_RE: Regex = Regex::new(r"Doors here lead:((?:\n\- \w+)+)").unwrap();
    }
    parse_dashed_list(s, &*DOORS_RE)
}

fn parse_items(s: &str) -> Vec<&str> {
    lazy_static! {
        static ref ITEMS_RE: Regex = Regex::new(r"Items here:((?:\n\- [^\n]+)+)").unwrap();
    }
    parse_dashed_list(s, &*ITEMS_RE)
}

fn parse_inventory(s: &str) -> Vec<&str> {
    lazy_static! {
        static ref ITEMS_RE: Regex = Regex::new(r"Items in your inventory:((?:\n\- [^\n]+)+)").unwrap();
    }
    parse_dashed_list(s, &*ITEMS_RE)
}

fn parse_dashed_list<'a>(s: &'a str, re: &Regex) -> Vec<&'a str> {
    if let Some(captures) = re.captures(s) {
        let list = captures.get(1).unwrap().as_str().trim();
        list.split('\n')
            .map(|line| line[2..].into())
            .collect::<Vec<&str>>()
    } else {
        Vec::new()
    }
}
