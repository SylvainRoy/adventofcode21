use std::fs;

struct Board {
    cols: usize,
    rows: usize,
    board: Vec<Vec<usize>>,
}

impl Board {
    fn new(input: &str) -> Board {
        let rows = input.lines().count();
        let cols = input.lines().nth(0).unwrap().trim().len();
        let mut board = Board {
            cols: cols,
            rows: rows,
            board: vec![vec![10; cols + 2]; rows + 2],
        };
        for (i, line) in input.lines().enumerate() {
            for (j, cell) in line.chars().enumerate() {
                *board.get(i, j) = cell.to_digit(10).unwrap() as usize;
            }
        }
        board
    }

    fn get(&mut self, i: usize, j: usize) -> &mut usize {
        &mut self.board[i + 1][j + 1]
    }

    fn to_string(&mut self) -> String {
        let mut out = String::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                out += &format!("{}", self.get(i, j));
            }
            out += "\n";
        }
        out
    }

    fn next_cycle(&mut self) -> usize {
        // Incr all octopuses
        let mut flashes = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                flashes += self.incr(i, j);
            }
        }
        // Octopuses that flashed go to 0
        for i in 0..self.rows {
            for j in 0..self.cols {
                let octopus = self.get(i, j);
                if *octopus > 9 {
                    *octopus = 0;
                }
            }
        }
        flashes
    }

    fn incr(&mut self, i: usize, j: usize) -> usize {
        let mut flashes = 0;
        let cell = self.get(i, j);
        *cell += 1;
        if *cell == 10 {
            flashes += 1;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let ii = i as isize + di;
                    let jj = j as isize + dj;
                    if (di != 0 || dj != 0) && (0 <= ii) && (0 <= jj) {
                        flashes += self.incr(ii as usize, jj as usize);
                    }
                }
            }
        }
        flashes
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("./data/input.txt").expect("Can't read input.");

    // Part 1
    let mut board = Board::new(&input);
    let mut flashes = 0;
    println!("{}", board.to_string());
    for i in 1..=100 {
        flashes += board.next_cycle();
        println!("step {} (flashes={}):\n{}", i, flashes, board.to_string());
    }
    println!("Part 1 - Flashes: {}", flashes);

    // Part 2
    let mut index = 0;
    let mut board = Board::new(&input);
    loop {
        index += 1;
        let flashes = board.next_cycle();
        if flashes == 100 {
            break;
        };
    }
    println!("Part 2 - Flashes: {}", index);
}
