use aoc2021::read_lines;
use std::collections::HashSet;
use std::cmp::min;


fn main() {
    //let (nums, boards) = parse_input(&read_lines(4));
    //let (pt1, pt2) = solve(nums, boards);
    //println!("pt1: {}", pt1); // 25023
    //println!("pt2: {}", pt2); // 2634
}

fn pt1(raw_inputs: Vec<String>) -> usize {
    let lines : Vec<Line> = raw_inputs.iter()
        .map(|line| parse_string(line))
        .filter(|line| line.is_horizontal_vertical())
        .collect();
    let mut allpoints = HashSet::new();
    let mut duppoints = HashSet::new();
    for line in lines {
        for point in line.points() {
            if !allpoints.insert(point) {
                duppoints.insert(point);
            }
        }
    }
    //println!("{:?}", duppoints);
    duppoints.len()
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
    fn intersection(&self, other: &Line) -> HashSet<(usize, usize)> {
        let mut res = HashSet::new();
        for me in self.points() {
            for you in other.points() {
                if me == you {
                    res.insert(me);
                }
            }
        }
        res
    }

    fn points(&self) -> HashSet<(usize, usize)> {
        let mut res = HashSet::new();
        //println!("LINE! {:?}", self);

        if self.start.0 == self.end.0 {
            //println!("atl calc!");
            let ymin = self.start.1.min(self.end.1);
            let ymax = self.start.1.max(self.end.1);
            for y in ymin ..= ymax {
                //println!("point {:?}", (self.start.0, y));
                res.insert((self.start.0,y));
            }
        } else {
            // Calc points in line with y = mx + c;
            let m0 = (self.start.1 as isize - self.end.1 as isize);
            let m1 = (self.start.0 as isize - self.end.0 as isize);
            let m = m0 / m1;
            let c = self.start.1 as isize - self.start.0 as isize * m;

            let minx = self.start.0.min(self.end.0);
            let maxx = self.start.0.max(self.end.0);


            for x in minx ..= maxx {
                res.insert((x, (m * x as isize + c) as usize));
                //println!("point {:?}", (x, (m * x as isize + c) as usize))
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
        assert_eq!(pt1(test_input), 5);

        let lines = read_lines(5);
        assert_eq!(pt1(lines), 5145);
    }
}
