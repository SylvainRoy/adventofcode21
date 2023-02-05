use std::fs;

fn main() {

    // Read input
    let input = fs::read_to_string("./data/input.txt").expect("Cannot read input file.");

    // Part 1:
    let mut fishes = vec![];
    for num in input.split(",") {
        fishes.push(num.trim().parse().unwrap());
    }
    // println!("Initial state:  {:?}", fishes);
    for _d in 1..=80 {
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                fishes[i] = 6;
                fishes.push(8);
            } else {
                fishes[i] -= 1;
            }
        }
        // println!("After {} day(s): {:?}", _d + 1, fishes);
    }
    println!("Part 1: {} fishes", fishes.len());

    // Part 2:
    let mut fishes = [0; 9];
    input.split(",").map(|s| s.trim().parse().unwrap()).for_each(|n:usize| fishes[n] += 1);
    for _d in 1..=256 {
        let at0 = fishes[0];
        for i in 0..8 {
            fishes[i] = fishes[i+1]
        }
        fishes[6] += at0;
        fishes[8] = at0;
        //println!("[{}]    state:   {:?} [{}]", _d, fishes, fishes.iter().sum::<usize>());
    }
    let num_fishes: usize = fishes.iter().sum();
    println!("Part 2: {} fishes", num_fishes);

}
