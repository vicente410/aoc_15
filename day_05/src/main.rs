use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");

    println!("Part 1: {} nice words", part_one(&input));
    println!("Part 2: {} nice words", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let mut count = 0;

    for word in input.split_whitespace() {
        if has_three_vowels(word) && has_repeated_letters(word, 1) && !has_bad_pair(word) {
            count += 1;
        }
    }

    count
}

fn part_two(input: &str) -> u32 {
    let mut count = 0;

    for word in input.split_whitespace() {
        if has_repeated_pair(word) && has_repeated_letters(word, 2) {
            count += 1;
        }
    }

    count
}

fn has_three_vowels(word: &str) -> bool {
    let mut count = 0;

    for c in word.chars() {
        match c {
            'a' => count += 1,
            'i' => count += 1,
            'u' => count += 1,
            'e' => count += 1,
            'o' => count += 1,
            _ => (),
        }

        if count == 3 {
            return true;
        }
    }

    false
}

fn has_repeated_letters(word: &str, spacing: usize) -> bool {
    let letters: Vec<char> = word.chars().collect();

    for i in spacing..letters.len() {
        if letters[i - spacing] == letters[i] {
            return true;
        }
    }

    false
}

fn has_bad_pair(word: &str) -> bool {
    let letters: Vec<char> = word.chars().collect();

    for i in 1..letters.len() {
        match (letters[i - 1], letters[i]) {
            ('a', 'b') => return true,
            ('c', 'd') => return true,
            ('p', 'q') => return true,
            ('x', 'y') => return true,
            _ => (),
        }
    }

    false
}

fn has_repeated_pair(word: &str) -> bool {
    let letters: Vec<char> = word.chars().collect();

    for i in 1..letters.len() {
        for j in 1..letters.len() {
            if letters[i] == letters[j]
                && letters[i - 1] == letters[j - 1]
                && i != j - 1
                && i != j
                && i != j + 1
            {
                return true;
            }
        }
    }

    false
}
