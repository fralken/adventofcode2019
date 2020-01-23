use std::fs;

#[derive(Clone)]
struct Grid {
    value: u32,
    width: isize,
    height: isize
}

impl Grid {
    fn new(contents: &str) -> Self {
        let height = contents.lines().count() as isize;
        let width = contents.lines().next().map_or_else(|| 0, |l| l.len()) as isize;
        let value = contents
            .lines()
            .enumerate()
            .fold(0,|acc, (i, r)| {
                let len = r.len() as u32;
                let v = r.char_indices()
                    .fold(0, |acc, (i, c)|
                        if c == '#' { acc + (1 << i) } else { acc }
                    ) as u32;
                acc + (v << (len * i as u32))
            });
        Grid { value, width, height }
    }

    fn spawn(&self) -> Self {
        Grid { value: 0, width: self.width, height: self.height }
    }

    fn center_x(&self) -> isize { self.width / 2 }

    fn center_y(&self) -> isize { self.height / 2 }

    fn cell_alive(&self, x: isize, y: isize) -> u32 {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            0
        } else {
            self.value >> (x + y * self.width) as u32 & 1
        }
    }

    fn will_cell_live(&self, x: isize, y: isize, neighbours: u32) -> bool {
        let alive = self.cell_alive(x, y) == 1;
        (alive && neighbours == 1) || (!alive && neighbours > 0 && neighbours <= 2)
    }

    fn count_alive_cells(&self) -> u32 {
        let mut value = self.value;
        let mut count = 0;
        for _ in 0..(self.width * self.height) {
            count += value & 1;
            value >>= 1;
        }
        count
    }

    fn is_grid_alive(&self) -> bool { self.value > 0 }

    fn step(&self, count_neighbours: impl Fn(isize, isize) -> u32) -> Self {
        let mut value = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbours = count_neighbours(x, y);
                if self.will_cell_live(x, y, neighbours) {
                    value += 1 << (x + y * self.width) as u32
                }
            }
        }
        Grid { value, width: self.width, height: self.height }
    }

    fn count_neighbours<'a>(&'a self) -> impl Fn(isize, isize) -> u32 + 'a {
        move |x, y| {
            self.cell_alive(x - 1, y) +
                self.cell_alive(x + 1, y) +
                self.cell_alive(x, y - 1) +
                self.cell_alive(x, y + 1)
        }
    }

    fn count_neighbours_multilevel<'a>(&'a self, inner: &'a Grid, outer: &'a Grid) -> impl Fn(isize, isize) -> u32 + 'a {
        let center_x = self.center_x();
        let center_y = self.center_y();
        let o_center_x = outer.center_x();
        let o_center_y = outer.center_y();

        move |x, y| {
            if x == center_x && y == center_y {
                // here there is an inner level
                0
            } else {
                let left_count = if x - 1 < 0 {
                    outer.cell_alive(o_center_x - 1, o_center_y)
                } else if y == center_y && x - 1 == center_x {
                    (0..inner.height).map(|y| inner.cell_alive(inner.width - 1, y)).sum()
                } else {
                    self.cell_alive(x - 1, y)
                };

                let right_count = if x + 1 >= self.width {
                    outer.cell_alive(o_center_x + 1, o_center_y)
                } else if y == center_y && x + 1 == center_x {
                    (0..inner.height).map(|y| inner.cell_alive(0, y)).sum()
                } else {
                    self.cell_alive(x + 1, y)
                };

                let top_count = if y - 1 < 0 {
                    outer.cell_alive(o_center_x, o_center_y - 1)
                } else if x == center_x && y - 1 == center_y {
                    (0..inner.width).map(|x| inner.cell_alive(x, inner.height - 1)).sum()
                } else {
                    self.cell_alive(x, y - 1)
                };

                let bottom_count = if y + 1 >= self.height {
                    outer.cell_alive(o_center_x, o_center_y + 1)
                } else if x == center_x && y + 1 == center_y {
                    (0..inner.width).map(|x| inner.cell_alive(x, 0)).sum()
                } else {
                    self.cell_alive(x, y + 1)
                };

                left_count + right_count + top_count + bottom_count
            }
        }
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        (0..self.height)
            .map(|y| (0..self.width)
                .map(|x|
                    if self.value & 1 << (x + y * self.width) as u32 != 0 { '#' } else { '.' }
                )
                .collect::<String>()
            )
            .fold(String::new(), |acc, row|
                if acc.is_empty() { row } else { format!("{}\n{}", acc, row) }
            )
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day24.txt")
        .expect("Something went wrong reading the file");

    let grid = impl_first_star(&contents);

    println!("day 24.1 - biodiversity rating for the first layout that appears twice: {}", grid);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day24.txt")
        .expect("Something went wrong reading the file");

    let count = impl_second_star(&contents, 200);

    println!("day 24.2 - bugs present after 200 minutes: {}", count);
}

