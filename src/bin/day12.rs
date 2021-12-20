use std::collections::{HashSet, HashMap};

use aoc2021::read_lines;

fn main() {
    let input = read_lines(12);
    println!("pt1: {}", pt1(&input)); // 3779
    println!("pt2: {}", pt2(&input)); // 96988
}

fn pt1(input: &Vec<String>) -> usize {
    solve(input, false)
}

fn pt2(input: &Vec<String>) -> usize {
    solve(input, true)
}

fn solve(input: &Vec<String>, more_time_avail: bool) -> usize {
    let inputlink = input.iter().map(|line| {
            let mut sp = line.split("-");
            (sp.next().unwrap(), sp.next().unwrap())
        })
        .fold(HashMap::new(), |mut acc, ln| {
            acc.entry(ln.0).or_insert_with(|| HashSet::new()).insert(ln.1);
            acc.entry(ln.1).or_insert_with(|| HashSet::new()).insert(ln.0);
            acc
        });
    let mut states = vec![State {link: inputlink, pos: "start", path: vec![], time_avail: more_time_avail}];
    let mut ended_paths : HashSet<Vec<&str>> = HashSet::new();

    while let Some(State {link, pos, path : mut path, time_avail }) = states.pop() {
        path.push(pos);
        if pos == "start" && path.len() > 1 {
            continue; // visiting start again. nuh-uh
        } else if pos == "end" {
            ended_paths.insert(path);
        } else if let Some(next_ones) = link.get(pos) {
            for next in next_ones {
                let mut nextlink = link.clone();

                if pos.chars().next().unwrap().is_lowercase() {
                    nextlink = remove_reference(pos, &link);
                    if time_avail {
                        // got some more time. maybe try reaching this one again next time?
                        states.push(State {link: link.clone(), pos: next, path: path.clone(), time_avail: false});
                    }
                }
                states.push(State {link: nextlink, pos: next, path: path.clone(), time_avail});
            }
        }
    }
    ended_paths.len()
}

fn remove_reference<'a>(pos: &'a str, link: &HashMap<&'a str, HashSet<&'a str>>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut nextlink = link.clone();
    nextlink.remove(pos);
    for key in nextlink.clone().keys() {
        if nextlink[key].contains(pos) {
            let mut newlink = nextlink[key].clone();
            newlink.remove(pos);
            nextlink.insert(key,newlink);
        }
    }
    nextlink
}

struct State<'a>{
    link: HashMap<&'a str, HashSet<&'a str>>,
    pos: &'a str,
    path: Vec<&'a str>,
    time_avail: bool
}

#[cfg(test)]
mod test {
    use aoc2021::read_lines;

    use super::*;


    #[test]
    fn regression() {
        let test_input =
            "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc".split("\n").map(|x| x.to_string()).collect();
        assert_eq!(pt1(&test_input), 19);
        assert_eq!(pt2(&test_input), 103);

        let large_test_input : Vec<String> =
        "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW".split("\n").map(|x| x.to_string()).collect();
        assert_eq!(pt1(&large_test_input), 226);
        assert_eq!(pt2(&large_test_input), 3509);


        let input = read_lines(12);
        assert_eq!(pt1(&input), 3779);
        assert_eq!(pt2(&input), 96988);
    }
}
