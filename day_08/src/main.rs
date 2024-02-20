use std::fs;

fn main() {
    let input = fs::read_to_string("../input2.txt").expect("Failed to read file");

    println!("Part 1: {} escaped characters", part_one(&input));
    println!("Part 1: {} added characters", part_two(&input));
}

fn part_one(input: &String) -> usize {
    let mut escaped = 0;
    let mut it = input.chars();

    while let Some(c) = it.next() {
        match c {
            '"' => escaped += 1,
            '\\' => match it.next() {
                Some('x') => escaped += 3,
                Some('\\') => escaped += 1,
                Some('"') => escaped += 1,
                _ => (),
            },
            _ => (),
        }
    }

    escaped
}

fn part_two(input: &String) -> usize {
    let mut added = 0;
    let mut it = input.chars();

    while let Some(c) = it.next() {
        match c {
            '"' => added += 1,
            '\\' => added += 1,
            '\n' => added += 2,
            _ => (),
        }
    }

    added
}
