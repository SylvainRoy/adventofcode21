use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

fn main() {
    // Read input
    let input = fs::read_to_string("./data/input.txt").expect("Can't read input file");
    let mut input_iter = input.lines();

    // Decode input
    let chain = input_iter.next().unwrap().trim();
    input_iter.next();
    let subs: Vec<((char, char), char)> = input_iter
        .map(|l| {
            let mut c = l.chars();
            let left = c.next().unwrap();
            let right = c.next().unwrap();
            let middle = c.skip(4).next().unwrap();
            ((left, right), middle)
        })
        .collect();
    let insertions: HashMap<(char, char), char> = HashMap::from_iter(subs.iter().map(|x| *x));
    // println!("Chain[0]: {}", &chain);
    // println!("subs: {:?}", &subs);
    // println!("insertions: {:?}", &insertions);

    //
    // Part 1
    //
    let mut newchain: String = String::from(chain);

    for _i in 1..=10 {
        // Create intermediates
        let mut inter: String = newchain
            .chars()
            .zip(newchain.chars().skip(1))
            .map(|leftright| match insertions.get(&leftright) {
                Some(c) => *c,
                None => ' ',
            })
            .collect();
        inter.push('.');
        // println!("inserts: {:?}", &inter);

        newchain = newchain
            .chars()
            .zip(inter.chars())
            .fold(String::new(), |mut acc, (a, b)| {
                acc.push(a);
                acc.push(b);
                acc
            });
        newchain.pop();
        // println!("Chain[{}]: {}", _i, newchain);
    }
    // println!("Chain: {}", newchain);

    let mut counters = HashMap::new();
    for c in newchain.chars() {
        *counters.entry(c).or_insert(0) += 1;
    }
    let most = counters.iter().fold(
        ('?', 0),
        |acc, kv| if acc.1 < *kv.1 { (*kv.0, *kv.1) } else { acc },
    );
    // println!("most: {:?}", most);

    let least = counters.iter().fold(('?', usize::MAX), |acc, kv| {
        if acc.1 > *kv.1 {
            (*kv.0, *kv.1)
        } else {
            acc
        }
    });
    // println!("least: {:?}", least);

    println!("Part 1 -  most - least = {}", most.1 - least.1);

    //
    // Part 2
    //

    // println!("{}", chain);

    // Build a map of pairs -> counter
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    chain
        .chars()
        .zip(chain.chars().skip(1))
        .for_each(|leftright| {
            *pairs.entry(leftright).or_insert(0) += 1;
        });
    // println!("[0] {:?}", pairs);

    // Iterate!
    for _i in 1..=40 {
        let mut npairs: HashMap<(char, char), usize> = HashMap::new();
        for (pair, counter) in pairs.iter() {
            if let Some(insert) = insertions.get(&pair) {
                *npairs.entry((pair.0, *insert)).or_insert(0) += counter;
                *npairs.entry((*insert, pair.1)).or_insert(0) += counter;
            }
        }
        pairs = npairs;
        // println!("[{}] {:?}", _i, pairs);
    }
    // Count the chars
    let mut chars: HashMap<char, usize> = HashMap::new();
    chars.insert(chain.chars().next().unwrap(), 1);
    chars.insert(chain.chars().last().unwrap(), 1);
    for (pair, counter) in pairs.iter() {
        *chars.entry(pair.0).or_insert(0) += counter;
        *chars.entry(pair.1).or_insert(0) += counter;
    }

    let mut most = chars.iter().fold(
        ('?', 0),
        |acc, kv| if acc.1 < *kv.1 { (*kv.0, *kv.1) } else { acc },
    );
    most.1 /= 2;
    // println!("most: {:?}", most);

    let mut least = chars.iter().fold(('?', usize::MAX), |acc, kv| {
        if acc.1 > *kv.1 {
            (*kv.0, *kv.1)
        } else {
            acc
        }
    });
    least.1 /= 2;
    // println!("least: {:?}", least);

    println!("Part 2 -  most - least = {}", most.1 - least.1);
}
