use std::fs;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
struct Image {
    map: HashMap<(isize, isize), bool>,
    imin: isize,
    imax: isize,
    jmin: isize,
    jmax: isize,

}

impl Image {
    fn new() -> Self {
        Image {
            map: HashMap::new(),
            imin: isize::MAX,
            imax: isize::MIN,
            jmin: isize::MAX,
            jmax: isize::MIN,
        }
    }

    fn update_min_max(&mut self, i: isize, j: isize) {
        self.imax = cmp::max(self.imax, i);
        self.imin = cmp::min(self.imin, i);
        self.jmax = cmp::max(self.jmax, j);
        self.jmin = cmp::min(self.jmin, j);
    }

    fn set_on(&mut self, i: isize, j: isize) {
        self.update_min_max(i, j);
        self.map.insert((i, j), true);
    }

    fn _to_string(&self) -> String {
        let mut out = String::new();
        for i in self.imin..=self.imax {
            for j in self.jmin..=self.jmax {
                if self.map.contains_key(&(i, j)) {
                    out += "#";
                } else {
                    out += ".";
                }
            }
            out += "\n";
        };
        out
    }

    fn next_image(&self, algorithm: &str, infinite: usize) -> Image {
        let mut image = Image::new();
        image.update_min_max(self.imin-1, self.jmin-1);
        image.update_min_max(self.imax+1, self.jmax+1);
        // For each future pixel
        for i in (self.imin-1)..=(self.imax+1) {
            for j in (self.jmin-1)..=(self.jmax+1) {
                // State of the square on the current image
                let mut val = 0;
                for ii in (i-1)..=(i+1) {
                    for jj in (j-1)..=(j+1) {
                        let digit = if self.map.contains_key(&(ii, jj)) {
                            1 
                        } else if self.jmin <= jj && jj <= self.jmax && self.imin <= ii && ii <= self.imax {
                            0
                        } else {
                            infinite
                        };
                        val = 2 * val + digit;
                    }
                }
                // Set digit in new image
                if algorithm.chars().nth(val).unwrap() == '#' {
                    image.set_on(i, j);
                }
            }
        }
        image
    }

    fn pixels_on(&self) -> usize {
        self.map.len()
    }
}


fn main() {

    // read input
    let input = fs::read_to_string("./data/test.txt").expect("Cannot read input file.");
    let mut line_iter = input.lines();
    let algorithm = line_iter.next().unwrap().trim();
    let mut image: Image = Image::new();
    line_iter.next();
    for (i, line) in line_iter.enumerate() {
        for (j, car) in line.trim().chars().enumerate() {
            if car == '#' {
                image.set_on(i as isize, j as isize);
            }
        }
    }

    //
    // Part 1
    //
    
    // state at infinite
    let d_inf = if algorithm.chars().nth(0).unwrap() == '#' { 1 } else { 0 };
    let mut infinite = 0;

    for _i in 0..2 {
        image = image.next_image(algorithm, infinite);
        infinite = (infinite + d_inf) % 2;
    }
    println!("Part 1 - pixels on: {}", image.pixels_on());

    for _i in 2..50 {
        image = image.next_image(algorithm, infinite);
        infinite = (infinite + d_inf) % 2;
    }
    println!("Part 2 - pixels on: {}", image.pixels_on());
}
