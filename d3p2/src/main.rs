use std::collections::HashSet;

fn main() {
    let content: String = load_input();
    let mut final_score: u32 = 0;
    let mut sack: Vec<&str> = Vec::new();
    for line in content.lines() {
        sack.push(line);
        if sack.len() == 3 {
            final_score += calculate(common(&sack));
            sack.clear();
        }
    }

    println!("{}", final_score); // Answer: 2607
}

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn common(sack: &Vec<&str>) -> Vec<char> {
    let first = sack[0];
    let second = sack[1];
    let third = sack[2];

    println!("{} {} {}", first, second, third);

    let mut array: Vec<char> = Vec::new();

    for c in first.chars() {
        for i in second.chars() {
            for j in third.chars() {
                if i == c && j == c {
                    array.push(c);
                }
            }
        }
    }

    // https://stackoverflow.com/questions/72921334/how-to-remove-duplicate-in-an-array-in-rust
    let unique = array
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    return unique;
}

fn calculate(letter: Vec<char>) -> u32 {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score: u32 = 0;
    for c in letter {
        let index = ALPHABET.find(c).unwrap();
        score += index as u32 + 1;
    }
    return score;
}
