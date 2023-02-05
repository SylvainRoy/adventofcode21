use std::fs;

#[derive(Debug)]
struct Item {
    unique: Vec<[bool; 7]>,
    output: Vec<[bool; 7]>
}

fn read_display(s: &str) -> [bool; 7] {
    let mut out = [false; 7];
    for (i, seg) in "abcdefg".chars().enumerate() {
        out[i] = s.find(seg).is_some();
    };
    out
}

fn translate(tran: [i32;7], display: [bool;7]) -> [bool;7] {
    let mut out = [false; 7];
    for i in 0..7 {
        let j:usize = tran[i] as usize;
        out[j] = display[i];
    }
    out
}

fn to_string(d: [bool; 7]) -> String {
    if d[0] && d[1] && d[2] && !d[3] && d[4] && d[5] && d[6] {return String::from("0")}
    if !d[0] && !d[1] && d[2] && !d[3] && !d[4] && d[5] && !d[6] {return String::from("1")}
    if d[0] && !d[1] && d[2] && d[3] && d[4] && !d[5] && d[6] {return String::from("2")}
    if d[0] && !d[1] && d[2] && d[3] && !d[4] && d[5] && d[6] {return String::from("3")}
    if !d[0] && d[1] && d[2] && d[3] && !d[4] && d[5] && !d[6] {return String::from("4")}
    if d[0] && d[1] && !d[2] && d[3] && !d[4] && d[5] && d[6] {return String::from("5")}
    if d[0] && d[1] && !d[2] && d[3] && d[4] && d[5] && d[6] {return String::from("6")}
    if d[0] && !d[1] && d[2] && !d[3] && !d[4] && d[5] && !d[6] {return String::from("7")}
    if d[0] && d[1] && d[2] && d[3] && d[4] && d[5] && d[6] {return String::from("8")}
    if d[0] && d[1] && d[2] && d[3] && !d[4] && d[5] && d[6] {return String::from("9")}
    String::from("?")
}

fn analyse(item: &Item) -> [i32; 7] {
    // Translation 'segment' --> 'correct segment'
    let mut tran = [-1; 7];
    // Compute total number of time each segment appears in
    // the unique displays
    let mut occurences = [0; 7];
    for display in &item.unique {
        for i in 0..7 {
            if display[i] {
                occurences[i] += 1;
            }
        }
    }
    //println!("occurences: {:?}", occurences);
    // b, e and f can be identified based on the number of occurences.
    for (segment, num) in occurences.iter().enumerate() {
        if *num == 6 { tran[segment] = 1 }                                // b 
        if *num == 4 { tran[segment] = 4 }                                // e
        if *num == 9 { tran[segment] = 5 }                                // f
    }
    // c is the only segment in the '1' that appears 8 times
    // d is the only segment in the '4' that appears 7 times
    for display in &item.unique {
        let num_seg_in_display = display.iter().filter(|b| **b).count();
        if num_seg_in_display == 2 { // it's a '1'
            for segment in 0..display.len() {
                if display[segment] && occurences[segment] == 8 {
                    tran[segment] = 2;                                    // c
                    break;
                }
            }
        }
        if num_seg_in_display == 4 { // it's a '4'
            for segment in 0..display.len() {
                if display[segment] && occurences[segment] == 7 {
                    tran[segment] = 3;                                    // d
                    break;
                }
            }
        }
    }
    // g is only segment left that appears 7 times
    for segment in 0..tran.len() {
        if occurences[segment] == 7 && tran[segment] == -1 {
            tran[segment] = 6;                                            // g
            break;
        }
    }
    // a is the only segment left
    for segment in 0..tran.len() {
        if tran[segment] == -1 {
            tran[segment] = 0;                                            // a
            break;
        }                                      
    }
    tran
}

fn main() {

    let input = fs::read_to_string("./data/input.txt").expect("Cannot read input file");

    let items: Vec<Item> = input
        .lines()
        .map(|line| line
            .trim()
            .split(" ")
            .filter(|s| *s != "|")
            .map(|s| read_display(s))
            .collect::<Vec<[bool; 7]>>())
        .map(|a| Item { unique: a[0..10].to_vec(), output: a[10..14].to_vec() })
        .collect();

    let mut count = 0;
    let mut sum = 0;

    for item in items {
        let tran = analyse(&item);
        let mut s = String::new();
        for display in &item.output {
            let n = translate(tran, *display);
            let c = to_string(n);
            if (c == "1") || (c == "4") || (c == "7") || (c == "8") {
                count += 1;
            }
            s += &c;
        }
        sum += s.parse::<u32>().unwrap();
    }
    println!("Part 1 - Count: {}", count);
    println!("Part 2 - Sum: {}", sum);
}
