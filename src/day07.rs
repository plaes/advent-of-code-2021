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

    println!(
        "Required fuel (part 1): {}",
        arr.iter().map(|x| (*x - mean).abs()).sum::<i32>()
    );

    // Part 2
    // n + (n + 1) + (n + 2) + (n + 3) .. -> n(n + 1) ?

    let avg: i32 = arr.iter().sum::<i32>() / arr.len() as i32;
    let res = ((avg - 2)..=(avg + 2))
        .map(|x| {
            arr.iter()
                .map(|n| {
                    let d = (x - n).abs();
                    d * (d + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap();
    println!("Required fuel (part 2): {}", res);
}
