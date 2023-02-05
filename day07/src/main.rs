use std::fs;
use std::cmp;
use std::collections::HashMap;

fn moves(target: u32, counters: &HashMap<u32, u32>) -> u32 {
    counters
        .iter()
        .map(|(k, v)| v * (if target > *k { target - *k } else { *k - target }))
        .sum()
}

fn fuel(target: u32, counters: &HashMap<u32, u32>) -> u32 {
    counters
        .iter()
        .map(|(k, v)| {
            let n = if target > *k { target - *k } else { *k - target };
            let f = n * (n + 1) / 2;
            v * f})
        .sum()
}

fn main() {

    // Read input
    let input = fs::read_to_string("./data/input.txt")
        .expect("Cannot read input file.");
    let crabs: Vec<u32> = input.trim().split(",").map(|s| s.trim().parse::<u32>().unwrap()).collect();

    let mut counters: HashMap<u32, u32> = HashMap::new();
    let mut mini = u32::MAX;
    let mut maxi = 0;
    for crab in crabs {
        let counter = counters.entry(crab).or_insert(0);
        *counter += 1;
        mini = cmp::min(mini, crab);
        maxi = cmp::max(maxi, crab);
    }

    //Part 1:
    let mut best_score = u32::MAX;
    let mut best_position = 0;
    for i in mini..=maxi {
        let score = moves(i, &counters);
        if best_score > score {
            best_score = score;
            best_position = i;
        }
    }
    println!("Part 1: Best score is {} at {}.", best_score, best_position);

    // Part 2:
    let mut best_score = u32::MAX;
    let mut best_position = 0;
    for i in mini..=maxi {
        let score = fuel(i, &counters);
        if best_score > score {
            best_score = score;
            best_position = i;
        }
    }
    println!("Part 2: Best score is {} at {}.", best_score, best_position);
}

