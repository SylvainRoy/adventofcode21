use std::fs;
use regex::Regex;

#[derive(Debug, Clone)]
struct Cuboid {
    state: bool,
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    zmin: isize,
    zmax: isize,
}

impl Cuboid {
    fn new(state: bool, xmin: isize, xmax: isize, ymin: isize, ymax: isize, zmin: isize, zmax: isize) -> Cuboid {
        Cuboid {
            state,
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
        }
    }

    fn _to_string(&self) -> String {
        format!("Cube[{}, {}:{} {}:{} {}:{}]", self.state, self.xmin, self.xmax, self.ymin, self.ymax, self.zmin, self.zmax)
    }

    fn overlap(&self, cube: &Cuboid) -> bool {
        !(self.xmax < cube.xmin || cube.xmax < self.xmin ||
          self.ymax < cube.ymin || cube.ymax < self.ymin ||
          self.zmax < cube.zmin || cube.zmax < self.zmin)
    }

    fn includes(&self, cube: &Cuboid) -> bool {
        self.xmin <= cube.xmin && cube.xmax <= self.xmax && self.ymin <= cube.ymin && cube.ymax <= self.ymax && self.zmin <= cube.zmin && cube.zmax <= self.zmax 
    }

    fn isvalid(&self) -> bool {
        self.xmin <= self.xmax && self.ymin <= self.ymax && self.zmin <= self.zmax
    }

    fn sum_on(&self) -> isize {
        if self.state {
            (1 + self.xmax - self.xmin) * (1 + self.ymax - self.ymin) * (1 + self.zmax - self.zmin)
        } else {
            0
        }
    }
}

fn splitx(cube: Cuboid, splitter: &Cuboid) -> Vec<Cuboid> {
    if !cube.overlap(splitter) {
        return vec![cube];
    }
    let mut xs = vec![cube.xmin - 1];
    if cube.xmin <= splitter.xmin && splitter.xmin <= cube.xmax {
        xs.push(splitter.xmin - 1);
    }
    if cube.xmin <= splitter.xmax && splitter.xmax <= cube.xmax {
        xs.push(splitter.xmax);
    }
    xs.push(cube.xmax);
    let mut out = vec![];
    for i in 0..xs.len()-1 {
        let c = Cuboid::new(cube.state, xs[i]+1, xs[i+1], cube.ymin, cube.ymax, cube.zmin, cube.zmax);
        if c.isvalid() {
            out.append(&mut splity(c, splitter));
        }
    }
    out
}

fn splity(cube: Cuboid, splitter: &Cuboid) -> Vec<Cuboid> {
    if !cube.overlap(splitter) {
        return vec![cube];
    }
    let mut ys = vec![cube.ymin - 1];
    if cube.ymin <= splitter.ymin && splitter.ymin <= cube.ymax {
        ys.push(splitter.ymin - 1);
    }
    if cube.ymin <= splitter.ymax && splitter.ymax <= cube.ymax {
        ys.push(splitter.ymax);
    }
    ys.push(cube.ymax);
    let mut out = vec![];
    for i in 0..ys.len()-1 {
        let c = Cuboid::new(cube.state, cube.xmin, cube.xmax, ys[i]+1, ys[i+1], cube.zmin, cube.zmax);
        if c.isvalid() {
            out.append(&mut splitz(c, splitter));
        }
    }
    out
}

fn splitz(cube: Cuboid, splitter: &Cuboid) -> Vec<Cuboid> {
    if !cube.overlap(splitter) {
        return vec![cube];
    }
    let mut zs = vec![cube.zmin - 1];
    if cube.zmin <= splitter.zmin && splitter.zmin <= cube.zmax {
        zs.push(splitter.zmin - 1);
    }
    if cube.zmin <= splitter.zmax && splitter.zmax <= cube.zmax {
        zs.push(splitter.zmax);
    }
    zs.push(cube.zmax);
    let mut out = vec![];
    for i in 0..zs.len()-1 {
        let c = Cuboid::new(cube.state, cube.xmin, cube.xmax, cube.ymin, cube.ymax, zs[i]+1, zs[i+1]);
        if c.isvalid() && !c.overlap(splitter) {
            out.push(c);
        }        
    }
    out
}

fn main() {
    // Read input
    let input = fs::read_to_string("./data/input.txt").expect("Cannot read input.");
    let re = Regex::new(r"^(\w+) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)").unwrap();
    let cubes: Vec<Cuboid> = input
        .lines()
        .map(|line| {
            let mat = re.captures(line).unwrap();
            Cuboid::new(
                &mat[1] == "on",
                mat[2].parse::<isize>().unwrap(),
                mat[3].parse::<isize>().unwrap(),
                mat[4].parse::<isize>().unwrap(),
                mat[5].parse::<isize>().unwrap(),
                mat[6].parse::<isize>().unwrap(),
                mat[7].parse::<isize>().unwrap(),
            )
        })
        .collect();

    //
    // Part 1
    //
    let area = Cuboid::new(false, -50, 50, -50, 50, -50, 50);
    let mut nocubes: Vec<Cuboid> = Vec::new();
    for cube in cubes.clone() {
        if area.includes(&cube) {
            let mut newnocubes = Vec::new();
            for nocube in nocubes {
                newnocubes.append(&mut splitx(nocube, &cube));
            }
            if cube.state {
                newnocubes.push(cube);
            }
            nocubes = newnocubes;
        }
    }
    let on: isize = nocubes
        .iter()
        .map(|c| c.sum_on())
        .sum();
    println!("Part 1 - num of cube on: {:?}", on);

    //
    // Part 2
    //
    let mut nocubes: Vec<Cuboid> = Vec::new();
    for cube in cubes.clone() {
        let mut newnocubes = Vec::new();
        for nocube in nocubes {
            newnocubes.append(&mut splitx(nocube, &cube));
        }
        if cube.state {
            newnocubes.push(cube);
        }
        nocubes = newnocubes;
    }
    let on: isize = nocubes
        .iter()
        .map(|c| c.sum_on())
        .sum();
    println!("Part 2 - num of cube on: {:?}", on);
}
