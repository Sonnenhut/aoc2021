use aoc2021::read_lines;

fn main() {
    let input = read_lines(7)[0].split(",").map(|x| x.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&input)); // 344735
    println!("pt2: {}", pt2(&input)); // 96798233
}

fn pt1(seed: &Vec<u64>) -> u64 {
    let mut sorted_seed = seed.clone();
    sorted_seed.sort();
    let median = sorted_seed[seed.len() / 2];

    sorted_seed.iter().map(|n| (*n as i64 - median as i64).abs()).sum::<i64>() as u64
}

fn pt2(seed: &Vec<u64>) -> u64 {
    let sum : u64 = seed.iter().sum();
    let mean = (sum as f64 / seed.len() as f64).round() as u64;

    let mut res = u64::MAX;
    // although for my input it seems to be the mean rounded down, try to search around the mean
    for pos in mean-5..=mean+5 {
        let fuel = seed.iter()
            .map(|n| (*n as i64 - pos as i64).abs() as u64) // calc distance
            .map(|dist| (1..=dist).sum::<u64>()) // calc fuel cost
            .sum();
        if fuel < res {
            res = fuel;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2021::read_lines;

    #[test]
    fn regression() {
        let test_input = "16,1,2,0,4,2,7,1,2,14".split(",").map(|x| x.parse().unwrap()).collect();
        assert_eq!(pt1(&test_input), 37);
        assert_eq!(pt2(&test_input), 168);

        let input = read_lines(7)[0].split(",").map(|x| x.parse().unwrap()).collect();
        assert_eq!(pt1(&input), 344735);
        assert_eq!(pt2(&input), 96798233);
    }
}
