use std::collections::HashSet;

use aoc2021::read_lines;

fn main() {
    /*
    let input = read_lines(8);
    println!("pt1: {}", pt1(&input)); // 416
    println!("pt2: {}", pt2(&input)); // 1043697

     */
}

fn pt1(input: &Vec<String>) -> usize {
    let ground = parse(input);
    let dimensions = (ground[0].len(), ground.len());

    let mut res = 0;
    for (x,y) in find_lowest_points(&ground) {
        res += ground[y][x] + 1;
    }
    res
}

fn parse(input: &Vec<String>) -> Vec<Vec<usize>>{
    input.iter().map(|row| row.chars().map(|x| x.to_string().parse().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn find_lowest_points(ground: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let dimensions = (ground[0].len(), ground.len());

    let mut res = vec![];
    for x in 0..dimensions.0 {
        for y in 0..dimensions.1 {
            let othermin : usize = around((x, y), dimensions)
                .into_iter()
                .map(|(x,y)| ground[y][x])
                .min().unwrap();
            let my = ground[y][x];
            if my < othermin {
                res.push((x, y))
            }
        }
    }
    res
}

fn around(input: (usize, usize), dimensions: (usize, usize)) -> Vec<(usize, usize)> {
    let cmp = (input.0 as isize, input.1 as isize);
    vec![ (-1,0), (0,-1), (1,0), (0,1) ]
        .iter()
        .map(|off| (cmp.0 + off.0, cmp.1 + off.1))
        .filter(|res| res.0 >= 0 && res.1 >= 0)
        .map(|res| (res.0 as usize, res.1 as usize))
        .filter(|res| res.0 < dimensions.0 && res.1 < dimensions.1)
        .collect()
}

#[cfg(test)]
mod test {
    use aoc2021::read_lines;

    use super::*;

    #[test]
    fn regression() {
        let test_input =
            "2199943210
3987894921
9856789892
8767896789
9899965678".split("\n").map(|x| x.to_string()).collect();
        assert_eq!(pt1(&test_input), 15);

        let input = read_lines(9);
        assert_eq!(pt1(&input), 588);
    }
}
