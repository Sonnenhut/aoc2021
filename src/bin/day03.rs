use aoc2021::read_lines;

fn main() {
    //let input = read_lines(2);
    //println!("pt1: {}", pt1(&input)); // 1604850
    //println!("pt2: {}", pt2(&input)); // 11685186100262
}
fn pt1(nums: &Vec<u32>) -> u32 {
    let mut gamma: u32 = 0;
    let highest_bit = (32 - nums.iter().map(|num| num.leading_zeros()).min().unwrap() ) as u8;

    for sig in 0..highest_bit {
        let mut high_cnt = 0;
        for num in nums {
            if is_high_bit(*num, sig) { high_cnt += 1}
        }
        if high_cnt > (nums.len() / 2) {
            gamma |= (1 << sig)
        }
    }
    let eps = gamma ^ (std::u32::MAX >> (32 - highest_bit));


    println!("gamma {} eps {}", gamma, eps);
    gamma * eps
}


fn is_high_bit(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
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

        let input = read_lines(3).iter().map(|x| u32::from_str_radix(x, 2).unwrap()).collect();
        assert_eq!(pt1(&input), 198);
    }
}
