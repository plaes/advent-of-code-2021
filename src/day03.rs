use std::io::Error;

const _DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

fn main() -> Result<(), Error> {
    let data = include_str!("../data/day03.txt");

    //let data = _DATA;

    let mut lines = data.lines().peekable();
    let num = &lines.peek().unwrap().len().clone();
    let mut gv: Vec<u32> = Vec::with_capacity(*num);
    gv.resize(*num, 0);

    // Fill in gamma bit counts and find number of total entries
    let total = &lines
        .filter_map(|l| u32::from_str_radix(l, 2).ok())
        .map(|n| {
            // println!("n: {}, {:b}", n, n);
            for x in 0..(*num) {
                gv[x] += (n >> x) & 1;
                // print!("{}", (n >> x) & 1)
            }
            // println!("");
            // n
        })
        .count();

    let gs: String = gv
        .iter()
        .rev()
        .map(|x| if *x as usize > (total / 2) { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.chars())
        .flatten()
        .collect();

    let es: String = gv
        .iter()
        .rev()
        .map(|x| if *x as usize > (total / 2) { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.chars())
        .flatten()
        .collect();

    // println!("{} {}", gs, es);

    let gamma = usize::from_str_radix(&gs, 2).unwrap();
    let epsilon = usize::from_str_radix(&es, 2).unwrap();

    println!(
        "{} [gamma] x {} [epsilon] = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    // part 2
    // build initial lists
    let (mut ones, mut zero): (Vec<&str>, Vec<&str>) =
        data.lines().partition(|&l| l.starts_with("1"));

    let mut o2 = vec![];
    let mut co2 = vec![];

    println!("{:?}, {}", ones.len(), ones[0]);
    println!("{:?}, {}", zero.len(), zero[0]);

    // Divide up the lists
    if ones.len() >= zero.len() {
        o2 = ones;
        co2 = zero;
    } else {
        o2 = zero;
        co2 = ones;
    }

    println!("0: {:?}", o2);
    for i in 1..(*num) {
        let (mut x, mut y): (Vec<&str>, Vec<&str>) = o2
            .iter()
            .partition(|&l| l.chars().collect::<Vec<char>>()[i] == '1');
        println!("{}: x:{:?} y:{:?}", i, x, y);
        if x.len() >= y.len() {
            o2 = x;
        } else {
            o2 = y;
        }

        if o2.len() == 1 {
            break;
        }
    }

    for i in 1..(*num) {
        let (mut x, mut y): (Vec<&str>, Vec<&str>) = co2
            .iter()
            .partition(|&l| l.chars().collect::<Vec<char>>()[i] == '0');
        println!("{}: x:{:?} y:{:?}", i, x, y);
        if x.len() <= y.len() {
            co2 = x;
        } else {
            co2 = y;
        }

        if co2.len() == 1 {
            break;
        }
    }

    println!("{:?}", o2);
    println!("{:?}", co2);

    let x = vec![o2, co2];

    let result: u32 = x
        .iter()
        .flatten()
        .filter_map(|l| u32::from_str_radix(l, 2).ok())
        .product();
    println!("{:?}", result);

    Ok(())
}
