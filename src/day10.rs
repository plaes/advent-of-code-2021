use std::collections::HashMap;

const _DATA: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

fn main() {
    let data = include_str!("../data/day10.txt");
    //let data = _DATA;

    let elem: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut sum = 0;

    for line in data.lines() {
        let mut stack: Vec<char> = vec![];
        for c in line.chars() {
            if matches!(c, '(' | '[' | '{' | '<') {
                stack.push(c);
            } else if matches!(&c, ')' | ']' | '}' | '>') {
                let f = elem.get(&c).unwrap().to_owned();
                if stack.pop() != Some(f) {
                    // println!("Failed: {:?}", c);
                    sum += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    };
                    break;
                }
            } else {
                unreachable!();
            }
        }
    }

    println!("Total syntax error score (part1): {}", sum);
}
