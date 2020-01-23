use std::fs;

#[derive(Debug)]
struct PipeSlice {
    position: i32,
    start: i32,
    end: i32,
    horizontal: bool,
    reverse: bool
}

type Pipe = Vec<PipeSlice>;

pub fn first_star() {
     let contents = fs::read_to_string("./input/day03.txt")
         .expect("Something went wrong reading the file");

    let res = impl_first_star(&contents);

    println!("day  3.1 - manhattan distance to closest intersection: {:?}", res);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day03.txt")
        .expect("Something went wrong reading the file");

    let res = impl_second_star(&contents);

    println!("day  3.2 - fewest combined steps to reach an intersection: {:?}", res);
}

fn compute_pipes(contents: &str) -> Vec<Pipe> {
    contents
        .lines()
        .map(|s| {
            let mut x = 0;
            let mut y = 0;
            let mut pipe: Pipe = Pipe::new();
            let parts = s.split(',').filter(|s| !s.is_empty());
            for part in parts {
                let dir = part.chars().next().unwrap();
                let len = part[1..].parse::<i32>().unwrap();
                match dir {
                    'R' => {
                        pipe.push(PipeSlice { position: y, start: x, end: x + len, horizontal: true, reverse: false });
                        x += len;
                    },
                    'L' => {
                        pipe.push(PipeSlice { position: y, start: x - len, end: x, horizontal: true, reverse: true });
                        x -= len;
                    }
                    'U' => {
                        pipe.push(PipeSlice { position: x, start: y, end: y + len, horizontal: false, reverse: false });
                        y += len;
                    },
                    'D' => {
                        pipe.push(PipeSlice { position: x, start: y - len, end: y, horizontal: false, reverse: true });
                        y -= len;
                    },
                    _ => panic!("invalid direction {}", dir)
                }
            }
            pipe
        })
        .collect()
}

fn find_min_manhattan_distance(firsts: &[PipeSlice], seconds: &[PipeSlice]) -> u32 {
    let mut min_distance = std::u32::MAX;
    for first in firsts {
        for second in seconds.iter().filter(|p| p.horizontal != first.horizontal) {
            if first.position > second.start &&
                first.position < second.end &&
                second.position > first.start &&
                second.position < first.end {
                let distance = first.position.abs() as u32 + second.position.abs() as u32;
                if distance > 0 {
                    min_distance = min_distance.min(distance);
                }
            }
        }
    }
    min_distance
}

fn find_min_steps_distance(firsts: &[PipeSlice], seconds: &[PipeSlice]) -> u32 {
    let mut min_distance = std::u32::MAX;
    let mut first_steps = 0;
    for first in firsts {
        let mut second_steps = 0;
        for second in seconds {
            if first.horizontal != second.horizontal &&
                first.position > second.start &&
                first.position < second.end &&
                second.position > first.start &&
                second.position < first.end {
                let first_partial = if first.reverse { first.end - second.position } else { second.position - first.start };
                let second_partial = if second.reverse { second.end - first.position } else { first.position - second.start };
                min_distance = min_distance.min((first_steps + first_partial + second_steps + second_partial) as u32);
                break;
            }
            second_steps += second.end - second.start
        }
        first_steps += first.end - first.start
    }
    min_distance
}

fn compute_min_distance(pipes: &[Pipe], find_min_distance: &dyn Fn(&[PipeSlice], &[PipeSlice]) -> u32) -> u32 {
    let mut min_distance = std::u32::MAX;

    for i in 0..pipes.len()-1 {
        let first = &pipes[i];
        for second in pipes.iter().skip(i+1) {
            min_distance = min_distance.min(find_min_distance(&first, &second));
        }
    }

    min_distance
}

fn impl_first_star(contents: &str) -> u32 {
    let pipes: Vec<Pipe> = compute_pipes(contents);
    compute_min_distance(&pipes, &find_min_manhattan_distance)
}

fn impl_second_star(contents: &str) -> u32 {
    let pipes: Vec<Pipe> = compute_pipes(contents);
    compute_min_distance(&pipes, &find_min_steps_distance)
}

#[test]
fn test0_first_star() {
    assert_eq!(impl_first_star("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
}

#[test]
fn test1_first_star() {
    assert_eq!(impl_first_star("\
        R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
        U62,R66,U55,R34,D71,R55,D58,R83"), 159);
}

#[test]
fn test2_first_star() {
    assert_eq!(impl_first_star("\
        R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
}

#[test]
fn test0_second_star() {
    assert_eq!(impl_second_star("\
        R8,U5,L5,D3\n\
        U7,R6,D4,L4"), 30);
}

#[test]
fn test1_second_star() {
    assert_eq!(impl_second_star("\
        R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
        U62,R66,U55,R34,D71,R55,D58,R83"), 610);
}

#[test]
fn test2_second_star() {
    assert_eq!(impl_second_star("\
        R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
}
