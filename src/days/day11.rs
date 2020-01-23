use std::fs;
use std::collections::HashMap;
use crate::intcode::{ IntCode, extract_codes };

pub fn first_star() {
    let contents = fs::read_to_string("./input/day11.txt")
        .expect("Something went wrong reading the file");

    let panels = paint(&contents, 0);

    println!("day 11.1 - number of panels painted at least once: {}", panels.len());
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day11.txt")
        .expect("Something went wrong reading the file");

    let panels = paint(&contents, 1);
    let identifier = draw(&panels);

    println!("day 11.2 - painted registration identifier:\n{}", identifier);
}

fn paint(contents: &str, start_color: i64) -> HashMap<(i32, i32), i64> {
    let mut position = (0, 0);
    let mut direction = (0, -1);
    let mut current_color = start_color;
    let mut finished = false;
    let mut robot = IntCode::new(extract_codes(&contents));
    let mut panels = HashMap::new();
    while !finished {
        robot.write(&[current_color]);
        finished = robot.interpreter();
        if !finished {
            let mut output = robot.read();
            let new_color = output.remove(0);
            let rotation = output.remove(0);
            if new_color != current_color {
                panels.insert(position, new_color);
            }
            direction = rotate(direction, rotation);
            position.0 += direction.0;
            position.1 += direction.1;
            current_color = *panels.get(&position).or(Some(&0)).unwrap();
        }
    }
    panels
}

fn rotate(direction: (i32, i32), rotation: i64) -> (i32, i32) {
    let new_dir = match direction {
        (0, -1) => if rotation == 0 { (-1, 0) } else if rotation == 1 { (1, 0) } else { (0, 0) },
        (1, 0) => if rotation == 0 { (0, -1) } else if rotation == 1 { (0, 1) } else { (0, 0) },
        (0, 1) => if rotation == 0 { (1, 0) } else if rotation == 1 { (-1, 0) } else { (0, 0) },
        (-1, 0) => if rotation == 0 { (0, 1) } else if rotation == 1 { (0, -1) } else { (0, 0) },
        _ => unreachable!()
    };
    if new_dir == (0, 0) {
        panic!("wrong rotation {}", rotation);
    }
    new_dir
}

fn draw(panels: &HashMap<(i32, i32), i64>) -> String {
    let offset_x = panels.keys().map(|p| p.0).min().unwrap();
    let offset_y = panels.keys().map(|p| p.1).min().unwrap();
    let width = panels.keys().map(|p| p.0).max().unwrap() - offset_x + 1;
    let height = panels.keys().map(|p| p.1).max().unwrap() - offset_y + 1;
    let mut output = vec![vec![' '; width as usize]; height as usize];
    for (p, c) in panels.iter() {
        if *c == 1 {
            let line = &mut output[(p.1 - offset_y) as usize];
            line[(p.0 - offset_x) as usize] = '*';
         }
    }
    output.iter().fold(String::new(), |s, v|
        format!("{}{}\n", s, v.iter().collect::<String>())
    )
}