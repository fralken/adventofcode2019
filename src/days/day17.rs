use std::fs;
use crate::intcode::{ IntCode, extract_codes };

#[derive(Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Clone, PartialEq)]
enum Rotation {
    Left,
    Right
}

#[derive(Clone, PartialEq)]
enum Command {
    Rotate(Rotation),
    Move(usize),
    Function(u8)
}

type Position = (usize, usize);

impl Direction {
    fn get_direction(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    fn get_next_position(&self, p: &Position, width: usize, height: usize) -> Option<Position> {
        match self{
            Direction::North => if p.1 == 0 { None } else { Some((p.0, p.1 - 1)) },
            Direction::East => if p.0 == width - 1 { None } else { Some((p.0 + 1, p.1)) },
            Direction::South => if p.1 == height - 1 { None } else  { Some((p.0, p.1 + 1)) },
            Direction::West => if p.0 == 0 { None } else { Some((p.0 - 1, p.1)) }
        }
    }
}

impl Rotation {
    fn print(&self) -> char {
        match self {
            Rotation::Left => 'L',
            Rotation::Right => 'R'
        }
    }
}

impl Command {
    fn print(&self) -> String {
        match self {
            Command::Rotate(r) => format!("{}", r.print()),
            Command::Move(m) => format!("{}", m),
            Command::Function(n) => format!("{}", (n + b'A') as char)
        }
    }

