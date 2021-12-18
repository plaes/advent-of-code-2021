const _DATA: &str = "11111
19991
19191
19991
11111";

#[rustfmt::skip]
const adjacent: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), /* ... */ (0, 1),
    (1, -1), (1, 0), (1, 1),
];

fn flash(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    // Set current location to 0
    println!("{},{}", x, y);
    map[x][y] = 0;
    // Iterate over adjacent coords...
    adjacent
        .iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |sum, (x, y)| {
            println!("({},{})", x, y);
            match map.get_mut(y).and_then(|line| line.get_mut(x)) {
                Some(item) if *item > 0 => {
                    *item += 1;
                    sum + (*item > 9).then(|| flash(map, x, y)).unwrap_or(0)
                }
                _ => sum,
            }
        })
}

fn main() {
    //let data = include_str!("../data/day09.txt");
    let data = _DATA;

    // Part 1
    // Parse map
    let mut map = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    // Part 1
    // XXX: Simplification assuming that we always have n x n map.. :(
    let N = map.len();

    (0..=2).fold(0, |sum, _| {
        println!("{:?}", map);

        // Increase all the items...
        map.iter_mut()
            .for_each(|line| line.iter_mut().for_each(|item| *item += 1));
        // Run over the maze and collect flashes recursively...
        sum + (0..N)
            .flat_map(|y| (0..N).map(move |x| (x, y)))
            .fold(0, |sum, (x, y)| {
                sum + (map[x][y] > 9).then(|| flash(&mut map, x, y)).unwrap_or(0)
            })
    });

    // Part 2
    // TODO
}
