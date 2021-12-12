const _DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

fn main() {
    let data = include_str!("../data/day09.txt");
    //let data = _DATA;

    // Part 1
    // Parse board
    let board = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    // Part 1
    let adjacent = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut sum = 0;
    for (y, line) in board.iter().enumerate() {
        for (x, item) in line.iter().enumerate() {
            if adjacent.iter().all(|(xx, yy)| {
                let f = board.get((y as isize + yy) as usize);
                let ff = f.and_then(|l| l.get((x as isize + xx) as usize));
                let fff = ff.map(|n| item < n);
                fff.unwrap_or(true)
            }) {
                sum += item + 1;
            }
        }
    }
    println!("Sum of risk levels of all low points (part1): {:?}", sum);

    // Part 2
    // WIP: Find all 9?
    // TODO
}
