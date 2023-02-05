// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

use std::error::Error;
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
// use std::cmp::Eq;
use priority_queue::PriorityQueue;

#[derive(Clone, Hash, PartialEq)]
struct Cell {
    i: usize,
    j: usize,
    risk: usize,
    score: usize,
    parent: Option<Rc<RefCell<Cell>>>,
    childs: Vec<Rc<RefCell<Cell>>>,
}

impl Cell {
    fn new(i: usize, j: usize, risk: usize) -> Cell {
        Cell {
            i,
            j,
            risk,
            score: 0,
            parent: None,
            childs: Vec::new()
        }
    }
}

fn display_board(map: &Vec<Vec<Cell>>) {
    let mut out = String::new();
    for line in map {
        for cell in line {
            out += &format!("{:2},", cell.score);
        }
        out += "\n";
    }
    println!("{}", out);
}

fn djikstra(
    graph: &mut Vec<Rc<RefCell<Cell>>>, 
    start: Rc<RefCell<Cell>>, 
    end: Rc<RefCell<Cell>>,
    path: &Vec<Rc<RefCell<Cell>>>
) -> usize {

    // dist[source] ← 0                           // Initialization
    start.borrow().score = 0;

    // create vertex priority queue Q
    let mut prioqueue = PriorityQueue::new();

    // for each vertex v in Graph:          
    //     if v ≠ source
    //         dist[v] ← INFINITY                 // Unknown distance from source to v
    //         prev[v] ← UNDEFINED                // Predecessor of v
    //     Q.add_with_priority(v, dist[v])    
    for vertex in graph {
        if vertex != start {
            vertex.score = usize::MAX;
            vertex.parent = None;
        }
        prioqueue.push(vertex, -vertex.score);
    }

    // while Q is not empty:                      // The main loop
    //     u ← Q.extract_min()                    // Remove and return best vertex
    //     for each neighbor v of u:              // only v that are still in Q
    //         alt ← dist[u] + length(u, v)
    //         if alt < dist[v]
    //             dist[v] ← alt
    //             prev[v] ← u
    //             Q.decrease_priority(v, alt)
    while !prioqueue.is_empty() {
        let (i, j) = prioqueue.pop().unwrap();
        
        // for each neighbour still in prioqueue
        for di in 0..3 {
            for dj in 0..3 {
                let ii = i + di as usize;
                let jj = j + dj as usize;
                if ii == i && jj == j { continue; }
                if ii < 0 || nrows <= ii { continue; }
                if jj < 0 || ncols <= jj { continue; } 
                if prioqueue.get((ii, jj)).is_none() { continue; }
                let score = map[i][j].score + map[ii][jj].risk;
                if score < map[ii][jj].score {
                    map[ii][jj].score = score;
                    map[ii][jj].parent = (i, j);
                    prioqueue.change_priority(item: &Q, new_priority: P)
                }

            }
        }
    }

    // return dist, prev
    0
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read input
    let input = fs::read_to_string("./data/test.txt")?;
    //let ncols = input.lines().next().unwrap().chars().count();
    //let nrows = input.lines().count();
    let mut graph: Vec<Rc<RefCell<Cell>>> = Vec::with_capacity(10000);
    input.lines().enumerate().for_each(|(i, line)| {
        line.trim()
            .chars()
            .map(|car| car.to_digit(10).unwrap())
            .enumerate()
            .for_each(|(j, risk)| {
                graph.push(Rc::new(RefCell::new(Cell::new(i, j, risk as usize))));
            })
    });

    // Get shortest path
    // todo

    Ok(())
}
