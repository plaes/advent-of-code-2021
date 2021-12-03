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
    let (position, depth): (Vec<&str>, Vec<&str>) =
        data.lines().partition(|&l| l.starts_with("forward"));

    // first part
    let pos: u32 = position
        .iter()
        .filter_map(|l| {
            let arr: Vec<&str> = l.split(' ').collect();
            arr[1].parse::<u32>().ok()
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    // println!("pos: {:?}", pos);

    let depth: i32 = depth
        .iter()
        .filter_map(|l| {
            let arr: Vec<&str> = l.split(' ').collect();
            //println!("{:?}", arr);
            match arr[1].parse::<i32>() {
                Ok(num) => match arr[0] {
                    "down" => Some(num),
                    "up" => Some(-num),
                    _ => None,
                },
                _ => None,
            }
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    //println!("depth: {:?}", depth);
    println!("Position (1): {:?}", pos as i32 * depth);

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
