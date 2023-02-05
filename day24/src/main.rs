use std::fs;

#[derive(Debug)]
struct Ctxt<'a> {
    input: &'a[isize; 14],
    index: usize,
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl<'a> Ctxt<'a> {
    fn new(input: &[isize; 14]) -> Ctxt {
        Ctxt {
            input,
            index: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0
        }
    }
    fn put(&mut self, var: &Var, value: isize) {
        match var {
            Var::W => self.w = value,
            Var::X => self.x = value,
            Var::Y => self.y = value,
            Var::Z => self.z = value,
            Var::Num(_) => panic!("Can only put in registers!"),
        }
    }
    fn get(&self, var: &Var) -> isize {
        match var {
            Var::W => self.w,
            Var::X => self.x,
            Var::Y => self.y,
            Var::Z => self.z,
            Var::Num(val) => *val,
        }
    }
    fn next_input(&mut self) -> isize {
        let v = self.input[self.index];
        self.index += 1;
        v
    }
    fn to_string(&self) -> String {
        format!("Ctxt[W:{} X:{} Y:{} Z:{}]", self.w, self.x, self.y, self.z)
    }
}

#[derive(Debug)]
enum Var {
    W,
    X,
    Y,
    Z,
    Num(isize),
}

impl Var {
    fn from(input: &str) -> Self {
        match input {
            "w" => Var::W,
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            _ => Var::Num(input.parse::<isize>().unwrap()),
        }
    }
    fn _to_string(&self) -> String {
        match self {
            Var::W => String::from("w"),
            Var::X => String::from("x"),
            Var::Y => String::from("y"),
            Var::Z => String::from("z"),
            Var::Num(val) => format!("{}", val),
        }
    }
}

#[derive(Debug)]
enum Inst {
    Inp(Var),
    Add(Var, Var),
    Mul(Var, Var),
    Div(Var, Var),
    Mod(Var, Var),
    Eql(Var, Var)
}

impl Inst {
    fn exec(&self, ctxt: &mut Ctxt) {
        match self {
            Self::Inp(a) => {
                let val = ctxt.next_input();
                ctxt.put(a, val);
            },
            Self::Add(a, b) => {
                let val = ctxt.get(a) + ctxt.get(b);
                ctxt.put(a, val);
            },   
            Self::Mul(a, b) => {
                let val = ctxt.get(a) * ctxt.get(b);
                ctxt.put(a, val);
            }, 
            Self::Div(a, b) => {
                let val = ctxt.get(a) / ctxt.get(b);
                ctxt.put(a, val);
            }, 
            Self::Mod(a, b) => {
                let val = ctxt.get(a) % ctxt.get(b);
                ctxt.put(a, val);
            }, 
            Self::Eql(a, b) => {
                if ctxt.get(a) == ctxt.get(b) {
                    ctxt.put(a, 1)
                } else {
                    ctxt.put(a, 0)
                }
            }, 
        }
    }
    fn from(input: &str) -> Self {
        let mut tokens = input.split(" ");
        let cmd = tokens.next().unwrap();
        let a = Var::from(tokens.next().unwrap());
        let b = if let Some(val) = tokens.next() {
            Some(Var::from(val))
        } else {
            None
        };
        match (cmd, a, b) {
            ("inp", i, None) => Self::Inp(i),
            ("add", i, Some(j)) => Self::Add(i, j),
            ("mul", i, Some(j)) => Self::Mul(i, j),
            ("div", i, Some(j)) => Self::Div(i, j),
            ("mod", i, Some(j)) => Self::Mod(i, j),
            ("eql", i, Some(j)) => Self::Eql(i, j),
            _ => panic!("Invalid instruction!"),
        }
    }
    fn _to_string(&self) -> String {
        match self {
            Self::Inp(v) => format!("Inp({})", v._to_string()),
            Self::Add(v, w) => format!("Add({}, {})", v._to_string(), w._to_string()),
            Self::Mul(v, w) => format!("Mul({}, {})", v._to_string(), w._to_string()),
            Self::Div(v, w) => format!("Div({}, {})", v._to_string(), w._to_string()),
            Self::Mod(v, w) => format!("Mod({}, {})", v._to_string(), w._to_string()),
            Self::Eql(v, w) => format!("Eql({}, {})", v._to_string(), w._to_string()),
        }
    }
    fn to_rust(&self) -> String {
        match self {
            Self::Inp(v) =>    format!("        {} = input[j];\n        j += 1;", v._to_string()),
            Self::Add(v, w) => format!("        {} = {} + {};", v._to_string(), v._to_string(), w._to_string()),
            Self::Mul(v, w) => format!("        {} = {} * {};", v._to_string(), v._to_string(), w._to_string()),
            Self::Div(v, w) => format!("        {} = {} / {};", v._to_string(), v._to_string(), w._to_string()),
            Self::Mod(v, w) => format!("        {} = {} % {};", v._to_string(), v._to_string(), w._to_string()),
            Self::Eql(v, w) => format!("        {} = if {} == {} {{ 1 }} else {{ 0 }};", v._to_string(), v._to_string(), w._to_string()),
        }
    }
}

fn main() {
    //
    // Read input
    //
    let input = fs::read_to_string("./data/input.txt").expect("Can't read input!");
    let program: Vec<Inst> = input
        .lines()
        .map(|line| Inst::from(line))
        .collect();

    //
    // Part 1b: generate a Rust program 
    //
    // for inst in &program {
    //     println!("{}", inst.to_rust());
    // }
    // return;

    //
    // Test first sub to validate python prog
    //
    let mut input: [isize; 14] = [0,9,9,9,9,9,9,9,9,9,9,9,9,9];
    let mut ctxt = Ctxt::new(&input);
    for inst in &program[0..18] {
        println!("{:?} -> {:?}", inst, ctxt);
        inst.exec(&mut ctxt);
    }
    println!("At the end: {:?} -> {}", input, ctxt.z);



    //
    // Part 1
    //
    let mut input: [isize; 14] = [9,9,9,9,9,9,9,9,9,9,9,9,9,9];
    loop {

        // Execute program
        let mut ctxt = Ctxt::new(&input);
        for inst in &program {
            inst.exec(&mut ctxt);
        }
        if ctxt.z == 0 {
            println!("{:?} -> {}", input, ctxt.z);
            break;
        }
        // Decrement input
        let mut i = 13;
        while i != 0 {
            if input[i] == 1 {
                input[i] = 9;
                if i != 0 {
                    i -= 1;
                } else {
                    break;
                }
            } else {
                input[i] -= 1;
                break;
            }
        }
    }
}
