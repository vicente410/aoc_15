use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let (cities, distances) = parse(input.trim());
    let (min, max) = calc_routes(&cities, &distances);

    println!("Part 1: the shortest route is {} units long", min);
    println!("Part 2: the longest route is {} units long", max);
}

fn parse(input: &str) -> (Vec<&str>, HashMap<(&str, &str), u32>) {
    let mut cities: Vec<&str> = Vec::new();
    let mut distances: HashMap<(&str, &str), u32> = HashMap::new();

    for line in input.split('\n') {
        let words: Vec<&str> = line.split(' ').collect();
        if !cities.contains(&words[0]) { cities.push(words[0]); }
        if !cities.contains(&words[2]) { cities.push(words[2]); }
        distances.insert((words[0], words[2]), words[4].parse().unwrap());
        distances.insert((words[2], words[0]), words[4].parse().unwrap());
    }

    (cities, distances)
}

fn calc_routes(cities: &Vec<&str>, distances: &HashMap<(&str, &str), u32>) -> (u32, u32) {
    let permutations = cities.iter().permutations(cities.len());
    let mut min = 1000000;
    let mut max = 0;

    for perm in permutations {
        let mut dist = 0;

        for i in 0..perm.len() - 1 {
            dist += distances[&(*perm[i], *perm[i + 1])];
        }

        if dist < min { min = dist; }
        if dist > max { max = dist; }
    }

    (min, max)
}
