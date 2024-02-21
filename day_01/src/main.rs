use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed reading file");
    let (final_floor, steps_to_neg) = calc_floors(&input);

    println!("Part 1: the final floor is {}", final_floor);
    println!("Part 2: {} steps taken to reach floor -1", steps_to_neg);
}

fn calc_floors(input: &String) -> (i32, usize) {
    let mut floor = 0;
    let mut steps_to_neg = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 && steps_to_neg == 0 {
            steps_to_neg = i + 1;
        }
    }

    (floor, steps_to_neg)
}
