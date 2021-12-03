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
    let data: Vec<_> = include_str!("../data/day01.txt")
        //let data = DATA
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect();

    // First part
    // Count the number of times a depth measurement increases from the
    // previous measurement.
    println!(
        "Number of increases (1): {}",
        data.windows(2).filter(|w| w[0] < w[1]).count()
    );

    // Second part
    // Instead, consider sums of a three-measurement sliding window.
    // Your goal now is to count the number of times the sum of measurements in
    // this sliding window increases from the previous sum.

    // Build sums for sliding window with size 3
    let sums: Vec<usize> = data.windows(3).map(|w| w.iter().sum()).collect();

    println!(
        "Number of increases (2): {}",
        sums.windows(2).filter(|w| w[0] < w[1]).count()
    );

    Ok(())
}
