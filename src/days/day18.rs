use std::fs;
use std::cmp::Reverse;
use std::collections::{ BinaryHeap, BTreeMap, BTreeSet, VecDeque };

type Position = (isize, isize);

struct Map {
    start: Position,
    walls: BTreeSet<Position>,
    doors: BTreeMap<Position, char>,
    keys: BTreeMap<Position, char>
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Path {
    to: Position,
    length: usize,
    found_doors: BTreeSet<char>,
    found_keys: BTreeSet<char>
}

impl Map {
    fn new(map: &[Vec<char>]) -> Self {
        let mut walls = BTreeSet::new();
        let mut doors = BTreeMap::new();
        let mut keys = BTreeMap::new();
        let mut start = (0, 0);
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    '#' => { walls.insert((x as isize, y as isize )); },
                    '@' => { start = (x as isize, y as isize); },
                    c if c.is_ascii_uppercase() => { doors.insert((x as isize, y as isize),c.to_ascii_lowercase()); },
                    c if c.is_ascii_lowercase() => { keys.insert((x as isize, y as isize), *c); },
                    '.' => (),
                    _ => unreachable!()
                }
            }
        }
        Map { start, walls, doors, keys }
    }

    fn find_paths(&self, start: Position) -> BTreeSet<Path> {
        let mut paths = BTreeSet::new();
        let mut queue = VecDeque::new();
        let mut seen = BTreeSet::new();

        queue.push_back((0, start, BTreeSet::new(), BTreeSet::new()));
        while let Some((length, position, found_doors, found_keys)) = queue.pop_front() {
            if seen.insert(position) {
                queue.extend([(1, 0), (-1, 0), (0, 1), (0, -1)].iter()
                    .map(|(x, y)| (position.0 + *x, position.1 + *y))
                    .filter(|new_position| !self.walls.contains(&new_position))
                    .map(|new_position| {
                        let mut new_found_doors = found_doors.clone();
                        if let Some(door) = self.doors.get(&new_position) { new_found_doors.insert(*door); }
                        let mut new_found_keys = found_keys.clone();
                        if let Some(key) = self.keys.get(&new_position) { new_found_keys.insert(*key); }
                        (length + 1, new_position, new_found_doors, new_found_keys)
                    })
                );
                if self.keys.get(&position).is_some() {
                    paths.insert(Path { to: position, length, found_doors, found_keys });
                }
            }
        }

        paths
    }

    fn find_doors_and_keys(&self, start: Position) -> (BTreeMap<Position, char>, BTreeMap<Position, char>) {
        let paths = self.find_paths(start);
        let (found_doors, found_keys) = paths.iter().fold((BTreeSet::new(), BTreeSet::new()), |a, p| {
            let found_doors = a.0.union(&p.found_doors).cloned().collect();
            let found_keys = a.1.union(&p.found_keys).cloned().collect();
            (found_doors, found_keys)
        });
        let doors = self.doors.clone().into_iter()
            .filter(|(_, d)| found_doors.contains(d))
            .collect();
        let keys = self.keys.clone().into_iter()
            .filter(|(_, k)| found_keys.contains(k))
            .collect();
        (doors, keys)
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day18.txt")
        .expect("Something went wrong reading the file");

    let length = impl_first_star(&contents);

    println!("day 18.1 - steps of shortest path that collects all of the keys: {}", length);

}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day18.txt")
        .expect("Something went wrong reading the file");

    let length = impl_second_star(&contents);

    println!("day 18.2 - fewest steps necessary to collect all of the keys: {}", length);
}

fn impl_first_star(contents: &str) -> usize {
    let grid = extract_map(contents);
    find_shortest_path_length(&[Map::new(&grid)])
}

fn impl_second_star(contents: &str) -> usize {
    let mut map = Map::new(&extract_map(contents));
    let start = map.start;

    map.walls.extend(
        [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)].iter()
            .map(|(x, y)| (start.0 + *x, start.1 + *y))
    );

    let maps = [(1, 1), (1, -1), (-1, 1), (-1, -1)].iter().map(|&(sx, sy)| {
        let start = (start.0 + sx, start.1 + sy);
        let (doors, keys) = map.find_doors_and_keys(start);

        Map {
            start,
            walls: map.walls.clone(),
            doors,
            keys
        }
    }).collect::<Vec<_>>();

    find_shortest_path_length(&maps)
}

