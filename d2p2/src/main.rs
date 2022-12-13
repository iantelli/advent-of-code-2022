use std::collections::HashMap;

fn main() {
    let content: String = load_input();

    let mut score: u32 = 0;
    for line in content.lines() {
        let game = (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
        score += rps(game);
    }
    println!("{}", score); // Answer: 13726
}

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn rps(game: (char, char)) -> u32 {
    struct Game {
        outcome: HashMap<char, HashMap<char, u32>>,
    }
    struct You {
        points: HashMap<char, u32>,
    }
    let mut score = Game {
        outcome: HashMap::new(),
    };
    let mut you = You {
        points: HashMap::new(),
    };

    you.points.insert('X', 0); // lose
    you.points.insert('Y', 3); // draw
    you.points.insert('Z', 6); // win

    let paper = 2;
    let scissors = 3;
    let rock = 1;

    score.outcome.insert('A', {
        let mut a = HashMap::new();
        a.insert('X', scissors);
        a.insert('Y', rock);
        a.insert('Z', paper);
        a
    });

    score.outcome.insert('B', {
        let mut b = HashMap::new();
        b.insert('X', rock);
        b.insert('Y', paper);
        b.insert('Z', scissors);
        b
    });

    score.outcome.insert('C', {
        let mut c = HashMap::new();
        c.insert('X', paper);
        c.insert('Y', scissors);
        c.insert('Z', rock);
        c
    });

    let extra = you.points.get(&game.1).copied().unwrap_or(0);
    let outcome = score.outcome.get(&game.0).unwrap().get(&game.1).unwrap();
    return extra + outcome;
}
