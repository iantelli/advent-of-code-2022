fn main() {
    let content: String = load_input();
    let mut array: Vec<u32> = Vec::new();

    let mut sum: u32 = 0;
    for calorie in content.lines() {
        if calorie.is_empty() {
            array.push(sum);
            sum = 0
        } else {
            sum += calorie.parse::<u32>().unwrap();
        }
    }

    let length: usize = array.len();
    let tail: Vec<u32> = sort(array).split_off(length - 3); // [68579, 69863, 74394]

    let sum: u32 = tail.iter().sum();
    println!("{:?}", sum); // Answer: 212836
}

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

// https://stackoverflow.com/questions/44888196/how-do-i-sort-an-array
fn sort<A, T>(mut array: A) -> A
where
    A: AsMut<[T]>,
    T: Ord,
{
    let slice: &mut [T] = array.as_mut();
    slice.sort();

    array
}
