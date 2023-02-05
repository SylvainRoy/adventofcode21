use std::fs;
use std::cmp;

#[derive(Debug)]
struct Move {
    amphipod: char,
    origin: (isize, isize),
    destination: (isize, isize),
    cost: isize
}

impl Move {
    fn new(amphipod: char, origin: (isize, isize), destination: (isize, isize)) -> Self {
        let coef = if amphipod == 'A' { 1 } else if amphipod == 'B' { 10 } else if amphipod == 'C' { 100 } else { 1000 };
        let cost = coef * ((origin.0 - destination.0).abs() + (origin.1 - destination.1).abs());
        Move {
            amphipod,
            origin,
            destination,
            cost
        }
    }
}

struct Board {
    arr: [[char; 11]; 3],
}

impl Board {
    fn new(str: &str) -> Board {
        let mut board = Board {
            arr: [[' '; 11]; 3]
        };
        for j in 0..11 {
            board.arr[0][j] = str.chars().nth(15+j).unwrap();
        }
        for j in (2..9).step_by(2) {
            board.arr[1][j] = str.chars().nth(29+j).unwrap();            
        }
        for j in (2..9).step_by(2) {
            board.arr[2][j] = str.chars().nth(43+j).unwrap();            
        }
        board
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        for i in 0..3 {
            for j in 0..11 {
                out.push(self.arr[i][j]);
            }
            out += " \n";
        }
        out
    }

