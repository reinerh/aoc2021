use std::collections::HashSet;

fn main() {
    let input = advent::read_all_lines(4);
    println!("4a: {}", find_winning_score(&input));
    println!("4b: {}", find_losing_score(&input));
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Bingo {
    board: Vec<Vec<Option<u32>>>,
    original_board: Vec<Vec<Option<u32>>>,
}

impl Bingo {
    const BOARD_SIZE: usize = 5;

    fn new<T: AsRef<str>>(input: &[T]) -> Bingo {
        let mut board = Vec::new();

        for line in input {
            let line : Vec<Option<u32>> = line.as_ref().split(' ')
                                              .filter(|n| !n.is_empty())
                                              .map(|n| n.parse::<u32>().ok())
                                              .collect();
            board.push(line)
        }

        Bingo { board: board.clone(), original_board: board }
    }

    fn has_bingo(&self) -> bool {
        for line in &self.board {
            let hits = line.iter()
                           .filter(|n| n.is_none())
                           .count();
            if hits == Bingo::BOARD_SIZE {
                return true;
            }
        }
        for i in 0 .. Bingo::BOARD_SIZE {
            let hits = self.board.iter()
                                 .filter(|line| line[i].is_none())
                                 .count();
            if hits == Bingo::BOARD_SIZE {
                return true;
            }
        }
        false
    }

    fn draw_number(&mut self, drawn: u32) {
        for line in 0 .. Bingo::BOARD_SIZE {
            for column in 0 .. Bingo::BOARD_SIZE {
                if self.board[line][column] == Some(drawn) {
                    self.board[line][column] = None;
                }
            }
        }
    }

    fn score(&self) -> u32 {
        self.board.iter()
                  .flatten()
                  .flatten()
                  .sum()
    }
}

fn parse_boards<T: AsRef<str>>(input: &[T]) -> Vec<Bingo> {
    let mut boards = Vec::new();
    for i in 0 .. input.len() / (Bingo::BOARD_SIZE + 1) {
        let start = i * (Bingo::BOARD_SIZE + 1) + 1;
        let end = start + Bingo::BOARD_SIZE;
        let board = Bingo::new(&input[start .. end]);
        boards.push(board);
    }
    boards
}

fn find_winning_score<T: AsRef<str>>(input: &[T]) -> u32 {
    find_score(input, false)
}

fn find_losing_score<T: AsRef<str>>(input: &[T]) -> u32 {
    find_score(input, true)
}

fn find_score<T: AsRef<str>>(input: &[T], wins_last: bool) -> u32 {
    let numbers : Vec<u32> = input[0].as_ref()
                                     .split(',')
                                     .map(|n| n.parse::<u32>().unwrap())
                                     .collect();
    let mut boards = parse_boards(&input[1..]);
    let board_count = boards.len();

    let mut solved_boards = HashSet::new();
    for number in numbers {
        for board in &mut boards {
            board.draw_number(number);
            if board.has_bingo() {
                if !wins_last {
                    return board.score() * number;
                }
                solved_boards.insert(board.original_board.clone());
                if solved_boards.len() == board_count {
                    return board.score() * number;
                }
            }
        }
    }
    panic!("No board with bingo found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ];
        assert_eq!(find_winning_score(&input), 4512);
        assert_eq!(find_losing_score(&input), 1924);
    }
}
