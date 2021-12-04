const _DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

fn main() {
    let data = include_str!("../data/day04.txt");

    //let data = _DATA;

    let (num_raw, table_raw) = data.split_once("\n\n").unwrap();

    let tables: Vec<Vec<u32>> = table_raw
        .split("\n\n")
        .map(|t| {
            t.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut rows = vec![vec![vec![] as Vec<u32>; 5]; tables.len()];
    let mut cols = vec![vec![vec![] as Vec<u32>; 5]; tables.len()];
    let mut solved: bool = false;

    let numbers: Vec<u32> = num_raw
        .split(',')
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    for n in &numbers {
        // Attempt to find numbers...
        for (i, t) in tables.iter().enumerate() {
            if let Some(x) = t.iter().position(|x| *x == *n) {
                let row = x / 5;
                let col = x % 5;
                let r = &rows[i][row];
                let c = &cols[i][col];
                // We have a hit :)
                if (r.len() == 4) || (c.len() == 4) {
                    // Find total sum of table..
                    let ts: u32 = tables[i].iter().sum();
                    // ... subtract all the marked ones including current
                    let s: u32 = rows[i].iter().flatten().sum();

                    let result = (ts - s - n) * n;

                    solved = true;
                    println!("Bingo (part 1): {}", result);
                    break;
                }
                rows[i][row].push(*n);
                cols[i][col].push(*n);
            }
        }

        if solved {
            break;
        }
    }

    // Part two (find the last remaining table...
    let mut rows = vec![vec![vec![] as Vec<u32>; 5]; tables.len()];
    let mut cols = vec![vec![vec![] as Vec<u32>; 5]; tables.len()];
    let mut solved = vec![false; tables.len()];
    for n in numbers {
        // Attempt to find numbers...
        for (i, t) in tables.iter().enumerate() {
            if solved[i] {
                continue;
            }
            if let Some(x) = t.iter().position(|x| *x == n) {
                let row = x / 5;
                let col = x % 5;
                let r = &rows[i][row];
                let c = &cols[i][col];
                // We have a hit :)
                if (r.len() == 4) || (c.len() == 4) {
                    // Find total sum of table..
                    let ts: u32 = tables[i].iter().sum();
                    // ... subtract all the marked ones including current
                    let s: u32 = rows[i].iter().flatten().sum();

                    let result = (ts - s - n) * n;

                    if solved.iter().filter(|&x| !(*x)).count() == 1 {
                        println!("Bingo (part 2): {}", result);
                    }

                    solved[i] = true;
                }
                rows[i][row].push(n);
                cols[i][col].push(n);
            }
        }
        if solved.iter().all(|&x| x) {
            break;
        }
    }
}
