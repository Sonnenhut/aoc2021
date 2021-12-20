use aoc2021::read_lines;

fn main() {
    let input = read_lines(2);
    println!("pt1: {}", pt1(&input)); // 1604850
    println!("pt2: {}", pt2(&input)); // 11685186100262
}

fn pt1(directions: &Vec<String>) -> i64 {
    let (x, y) =
        directions.into_iter()
            .map(parse_direction_pt1)
            .fold((0, 0),|acc, curr| (acc.0 + curr.0 , acc.1 + curr.1));
    x * y
}

fn parse_direction_pt1(instruction: &String) -> (i64, i64) {
    let mut splits = instruction.split(" ");
    let direction = splits.next().unwrap();
    let amt = splits.next().map(|s| s.parse::<i64>()).unwrap().unwrap();

    match direction{
        "forward" => (amt, 0),
        "up" => (0, -amt),
        "down" => (0, amt),
        _ => (0,0)
    }
}

fn pt2(directions: &Vec<String>) -> i64 {
    let (x, y, _) = directions.into_iter()
        .fold((0, 0, 0),|acc, curr|  {
            let diff = parse_direction_pt2(curr, acc.2);
            (acc.0 + diff.0 ,acc.1 + diff.1, acc.2 + diff.2)
        });
    x * y
}

fn parse_direction_pt2(instruction: &String, aim: i64) -> (i64, i64, i64) { // (x, y, aim)
    let mut splits = instruction.split(" ");
    let direction = splits.next().unwrap();
    let amt = splits.next().map(|s| s.parse::<i64>()).unwrap().unwrap();

    match direction{
        "forward" => (amt, amt * aim, 0),
        "up" => (0, 0, -amt),
        "down" => (0, 0, amt),
        _ => (0,0,0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let test_input = "forward 5
down 5
forward 8
up 3
down 8
forward 2".split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(pt1(&test_input), 150);
        assert_eq!(pt2(&test_input), 900);

        let input = read_lines(2);
        assert_eq!(pt1(&input), 1604850);
        assert_eq!(pt2(&input), 1685186100);
    }
}
