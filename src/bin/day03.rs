use aoc2021::read_lines;

fn main() {
    let input = read_lines(3).iter().map(|x| u32::from_str_radix(x, 2).unwrap()).collect();
    println!("pt1: {}", pt1(&input)); // 3959450
    println!("pt2: {}", pt2(&input)); // 7440311
}

fn pt1(nums: &Vec<u32>) -> u32 {
    let mut gamma: u32 = 0;
    let highest_bit = (31 - nums.iter().map(|num| num.leading_zeros()).min().unwrap() ) as u8;

    for sig in 0..=highest_bit {
        let mut high_cnt = 0;
        for num in nums {
            if is_high_bit(*num, sig) { high_cnt += 1}
        }
        if high_cnt > (nums.len() / 2) {
            gamma |= 1 << sig
        }
    }
    let eps = gamma ^ (u32::MAX >> (31 - highest_bit));

    gamma * eps
}

fn is_high_bit(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn pt2(input: &Vec<u32>) -> u32 {
    let oxy = calc_rating_pt2(input, |hi, lo| hi >= lo);
    let co2 = calc_rating_pt2(input, |hi, lo| hi < lo);
    oxy * co2
}

fn calc_rating_pt2(input: &Vec<u32>, keep: fn(usize, usize) -> bool) -> u32 {
    let mut res = input.to_vec();
    let highest_bit = (31 - res.iter().map(|num| num.leading_zeros()).min().unwrap() ) as u8;
    let mut sig = highest_bit;

    while res.len() != 1 {
        let high_cnt = res.iter().filter(|num| is_high_bit(**num, sig)).count();
        let keep_high = keep(high_cnt, res.len() - high_cnt);
        res.retain(|num| is_high_bit(*num, sig) == keep_high);
        sig = sig.wrapping_sub(1);
    }
    *res.get(0).unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let test_input : Vec<u32> =
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".split("\n").map(|x| u32::from_str_radix(x, 2).unwrap()).collect();

        assert_eq!(pt1(&test_input), 198);
        assert_eq!(pt2(&test_input), 230);

        let input = read_lines(3).iter().map(|x| u32::from_str_radix(x, 2).unwrap()).collect();
        assert_eq!(pt1(&input), 3959450);
        assert_eq!(pt2(&input), 7440311);
    }
}
