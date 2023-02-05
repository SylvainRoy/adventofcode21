use std::fs;

#[derive(Copy, Clone)]
struct Cell {
    value: u32,
    token: bool,
}

impl Cell {
    fn to_string(&self) -> String {
        let mut out = String::from("");
        out += &format!("{:2}", self.value);
        if self.token {
            out += ". ";
        } else {
            out += "  ";
        }
        out
    }
}

struct Board {
    array: [[Cell; 5]; 5],
    complete: bool,
}

impl Board {
    fn from_iter<'a, I>(iter: &mut I) -> Option<Board>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut b = Board {
            array: [[Cell {
                value: 0,
                token: false,
            }; 5]; 5],
            complete: false,
        };
        for i in 0..5 {
            for j in 0..5 {
                let item = iter.next();
                match item {
                    Some(val) => {
                        b.array[i][j] = Cell {
                            value: val.to_string().parse().unwrap(),
                            token: false,
                        }
                    }
                    None => return None,
                }
            }
        }
        Some(b)
    }

    fn to_string(&self) -> String {
        let mut out = String::from("");
        for i in 0..5 {
            for j in 0..5 {
                out += &self.array[i][j].to_string();
            }
            out += "\n";
        }
        out
    }

    fn add_token(&mut self, token: u32) -> Option<u32> {
        if self.complete {
            return None;
        }
        for i in 0..5 {
            for j in 0..5 {
                let cell = &mut self.array[i][j];
                if (cell.value == token) && !cell.token {
                    cell.token = true;
                    if let Some(_) = self.check_row(i).or(self.check_col(j)) {
                        self.complete = true;
                        return Some(token * self.sum_unmarked());
                    }
                }
            }
        }
        None
    }

    fn check_row(&self, i: usize) -> Option<u32> {
        let mut score = 0;
        for j in 0..5 {
            if !self.array[i][j].token {
                return None;
            }
            score += self.array[i][j].value;
        }
        Some(score)
    }

    fn check_col(&self, j: usize) -> Option<u32> {
        let mut score = 0;
        for i in 0..5 {
            if !self.array[i][j].token {
                return None;
            }
            score += self.array[i][j].value;
        }
        Some(score)
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.array[i][j].token {
                    sum += self.array[i][j].value;
                }
            }
        }
        sum
    }
}

struct Data {
    tokens: Vec<u32>,
    boards: Vec<Board>,
}

impl Data {
    fn from_str(input: &String) -> Data {
        // Decode tokens on first line.
        let input_iter = &mut input.split_ascii_whitespace();
        let tokens: Vec<u32> = input_iter
            .next()
            .unwrap()
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect();
        // Decode boards from rest of the file.
        let mut boards: Vec<Board> = vec![];
        loop {
            match Board::from_iter(input_iter) {
                Some(board) => boards.push(board),
                None => break,
            }
        }
        Data {
            tokens: tokens,
            boards: boards,
        }
    }

    fn _to_string(&self) -> String {
        let mut out = String::from("");
        out += "Tokens: ";
        self.tokens.iter().for_each(|t| out += &format!("{} ", t));
        out += &format!("\n\nBoards ({}):\n", self.boards.len());
        for board in &self.boards {
            out += &format!("{}\n", board.to_string());
        }
        out
    }
}

fn main() {
    // read inputs
    let input = fs::read_to_string("./data/input.txt").expect("Cannot read input file.");

    // Part 1
    let mut data = Data::from_str(&input);
    'outer1: for token in data.tokens {
        for board in &mut data.boards {
            match board.add_token(token) {
                Some(score) => {
                    println!("Part 1 - board:\n{}score: {}\n", board.to_string(), score);
                    break 'outer1;
                }
                None => (),
            };
        }
    }

    // Part 2
    let mut data = Data::from_str(&input);
    'outer2: for token in data.tokens {
        let last_board = data.boards.iter().filter(|b| !b.complete).count() == 1;
        for board in &mut data.boards {
            match board.add_token(token) {
                Some(score) => {
                    if last_board {
                        println!(
                            "Part2 - last board:\n{}score: {}\n",
                            board.to_string(),
                            score
                        );
                        break 'outer2;
                    }
                }
                None => (),
            }
        }
    }
}
