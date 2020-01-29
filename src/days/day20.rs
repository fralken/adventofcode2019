use std::fs;
use std::collections::{ BTreeMap, BTreeSet, VecDeque };

type Position = (isize, isize);

type Portal = (Position, String, bool);

type Paths<'a> = BTreeMap<&'a Portal, BTreeMap<&'a Portal, usize>>;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day20.txt")
        .expect("Something went wrong reading the file");

    let length = impl_star(true, &contents);

    println!("day 20.1 - steps to get from the open tile marked AA to the open tile marked ZZ: {}", length);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day20.txt")
        .expect("Something went wrong reading the file");

    let length = impl_star(false, &contents);

    println!("day 20.2 - steps to get from the open tile marked AA to the open tile marked ZZ, both at the outermost layer: {}", length);
}

fn impl_star(first: bool, contents: &str) -> usize {
    let map = extract_map(&contents);
    let portals = find_portals(&map);
    let paths = extract_paths(&map);
    let paths_to_portals = portals.iter()
        .map(|p|{
            let start = p.0;
            (p, find_paths_to_portals(start, &paths, &portals))
        })
        .collect::<BTreeMap<_, _>>();
    let start = portals.iter().find(|p| p.1 == "AA").unwrap();
    let end = portals.iter().find(|p| p.1 == "ZZ").unwrap();
    if first {
        find_shortest_path(start, end, &paths_to_portals)
    } else {
        find_multilevel_shortest_path(start, end, &paths_to_portals)
    }
}

fn extract_map(contents: &str) -> Vec<Vec<char>> {
    contents.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn find_portals(map: &[Vec<char>]) -> BTreeSet<Portal> {
    let mut portals = BTreeSet::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x].is_ascii_alphabetic() {
                if y + 1 < map.len() && x < map[y + 1].len() && map[y + 1][x].is_ascii_alphabetic() {
                    if y + 2 < map.len() && x < map[y + 2].len() && map[y + 2][x] == '.' {
                        portals.insert(((x as isize, y as isize + 2), format!("{}{}", map[y][x], map[y + 1][x]), y == 0));
                    } else {
                        portals.insert(((x as isize, y as isize - 1), format!("{}{}", map[y][x], map[y + 1][x]), y + 2 == map.len()));
                    }
                } else if x + 1 < map[y].len() && map[y][x + 1].is_ascii_alphabetic() {
                    if x + 2 < map[y].len() && map[y][x + 2] == '.' {
                        portals.insert(((x as isize + 2, y as isize), format!("{}{}", map[y][x], map[y][x + 1]), x == 0));
                    } else {
                        portals.insert(((x as isize - 1, y as isize), format!("{}{}", map[y][x], map[y][x + 1]), x + 2 == map[y].len() ));
                    }
                }
            }
        }
    }
    portals
}

fn extract_paths(map: &[Vec<char>]) -> BTreeSet<Position> {
    let mut paths = BTreeSet::new();
    for (y, r) in map.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if *c == '.' {
                paths.insert((x as isize, y as isize));
            }
        }
    }
    paths
}

fn find_paths_to_portals<'a>(start: Position, valid_paths: &BTreeSet<Position>, portals: &'a BTreeSet<Portal>) -> BTreeMap<&'a Portal, usize> {
    let mut paths = BTreeMap::new();
    let mut queue = VecDeque::new();
    let mut seen = BTreeSet::new();

    queue.push_back((0, start));
    while let Some((length, position)) = queue.pop_front() {
        if seen.insert(position) {
            queue.extend([(1, 0), (-1, 0), (0, 1), (0, -1)].iter()
                .map(|(x, y)| (position.0 + *x, position.1 + *y))
                .filter(|new_position| valid_paths.contains(&new_position))
                .map(|new_position| (length + 1, new_position))
            );
            if position != start {
                if let Some(portal) = portals.iter().find(|p| p.0 == position) {
                    paths.insert(portal, length + 1);
                }
            }
        }
    }
    paths
}

fn find_shortest_path(start: &Portal, end: &Portal, paths: &Paths) -> usize {
    let mut queue = VecDeque::new();
    let mut min_length = std::usize::MAX;

    queue.push_back((0, start));
    while let Some((length, portal)) = queue.pop_front() {
        if length < min_length {
            queue.extend(paths[portal].iter()
                .filter_map(|(&p, &l)|
                    if let Some(p_next) = paths.keys().find(|p_next| p_next.1 == p.1 && p_next.2 != p.2) {
                        Some((length + l, *p_next))
                    } else {
                        if p == end { min_length = min_length.min(length + l); }
                        None
                    }
                )
            )
        }
    }
    min_length - 1
}

fn find_multilevel_shortest_path(start: &Portal, end: &Portal, paths: &Paths) -> usize {
    let mut queue = VecDeque::new();
    let mut min_length = std::usize::MAX;

    queue.push_back((0, 0, start));
    while let Some((length, level, portal)) = queue.pop_front() {
        if length < min_length {
            queue.extend(paths[portal].iter()
                .filter_map(|(&p, &l)|
                    if (level > 0 && (p == start || p == end)) || (level == 0 && p != start && p != end && p.2) {
                        None
                    } else if let Some(p_next) = paths.keys().find(|p_next| p_next.1 == p.1 && p_next.2 != p.2) {
                        Some((length + l, if p.2 { level - 1 } else { level + 1 }, *p_next))
                    } else {
                        if p == end { min_length = min_length.min(length + l); }
                        None
                    }
                )
            )
        }
    }
    min_length - 1
}

#[test]
fn test0_first_star() {
    let map =
"         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z
";
    assert_eq!(impl_star(true, &map), 23);
}

#[test]
fn test1_first_star() {
    let map =
"                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P
";
    assert_eq!(impl_star(true, &map), 58);
}

#[test]
fn test0_second_star() {
    let map =
"             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M
";
    assert_eq!(impl_star(false, &map), 396);
}