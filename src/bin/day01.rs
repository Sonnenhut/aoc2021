use aoc2021::read_lines;

fn main() {
    let nums = read_lines(1).iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("pt1: {}", solve(&nums, 1)); // 1292
    println!("pt2: {}", solve(&nums, 3)); // 1262
}

fn solve(nums: &Vec<i64>, win_size: usize) -> usize {
    let squashed_windows : Vec<i64>= nums.windows(win_size)
        .map(|x| x.iter().sum()).collect();
    squashed_windows.windows(2)
        .filter(|arr| arr[1] > arr[0])
        .count()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve(&test_input,1), 7);
        assert_eq!(solve(&test_input,3), 5);

        let input = read_lines(1).iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(solve(&input,1), 1292);
        assert_eq!(solve(&input, 3), 1262);
    }
}
