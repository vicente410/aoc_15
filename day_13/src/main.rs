use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let (mut names, mut happiness) = parse(input.trim());

    println!(
        "Part 1: the optimal seating arrangement has {} happiness",
        calc_happiness(&names, &happiness)
    );

    add_person(&mut names, &mut happiness, "Bruno");
    println!(
        "Part 2: the optimal seating arrangement with me has {} happiness",
        calc_happiness(&names, &happiness)
    );
}

fn parse(input: &str) -> (Vec<&str>, HashMap<(&str, &str), i32>) {
    let mut names: Vec<&str> = Vec::new();
    let mut happiness: HashMap<(&str, &str), i32> = HashMap::new();

    for line in input.split('\n') {
        let words: Vec<&str> = line.split(' ').collect();
        if !names.contains(&words[0]) {
            names.push(words[0]);
        }
        let mut hap = words[3].parse().unwrap();
        if words[2] == "lose" {
            hap *= -1;
        }
        happiness.insert((words[0], &words[10][0..words[10].len() - 1]), hap);
    }

    (names, happiness)
}

fn calc_happiness(names: &Vec<&str>, happiness: &HashMap<(&str, &str), i32>) -> i32 {
    let permutations = names.iter().permutations(names.len());
    let mut max = i32::MIN;

    for perm in permutations {
        let mut total_happiness = 0;

        for i in 0..perm.len() - 1 {
            total_happiness += happiness[&(*perm[i], *perm[i + 1])];
            total_happiness += happiness[&(*perm[i + 1], *perm[i])];
        }
        total_happiness += happiness[&(*perm[perm.len() - 1], *perm[0])];
        total_happiness += happiness[&(*perm[0], *perm[perm.len() - 1])];

        if total_happiness > max {
            max = total_happiness;
        }
    }

    max
}

fn add_person<'a>(
    names: &mut Vec<&'a str>,
    happiness: &mut HashMap<(&'a str, &'a str), i32>,
    new_name: &'a str,
) {
    for n in names.clone() {
        happiness.insert((n, new_name), 0);
        happiness.insert((new_name, n), 0);
    }
    names.push(new_name);
}
