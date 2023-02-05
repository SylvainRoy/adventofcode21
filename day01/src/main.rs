use std::fs;

fn main() {

    let input = fs::read_to_string("./data/input.txt")
        .expect("Something went wrong reading the file");

    let depths: Vec<usize> = input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let counter = depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(a , b)| a < b)
        .count();

    println!("Counter: {:?}", counter);

    let depths3: Vec<usize> = depths
        .windows(3)
        .map(|win| win.iter().sum::<usize>())
        .collect();

    let counter3 = depths3
        .iter()
        .zip(depths3.iter().skip(1))
        .filter(|(a, b)| a < b).
        count();

    println!("Counter3: {:?}", counter3);

}
