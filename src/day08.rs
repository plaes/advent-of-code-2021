use std::collections::HashSet;

const _DATA: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

fn main() {
    let data = include_str!("../data/day08.txt");
    //let data = _DATA;

    // Part 1
    // How many times digits 1,4,7,8 appear.
    // We look for items with 2, 4, 3, 7 segments.
    println!(
        "Digits 1, 4, 7 and 8 seen (part1): {}",
        data.lines()
            .map(|l| {
                let (_, out) = l.split_once(" | ").unwrap();
                out.split(' ')
                    .filter(|o| matches!(o.len(), 2 | 4 | 3 | 7))
                    .count()
            })
            .sum::<usize>()
    );

    // Part 2
    // Start decoding.. and add up all the digits :)
    let total: u32 = data
        .lines()
        .map(|l| {
            let (i, o) = l.split_once(" | ").unwrap();
            let one: HashSet<char> =
                HashSet::from_iter(i.split(' ').find(|s| s.len() == 2).unwrap().chars());
            let four: HashSet<char> =
                HashSet::from_iter(i.split(' ').find(|s| s.len() == 4).unwrap().chars());

            o.split(' ')
                .map(|s| match s.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    5 | 6 => {
                        let nn = s.len();

                        let item: HashSet<char> = HashSet::from_iter(s.chars());
                        let oo = item.intersection(&one).count();
                        let ff = item.intersection(&four).count();

                        match (oo, ff, nn) {
                            (2, 3, 6) => 0,
                            (1, 3, 6) => 6,
                            (2, 4, 6) => 9,
                            (1, 2, 5) => 2,
                            (2, 3, 5) => 3,
                            (1, 3, 5) => 5,
                            (_, _, _) => unreachable!(),
                        }
                    }
                    7 => 8,
                    n @ _ => unreachable!("{}", n),
                })
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .map(|n| n.parse::<u32>().unwrap())
        .sum();

    println!("Sum of all outputs (part2): {:?}", total);
}
