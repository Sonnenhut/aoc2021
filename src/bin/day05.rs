use aoc2021::read_lines;
use std::collections::HashSet;
use std::time::SystemTime;

fn main() {
    let lines = read_lines(5);
    let (pt1, pt2) = solve(&lines);

    println!("pt1: {}", pt1); // 5145
    println!("pt2: {}", pt2); // 16518
}

fn solve(raw_inputs: &Vec<String>) -> (usize, usize) {
    let lines : Vec<Line> = raw_inputs.iter()
        .map(|line| parse_string(line))
        .collect();
    let mut pt1 = HashSet::new();
    let mut pt1dups = HashSet::new();
    let mut pt2 = HashSet::new();
    let mut pt2dups = HashSet::new();
    for line in lines {
        for point in line.points() {
            if line.is_horizontal_vertical() && !pt1.insert(point) {
                pt1dups.insert(point);
            }
            if !pt2.insert(point) {
                pt2dups.insert(point);
            }
        }
    }
    (pt1dups.len(), pt2dups.len())
}

fn parse_string(line: &String)  -> Line {
    let mut parts = line.split(" -> ");
    let mut start = parts.next().unwrap().split(",").map(|s|s.parse::<usize>().unwrap());
    let mut end = parts.next().unwrap().split(",").map(|s|s.parse::<usize>().unwrap());
    Line {
        start: (start.next().unwrap(),start.next().unwrap()),
        end: (end.next().unwrap(), end.next().unwrap())
    }
}

#[derive(Debug)]
struct Line {
    start: (usize, usize),
    end: (usize, usize)
}

impl Line {
    fn points(&self) -> HashSet<(usize, usize)> {
        // poor mans approach to get all points of a line segment...

        let mut res = HashSet::new();
        let ((x1, y1), (x2,y2)) = (self.start, self.end);

        if self.start.0 == self.end.0 { // edgecase when on the same x-axis would divide by zero!
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            for y in ymin ..= ymax {
                res.insert((self.start.0, y));
            }
        } else {
            // Calc points in line with y = mx + c;
            let m = (y1 as isize - y2 as isize) / (x1 as isize - x2 as isize);
            let c = y1 as isize - x1 as isize * m;

            let minx = x1.min(x2);
            let maxx = x1.max(x2);

            for x in minx ..= maxx {
                res.insert((x, (m * x as isize + c) as usize));
            }
        }
        res
    }

    fn is_horizontal_vertical(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let test_input =
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2".split("\n").map(|x| x.to_string()).collect();
        assert_eq!(solve(&test_input).0, 5);

        let lines = read_lines(5);
        assert_eq!(solve(&lines), (5145, 16518));
    }
}
