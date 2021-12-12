const _DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

fn main() {
    let data = include_str!("../data/day09.txt");
    // let data = _DATA;

    // Part 1
    // Parse board
    let board: Vec<Vec<_>> = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let mut mins: Vec<(u8, usize, usize)> = vec![];

    // Find horizontal mins
    let _x: Vec<_> = board
        .iter()
        .enumerate()
        .map(|(i, l)| {
            if l[0] < l[1] {
                mins.insert(0, (l[0], i, 0));
            }
            for (j, n) in l.windows(3).enumerate() {
                if n[0] > n[1] && n[1] < n[2] {
                    mins.insert(0, (n[1], i, j + 1));
                }
            }
            if l[l.len() - 2] > l[l.len() - 1] {
                mins.insert(0, (l[l.len() - 1], i, l.len() - 1));
            }
        })
        .collect();
    let result: u32 = mins
        .iter()
        .filter_map(|(n, i, j)| {
            if i > &0 && n > &board[*i as usize - 1][*j] {
                return None;
            }
            if i < &(board.len() - 1) && n > &board[*i as usize + 1][*j] {
                return None;
            }
            Some(*n as u32 + 1)
        })
        .sum();
    println!("{:?}", result);
    // println!("{:?}", board);
}
