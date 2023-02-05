use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    name: String,
    vertices: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String) -> Node {
        Node {
            name: name,
            vertices: vec![],
        }
    }
    fn add_vertix(self: &mut Node, node: Rc<RefCell<Node>>) -> &Self {
        if node.borrow().name != String::from("start") {
            self.vertices.push(Rc::clone(&node));
        }
        self
    }
    fn _to_string(&self) -> String {
        let out = self
            .vertices
            .iter()
            .map(|n| n.borrow().name.clone())
            .reduce(|acc, v| format!("{}+{}", acc, v))
            .or(Some(String::from("")))
            .unwrap();
        format!("Node[{} -> {}]", self.name, out)
    }
}

fn find_path(
    origin: Rc<RefCell<Node>>,
    target: Rc<RefCell<Node>>,
    path: &mut Vec<String>,
    mut usedtwice: bool,
) -> usize {
    if origin.as_ptr() == target.as_ptr() {
        return 1;
    }
    if origin.borrow().name.chars().nth(0).unwrap().is_lowercase() {
        let passed = path.iter().filter(|&e| e == &origin.borrow().name).count();
        if usedtwice {
            if passed > 0 {
                return 0;
            }
        } else {
            if passed > 1 {
                return 0;
            } else if passed > 0 {
                usedtwice = true;
            }
        }
    }
    path.push(origin.borrow().name.clone());
    let mut out = 0;
    for destination in &origin.borrow().vertices {
        out += find_path(Rc::clone(destination), Rc::clone(&target), path, usedtwice)
    }
    path.pop();
    out
}

fn main() {
    // read input
    let input = fs::read_to_string("./data/input.txt").expect("Can't read input file.");

    // Create all nodes
    let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    for line in input.trim().lines() {
        let mut ends = line.split("-");
        let origin = ends.next().unwrap().to_string();
        let destination = ends.next().unwrap().to_string();
        let orgnode = Rc::clone(
            nodes
                .entry(destination.to_string())
                .or_insert(Rc::new(RefCell::new(Node::new(destination.to_string())))),
        );
        let destnode = Rc::clone(
            nodes
                .entry(origin.to_string())
                .or_insert(Rc::new(RefCell::new(Node::new(origin.to_string())))),
        );
        destnode.borrow_mut().add_vertix(Rc::clone(&orgnode));
        orgnode.borrow_mut().add_vertix(Rc::clone(&destnode));
    }

    let start = nodes.get("start").unwrap();
    let end = nodes.get("end").unwrap();

    // Part 1
    let mut path = vec![];
    let numpath = find_path(Rc::clone(start), Rc::clone(end), &mut path, true);
    println!("Part 1 - paths: {}", numpath);

    // Part 2
    let mut path = vec![];
    let numpath = find_path(Rc::clone(start), Rc::clone(end), &mut path, false);
    println!("Part 2 - paths: {}", numpath);
}
