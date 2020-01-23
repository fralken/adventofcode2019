use std::fs;
use modinverse::modinverse;

type Num = i128;
type Coeff = (Num, Num);

pub fn first_star() {
    let contents = fs::read_to_string("./input/day22.txt")
        .expect("Something went wrong reading the file");

    let size = 10_007;
    let iterations = 1;
    let (position_of_card, _) = compose_shuffles(&contents, size, iterations);

    println!("day 22.1 - position of card 2019: {}", position_of_card(2019));
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day22.txt")
        .expect("Something went wrong reading the file");

    let size = 119_315_717_514_047;
    let iterations = 101_741_582_076_661;
    let (_, card_at_position) = compose_shuffles(&contents, size, iterations);

    println!("day 22.2 - card at position 2020: {}", card_at_position(2020));
}

fn compose_shuffles(contents: &str, size: Num, mut iterations: Num) -> (impl Fn(Num) -> Num, impl Fn(Num) -> Num) {
    fn compose(a: Coeff, b: Coeff, size: Num) -> Coeff {
        ((a.0 * b.0).rem_euclid(size), (a.1 * b.0 + b.1).rem_euclid(size))
    }

    let mut p = contents.lines()
        .filter_map(|line|
            if cmd_deal_into_new_stack(line) {
                Some(((-1 as Num).rem_euclid(size), (-1 as Num).rem_euclid(size)))
            } else if let Some(n) = cmd_cut(line) {
                Some((1, -n.rem_euclid(size)))
            } else if let Some(n) = cmd_deal_with_increment(line) {
                Some((n.rem_euclid(size), 0))
            } else {
                None
            }
        )
        .fold((1, 0), |a, b| compose(a, b, size));

    let p = if iterations > 1 {
        // modular exponentiation by squaring:
        // https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
        let mut r = (1, 0);
        while iterations != 0 {
            if iterations % 2 == 1 { r = compose(r, p, size); }
            p = compose(p, p, size);
            iterations >>= 1;
        }
        r
    } else {
        p
    };

    (fn_position_of_card(p, size), fn_card_at_position(p, size))
}

fn cmd_deal_into_new_stack(s: &str) -> bool {
    s == "deal into new stack"
}

fn cmd_cut(s: &str) -> Option<Num> {
    let cmd = "cut ";
    if s.starts_with(cmd) { s[cmd.len()..].parse::<Num>().ok() } else { None }
}

fn cmd_deal_with_increment(s: &str) -> Option<Num> {
    let cmd = "deal with increment ";
    if s.starts_with(cmd) { s[cmd.len()..].parse::<Num>().ok() } else { None }
}

fn fn_position_of_card(p: Coeff, size: Num) -> impl Fn(Num) -> Num {
    move |card| { (p.0 * card + p.1).rem_euclid(size) }
}

fn fn_card_at_position(p: Coeff, size: Num) -> impl Fn(Num) -> Num {
    let denominator = modinverse(p.0, size).unwrap();
    move |position| { ((position - p.1) * denominator).rem_euclid(size) }
}

#[test]
fn test0_first_star() {
    let commands = "\
deal with increment 7
deal into new stack
deal into new stack
";
    let (_, card_at_position) = compose_shuffles(&commands, 10, 1);
    let cards = (0..10).map(|position| card_at_position(position)).collect::<Vec<_>>();
    assert_eq!(cards, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
}

#[test]
fn test1_first_star() {
    let commands = "\
cut 6
deal with increment 7
deal into new stack
";
    let (_, card_at_position) = compose_shuffles(&commands, 10, 1);
    let cards = (0..10).map(|position| card_at_position(position)).collect::<Vec<_>>();
    assert_eq!(cards, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
}

#[test]
fn test2_first_star() {
    let commands = "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
";
    let (_, card_at_position) = compose_shuffles(&commands, 10, 1);
    let cards = (0..10).map(|position| card_at_position(position)).collect::<Vec<_>>();
    assert_eq!(cards, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
}

#[test]
fn test0_second_star() {
    let commands = "\
deal with increment 7
deal into new stack
deal into new stack
";
    let (position_of_card, _) = compose_shuffles(&commands, 10, 1);
    let positions = [0, 3, 6, 9, 2, 5, 8, 1, 4, 7].iter().map(|card| position_of_card(*card)).collect::<Vec<_>>();
    assert_eq!(positions, (0..10).collect::<Vec<_>>());
}

#[test]
fn test1_second_star() {
    let commands = "\
cut 6
deal with increment 7
deal into new stack
";
    let (position_of_card, _) = compose_shuffles(&commands, 10, 1);
    let positions = [3, 0, 7, 4, 1, 8, 5, 2, 9, 6].iter().map(|card| position_of_card(*card)).collect::<Vec<_>>();
    assert_eq!(positions, (0..10).collect::<Vec<_>>());
}

#[test]
fn test2_second_star() {
    let commands = "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
";
    let (position_of_card, _) = compose_shuffles(&commands, 10, 1);
    let positions = [9, 2, 5, 8, 1, 4, 7, 0, 3, 6].iter().map(|card| position_of_card(*card)).collect::<Vec<_>>();
    assert_eq!(positions, (0..10).collect::<Vec<_>>());
}