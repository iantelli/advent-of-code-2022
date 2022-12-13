use std::collections::HashMap;

fn main() {
    let content: String = load_input();

    let mut score: u32 = 0;
    for line in content.lines() {
        let game = (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
        score += rps(game);
    }
    println!("{}", score); // Answer: 12855
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

    you.points.insert('X', 1);
    you.points.insert('Y', 2);
    you.points.insert('Z', 3);

    let win = 6;
    let lose = 0;
    let draw = 3;

    score.outcome.insert('A', {
        let mut a = HashMap::new();
        a.insert('X', draw);
        a.insert('Y', win);
        a.insert('Z', lose);
        a
    });

    score.outcome.insert('B', {
        let mut b = HashMap::new();
        b.insert('X', lose);
        b.insert('Y', draw);
        b.insert('Z', win);
        b
    });

    score.outcome.insert('C', {
        let mut c = HashMap::new();
        c.insert('X', win);
        c.insert('Y', lose);
        c.insert('Z', draw);
        c
    });

    let extra = you.points.get(&game.1).copied().unwrap_or(0);
    let outcome = score.outcome.get(&game.0).unwrap().get(&game.1).unwrap();
    return extra + outcome;
}
