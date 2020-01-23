use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day08.txt")
        .expect("Something went wrong reading the file");

    let checksum = impl_first_star(&contents, 25, 6);

    println!("day  8.1 - number of 1 digits multiplied by the number of 2 digits: {}", checksum);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day08.txt")
        .expect("Something went wrong reading the file");

    let image = impl_second_star(&contents, 25, 6);

    println!("day  8.2 - message is produced after decoding image: \n{}", image);
}

fn impl_first_star(contents: &str, width: usize, height: usize) -> usize {
    let size = width * height;
    decode(contents, size)
        .iter()
        .map(|l| {
            let zeroes = l.iter().filter(|&d| *d == '0').count();
            let ones = l.iter().filter(|&d| *d == '1').count();
            let twos = l.iter().filter(|&d| *d == '2').count();
            (zeroes, ones * twos)
        })
        .min_by_key(|l| l.0).unwrap().1
}

fn impl_second_star(contents: &str, width: usize, height: usize) -> String {
    let size = width * height;
    let layers = decode(contents, size);
    let mut output = vec![' '; size];
    for i in 0..size {
        for layer in &layers {
            if layer[i] != '2' {
                output[i] = if layer[i] == '1' { '*' } else { ' ' };
                break;
            }
        }
    }
    output
        .chunks(width)
        .map(|r| {
            let mut v = r.to_vec();
            v.push('\n');
            v
        })
        .fold(Vec::new(), |mut a, mut r| {
            a.append(&mut r);
            a
        })
        .iter()
        .collect()
}

fn decode(contents: &str, size: usize) -> Vec<Vec<char>> {
    contents
        .chars()
        .collect::<Vec<_>>()
        .chunks(size)
        .map(|c| c.to_vec())
        .collect()
}

#[test]
fn test0_first_star() {
    assert_eq!(decode("123456789012", 3 * 2), [['1','2','3','4','5','6'], ['7','8','9','0','1','2']]);
}

#[test]
fn test0_second_star() {
    assert_eq!(impl_second_star("0222112222120000", 2, 2), " *\n* \n");
}
