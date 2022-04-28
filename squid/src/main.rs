use std::{collections::HashMap, collections::VecDeque};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Board {
    // 2d boolean array of marked positions
    marks: Vec<Vec<bool>>,
    // track last turn to make it quicker to calculate win
    lastMark: (i32, i32),
    // map of numbers to positions
    index: HashMap<i32, (i32, i32)>,
    // map of positions to numbers
    reversedIndex: HashMap<(i32, i32), i32>,
}
impl Board {
    pub fn new(positions: &Vec<Vec<i32>>) -> Self {
        let marks = vec![vec![false; positions.first().unwrap().len()]; positions.len()];
        let mut index = HashMap::new();
        let mut reversedIndex = HashMap::new();
        for (x, numbers) in positions.iter().enumerate() {
            for (y, number) in numbers.iter().enumerate() {
                index.insert(*number, (x as i32, y as i32));
                reversedIndex.insert((x as i32, y as i32), *number);
            }
        }
        Board {
            marks: marks,
            index: index,
            lastMark: (-1, -1),
            reversedIndex: reversedIndex,
        }
    }

    // mark
    pub fn mark(&mut self, number: i32) {
        let (x, y) = match self.index.get(&number) {
            Some(position) => position,
            None => return,
        };
        self.marks[*x as usize][*y as usize] = true;
        self.lastMark = (*x, *y);
    }

    // won
    pub fn won(&self) -> bool {
        if self.lastMark == (-1, -1) {
            return false;
        }
        let (start_x, start_y) = self.lastMark;
        let bingo = self.marks.first().unwrap().len();

        // up
        let mut position = start_x;
        let mut bingo_count = 0;
        loop {
            if self.marks[position as usize][start_y as usize] {
                bingo_count += 1;
            }
            position += 1;
            if position >= self.marks.len() as i32 {
                break;
            }
        }
        if bingo_count == bingo {
            return true;
        }

        // down
        let mut position = start_x;
        bingo_count -= 1; // will count start position again
        loop {
            if self.marks[position as usize][start_y as usize] {
                bingo_count += 1;
            }
            position -= 1;
            if position < 0 {
                break;
            }
        }
        if bingo_count == bingo {
            return true;
        }

        // left
        let mut position = start_y;
        let mut bingo_count = 0;
        loop {
            if self.marks[start_x as usize][position as usize] {
                bingo_count += 1;
            }
            position += 1;
            if position >= self.marks.first().unwrap().len() as i32 {
                break;
            }
        }
        if bingo_count == bingo {
            return true;
        }

        // right
        let mut position = start_y;
        bingo_count -= 1; // will count start position again
        loop {
            if self.marks[start_x as usize][position as usize] {
                bingo_count += 1;
            }
            position -= 1;
            if position < 0 {
                break;
            }
        }
        if bingo_count == bingo {
            return true;
        }
        false
    }

    fn getSum(&self) -> i32 {
        let mut sum = 0;
        for (x, marks) in self.marks.iter().enumerate() {
            for (y, mark) in marks.iter().enumerate() {
                if !mark {
                    sum += self.reversedIndex.get(&(x as i32, y as i32)).unwrap();
                }
            }
        }
        return sum;
    }

    pub fn getScore(&self) -> i32 {
        return self.getSum() * self.reversedIndex.get(&self.lastMark).unwrap();
    }
}

struct Game {
    // list of numbers (turns)
    turns: VecDeque<i32>,

    // list of boards
    boards: Vec<Board>,
}
impl Game {
    pub fn findWinner(&mut self) -> Vec<Board> {
        loop {
            let turnComplete = Game::turn(&mut self.turns, &mut self.boards);
            if !turnComplete {
                panic!("game over")
            }
            let winner = Game::winner(self.boards.clone());
            match winner {
                Some(winner) => return winner,
                None => (),
            };
        }
    }

    pub fn findLastWinner(&mut self) -> Board {
        loop {
            let mut winners = self.findWinner();
            winners.iter().for_each(|w| {
                let idx = self.boards.iter().position(|b| *b == *w).unwrap();
                self.boards.remove(idx);
            });

            if self.turns.is_empty() || self.boards.is_empty() {
                dbg!(&winners);
                return winners.last().cloned().unwrap();
            }
        }
    }

    //        let mut lastWinner: Option<Board> = None;
    //        loop {
    //            let turnComplete = Game::turn(&mut self.turns, &mut self.boards);
    //            let winner = Game::winner(self.boards.clone());
    //            match winner {
    //                Some(winner) => {
    //                    let idx = self.boards.iter().position(|b| *b == winner).unwrap();
    //                    self.boards.remove(idx);
    //                    dbg!(&winner.marks);
    //                    lastWinner = Some(winner)
    //                }
    //                None => ()
    //            };
    //            if self.turns.is_empty() {
    //                return lastWinner
    //            }
    //        }

    // take turn
    pub fn turn(turns: &mut VecDeque<i32>, boards: &mut Vec<Board>) -> bool {
        let number = match turns.pop_front() {
            Some(number) => number,
            None => {
                //                println!("game over");
                return false;
            }
        };
        for board in boards.iter_mut() {
            board.mark(number);
        }
        true
    }

    // check for winner and return the board
    pub fn winner(boards: Vec<Board>) -> Option<Vec<Board>> {
        let boards: Vec<Board> = boards.iter().cloned().filter(|b| b.won()).collect();
        return if boards.is_empty() {
            None
        } else {
            Some(boards.clone())
        };
    }
}

