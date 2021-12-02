use std::io::Error;

const _DATA: &str = "199
200
208
210
200
207
240
269
260
263";

fn main() -> Result<(), Error> {
    let data = include_str!("../data/day01.txt")
    //let data = DATA
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<u16>>();

    // First part
    // Count the number of times a depth measurement increases from the
    // previous measurement.
    let inc_1 = data.iter().scan(u16::MAX, |prev, &x| {
        let inc = *prev < x;
        *prev = x;
        Some(inc)
    }).filter(|x| *x == true).collect::<Vec<bool>>().len();

    println!("Number of increases (1): {:?}", inc_1);

    // Second part
    // Instead, consider sums of a three-measurement sliding window.
    // Your goal now is to count the number of times the sum of measurements in
    // this sliding window increases from the previous sum. 

    let window_sums = data.windows(3).map(|x| x.to_vec().iter().sum::<u16>()).collect::<Vec<u16>>();

    let inc_2 = window_sums.iter().scan(u16::MAX, |prev, &x| {
        let inc = *prev < x;
        *prev = x;
        Some(inc)
    }).filter(|x| *x == true).collect::<Vec<bool>>().len();

    println!("Number of increases (2): {:?}", inc_2);

    Ok(())
}