    fn moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        // Moves from the hallway to a cell
        for j in 0..11 {
            if self.arr[0][j] == 'A' {
                if self.path((0, j as isize), (2, 2)) {
                    moves.push(Move::new(self.arr[0][j], (0, j as isize), (2, 2)));
                } else if self.path((0, j as isize), (1, 2)) && self.arr[2][2] == 'A' {
                    moves.push(Move::new(self.arr[0][j], (0, j as isize), (1, 2)));
                }
            } else if self.arr[0][j] == 'B' {
                if self.path((0, j as isize), (2, 4)) {
                    moves.push(Move::new(self.arr[0][j], (0, j as isize), (2, 4)));
                } else if self.path((0, j as isize), (1, 4)) && self.arr[2][4] == 'B' {
                    moves.push(Move::new(self.arr[0][j], (0, j as isize), (1, 4)));
                }
            } else if self.arr[0][j] == 'C' {
                if self.path((0, j as isize), (2, 6)) {
                    moves.push(Move::new(self.arr[0][j], (0, j as isize), (2, 6)));
                } else if self.path((0, j as isize), (1, 6)) && self.arr[2][6] == 'C' {
                    moves.push(Move::new(self.arr[0][j], (0, j as isize), (1, 6)));
                }
            } else if self.arr[0][j] == 'D' {
                if self.path((0, j as isize), (2, 8)) {
                    moves.push(Move::new(self.arr[0][j], (0, j as isize), (2, 8)));
                } else if self.path((0, j as isize), (1, 8)) && self.arr[2][8] == 'D' {
                    moves.push(Move::new(self.arr[0][j], (0, j as isize), (1, 8)));
                }
            } else if self.arr[0][j] == '.' {

            } else {
                panic!("What's this letter?!");
            }
        }
        // Moves from a cell to the hallway
        for x in [0, 1, 3, 5, 7, 9, 10] {
            if self.arr[2][2] != '.' && self.arr[2][2] != 'A' && self.path((2, 2), (0, x)) {
                moves.push(Move::new(self.arr[2][2], (2, 2), (0, x)));
            }
            if self.arr[1][2] != '.' && (self.arr[1][2] != 'A' || self.arr[2][2] != 'A') && self.path((1, 2), (0, x)) {
                moves.push(Move::new(self.arr[1][2], (1, 2), (0, x)));
            }

            if self.arr[2][4] != '.' && self.arr[2][4] != 'B' && self.path((2, 4), (0, x)) {
                moves.push(Move::new(self.arr[2][4], (2, 4), (0, x)));
            }
            if self.arr[1][4] != '.' && (self.arr[1][4] != 'B' || self.arr[2][4] != 'B') && self.path((1, 4), (0, x)) {
                moves.push(Move::new(self.arr[1][4], (1, 4), (0, x)));
            }

            if self.arr[2][6] != '.' && self.arr[2][6] != 'C' && self.path((2, 6), (0, x)) {
                moves.push(Move::new(self.arr[2][6], (2, 6), (0, x)));
            }
            if self.arr[1][6] != '.' && (self.arr[1][6] != 'C' || self.arr[2][6] != 'C') && self.path((1, 6), (0, x)) {
                moves.push(Move::new(self.arr[1][6], (1, 6), (0, x)));
            }

            if self.arr[2][8] != '.' && self.arr[2][8] != 'D' && self.path((2, 8), (0, x)) {
                moves.push(Move::new(self.arr[2][8], (2, 8), (0, x)));
            }
            if self.arr[1][8] != '.' && (self.arr[1][8] != 'D' || self.arr[2][8] != 'D') && self.path((1, 8), (0, x)) {
                moves.push(Move::new(self.arr[1][8], (1, 8), (0, x)));
            }
        }
        // println!("MOVES: {:?}", moves.len());
        moves
    }

    fn path(&self, origin: (isize, isize), destination: (isize, isize)) -> bool {
        // move in hallway
        for j in cmp::min(origin.1, destination.1)..=cmp::max(origin.1, destination.1) {
            if origin == (0, j) {
                continue
            }
            if self.arr[0][j as usize] != '.' {
                return false;
            }
        }
        // move in cell
        if origin.0 == 0 { // origin is in hallway: moving down in cell once above destination
            assert_ne!(destination.0, 0);
            for i in 1..=destination.0 {
                if self.arr[i as usize][destination.1 as usize] != '.' {
                    return false;
                }
            }
        } else { // origin is in cell: moving up out of cell at origin
            assert_eq!(destination.0, 0);
            for i in 1..=origin.0 {
                if origin.0 == i {
                    continue
                }
                if self.arr[i as usize][origin.1 as usize] != '.' {
                    return false;
                }
            }
        }
        true
    }

    fn do_move(&mut self, mov: &Move) {
        // println!("DO: {:?} =>\n{}", mov, self.to_string());
        assert_ne!(self.arr[mov.origin.0 as usize][mov.origin.1 as usize], '.');
        assert_eq!(self.arr[mov.destination.0 as usize][mov.destination.1 as usize], '.');
        self.arr[mov.destination.0 as usize][mov.destination.1 as usize] = self.arr[mov.origin.0 as usize][mov.origin.1 as usize];
        self.arr[mov.origin.0 as usize][mov.origin.1 as usize] = '.';
    }

    fn undo_move(&mut self, mov: &Move) {
        // println!("UNDO: {:?} =>\n{}", mov, self.to_string());
        assert_eq!(self.arr[mov.origin.0 as usize][mov.origin.1 as usize], '.');
        assert_ne!(self.arr[mov.destination.0 as usize][mov.destination.1 as usize], '.');
        self.arr[mov.origin.0 as usize][mov.origin.1 as usize] = self.arr[mov.destination.0 as usize][mov.destination.1 as usize];
        self.arr[mov.destination.0 as usize][mov.destination.1 as usize] = '.';
    }

    fn success(&self) -> bool {
        self.arr[1][2] == 'A' && self.arr[2][2] == 'A' && self.arr[1][4] == 'B' && self.arr[2][4] == 'B' && self.arr[1][6] == 'C' && self.arr[2][6] == 'C' && self.arr[1][8] == 'D' && self.arr[2][8] == 'D'
    }
}

fn search(board: &mut Board, cost: isize, found: &mut isize, depth: isize) -> isize {
    // println!("`SEARCH: depth:{} cost:{} found:{} ", depth, cost, *found);
    if board.success() {
        return cost;
    }
    if *found <= cost {
        return isize::MAX;
    } 
    let moves = board.moves();
    if moves.len() == 0 {
        return isize::MAX
    }
    let mut smallest = isize::MAX;
    for mov in moves {
        board.do_move(&mov);
        let res = search(board, cost + mov.cost, found, depth + 1);
        board.undo_move(&mov);
        if res < smallest {
            smallest = res;
        };
    }
    if smallest < *found {
        *found = smallest;
    }
    smallest
}

fn main() {

    // read input
    let input = fs::read_to_string("./data/input1.txt").expect("Can't read input");
    let mut board = Board::new(&input);
    println!("Initial board:\n{}", board.to_string());
    // println!("Moves: {:?}", &board.moves());

    let mut found = isize::MAX;
    let cheapest = search(&mut board, 0, &mut found, 1);
    println!("Part 1 - Energy: {:?}", cheapest);
}
