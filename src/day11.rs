/*
const _DATA: &str = "11111
19991
19191
19991
11111";
*/

const _DATA: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[rustfmt::skip]
const adjacent: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), /* ... */ (0, 1),
    (1, -1), (1, 0), (1, 1),
];

fn flash(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    // Set value of current item to 0 (and count it with fold initializer)
    map[x][y] = 0;
    // Iterate over adjacent coords...
    adjacent
        .iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |sum, (xxx, yyy)| {
            match map.get_mut(xxx).and_then(|line| line.get_mut(yyy)) {
                Some(item) if *item > 0 => {
                    *item += 1;
                    sum + (*item > 9).then(|| flash(map, xxx, yyy)).unwrap_or(0)
                }
                _ => sum,
            }
        })
}

fn main() {
    let data = include_str!("../data/day11.txt");
    //let data = _DATA;

    // Part 1
    // Parse map
    let mut map = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Part 1
    // XXX: Simplification assuming that we always have n x n map.. :(
    let N = map.len();
    let total = (0..100).fold(0, |sum, _| {
        // First, run through all the coords and increase them by one...
        map.iter_mut()
            .for_each(|line| line.iter_mut().for_each(|item| *item += 1));
        // ...iterate over the maze and recursively handle flashes...
        sum + (0..N)
            .flat_map(|x| (0..N).map(move |y| (x, y)))
            .fold(0, |sum, (x, y)| {
                sum + (map[x][y] > 9).then(|| flash(&mut map, x, y)).unwrap_or(0)
            })
    });
    println!("Total flashes after 100 steps (part1): {}", total);

    // Part 2
    // TODO
}
