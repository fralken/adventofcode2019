use std::fs;
use crate::intcode::{ IntCode, extract_codes };

#[derive(Default)]
struct Packet {
    address: usize,
    x: i64,
    y: i64
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day23.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);

    let size = 50;
    let mut computers = (0..size).map(|i| init_computer(i, &codes)).collect::<Vec<_>>();

    let mut packet = Packet::default();

    while packet.address != 255 {
        packet = network(&mut computers);

        if packet.address != 255 {
            computers.iter_mut().for_each(|c| {
                if c.no_input() {
                    c.write(&[-1]);
                }
                c.interpreter();
            });
        }
    }

    println!("day 23.1 - Y value of the first packet sent to address 255: {}", packet.y);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day23.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);

    let size = 50;
    let mut computers = (0..size).map(|i| init_computer(i, &codes)).collect::<Vec<_>>();

    let mut y = 0;

    loop {
        let packet = network(&mut computers);

        let idle = computers.iter_mut().all(|c| {
            let no_input = c.no_input();
            if no_input {
                c.write(&[-1]);
            }
            c.interpreter();
            no_input && c.no_output()
        });

        if idle && y == packet.y { break }

        if idle {
            computers[0].write(&[packet.x, packet.y]);
            y = packet.y;
        }
    }

    println!("day 23.2 - Y value delivered by the NAT to the computer at address 0 twice in a row: {}", y);
}

fn init_computer(i: usize, codes: &[i64]) -> IntCode {
    let mut computer = IntCode::new(codes.to_owned());
    computer.write(&[i as i64]);
    computer.interpreter();
    computer
}

fn network(computers: &mut [IntCode]) -> Packet {
    let size = computers.len();
    let mut address = 0;
    let mut x = 0;
    let mut y = 0;

    for i in 0..size {
        let output = computers[i].read();
        output.chunks(3).for_each(|p| {
            let a = p[0] as usize;
            if a < size {
                computers[a].write(&[p[1], p[2]]);
            } else {
                address = a;
                x = p[1];
                y = p[2];
            }
        })
    };
    Packet { address, x, y }
}