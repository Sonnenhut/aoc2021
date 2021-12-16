use aoc2021::read_lines;

fn main() {
/*
    let input = read_lines(6)[0].split(",").map(|x| x.parse().unwrap()).collect();
    let (pt1, pt2) = solve(&input);
    println!("pt1: {}", pt1); // 362740
    println!("pt2: {}", pt2); // 1644874076764

 */
}

fn pt1(seed: &Vec<u64>) -> u64 {
    let mut sorted_seed = seed.clone();
    sorted_seed.sort();
    let median = sorted_seed[seed.len() / 2];

    sorted_seed.iter().map(|n| (*n as i64 - median as i64).abs()).sum::<i64>() as u64
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2021::read_lines;

    #[test]
    fn regression() {
        let test_input = "16,1,2,0,4,2,7,1,2,14".split(",").map(|x| x.parse().unwrap()).collect();
        assert_eq!(pt1(&test_input), 37);

        let input = read_lines(7)[0].split(",").map(|x| x.parse().unwrap()).collect();
        assert_eq!(pt1(&input), 344735);

    }
}
