use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Part 1: the output is {} long", run(&args[1], 40));
    println!("Part 2: the output is {} long", run(&args[1], 50));
}

fn run(input: &str, n: u32) -> usize {
    let mut nums: Vec<u8> = Vec::new();

    for c in input.chars() {
        nums.push(c.to_digit(10).unwrap() as u8);
    }

    for _ in 0..n {
        nums = calc(&nums);
    }

    nums.len()
}

fn calc(nums: &Vec<u8>) -> Vec<u8>{
    let mut calc_nums: Vec<u8> = Vec::new();
    let mut count = 1;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            count += 1;
        } else {
            calc_nums.push(count);
            calc_nums.push(nums[i - 1]);
            count = 1;
        }
    }
    calc_nums.push(count);
    calc_nums.push(nums[nums.len() - 1]);

    calc_nums
}