fn impl_first_star(contents: &str) -> u32 {
    let grid = Grid::new(&contents);
    let mut steps = Vec::new();
    let mut next_grid = grid;
    steps.push(next_grid.value);

    loop {
        next_grid = next_grid.step(next_grid.count_neighbours());
        if steps.iter().any(|g| g == &next_grid.value) {
            break;
        }
        steps.push(next_grid.value)
    }

    next_grid.value
}

fn impl_second_star(contents: &str, iterations: usize) -> u32 {
    let mut grid = Grid::new(&contents);
    let mut inners = vec![grid.spawn(); 2];
    let mut outers = vec![grid.spawn(); 2];

    for _ in 0..iterations {
        let next_grid = grid.step(grid.count_neighbours_multilevel(&inners[0], &outers[0]));
        inners = [grid.clone()]
            .iter()
            .chain(inners.iter())
            .collect::<Vec<_>>()
            .windows(3)
            .map(|v|
                v[1].step(v[1].count_neighbours_multilevel(v[2], v[0]))
            )
            .collect::<Vec<_>>();

        outers = [grid]
            .iter()
            .chain(outers.iter())
            .collect::<Vec<_>>()
            .windows(3)
            .map(|v|
                v[1].step(v[1].count_neighbours_multilevel(v[0], v[2]))
            )
            .collect::<Vec<_>>();

        while inners.len() < 2 || inners[inners.len()-2].is_grid_alive() {
            inners.push(next_grid.spawn());
        }

        while outers.len() < 2 || outers[outers.len()-2].is_grid_alive() {
            outers.push(next_grid.spawn());
        }

        grid = next_grid;
    }

    grid.count_alive_cells() +
        inners.iter().map(|g| g.count_alive_cells()).sum::<u32>() +
        outers.iter().map(|g| g.count_alive_cells()).sum::<u32>()
}

#[test]
fn test0_first_star() {
    let contents = "\
        ....#\n\
        #..#.\n\
        #..##\n\
        ..#..\n\
        #....";
    let grid = Grid::new(&contents);
    let grid = grid.step(grid.count_neighbours());
    assert_eq!(grid.to_string(), "\
        #..#.\n\
        ####.\n\
        ###.#\n\
        ##.##\n\
        .##..");
    let grid = grid.step(grid.count_neighbours());
    assert_eq!(grid.to_string(), "\
        #####\n\
        ....#\n\
        ....#\n\
        ...#.\n\
        #.###");
    let grid = grid.step(grid.count_neighbours());
    assert_eq!(grid.to_string(), "\
        #....\n\
        ####.\n\
        ...##\n\
        #.##.\n\
        .##.#");
    let grid = grid.step(grid.count_neighbours());
    assert_eq!(grid.to_string(), "\
        ####.\n\
        ....#\n\
        ##..#\n\
        .....\n\
        ##...");
}

#[test]
fn test1_first_star() {
    let contents = "\
        ....#\n\
        #..#.\n\
        #..##\n\
        ..#..\n\
        #....";
    assert_eq!(impl_first_star(&contents), 2129920);
}

#[test]
fn test0_second_star() {
    let contents = "\
        ....#\n\
        #..#.\n\
        #..##\n\
        ..#..\n\
        #....";
    assert_eq!(impl_second_star(&contents, 10), 99);
}