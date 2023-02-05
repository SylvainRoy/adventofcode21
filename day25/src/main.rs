use std::fs;

#[derive(Clone, Copy)]
enum Slot {
    V,
    H,
    Empty,
}

impl Slot {
    fn to_string(&self) -> String {
        match self {
            Self::V => String::from("v"),
            Self::H => String::from(">"),
            Self::Empty => String::from("."),
        }
    }
}

struct SeaFloor {
    xmax: usize,
    ymax: usize,
    board: Vec<Vec<Slot>>,
}

impl SeaFloor {
    fn new(input: &str) -> Self {
        let ymax = input.trim().split("\n").count();
        let xmax = input.split("\n").nth(0).unwrap().len();
        let mut seafloor = SeaFloor {
            xmax,
            ymax,
            board: vec![vec![Slot::Empty; ymax]; xmax],
        };
        input
            .trim()
            .split("\n")
            .enumerate()
            .for_each(|(y, line)|
                line
                    .chars()
                    .enumerate()
                    .for_each(|(x, car)|
                        seafloor.board[x][y] = match car {
                            'v' => Slot::V,
                            '>' => Slot::H,
                            '.' => Slot::Empty,
                            _ => panic!("Unknown slot type"),
                        }
                    )
                );
        seafloor
    }

    fn next(&self, slot: &Slot, x: usize, y: usize) -> Option<(usize, usize)> {
        let (xx, yy) = match slot {
            Slot::H => ((x + 1) % self.xmax, y),
            Slot::V => (x, (y + 1) % self.ymax),
            Slot::Empty => panic!("Only V and > can move!"),
        };
        match self.board[xx][yy] {
            Slot::Empty  => Some((xx, yy)),
            _ => None,
        }
    }

    fn round(&mut self) -> usize {
        // Let's first consider east facing cucumbers
        let mut hmoves = Vec::new();
        for i in 0..self.xmax {
            for j in 0..self.ymax {
                if let Slot::H = self.board[i][j] {
                    match self.next(&self.board[i][j], i, j) {
                        Some((x, y)) => hmoves.push((i, j, x, y)),
                        None => (),
                    }
                }
            }
        };
        for (i, j, x, y) in &hmoves {
            self.board[*x][*y] = self.board[*i][*j];
            self.board[*i][*j] = Slot::Empty;
        };
        // Then, let's consider cucumbers facing south
        let mut vmoves = Vec::new();
        for i in 0..self.xmax {
            for j in 0..self.ymax {
                if let Slot::V = self.board[i][j] {
                    match self.next(&self.board[i][j], i, j) {
                        Some((x, y)) => vmoves.push((i, j, x, y)),
                        None => (),
                    }
                }
            }
        };
        for (i, j, x, y) in &vmoves {
            self.board[*x][*y] = self.board[*i][*j];
            self.board[*i][*j] = Slot::Empty;
        };
        hmoves.len() + vmoves.len()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        for j in 0..self.ymax {
            for i in 0..self.xmax {
                out += &self.board[i][j].to_string();
            }
            out += "\n";
        }
        out
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("./data/input.txt").expect("Can't read input");
    let mut seafloor = SeaFloor::new(&input);
    // println!("Initial seafloor:\n{}", seafloor.to_string());

    let mut i = 0;
    while seafloor.round() != 0 {
        i += 1;
        // println!("Round 1 ({} moves)\n{}", mv, seafloor.to_string());
    }
    // println!("Part 1 - {} rounds\n{}", i+1, seafloor.to_string());
    println!("Part 1 - {} rounds\n", i+1);
    
}
