use std::collections::BTreeMap;

use aoc2021::read_lines;
use std::array::IntoIter;
use std::iter::FromIterator;
use crate::CheckResult::{IncompleteSyntax, IllegalCharacter, OK};

fn main() {
    let input = read_lines(10);
    println!("pt1: {}", pt1(&input)); // 290691
    println!("pt2: {}", pt2(&input)); // 2768166558
}

fn pt1(input: &Vec<String>) -> usize {
    let points = BTreeMap::from_iter(IntoIter::new([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]));

    input.iter()
        .map(|line| check(line))
        .filter_map(|illegal_char| {
            match illegal_char {
                IllegalCharacter { illegal_sign} => points.get(&illegal_sign),
                _ => None
            }
        })
        .sum()
}
fn pt2(input: &Vec<String>) -> usize {
    let points = BTreeMap::from_iter(IntoIter::new([(')', 1), (']', 2), ('}', 3), ('>', 4)]));

    let mut syntax_scores = input.iter()
        .map(|line| check(line))
        .filter_map(|checkres| {
            match checkres {
                IncompleteSyntax { missing } => Some(missing),
                _ => None
            }
        })
        .map(|signs|
            signs.chars()
                .into_iter()
                .map(|c|points[&c])
                .fold(0, |acc, num | acc * 5 + num)
        ).collect::<Vec<_>>();
    syntax_scores.sort();
    syntax_scores[syntax_scores.len() / 2]
}

fn check(line : &String) -> CheckResult {
    let mut res = OK;
    let mut history = vec![line.chars().next().unwrap()];
    for item in line.chars().into_iter().skip(1) {
        if let Some(expected_opening) = matching_opening(item) {
            if history.last() == Some(&expected_opening) {
                history.pop();
            } else {
                // syntax error
                res = IllegalCharacter { illegal_sign: item };
                break;
            }
        } else {
            history.push(item);
        }
    }
    if let OK = res {
        let missing = history.into_iter().rev().filter_map(|s| matching_closing(s)).collect::<String>();
        res = IncompleteSyntax { missing }
    }
    res
}

fn matching_opening(s : char) -> Option<char> {
    match s {
        '}' => Some('{'),
        ']' => Some('['),
        ')' => Some('('),
        '>' => Some('<'),
        _ => None
    }
}

fn matching_closing(s : char) -> Option<char> {
    match s {
        '{' => Some('}'),
        '[' => Some(']'),
        '(' => Some(')'),
        '<' => Some('>'),
        _ => None
    }
}

#[derive(PartialEq)]
enum CheckResult {
    IllegalCharacter { illegal_sign: char},
    IncompleteSyntax { missing: String },
    OK
}

#[cfg(test)]
mod test {
    use aoc2021::read_lines;

    use super::*;


    #[test]
    fn regression() {
        let test_input =
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".split("\n").map(|x| x.to_string()).collect();
        assert_eq!(pt1(&test_input), 26397);
        assert_eq!(pt2(&test_input), 288957);

        let input = read_lines(10);
        assert_eq!(pt1(&input), 290691);
        assert_eq!(pt2(&input), 2768166558);
    }
}
