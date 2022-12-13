fn main() {
    let content: String = load_input();

    let mut max: u32 = 0;
    let mut sum: u32 = 0;
    for calorie in content.lines() {
        if calorie.is_empty() {
            if sum > max {
                max = sum
            }
            sum = 0
        } else {
            sum += calorie.parse::<u32>().unwrap();
        }
    }
    println!("{:?}", max); // Answer: 74394
}

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}
