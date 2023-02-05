use itertools::Itertools;
use std::fs;

struct Sheet {
    xmax: usize,
    ymax: usize,
    dots: Vec<(usize, usize)>,
}

impl Sheet {
    fn new(dots: Vec<(usize, usize)>) -> Sheet {
        let xmax = dots.iter().map(|c| c.0).max().unwrap();
        let ymax = dots.iter().map(|c| c.1).max().unwrap();
        Sheet { xmax, ymax, dots }
    }

    fn num_dots(&mut self) -> usize {
        self.dots.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));
        self.dots.iter().unique().count()
    }

    fn to_string(&mut self) -> String {
        self.dots.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));
        let mut out = String::new();
        let mut dot_iter = self.dots.iter().unique();
        let mut next_dot = dot_iter.next();
        for y in 0..=self.ymax {
            for x in 0..=self.xmax {
                match next_dot {
                    Some((i, j)) if *i == x && *j == y => {
                        out += "#";
                        next_dot = dot_iter.next();
                    }
                    _ => out += ".",
                }
            }
            out += "\n";
        }
        out
    }

    fn fold(&mut self, dim: char, pos: usize) {
        if dim == 'x' {
            for i in 0..self.dots.len() {
                let (x, y) = self.dots[i];
                if x > pos {
                    self.dots[i] = (2 * pos - x, y);
                }
            }
            self.xmax = (self.xmax - 1) / 2;
        } else {
            for i in 0..self.dots.len() {
                let (x, y) = self.dots[i];
                if y > pos {
                    self.dots[i] = (x, 2 * pos - y);
                }
            }
            self.ymax = (self.ymax - 1) / 2;
        }
    }
}

fn main() {
    // read input
    let input = fs::read_to_string("./data/input.txt").expect("Cannot read input file.");
    let dots: Vec<(usize, usize)> = input
        .lines()
        .take_while(|l| l.len() > 1)
        .map(|l| {
            let mut el = l.trim().split(",");
            (
                el.next().unwrap().parse::<usize>().unwrap(),
                el.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();
    let instructions: Vec<(char, usize)> = input
        .lines()
        .skip_while(|l| l.len() == 0 || l.chars().nth(0).unwrap() != 'f')
        .map(|l| {
            let el = l.trim().split(" ").last().unwrap();
            (
                el.chars().nth(0).unwrap(),
                el.split("=").last().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    // Part 1:
    let mut sheet = Sheet::new(dots);
    sheet.fold(instructions[0].0, instructions[0].1);
    println!("Part 1 - Num of dots: {}", sheet.num_dots());

    // Part 2:
    for inst in instructions.iter().skip(1) {
        sheet.fold(inst.0, inst.1);
    }
    println!("Part 2 - Digits:\n{}", sheet.to_string());
}
