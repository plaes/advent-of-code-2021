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
    println!("Final Position (1): {:?}", p * d);

    // Second part
    #[rustfmt::skip]
    let (p, d, _) = data.lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(p, d, a), (k, v)|
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (p + v, d + (v * a), a),
                ("down", v) => (p, d, a + v),
                ("up", v) => (p, d, a - v),
                _ => unreachable!(),
            },
    );

    println!("Final Position (2): {:?}", p * d);

    Ok(())
}