    fn is_rotate(&self) -> bool {
        match self {
            Command::Rotate(_) => true,
            _ => false
        }
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day17.txt")
        .expect("Something went wrong reading the file");

    let grid = extract_grid(&extract_codes(&contents));
    let value = impl_first_star(&grid);

    println!("day 17.1 - sum of alignment parameters: {}", value);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day17.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let grid = extract_grid(&codes);
    let sequence = impl_second_star(&grid);
    let input = format!("{}\nn\n", sequence);

    let mut ascii = IntCode::new(codes);
    ascii.init_code(2);
    ascii.write_string(&input);
    ascii.process();
    let value = ascii.read().pop().unwrap();

    println!("day 17.2 - dust collected by vacuum robot: {}", value);
}

fn impl_first_star(grid: &[Vec<char>])-> usize {
    grid.iter()
        .enumerate()
        .skip(1)
        .take(grid.len() - 2)
        .map(|(i, r)|
            r.iter().enumerate()
                .skip(1)
                .take(r.len() - 2)
                .map(|(j, &c)|
                    if c == '#' &&
                        r[j-1] == '#' &&
                        r[j+1] == '#' &&
                        grid[i-1][j] == '#' &&
                        grid[i+1][j] == '#'
                    { i * j } else { 0 }
                ).sum::<usize>()
        ).sum::<usize>()
}

fn impl_second_star(grid: &[Vec<char>])-> String {
    let commands = find_commands(&grid);
    let sequence = find_sequence(&commands);
    sequence_to_string(&sequence)
}

fn extract_grid(codes: &[i64]) -> Vec<Vec<char>>{
    let mut ascii = IntCode::new(codes.to_vec());
    ascii.process();

    let output = ascii.read_string();

    string_to_grid(&output)
}

fn string_to_grid(s: &str) -> Vec<Vec<char>> {
    s.split('\n')
        .filter(|r| !r.is_empty())
        .map(|r| r.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn is_path(grid: &[Vec<char>], position: &Option<Position>) -> bool {
    position.map_or_else( || false,|p| grid[p.1][p.0] == '#')
}

fn find_commands(grid: &[Vec<char>]) -> Vec<Command> {

    fn find_start(grid: &[Vec<char>]) -> (Position, Direction) {
        for (y, row) in grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if let Some(dir) = Direction::get_direction(c) {
                    return ((x, y), dir)
                }
            }
        }
        panic!("start not found")
    }

    let width = grid[0].len();
    let height = grid.len();
    let mut moves = Vec::new();
    let start = find_start(&grid);
    let mut cur_pos = start.0;
    let mut cur_dir = start.1;
    loop {
        let cur_dir_left = cur_dir.rotate_left();
        let cur_dir_right = cur_dir.rotate_right();
        let next_pos_left = cur_dir_left.get_next_position(&cur_pos, width, height);
        let next_pos_right = cur_dir_right.get_next_position(&cur_pos, width, height);
        let next_pos = if is_path(grid, &next_pos_left) {
            moves.push(Command::Rotate(Rotation::Left));
            cur_dir = cur_dir_left;
            next_pos_left
        } else if is_path(grid, &next_pos_right) {
            moves.push(Command::Rotate(Rotation::Right));
            cur_dir = cur_dir_right;
            next_pos_right
        } else {
            let next_pos_forward = cur_dir.get_next_position(&cur_pos, width, height);
            if is_path(grid, &next_pos_forward) {
                next_pos_forward
            } else {
                break;
            }
        };
        cur_pos = next_pos.unwrap();
        let mut steps = 1;
        while is_path(grid, &cur_dir.get_next_position(&cur_pos, width, height)) {
            steps += 1;
            cur_pos = cur_dir.get_next_position(&cur_pos, width, height).unwrap();
        }
        moves.push(Command::Move(steps))
    }
    moves
}

fn commands_to_string(commands: &[Command]) -> String {
    commands.iter()
        .skip(1)
        .fold(commands[0].print(), | a, c| format!("{},{}", a, c.print()))
}

fn sequence_to_string(sequence: &[Vec<Command>]) -> String {
    sequence.first()
        .map_or_else(String::new, |f|
            sequence.iter()
                .skip(1)
                .fold(commands_to_string(f), |a, c| format!("{}\n{}", a, commands_to_string(c)))
        )
}

fn find_sequence(commands: &[Command]) -> Vec<Vec<Command>> {

    fn repetitions(commands: &[Command], mut sequence: Vec<Command>, functions: Vec<Vec<Command>>) -> Option<(Vec<Command>, Vec<Vec<Command>>)>{
        let mut repeat = true;
        let mut next_commands = commands;
        while repeat {
            repeat = false;
            for (i, prev) in functions.iter().enumerate() {
                if prev.len() <= next_commands.len() && next_commands[..prev.len()] == *prev.as_slice() {
                    sequence.push(Command::Function(i as u8));
                    next_commands = &next_commands[prev.len()..];
                    repeat = true;
                    break;
                }
            }
        }

        // max 3 functions
        if functions.len() == 3 {
            return if !next_commands.is_empty() { None } else { Some((sequence, functions)) };
        }

        // no more than 10 commands per function
        let mut function_len = next_commands.len().min(10);

        // no more than 20 chars per function
        while commands_to_string(&next_commands[..function_len]).len() > 20 || next_commands[function_len-1].is_rotate() {
            function_len -= 1
        }

        while function_len > 0 {
            let mut more_functions = functions.clone();
            more_functions.push(next_commands[..function_len].to_vec());
            let r = repetitions(next_commands, sequence.clone(), more_functions);
            if r.is_some() {
                return r;
            }
            function_len -= 2;
        }

        None
    }

    if let Some((sequence, mut functions)) = repetitions(commands, Vec::new(), Vec::new()) {
        functions.insert(0, sequence);
        functions
    } else {
        Vec::new()
    }
}

#[test]
fn test0_first_star() {
    let grid = "\
        ..#..........\n\
        ..#..........\n\
        #######...###\n\
        #.#...#...#.#\n\
        #############\n\
        ..#...#...#..\n\
        ..#####...^..";
    assert_eq!(impl_first_star(&string_to_grid(&grid)), 76)
}

#[test]
fn test0_second_star() {
    let grid = "\
        #######...#####\n\
        #.....#...#...#\n\
        #.....#...#...#\n\
        ......#...#...#\n\
        ......#...###.#\n\
        ......#.....#.#\n\
        ^########...#.#\n\
        ......#.#...#.#\n\
        ......#########\n\
        ........#...#..\n\
        ....#########..\n\
        ....#...#......\n\
        ....#...#......\n\
        ....#...#......\n\
        ....#####......";
    assert_eq!(impl_second_star(&string_to_grid(&grid)), "\
        A,B,C\n\
        R,8,R,8,R,4,R,4,R,8\n\
        L,6,L,2,R,4,R,4,R,8\n\
        R,8,R,8,L,6,L,2")
}