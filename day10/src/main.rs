use std::fs;

fn is_closing(token: char) -> bool {
    token == '}' || token == ']' || token == ')' || token == '>'
}

fn is_matching(token1: char, token2: char) -> bool {
    (token1 == '{' && token2 == '}')
        || (token1 == '[' && token2 == ']')
        || (token1 == '(' && token2 == ')')
        || (token1 == '<' && token2 == '>')
}

fn ass_closing(token: char) -> char {
    match token {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        '<' => '>',
        _ => panic!("Unexpected char"),
    }
}

fn completion_score(completion: &String) -> usize {
    let mut score = 0;
    for c in completion.chars() {
        score = 5 * score;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected char."),
        }
    }
    score
}

fn parse(
    iter: &mut std::str::CharIndices,
    completion: &mut String,
) -> Result<Option<(usize, char)>, Option<(usize, char)>> {
    match iter.next() {
        Some((i, opening)) => {
            // closing bracket, it should match an opening one up in the recursion
            if is_closing(opening) {
                return Ok(Some((i, opening)));
            }

            // opening bracket, let parse till the closing one
            match parse(iter, completion) {
                Err(Some(err)) => Err(Some(err)),
                Err(None) => panic!("That shouldn't happen"),
                Ok(Some((j, closing))) => {
                    if is_matching(opening, closing) {
                        parse(iter, completion)
                    } else {
                        Err(Some((j, closing)))
                    }
                }
                Ok(None) => {
                    completion.push(ass_closing(opening));
                    Ok(None)
                }
            }
        }
        None => Ok(None),
    }
}

fn main() {
    // Parse input
    let input = fs::read_to_string("./data/input.txt").expect("Cannot read in put file");

    // Part 1 & 2
    let mut count = 0;
    let mut completion_scores: Vec<usize> = Vec::new();
    for (_linenum, line) in input.lines().enumerate() {
        let mut completion = String::new();
        count += match parse(&mut line.char_indices(), &mut completion) {
            // parsing found an illegal char.
            Err(Some((_position, illegal_char))) => {
                match illegal_char {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("That shouldn't happen."),
                }
            },
            // The string is legit (but maybe incomplete).
            Ok(_) => {
                let score = completion_score(&completion);
                completion_scores.push(score);
                0
            },
            Err(None) => panic!("That shouldn't happen."),
        };
    }
    println!("Part 1 - Count: {}", count);

    completion_scores.sort();
    println!(
        "Part 2 - Score: {}",
        completion_scores[completion_scores.len() / 2]
    );
}