fn parse_turns(filename: String) -> VecDeque<i32> {
    std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .into_iter()
        .take(1)
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<VecDeque<_>>()
}

fn parse_boards(filename: String) -> Vec<Board> {
    let lines: Vec<String> = std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(|x| x.parse::<String>().expect("invalid line"))
        .collect();
    let mut iter_lines = lines.into_iter().peekable();

    let mut boards = Vec::new();
    loop {
        let iter_rows = iter_lines.by_ref().take_while(|l| !l.is_empty());
        let positions = iter_rows.fold(Vec::new(), |mut cols, row| {
            cols.push(
                row.split(" ")
                    .into_iter()
                    // single digit numbers contain additional spaces
                    .filter(|s| !s.is_empty())
                    .map(|position| position.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
            cols
        });

        boards.push(Board::new(&positions));

        let remaining = iter_lines.peek();
        match remaining {
            Some(remaining) => (),
            None => break,
        };
    }
    boards
}

fn main() {
    let mut game = Game {
        turns: parse_turns(String::from("turns.txt")),
        boards: parse_boards(String::from("boards.txt")),
    };
    //    let winner = game.findWinner();
    let winner = game.findLastWinner();
    let score = winner.getScore();
    println!("final score: {}", score);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /*
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
     2  0 12  3  7
     */

    #[test]
    fn test_board() {
        let callOuts = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let positions = vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            //            vec![21,  9, 14, 16,  7],
            //            vec![6, 10,  3, 18, 5],
            //            vec![1, 12, 20, 15, 19]
        ];
        let b = Board::new(&positions);
        assert_eq!(
            b.index,
            HashMap::from([
                (22, (0, 0)),
                (13, (0, 1)),
                (17, (0, 2)),
                (11, (0, 3)),
                (0, (0, 4)),
                (8, (1, 0)),
                (2, (1, 1)),
                (23, (1, 2)),
                (4, (1, 3)),
                (24, (1, 4)),
            ])
        )
    }

    #[test]
    fn test_mark() {
        let callOuts = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let positions = vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            //            vec![21,  9, 14, 16,  7],
            //            vec![6, 10,  3, 18, 5],
            //            vec![1, 12, 20, 15, 19]
        ];
        let mut b = Board::new(&positions);
        b.mark(13);
        assert_eq!(
            b.marks,
            vec![
                vec![false, true, false, false, false],
                vec![false, false, false, false, false],
            ]
        )
    }

    #[test]
    fn test_won() {
        let callOuts = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let positions = vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ];
        let mut b = Board::new(&positions);
        b.mark(13);
        assert_eq!(b.won(), false);
        b.mark(22);
        b.mark(13);
        b.mark(17);
        b.mark(11);
        b.mark(0);
        assert_eq!(b.won(), true);

        let mut b = Board::new(&positions);
        b.mark(22);
        b.mark(2);
        b.mark(14);
        b.mark(18);
        b.mark(19);
        assert_eq!(b.won(), false);

        let mut b = Board::new(&positions);
        b.mark(22);
        b.mark(23);
        b.mark(14);
        b.mark(3);
        b.mark(20);
        assert_eq!(b.won(), false);

        let mut b = Board::new(&positions);
        b.mark(22);
        b.mark(23);
        b.mark(14);
        b.mark(3);
        b.mark(20);
        b.mark(17);
        assert_eq!(b.won(), true);

        let mut b = Board::new(&positions);
        b.mark(14);
        b.mark(21);
        b.mark(9);
        b.mark(16);
        b.mark(17);
        b.mark(7);
        assert_eq!(b.won(), true);
    }

    #[test]
    fn test_game() {
        let turns = VecDeque::from_iter([
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ]);

        let mut g = Game {
            turns: turns,
            boards: vec![
                Board::new(&vec![
                    vec![22, 13, 17, 11, 0],
                    vec![8, 2, 23, 4, 24],
                    vec![21, 9, 14, 16, 7],
                    vec![6, 10, 3, 18, 5],
                    vec![1, 12, 20, 15, 19],
                ]),
                Board::new(&vec![
                    vec![3, 15, 0, 2, 22],
                    vec![9, 18, 13, 17, 5],
                    vec![19, 8, 7, 25, 23],
                    vec![20, 11, 10, 24, 4],
                    vec![14, 21, 16, 12, 6],
                ]),
                Board::new(&vec![
                    vec![14, 21, 17, 24, 4],
                    vec![10, 16, 15, 9, 19],
                    vec![18, 8, 23, 26, 20],
                    vec![22, 11, 13, 6, 5],
                    vec![2, 0, 12, 3, 7],
                ]),
            ],
        };

        let winner = g.findWinner();
        assert_eq!(winner.getScore(), 4512);
    }

    #[test]
    fn test_parse_boards() {
        let boards = parse_boards(String::from("boards_test.txt"));
        assert_eq!(boards.len(), 3);
        let expected_boards = vec![
            Board::new(&vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Board::new(&vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ]),
            Board::new(&vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ];
        assert_eq!(boards, expected_boards);
    }

    #[test]
    fn test_last_winner() {
        let turns = parse_turns(String::from("turns_test.txt"));
        let boards = parse_boards(String::from("boards_test.txt"));
        let mut g = Game { turns, boards };
        let winner = g.findLastWinner();
        assert_eq!(winner.getScore(), 1924);
    }
}
