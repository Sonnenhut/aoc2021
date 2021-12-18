use std::collections::HashSet;

use aoc2021::read_lines;

fn main() {
    let input = read_lines(8);
    println!("pt1: {}", pt1(&input)); // 416
    println!("pt2: {}", pt2(&input)); // 1043697
}

fn pt1(input: &Vec<String>) -> u64 {
    input.iter()
        .map(|line| line.split(" | ").last().unwrap())
        .map(|last_part| last_part.split(" ").into_iter().filter(|digit| [2,3,4,7].contains(&digit.len())).count())
        .sum::<usize>() as u64
}

fn pt2(input: &Vec<String>) -> u64 {
    input.iter()
        .map(|line| {
            let mut splits = line.split(" | ")
                .map(|part| part.split(" ").collect::<Vec<_>>())
                .map(|s| s.iter().map(|word| word.chars().collect::<HashSet<char>>()).collect::<Vec<_>>());
            let wires = splits.next().unwrap();
            let output = splits.next().unwrap();
            (wires, output)
        })
        .map(|(wires, output)| match_wires(&wires, &output))
        .sum()
}

fn match_wires(wires: &Vec<HashSet<char>>, output: &Vec<HashSet<char>>) -> u64 {
    let mut lowerleft_corner : Option<HashSet<char>> = None;
    let mut n : [Option<&HashSet<char>>;10] = [None; 10];

    while n.contains(&None) {
        for wire in wires {
            let matchnum = match wire.len() {
                2 => Some(1),
                4 => Some(4),
                3 => Some(7),
                7 => Some(8),
                5 => {
                    if let Some(_) = n[1].filter(|n| wire.is_superset(n)) {
                        Some(3)
                    } else if let Some(_) = lowerleft_corner.as_ref().filter(|n| wire.is_superset(n)) {
                        Some(2)
                    } else if let Some(_) = n[6].filter(|n| wire.is_subset(n)) {
                        Some(5)
                    } else { None }
                },
                6 => {
                    if let Some(_) = n[4].filter(|n| wire.is_superset(n)) {
                        Some(9)
                    } else if let Some(_) = n[1].filter(|n| wire.is_superset(n)) {
                        if let Some(_) = lowerleft_corner.as_ref().filter(|n| wire.is_superset(n)) {
                            Some(0)
                        } else { None }
                    } else {
                        Some(6) // specifically when its not overlapping with 1
                    }
                }
                _ => panic!("Cannot parse wire length!")
            };
            if let Some(idx) = matchnum {
                n[idx] = Some(wire);
            }

            // use easy identifiable parts to get to other numbers
            if let (Some(n8), Some(n7), Some(n4)) = (n[8], n[7], n[4]) {
                lowerleft_corner = Some(n8.difference(&n7.union(n4).map(|x| *x).collect()).map(|x| *x).collect());
            }
        }
    }
    output.iter()
        .map(|word| n.iter().position(|other| &word == &other.unwrap()).unwrap())
        .fold(0, |acc, elem| acc * 10 + elem as u64)
}

#[cfg(test)]
mod test {
    use aoc2021::read_lines;

    use super::*;

    #[test]
    fn regression() {
        let test_input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".split("\n").map(|x| x.to_string()).collect();
        assert_eq!(pt1(&test_input), 26);
        assert_eq!(pt2(&test_input),61229);

        let input = read_lines(8);
        assert_eq!(pt1(&input), 416);
        assert_eq!(pt2(&input), 1043697);
    }
}
