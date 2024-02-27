use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");

    let password = format!("{}", run(input.trim()));
    println!("Part 1: the next password should be {}", &password);
    println!("Part 2: the password after that should be {}", &run(&password));
}

fn run(input: &str) -> String {
    let mut nums: Vec<u8> = input.chars().map(|c| (c as u8)).collect();
    increment(&mut nums);

    while !has_straight(&nums) || has_iol(&nums) || !has_two_pairs(&nums) {
        increment(&mut nums);
    }

    nums.into_iter().map(|n| (n as char)).collect()
}

fn increment(nums: &mut Vec<u8>) {
    let mut i = nums.len() - 1;

    nums[i] += 1;
    while nums[i] == b'{' && i > 0 {
        nums[i - 1] += 1;
        i -= 1;
    }

    for i in 0..nums.len() {
        if nums[i] > b'z' {
            nums[i] = b'a';
        }
    }
}

fn has_straight(nums: &Vec<u8>) -> bool {
    for i in 2..nums.len() {
        if nums[i] == nums[i - 1] + 1 && nums[i - 1] == nums[i - 2] + 1 {
            return true;
        }
    }

    return false;
}

fn has_iol(nums: &Vec<u8>) -> bool {
    for i in 0..nums.len() {
        if nums[i] == b'i' || nums[i] == b'o' || nums[i] == b'l' {
            return true;
        }
    }

    return false;
}

fn has_two_pairs(nums: &Vec<u8>) -> bool {
    let mut i = 0;
    let mut count = 0;

    while i < nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            count += 1;
            i += 1
        }

        if count == 2 {
            return true;
        }

        i += 1;
    }

    return false;
}
