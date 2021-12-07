const _DATA: &str = "3,4,3,1,2";

fn main() {
    let data = include_str!("../data/day06.txt");

    //let data = _DATA;

    let mut arr: Vec<u32> = data
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    for _day in 1..=80 {
        let mut new = 0;
        arr.iter_mut().for_each(|c| {
            *c = match *c == 0 {
                true => {
                    new += 1;
                    6
                }
                false => *c - 1,
            }
        });

        arr.append(&mut vec![8; new]);
        println!("{:?}", _day);
    }
    println!("Number of lanternfish (part1) {:?}", arr.len());

    // Part 2
    let mut c = vec![0; 9];

    // Buld initial buckets
    for age in data.trim().split(',').map(|n| n.parse::<usize>().unwrap()) {
        c[age] += 1;
    }

    println!("{:?}", c);
    for _day in 1..=256 {
        let mut new = 0;
        new = c[0];
        c[0] = c[1];
        c[1] = c[2];
        c[2] = c[3];
        c[3] = c[4];
        c[4] = c[5];
        c[5] = c[6];
        c[6] = c[7] + new;
        c[7] = c[8];
        c[8] = new;
        //println!("DAY: {}, NEW: {} - {:?} - total: {}", _day, new, c, c.iter().sum::<usize>());
    }

    println!(
        "Number of lanternfish on day 256 (part 2): {:?}",
        c.iter().sum::<usize>()
    );
}
