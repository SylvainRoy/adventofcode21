use std::fs;


fn fill(map: &mut Vec<Vec<i32>>, i:usize, j:usize) -> usize {
    if map[i][j] == 9 {
        0
    } else {
        map[i][j] = 9;
        let mut res = 1;
        res += if map[i+1][j] != 9 { fill(map, i+1, j) } else { 0 };
        res += if map[i-1][j] != 9 { fill(map, i-1, j) } else { 0 };
        res += if map[i][j+1] != 9 { fill(map, i, j+1) } else { 0 };
        res += if map[i][j-1] != 9 { fill(map, i, j-1) } else { 0 };
        res
    }
}

fn main() {

    // Read input
    let input = fs::read_to_string("./data/input.txt")
        .expect("Cannot read input file.");

    let lines: Vec<&str> = input
        .lines()
        .collect();

    let cols = lines[0].trim().len();
    let rows = lines.len();

    let mut map = vec![vec![9_i32; cols+2]; rows+2];

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.trim().chars().enumerate() {
            map[i+1][j+1] = char.to_digit(10).unwrap() as i32;
        }
    }

    // Part 1
    let mut lows: Vec<(usize, usize)> = Vec::new();
    let mut risk = 0;
    for i in 1..rows+1 {
        for j in 1..cols+1 {
            if map[i][j] < map[i-1][j] &&
               map[i][j] < map[i+1][j] &&               
               map[i][j] < map[i][j-1] &&
               map[i][j] < map[i][j+1] {
                   lows.push((i, j));
                   risk += map[i][j] + 1;
            }
        }
    }
    println!("Part 1 - sum of risk: {}", risk);

    // Part 2
    let mut bassins: Vec<usize> = Vec::new();
    for (i, j) in lows {
        let size = fill(&mut map, i, j);
        bassins.push(size);
    }
    bassins.sort();
    let res = bassins[bassins.len()-3..].iter().fold(1, |acc, v| acc * v);
    println!("Part 2 - mult of 3 biggest region's area: {:?}", res);

}
