use aoc2021::read_lines;

fn main() {
    //let input = read_lines(3).iter().map(|x| u32::from_str_radix(x, 2).unwrap()).collect();
    //println!("pt1: {}", pt1(&input)); // 3959450
    //println!("pt2: {}", pt2(&input)); // 7440311
}

fn pt1(nums: Vec<u8>, boards_immut: Vec<u8>) -> u32 {
    let mut boards: Vec<(u8, bool)> = boards_immut.clone().iter()
        .map(|poss| (*poss, false))
        .collect();
    let (winning_num, winner_board) = pt1_find_winner_board(nums, &mut boards).unwrap();
    let unmarked_sum: u32 = winner_board.iter().filter(|(_,pred)|!*pred).map(|(num,_)|*num as u32).sum();
    winning_num as u32 * unmarked_sum
}

fn pt1_find_winner_board(nums: Vec<u8>, boards: &mut Vec<(u8, bool)>) -> Option<(u8, Vec<(u8, bool)>)> {
    let board_num_cnt = 5*5;
    for num in nums {
        for idx in (0..boards.len()).step_by(board_num_cnt) {
            let board = &mut boards[idx..(idx + board_num_cnt)];
            set_on_board(num, board);
            if is_bingo(board) {
                println!("BINDO");
                println!("{:?}", board);
                return Some((num, board.to_vec()));
            }
        }
    }
    None
}

fn is_bingo(board: &[(u8, bool)]) -> bool {
    // check rows

    let dimension = 5;

    let max_checked_rows= board.chunks(dimension)
        .map(|row| row.iter().filter(|(_, checked)| *checked).count())
        .max().unwrap();

    let mut col_cnts = vec![];
    for col_start in 0..dimension {
        // check columns
        let col_cnt = (0..(dimension*dimension)).step_by(dimension).into_iter().filter(|off| board[col_start + off].1).count();
        col_cnts.push(col_cnt)
    }
    let max_checked_cols = *col_cnts.iter().max().unwrap();

    //println!("rows : {}; cols: {}", max_checked_rows, max_checked_cols);
    //println!("rows chunks : {:?}; ", board.chunks(dimension).collect::<Vec<&[(u8, bool)]>>());
    max_checked_rows == 5 || max_checked_cols == 5
}

fn set_on_board(num: u8, board: &mut [(u8, bool)]) {
    for poss in board {
        if poss.0 == num {
            poss.1 = true;
            break;
        }
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<u8>, Vec<u8>) {
    let nums : Vec<u8>= input[0].split(",").map(|s| s.parse().unwrap()).collect();

    let mut boards: Vec<u8> = vec![];
    for line in &input[2..] {
        let mut row = line.split(" ")
            .filter_map(|n| n.parse::<u8>().ok())
            .collect();
        boards.append( &mut row);
    }
    (nums, boards)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let test_input =
"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7".split("\n").map(|x| x.to_string()).collect();

        let (test_nums, test_boards) = parse_input(&test_input);
        assert_eq!(pt1(test_nums, test_boards), 4512);

        let (nums, boards) = parse_input(&read_lines(4));
        assert_eq!(pt1(nums, boards), 25023);
    }
}
