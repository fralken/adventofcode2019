use std::fs;
use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use crate::intcode::{ IntCode, Status, extract_codes, interrupt_after };

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

    let codes = extract_codes(&contents);
    let target = "Security Checkpoint";
    let mut bad_items = HashSet::new();
    let mut droid = IntCode::new(codes.clone());
    loop {
        let result = visit_and_collect(&mut droid, None, target, &mut bad_items, &print);
        match result {
            Ok(path_to_target) => {
                let last_door = goto_target_location(&mut droid, &path_to_target, &print);
                pass_last_door(&mut droid, last_door, &|_| ()); // Never print attempts to cross last door
                break;
            },
            Err(_) => droid = IntCode::new(codes.clone()) // Found bad item, reset and try again
        }
    }

    droid.read_string()
}

fn visit_and_collect(droid: &mut IntCode, from: Option<&str>, target: &str, bad_items: &mut HashSet<String>, print: &impl Fn(&str)) -> Result<Vec<String>, Status> {
    let mut status = droid.process();
    let output = droid.read_string();
    print(&output);

    // Automatically pick up items
    let items = parse_items(&output);
    for item in &items {
        if !bad_items.contains(*item) {
            let command = format!("take {}\n", item);
            print(&command);
            droid.write_string(&command);
            // We guess that if droid is still running after 5000 cycles
            // it is stuck in an infinite loop
            status = droid.process_interruptable(interrupt_after(5000));
            if status != Status::Waiting {
                bad_items.insert((*item).to_string());
                return Err(status)
            }
            print(&droid.read_string());
        }
    }

    let mut path_to_target = Vec::new();

    if let Some(location) = parse_location(&output) {
        let doors = parse_doors(&output);
        if location == target {
            if let Some(dir) = from {
                path_to_target.push(dir.to_string());
            }
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
                    let result = visit_and_collect(droid, Some(dir), target, bad_items, print);
                    match result {
                        Ok(path) =>
                            if !path.is_empty() {
                                if let Some(f) = from {
                                    path_to_target.push(f.to_string());
                                }
                                path_to_target.extend_from_slice(&path);
                            },
                        Err(status) => {
                            if status == Status::Waiting {
                                // Items in this room are blocking the droid here, consider them bad
                                for item in &items {
                                    bad_items.insert((*item).to_string());
                                }
                            }
                            return Err(Status::End)
                        }
                    }
                    // Go back
                    let command = format!("{}\n", opposite_direction(&dir));
                    go(droid, &command, &print);
                }
            }
        }
    } else {
        // We are stuck in the same room, waiting for input
        return Err(status)
    }

    Ok(path_to_target)
}

fn goto_target_location<'a>(droid: &mut IntCode, path_to_target: &'a [String], print: &impl Fn(&str)) -> &'a String {
    // do not cross the last door
    for dir in path_to_target.iter().take(path_to_target.len() - 1) {
        let command = format!("{}\n", dir);
        go(droid, &command, &print);
    }

    path_to_target.last().unwrap()
}

fn pass_last_door(droid: &mut IntCode, last_door: &str, print: &impl Fn(&str)) {
    let command = "inv\n";
    let output = go(droid, &command, &print);

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
                go(droid, &command, &print);
            } else if !take && item.1 {
                let command = format!("drop {}\n", item.0);
                go(droid, &command, &print);
            }
            item.1 = take;
        }
        // try enter last door again
        let command = format!("{}\n", last_door);
        print(&command);
        droid.write_string(&command);
    }
}

fn go(droid: &mut IntCode, command: &str, print: &impl Fn(&str)) -> String {
    print(&command);
    droid.write_string(&command);
    droid.process();
    let output = droid.read_string();
    print(&output);
    output
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
