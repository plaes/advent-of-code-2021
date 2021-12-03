use std::io::Error;

const _DATA: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

fn main() -> Result<(), Error> {
    let data = include_str!("../data/day02.txt");

    // let data = _DATA;
    let (p, d) = data
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(p, d), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (p + v, d),
                ("down", v) => (p, d + v),
                ("up", v) => (p, d - v),
                _ => unreachable!(),
            }
        });
    println!("Position (1): {:?}", p * d);

    // Second part
    let x = data.lines().fold((0, 0, 0), |state, l| {
        let (p, d, a) = state;
        let arr: Vec<&str> = l.split(' ').collect();
        match arr[1].parse::<u32>() {
            Ok(num) => match arr[0] {
                "forward" => (p + num, d + (num * a), a),
                "down" => (p, d, a + num),
                "up" => (p, d, a - num),
                _ => (p, d, a),
            },
            _ => (p, d, a),
        }
    });

    println!("Position (2): {:?}", x.0 * x.1);

    Ok(())
}
