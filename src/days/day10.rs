use std::fs;
use std::f64;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day10.txt")
        .expect("Something went wrong reading the file");

    let (_, count) = impl_first_star(&contents);

    println!("day 10.1 - number of asteroids detected: {}", count);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day10.txt")
        .expect("Something went wrong reading the file");

    let vaporized = impl_second_star(&contents);
    let (x, y) = vaporized.get(199).unwrap();

    println!("day 10.2 - coordinates of 200th vaporized asteroid (100 * x + y): {}", 100 * x + y);
}

fn impl_first_star(contents: &str) -> ((i32, i32), usize) {
    let asteroids = extract(contents);
    *visible_count(&asteroids).first().unwrap()
}

fn impl_second_star(contents: &str) -> Vec<(i32, i32)> {
    let asteroids = extract(contents);
    let (base, count) = *visible_count(&asteroids).first().unwrap();

    let mut rays = asteroids
        .iter()
        .filter(|&a| base != *a)
        .map(|a| {
            let mut angle = angle(base, *a);
            if angle < -900 { angle += 3_600 }
            (angle, a)
        })
        .fold(BTreeMap::new(), |mut acc, (angle, a)| {
            acc.entry(angle)
                .or_insert_with(Vec::new)
                .push(a);
            acc
        });

    rays.iter_mut().for_each(|(_, asteroids)|
        asteroids.sort_unstable_by_key(|a| (a.1 - base.1).abs() + (a.0 - base.0).abs())
    );

    let mut vaporized = Vec::new();
    while vaporized.len() < count {
        rays.iter_mut().for_each(|(_, asteroids)|
            if !asteroids.is_empty() { vaporized.push(*asteroids.remove(0)) }
        )
    }

    vaporized
}

fn extract(contents: &str) -> Vec<(i32, i32)>{
    contents
        .lines()
        .enumerate()
        .flat_map(|(i, s)|
            s.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| (j as i32, i as i32))
        )
        .collect()
}

fn angle(a: (i32, i32), b: (i32, i32)) -> i64 {
    (((b.1 - a.1) as f64).atan2((b.0 - a.0) as f64) / f64::consts::PI * 1_800_f64) as i64
}

fn visible_count(asteroids: &[(i32, i32)]) -> Vec<((i32, i32), usize)> {
    let mut visible = asteroids
        .iter()
        .map(|a| {
            let angles = asteroids
                .iter()
                .filter(|b| a != *b)
                .map(|b| angle(*a, *b))
                .collect::<BTreeSet<i64>>();
            (*a, angles.len())
        })
        .collect::<Vec<((i32, i32), usize)>>();

    visible.sort_by(|a, b | b.1.cmp(&a.1));
    visible
}

#[test]
fn test0_first_star() {
    let map =
        ".#..#\n\
         .....\n\
         #####\n\
         ....#\n\
         ...##";
    assert_eq!(impl_first_star(map), ((3, 4), 8));
}

#[test]
fn test1_first_star() {
    let map =
        "......#.#.\n\
         #..#.#....\n\
         ..#######.\n\
         .#.#.###..\n\
         .#..#.....\n\
         ..#....#.#\n\
         #..#....#.\n\
         .##.#..###\n\
         ##...#..#.\n\
         .#....####";
    assert_eq!(impl_first_star(map), ((5, 8), 33));
}

#[test]
fn test2_first_star() {
    let map =
        "#.#...#.#.\n\
         .###....#.\n\
         .#....#...\n\
         ##.#.#.#.#\n\
         ....#.#.#.\n\
         .##..###.#\n\
         ..#...##..\n\
         ..##....##\n\
         ......#...\n\
         .####.###.";
    assert_eq!(impl_first_star(map), ((1, 2), 35));
}

#[test]
fn test3_first_star() {
    let map =
        ".#..#..###\n\
         ####.###.#\n\
         ....###.#.\n\
         ..###.##.#\n\
         ##.##.#.#.\n\
         ....###..#\n\
         ..#.#..#.#\n\
         #..#.#.###\n\
         .##...##.#\n\
         .....#.#..";
    assert_eq!(impl_first_star(map), ((6, 3), 41));
}

#[test]
fn test4_first_star() {
    let map =
        ".#..##.###...#######\n\
         ##.############..##.\n\
         .#.######.########.#\n\
         .###.#######.####.#.\n\
         #####.##.#.##.###.##\n\
         ..#####..#.#########\n\
         ####################\n\
         #.####....###.#.#.##\n\
         ##.#################\n\
         #####.##.###..####..\n\
         ..######..##.#######\n\
         ####.##.####...##..#\n\
         .#####..#.######.###\n\
         ##...#.##########...\n\
         #.##########.#######\n\
         .####.#.###.###.#.##\n\
         ....##.##.###..#####\n\
         .#.#.###########.###\n\
         #.#.#.#####.####.###\n\
         ###.##.####.##.#..##";
    assert_eq!(impl_first_star(map), ((11, 13), 210));
}

#[test]
fn test0_second_star() {
    let map =
        ".#..##.###...#######\n\
         ##.############..##.\n\
         .#.######.########.#\n\
         .###.#######.####.#.\n\
         #####.##.#.##.###.##\n\
         ..#####..#.#########\n\
         ####################\n\
         #.####....###.#.#.##\n\
         ##.#################\n\
         #####.##.###..####..\n\
         ..######..##.#######\n\
         ####.##.####...##..#\n\
         .#####..#.######.###\n\
         ##...#.##########...\n\
         #.##########.#######\n\
         .####.#.###.###.#.##\n\
         ....##.##.###..#####\n\
         .#.#.###########.###\n\
         #.#.#.#####.####.###\n\
         ###.##.####.##.#..##";
    assert_eq!(impl_second_star(map).get(199).unwrap(), &(8,2));
}
