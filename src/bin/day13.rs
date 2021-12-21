use std::collections::HashSet;

use aoc2021::read_lines;

fn main() {
    let input = read_lines(13);
    println!("pt1: {}", pt1(&input)); // 607
    println!("pt2: {}", pt2(&input)); // CPZLPFZL to stdout
}

fn pt1(input: &Vec<String>) -> usize {
    let (dots, fold) = parse(input);

    fold_dots(dots, fold[0]).len()
}

fn pt2(input: &Vec<String>) -> usize {
    let (mut dots, fold) = parse(input);

    for foldi in fold {
        dots = fold_dots(dots, foldi);
    }

    let maxx = dots.iter().fold(0, |acc, num| acc.max(num.0));
    let maxy = dots.iter().fold(0, |acc, num| acc.max(num.1));

    for y in 0..=maxy {
        for x in 0..=maxx {
            if let Some(out) = dots.get(&(x,y)).map(|_| "#").or(Some(" ")) {
               print!("{}", out);
            }
        }
        println!();
    }
    dots.len()
}

fn fold_dots(dots: HashSet<(usize, usize)>, (xfold, yfold) : (usize, usize)) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    for (mut x, mut y) in dots {
        if xfold > 0 {
            x = xfold - (x as isize - xfold as isize).abs() as usize;
            res.insert((x, y));
        } else {
            y = yfold - (y as isize - yfold as isize).abs() as usize;
            res.insert((x, y));
        }
    }
    res
}

fn parse(input: &Vec<String>) -> (HashSet<(usize, usize)>, Vec<(usize, usize)>) {
    let mut dots = HashSet::new();
    let mut fold = vec![];

    for line in input {
        let mut split = line.split(&[',', '='][..]);
        let (first, second) = (split.next(), split.next());
        let last_num = second.and_then(|s| s.parse::<usize>().ok());
        if let Some("") = first {
            continue; // skip empty line
        } else if let Some(x) = first.and_then(|s| s.parse::<usize>().ok()) {
            dots.insert((x, last_num.unwrap()));
        } else if let Some(text) = first {
            let fold_at = last_num.unwrap();
            match text.chars().last() {
                Some('x') => fold.push((fold_at, 0)),
                Some('y') => fold.push((0, fold_at)),
                _ => panic!("unable to parse fold..")
            }
        }
    }
    (dots, fold)
}

#[cfg(test)]
mod test {
    use aoc2021::read_lines;

    use super::*;

    #[test]
    fn regression() {
        let test_input =
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5".split("\n").map(|x| x.to_string()).collect();
        assert_eq!(pt1(&test_input), 17);
        assert_eq!(pt2(&test_input), 16); // prints O

        let input = read_lines(13);
        assert_eq!(pt1(&input), 607);
        assert_eq!(pt2(&input), 87); // prints CPZLPFZL
    }
}
