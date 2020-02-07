use std::fs;
use crate::intcode::{ IntCode, Status, extract_codes };

pub fn first_star() {
    let contents = fs::read_to_string("./input/day13.txt")
        .expect("Something went wrong reading the file");

    let mut game = IntCode::new(extract_codes(&contents));
    game.process();
    let output = game.read();
    let count = output.chunks(3).filter(|c| c[2] == 2).count();

    println!("day 13.1 - num of block tiles on the screen when the game exits: {}", count);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day13.txt")
        .expect("Something went wrong reading the file");

    let mut game = IntCode::new(extract_codes(&contents));
    game.init_code(2);
    let mut status = Status::Running;
    let mut score = 0;
    let mut paddle_x = 0;
    let mut ball_x = 0;
    while status != Status::End {
        status = game.process();
        let output = game.read();
        output.chunks(3).for_each(|c| {
            if c[0] == -1 && c[1] == 0 { score = c[2] }
            else if c[2] == 3 { paddle_x = c[0]; }
            else if c[2] == 4 { ball_x = c[0]; }
        });
        let joystick = (ball_x - paddle_x).signum();
        game.write_one(joystick);
    }

    println!("day 13.2 - final score after the last block is broken: {}", score);
}
