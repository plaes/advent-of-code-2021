const _DATA: &str = "16,1,2,0,4,2,7,1,2,14";

fn main() {
    let data = include_str!("../data/day07.txt");

    //let data = _DATA;

    let mut arr: Vec<i32> = data
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    // Find middle item..
    let mid = arr.len() / 2;
    let mean = *arr.select_nth_unstable(mid).1;

    println!("Required fuel (part 1): {}", arr.iter().map(|x| (*x - mean).abs()).sum::<i32>());
}
