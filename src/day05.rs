use std::cmp::{min, max};
const _DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

fn main() {
    let data = include_str!("../data/day05.txt");
    #[allow(non_snake_case)]
    let N = 1000;

    //let data = _DATA; let N = 10;

    let mut board: Vec<u32> = vec![0; N*N];

    let lines: Vec<_> = data
        .lines()
        .filter_map(|t| {
            // Parse coordinates...
            let (c1, c2) = t.split_once(" -> ").unwrap();
            // TODO: #![feature(iter_zip)] ?
            let a:Vec<_> = c1.splitn(2, ',').map(|c| {
                c.parse::<u32>().unwrap()
            }).collect();
            let b:Vec<_> = c2.splitn(2, ',').map(|c| {
                c.parse::<u32>().unwrap()
            }).collect();
            // ...and filter out non-vert/horiz lines
            if a[0] == b[0] || a[1] == b[1] {
                Some((a, b))
            } else {
                None
            }
        })
        .collect();
    // println!("{:?}", lines);

    for (a, b) in lines {
        let (x0, y0) = (a[0], a[1]);
        let (x1, y1) = (b[0], b[1]);
        for i in min(x0, x1)..=max(x0, x1) {
            for j in min(y0, y1)..=max(y0, y1) {
                let idx: usize = i as usize * N + j as usize;
                // println!("idx: {}", idx);
                board[idx] += 1;
                // println!("{},{}", i, j);
            }
        }
    }
    println!("Matching points (part1): {}", board.iter().filter(|&x| *x > 1).count());

    // Part 2 TODO

}