fn extract_map(contents: &str) -> Vec<Vec<char>> {
    contents.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn find_shortest_path_length(maps: &[Map]) -> usize {
    let multi_paths_to_keys = maps.iter()
        .map(|map|
            [map.start].iter().chain(map.keys.keys())
                .map(|position| (*position, map.find_paths(*position)))
                .collect::<BTreeMap<_,_>>()
        ).collect::<Vec<_>>();

    let total_keys = maps.iter().map(|map| map.keys.len()).sum();
    let mut queue = BinaryHeap::new();
    let mut seen = BTreeSet::new();
    let start_positions = maps.iter().map(|map| map.start).collect::<Vec<_>>();
    queue.push(Reverse((0, start_positions, BTreeSet::new())));

    while let Some(Reverse((length, positions, found_keys))) = queue.pop() {
        if found_keys.len() == total_keys {
            return length;
        }
        if seen.insert((positions.clone(), found_keys.clone())) {
            for (i, position) in positions.iter().enumerate() {
                let paths_to_keys = &multi_paths_to_keys[i];
                paths_to_keys[&position].iter()
                    .filter(|path|
                        path.found_keys.iter().any(|k| !found_keys.contains(k)) &&
                            path.found_doors.is_subset(&found_keys)
                    )
                    .for_each(|path| {
                        let mut new_positions = positions.clone();
                        new_positions[i] = path.to;
                        queue.push(Reverse((length + path.length, new_positions, found_keys.union(&path.found_keys).cloned().collect())));
                    });
            }
        }
    }

    panic!("path not found")
}

#[test]
fn test0_first_star() {
    let map = "\
        #########\n\
        #b.A.@.a#\n\
        #########\n\
    ";
    assert_eq!(impl_first_star(&map), 8);
}

#[test]
fn test1_first_star() {
    let map = "\
        ########################\n\
        #f.D.E.e.C.b.A.@.a.B.c.#\n\
        ######################.#\n\
        #d.....................#\n\
        ########################\n\
    ";
    assert_eq!(impl_first_star(&map), 86);
}

#[test]
fn test2_first_star() {
    let map = "\
        ########################\n\
        #...............b.C.D.f#\n\
        #.######################\n\
        #.....@.a.B.c.d.A.e.F.g#\n\
        ########################\n\
    ";
    assert_eq!(impl_first_star(&map), 132);
}

#[test]
fn test3_first_star() {
    let map = "\
        #################\n\
        #i.G..c...e..H.p#\n\
        ########.########\n\
        #j.A..b...f..D.o#\n\
        ########@########\n\
        #k.E..a...g..B.n#\n\
        ########.########\n\
        #l.F..d...h..C.m#\n\
        #################\n\
    ";
    assert_eq!(impl_first_star(&map), 136);
}

#[test]
fn test4_first_star() {
    let map = "\
        ########################\n\
        #@..............ac.GI.b#\n\
        ###d#e#f################\n\
        ###A#B#C################\n\
        ###g#h#i################\n\
        ########################\n\
    ";
    assert_eq!(impl_first_star(&map), 81);
}

#[test]
fn test0_second_star() {
    let map = "\
        #######\n\
        #a.#Cd#\n\
        ##...##\n\
        ##.@.##\n\
        ##...##\n\
        #cB#Ab#\n\
        #######\n\
    ";
    assert_eq!(impl_second_star(&map), 8);
}

#[test]
fn test1_second_star() {
    let map = "\
        ###############\n\
        #d.ABC.#.....a#\n\
        ######...######\n\
        ######.@.######\n\
        ######...######\n\
        #b.....#.....c#\n\
        ###############\n\
    ";
    assert_eq!(impl_second_star(&map), 24);
}

#[test]
fn test2_second_star() {
    let map = "\
        #############\n\
        #DcBa.#.GhKl#\n\
        #.###...#I###\n\
        #e#d#.@.#j#k#\n\
        ###C#...###J#\n\
        #fEbA.#.FgHi#\n\
        #############\n\
    ";
    assert_eq!(impl_second_star(&map), 32);
}

#[test]
fn test3_second_star() {
    let map = "\
        #############\n\
        #g#f.D#..h#l#\n\
        #F###e#E###.#\n\
        #dCba...BcIJ#\n\
        #####.@.#####\n\
        #nK.L...G...#\n\
        #M###N#H###.#\n\
        #o#m..#i#jk.#\n\
        #############\n\
    ";
    assert_eq!(impl_second_star(&map), 72);
}
