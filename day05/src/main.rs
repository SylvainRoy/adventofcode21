use std::fs;
use std::cmp;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    a: (u32, u32),
    b: (u32, u32),
}

impl Line {
    fn _to_string(&self) -> String {
        format!("{:?}->{:?}", self.a, self.b)
    }

    fn is_vert_or_hor(&self) -> bool {
        (self.a.0 == self.b.0) || (self.a.1 == self.b.1)
    }

    fn points(&self) -> Vec<(u32, u32)> {
        let mut v:Vec<(u32, u32)> = vec![];
        if self.a.0 == self.b.0 {
            let mini = cmp::min(self.a.1, self.b.1);
            let maxi = cmp::max(self.a.1, self.b.1);
            for j in mini..=maxi {
                v.push((self.a.0, j));
            }
        } else if self.a.1 == self.b.1 {
            let mini = cmp::min(self.a.0, self.b.0);
            let maxi = cmp::max(self.a.0, self.b.0);
            for i in mini..=maxi {
                v.push((i, self.a.1));
            }
        } else {
            let a0: i32 = self.a.0 as i32;
            let a1: i32 = self.a.1 as i32;
            let b0: i32 = self.b.0 as i32;
            let b1: i32 = self.b.1 as i32;
            let (init, len) = if a0 < b0 {
                ((a0, a1), b0 - a0 + 1)
            } else {
                ((b0, b1), a0 - b0 + 1)
            };
            let sign = ((a1 - b1) * (a0 - b0)).signum();
            for k in 0..len {
                let i: u32 = (init.0 + k) as u32;
                let j: u32 = (init.1 + sign * k) as u32;
                v.push((i, j));
            }
        }
        v
    }
}

fn main() {

    // Read input
    let input = fs::read_to_string("./data/input.txt").expect("Cannot read input file.");

    // Part 1
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("Invalid regex");
    let lines: Vec<Line> = re
        .captures_iter(&input).map(|cap| Line {
            a: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            b: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),})
        .collect();

    let vert_or_hori_lines: Vec<&Line> = lines
        .iter()
        .filter(|line| line.is_vert_or_hor())
        .collect();
              
    let mut map = HashMap::new();

    for line in vert_or_hori_lines {
        for point in line.points() {
            let key = format!("{},{}", point.0, point.1);
            let val = map.entry(key).or_insert(0);
            *val += 1;
        }
    }

    let num_inter = map.iter()
        .filter(|(_, v)| **v > 1)
        .count();

    println!("Part 1 - Num inter = {}", num_inter);

    // Part 2
    let diagonals: Vec<&Line> = lines
        .iter()
        .filter(|line| !line.is_vert_or_hor())
        .collect();

    for line in diagonals {
        for point in line.points() {
            let key = format!("{},{}", point.0, point.1);
            let val = map.entry(key).or_insert(0);
            *val += 1;
        }
    }

    let num_inter = map.iter()
        .filter(|(_, v)| **v > 1)
        .count();

    println!("Part 2 - Num inter = {}", num_inter);
}
