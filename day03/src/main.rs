use std::convert::TryInto;
use std::fs;

fn main() {

    let input = fs::read_to_string("./data/input.txt").expect("Error reading input file");

    let len_line = input.lines().next().unwrap().len();
    let len_data: u32 = input.lines().count().try_into().unwrap();

    let counters = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .fold(vec![0; len_line], |acc, v| {
            acc.iter()
                .zip(v.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<u32>>()
        });

    let gamma = counters
        .iter()
        .map(|v| if v > &(len_data / 2) { 1 } else { 0 })
        .enumerate()
        .map(|(i, v): (usize, usize)| v * 2_usize.pow((len_line - 1 - i).try_into().unwrap()))
        .reduce(|acc, v| acc + v)
        .unwrap();

    let epsilon = 2_usize.pow(len_line.try_into().unwrap()) - 1 - gamma;

    println!(
        "Part 1: {:?} / {} / gamma: {} / epsilon: {} / gamma * epsilon: {}",
        counters,
        len_data,
        gamma,
        epsilon,
        gamma * epsilon
    );

    let inputs: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let candidates: Vec<&Vec<u32>> = inputs.iter().collect();
    let oxygen = filter_until_one(candidates, |v, count, len| {v == (2 * count) / len});

    let candidates: Vec<&Vec<u32>> = inputs.iter().collect();
    let co2 = filter_until_one(candidates, |v, count, len| {v == 1 - (2 * count) / len});

    println!("Part 2: oxygen: {}, co2: {}, oxygen*co2: {}", oxygen, co2, oxygen*co2);
}


///
/// Applies filtering on the set of candidates until only one is left.
/// 
fn filter_until_one<F>(mut candidates: Vec<&Vec<u32>>, select: F) -> u32 where
    F: Fn(u32, u32, u32) -> bool {

    let len_line = candidates[0].len();
    // Progressively reduce the data set until only one entry is left
    for i in 0..len_line {
        if candidates.len() == 1 {
            break;
        }
        let counters = candidates
            .iter()
            .fold(vec![0; len_line], |acc, v| {
                acc.iter()
                    .zip(v.iter())
                    .map(|(a, b)| a + b)
                    .collect::<Vec<u32>>()
            });
        candidates = candidates
            .iter()
            .filter(|v| select(v[i], counters[i], candidates.len().try_into().unwrap()))
            .map(|v| *v)
            .collect();
        // println!("candidates: {:?}", candidates);
    }
    // Convert to decimal
    candidates[0]
        .iter()
        .fold(
            0_u32,
            |acc, v| acc * 2 + v
        )
}