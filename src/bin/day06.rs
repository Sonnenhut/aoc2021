use aoc2021::read_lines;

fn main() {

    let input = read_lines(6)[0].split(",").map(|x| x.parse().unwrap()).collect();
    let (pt1, pt2) = solve(&input);
    println!("pt1: {}", pt1); // 362740
    println!("pt2: {}", pt2); // 1644874076764
}

fn solve(seed: &Vec<u8>) -> (u64, u64) {
    let mut fishes = vec![0; 9];
    for age in seed {
        fishes[*age as usize] += 1;
    }

    let mut day80cnt = 0;
    for day in 0..256 {
        let mut new_fish_cnt = 0;
        // move fish counts down one day
        for (age, amt) in fishes.clone().iter().enumerate() {
            if age == 0 {
                new_fish_cnt = *amt;
                fishes[age] = 0;
            } else {
                fishes[age - 1] += amt;
                fishes[age] = 0;
            }
        }
        fishes[8] = new_fish_cnt;
        fishes[6] += new_fish_cnt;
        if day == 79 {
            day80cnt = fishes.iter().sum();
        }
    }
    (day80cnt, fishes.iter().sum())
}


#[cfg(test)]
mod test {
    use super::*;
    use aoc2021::read_lines;

    #[test]
    fn regression() {
        let test_input = "3,4,3,1,2".split(",").map(|x| x.parse().unwrap()).collect();
        assert_eq!(solve(&test_input), (5934, 26984457539));

        let input = read_lines(6)[0].split(",").map(|x| x.parse().unwrap()).collect();
        assert_eq!(solve(&input), (362740, 1644874076764));
    }
}
