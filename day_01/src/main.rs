use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed reading file");

    println!("The final floor is {}", final_floor(&input));
    println!(
        "The number of steps to floor -1 is {}",
        steps_to_neg(&input)
    );
}

fn final_floor(input: &String) -> isize {
    let mut count = 0;

    for c in input.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => (),
        }
    }

    count
}

fn steps_to_neg(input: &String) -> usize {
    let mut count = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => (),
        }

        if count == -1 {
            return i + 1;
        }
    }

    0
}
