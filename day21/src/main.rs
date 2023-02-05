use std::fs;
use std::cmp;

const COEFF: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

struct Dice {
    rolled: usize,
    value: usize,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            rolled: 0,
            value: 0,
        }
    }
    fn roll(&mut self) -> usize {
        self.rolled += 1;
        self.value = (self.value % 100) + 1;
        self.value
    }
}

#[derive(Clone)]
struct Player {
    id: usize,
    position: usize,
    score: usize,
}

impl Player {
    fn new(id:usize, position: usize) -> Self {
        Player {
            id,
            position,
            score: 0,
        }
    }
    fn play(&mut self, dice: &mut Dice) {
        // Roll the dices
        let mut rolls = 0;
        for _i in 0..3 {
            rolls += dice.roll();
        }
        // Move to new position
        self.position = (self.position - 1 + rolls) % 10 + 1;
        // Update score
        self.score += self.position;
    }
    fn play3(&self, rolls: usize) -> Player {
        let position = (self.position - 1 + rolls) % 10 + 1;
        Player {
            id: self.id,
            position,
            score: self.score + position,
        }
    }
    fn _to_string(&self) -> String {
        format!("Player[{} position:{} score:{}]", self.id, self.position, self.score)
    }
}

fn play(player: Player, other: Player) -> (usize, usize) {
    if other.score >= 21 {
        return (0, 1)
    }
    let mut win_player = 0;
    let mut win_other = 0;
    for i in 3..=9 {
        let wins = play(other.clone(), player.play3(i));
        win_other += wins.0 * COEFF[i];
        win_player += wins.1 * COEFF[i];
    }
    (win_player, win_other)
}

fn main() {

    // Read input
    let input = fs::read_to_string("./data/input.txt").expect("Can't read input file");
    let positions: Vec<usize> = input
        .lines()
        .map(|line| line.trim().split(" ").last().unwrap().parse::<usize>().unwrap())
        .collect();
    println!("Positions: {:?}", positions);

    //
    // Part 1
    //
    let mut dice = Dice::new();
    let mut players = Vec::new();
    for i in 0..2 {
        players.push(Player::new(i+1, positions[i]))
    }

    'outer: loop {
        for i in 0..players.len() {
            players[i].play(&mut dice);
            if players[i].score >= 1000 {
                break 'outer;
            }
        }
    }

    let losing_score = cmp::min(players[0].score, players[1].score);
    println!("Part 1 - {}", losing_score * dice.rolled);

    //
    // Part 2
    //
    let mut players = Vec::new();
    for i in 0..2 {
        players.push(Player::new(i+1, positions[i]))
    }
    let scores = play(
        Player::new(1, positions[0]), 
        Player::new(2, positions[1])
    );
    println!("Part 2 - winner wins {:?} times", cmp::max(scores.0, scores.1));
}
