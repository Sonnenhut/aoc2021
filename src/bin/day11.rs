use std::collections::HashSet;

use aoc2021::read_lines;

fn main() {
    let input = read_lines(11);
    println!("pt1: {}", pt1(&input)); // 1647
    println!("pt2: {}", pt2(&input)); // 348
}

fn pt1(input: &Vec<String>) -> usize {
    let mut grid: Vec<Vec<u32>> = input.iter().map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect()).collect();

    (0..100).map(|_| step(&mut grid)).sum()
}

fn pt2(input: &Vec<String>) -> usize {
    let mut grid: Vec<Vec<u32>> = input.iter().map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect()).collect();

    let mut flashes = 0;
    let mut idx = 0;
    while flashes != 100 {
        flashes = step(&mut grid);
        idx += 1;
    }

    idx
}

fn step(grid: &mut Vec<Vec<u32>>) -> usize {
    let mut to_flash = vec![];
    // step 1: inc 1
    for x in 0..10 {
        for y in 0..10 {
            grid[y][x] += 1;
            if grid[y][x] > 9 {
                to_flash.push((x,y))
            }
        }
    }

    // step 2: chain flash
    let mut flashed = HashSet::new();
    while let Some((x,y)) = to_flash.pop() {
        if grid[y][x] > 9 {
            for (xa, ya) in around((x,y)) {
                if !flashed.contains(&(xa, ya)) {
                    grid[ya][xa] += 1;
                    to_flash.push((xa, ya));
                }
            }

            flashed.insert((x,y));
            grid[y][x] = 0; // step 3: reset
        }
    }

    flashed.len()
}

fn around((x,y): (usize, usize)) -> Vec<(usize, usize)>{
    [(0,1),(1,0),(-1,0),(0,-1),(-1,1),(1,-1),(-1,-1),(1, 1)]
        .iter()
        .map(|(xoff, yoff)| (x as isize + xoff, y as isize + yoff))
        .filter(|(xn, yn)| *xn >= 0 && *yn >= 0)
        .filter(|(xn, yn)| *xn < 10 && *yn < 10)
        .map(|(xn, yn)|(xn as usize, yn as usize))
        .collect()
}

#[cfg(test)]
mod test {
    use aoc2021::read_lines;

    use super::*;


    #[test]
    fn regression() {
        let test_input =
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526".split("\n").map(|x| x.to_string()).collect();
        assert_eq!(pt1(&test_input), 1656);
        assert_eq!(pt2(&test_input), 195);

        let input = read_lines(11);
        assert_eq!(pt1(&input), 1647);
        assert_eq!(pt2(&input), 348);
    }
}
