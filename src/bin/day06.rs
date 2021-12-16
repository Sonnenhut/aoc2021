use aoc2021::read_lines;
use std::collections::HashSet;
use std::time::SystemTime;

fn main() {
    /*let lines = read_lines(5);
    let (pt1, pt2) = solve(&lines);

    println!("pt1: {}", pt1); // 5145
    println!("pt2: {}", pt2); // 16518

     */
}

fn pt1(seed: &Vec<u8>) -> u64{
    println!("{:?}", seed);
    0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let test_input = "3,4,3,1,2".split(",").map(|x| x.parse().unwrap()).collect();
        assert_eq!(pt1(&test_input), 5);
    }
}
