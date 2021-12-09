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
        "Digits 1, 4, 7 and 8 seen: {}",
        data.lines()
            .map(|l| {
                let (_, out) = l.split_once(" | ").unwrap();
                out.split(' ')
                    .filter(|o| match o.len() {
                        2 | 4 | 3 | 7 => true,
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>()
    );
}
