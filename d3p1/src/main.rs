use std::collections::HashSet;

fn main() {
    let content: String = load_input();
    let mut final_score: u32 = 0;

    for line in content.lines() {
        let length = line.len();
        let half = length / 2;

        let first = &line[0..half];
        let second = &line[half..length];
        final_score += calculate(common(first, second));
    }

    println!("{}", final_score); // Answer: 7597
}

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn common(f: &str, s: &str) -> Vec<char> {
    let mut array: Vec<char> = Vec::new();
    for c in f.chars() {
        for i in s.chars() {
            if i == c {
                array.push(i)
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
