use std::fs;

fn main() {

    let input = fs::read_to_string("./data/input.txt")
        .expect("Error when reading input file.");

    let mut depth = 0;
    let mut horizontal = 0;

    for line in input.lines() {
        let words: Vec<String> = line
            .split_ascii_whitespace()
            .map(|w| String::from(w))
            .collect();
        let direction = &words[0];
        let distance:usize = words[1].parse().unwrap();

        if direction == "forward" {
            horizontal += distance;
        } else if direction == "down" {
            depth += distance;
        } else if direction == "up" {
            depth -= distance;
        }
    }

    println!("Part 1: {} * {} = {}", depth, horizontal, depth * horizontal);

    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in input.lines() {
        let words: Vec<String> = line
            .split_ascii_whitespace()
            .map(|w| String::from(w))
            .collect();
        let direction = &words[0];
        let units:usize = words[1].parse().unwrap();

        if direction == "forward" {
            horizontal += units;
            depth += aim * units;
        } else if direction == "down" {
            aim += units;
        } else if direction == "up" {
            aim -= units;
        }
    }

    println!("Part 2: {} * {} = {}", depth, horizontal, depth * horizontal);

}
