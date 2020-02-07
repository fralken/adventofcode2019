use std::fs;
use crate::intcode::{ IntCode, extract_codes };

#[derive(PartialEq)]
enum Command {
    North = 1,
    South = 2,
    West = 3,
    East = 4
}

impl Command {
    fn opposite(&self) -> &'static Self {
        match self {
            Command::North => &Command::South,
            Command::South => &Command::North,
            Command::West => &Command::East,
            Command::East => &Command::West
        }
    }

    fn next(&self) -> &'static Self {
        match self {
            Command::North => &Command::East,
            Command::South => &Command::West,
            Command::West => &Command::North,
            Command::East => &Command::South
        }
    }

    fn new_position(&self, position: (i64, i64)) -> (i64, i64) {
        match self {
            Command::North => (position.0, position.1 - 1),
            Command::South => (position.0, position.1 + 1),
            Command::West => (position.0 - 1, position.1),
            Command::East => (position.0 + 1, position.1)
        }
    }

    fn to(&self) -> i64 {
        match self {
            Command::North => 1,
            Command::South => 2,
            Command::West => 3,
            Command::East => 4
        }
    }

    fn print(&self) -> char {
        match self {
            Command::North => 'N',
            Command::South => 'S',
            Command::West => 'W',
            Command::East => 'E'
        }
    }

    fn start() -> &'static Self { &Command::North }
}

#[derive(PartialEq)]
enum Status {
    HitWall = 0,
    Moved = 1,
    FoundOxygen = 2
}

impl From<i64> for Status {
    fn from(n: i64) -> Self {
        match n {
            0 => Status::HitWall,
            1 => Status::Moved,
            2 => Status::FoundOxygen,
            _ => unreachable!()
        }
    }
}

struct Step<'a> {
    command: &'a Command,
    position: (i64, i64)
}

impl<'a> Step<'a> {
    fn new(command: &'a Command, position: (i64, i64)) -> Self {
        Step { command, position }
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day15.txt")
        .expect("Something went wrong reading the file");

    let mut steps = vec![Step::new(Command::start(), (0, 0))];
    let mut droid = IntCode::new(extract_codes(&contents));
    let steps_to_oxygen = run(&mut droid, &mut steps);

    println!("day 15.1 - fewest number of movement commands: {}", steps_to_oxygen);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day15.txt")
        .expect("Something went wrong reading the file");

    // first find oxygen system (first star)
    let mut steps = vec![Step::new(Command::start(), (0, 0))];
    let mut droid = IntCode::new(extract_codes(&contents));
    run(&mut droid, &mut steps);
    // then from oxygen system find longest path
    let oxygen_position = steps.last().unwrap().position;
    let mut steps = vec![Step::new(Command::start(), oxygen_position)];
    let max_path = run(&mut droid, &mut steps);

    println!("day 15.2 - minutes to fill with oxygen: {}", max_path);
}

fn run(droid: &mut IntCode, steps: &mut Vec<Step>) -> usize {
    let mut max_path = 0;
    let mut next_command = steps.last().unwrap().command;

    fn turn<'a>(command: &'a Command, steps: &[Step]) -> &'a Command {
        let len = steps.len();
        let mut next_command = command.next();
        if len > 1 && (next_command == steps[len - 2].command.opposite()) {
            next_command = next_command.next();
        };
        next_command
    }

    fn backtrack<'a>(droid: &mut IntCode, command: &'a Command, steps: &mut Vec<Step>) -> &'a Command {
        let mut next_command = command;
        loop {
            let len = steps.len();
            if len > 1 && next_command == steps[len - 2].command {
                go(droid, &next_command.opposite());
                steps.pop();
                next_command = turn(&next_command, &steps);
            } else {
                if len == 1 && next_command == Command::start() { steps.pop(); }
                return next_command;
            }
        }
    }

    fn go(droid: &mut IntCode, command: &Command) -> Status {
        droid.write_one(command.to());
        droid.process();
        Status::from(droid.read_one().unwrap())
    }

    while !steps.is_empty() {
        steps.last_mut().unwrap().command = next_command;
        let status = go(droid, &next_command);
        match status {
            Status::HitWall => {
                next_command = turn(next_command, &steps);
                next_command = backtrack(droid, next_command, steps);
            },
            Status::Moved => {
                let new_position = next_command.new_position(steps.last().unwrap().position);
                steps.push(Step::new(next_command, new_position));
                if max_path < steps.len() { max_path = steps.len() }
            },
            Status::FoundOxygen => return steps.len()
        }
    }
    max_path - 1
}

#[allow(dead_code)]
fn print_path(steps: &[Step]) -> String {
    let min_x = steps.iter().map(|s| s.position.0).min().unwrap();
    let min_y = steps.iter().map(|s| s.position.1).min().unwrap();
    let max_x = steps.iter().map(|s| s.position.0).max().unwrap();
    let max_y = steps.iter().map(|s| s.position.1).max().unwrap();
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut s = vec![vec![' '; width]; height];
    for step in steps {
        s[(step.position.1 - min_y) as usize][(step.position.0 - min_x) as usize] = step.command.print();
    }
    s.iter().fold(String::new(), |a, r| format!("{}{}\n", a, r.iter().collect::<String>()))
}
