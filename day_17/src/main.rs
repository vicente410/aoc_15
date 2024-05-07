use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let containers = parse(input.trim());
    let (count, min_num) = count_combinations(containers, 150);

    println!("Part 1: there are {} combinations of containers", count);
    println!(
        "Part 2: the smallest number of containers has {} combinations",
        min_num
    );
}

fn parse(input: &str) -> Vec<u32> {
    let mut containers: Vec<u32> = Vec::new();

    for line in input.split('\n') {
        containers.push(line.parse().unwrap());
    }

    containers
}

fn count_combinations(containers: Vec<u32>, target_size: u32) -> (u32, u32) {
    let mut count = 0;
    let mut min_num = 0;

    for i in 1..containers.len() {
        let combinations = containers.iter().combinations(i);

        for comb in combinations {
            let mut size = 0;
            for n in comb {
                size += n;
            }

            if size == target_size {
                count += 1;
            }
        }

        if min_num == 0 {
            min_num = count;
        }
    }

    (count, min_num)
}
