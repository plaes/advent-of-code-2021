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

    let mut scores: Vec<usize> = vec![];

    let item_score: HashMap<char, usize> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    for line in data.lines() {
        let mut stack: Vec<char> = vec![];
        let mut syntax_ok = true;
        for c in line.chars() {
            if matches!(c, '(' | '[' | '{' | '<') {
                stack.push(c);
            } else if matches!(&c, ')' | ']' | '}' | '>') {
                let f = elem.get(&c).unwrap().to_owned();
                if stack.pop() != Some(f) {
                    //println!("Line is invalid: {:?}", line);
                    // Skip invalid line..
                    syntax_ok = false;
                    break;
                }
            } else {
                unreachable!();
            }
        }
        if syntax_ok && stack.len() > 0 {
            // println!("Line incomplete: {:?}, stack left: {:?}", line, stack);
            let line_score = stack
                .iter()
                .rev()
                .fold(0, |sum, c| sum * 5 + item_score.get(&c).unwrap());
            scores.push(line_score);
            //println!("{}", line_score);
        }
    }
    //&scores.select_nth_unstable(&scores.len() / 2);
    let num = &scores.len() / 2;
    let (_, avg, _) = &scores.select_nth_unstable(num);
    println!("Middle score for incomplete lines (part2): {:?}", avg);
}
