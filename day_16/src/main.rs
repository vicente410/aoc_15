use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let aunts = parse_aunts(input.trim());

    let input = fs::read_to_string("../ticket.txt").expect("Failed to read file");
    let ticket = parse_ticket(input.trim());

    println!(
        "Part 1: aunt Sue number {} gave the gift",
        find_aunt_1(&aunts, &ticket)
    );
    println!(
        "Part 2: aunt Sue number {} gave the gift",
        find_aunt_2(&aunts, &ticket)
    );
}

fn parse_aunts(input: &str) -> Vec<HashMap<&str, i32>> {
    let mut aunts = Vec::new();

    for line in input.split('\n') {
        let mut words: Vec<_> = line.split(' ').collect();
        let mut aunt = HashMap::new();

        for i in 1..words.len() - 1 {
            words[i] = &words[i][..words[i].len() - 1];
        }

        aunt.insert(words[2], words[3].parse().unwrap());
        aunt.insert(words[4], words[5].parse().unwrap());
        aunt.insert(words[6], words[7].parse().unwrap());

        aunts.push(aunt);
    }

    aunts
}

fn parse_ticket(input: &str) -> HashMap<&str, i32> {
    let mut ticket = HashMap::new();

    for line in input.split('\n') {
        let mut words: Vec<_> = line.split(' ').collect();
        words[0] = &words[0][..words[0].len() - 1];

        ticket.insert(words[0], words[1].parse().unwrap());
    }

    ticket
}

fn find_aunt_1(aunts: &Vec<HashMap<&str, i32>>, ticket: &HashMap<&str, i32>) -> usize {
    let mut found;

    for (i, aunt) in aunts.iter().enumerate() {
        found = true;
        for key in aunt.keys() {
            if ticket[key] != aunt[key] {
                found = false;
                continue;
            }
        }

        if found == true {
            return i + 1;
        }
    }

    0
}

fn find_aunt_2(aunts: &Vec<HashMap<&str, i32>>, ticket: &HashMap<&str, i32>) -> usize {
    let mut found;

    for (i, aunt) in aunts.iter().enumerate() {
        found = true;
        for key in aunt.keys() {
            match key {
                &"cats" | &"trees" => {
                    if ticket[key] >= aunt[key] {
                        found = false;
                        continue;
                    }
                }
                &"pomeranians" | &"goldfish" => {
                    if ticket[key] <= aunt[key] {
                        found = false;
                        continue;
                    }
                }
                _ => {
                    if ticket[key] != aunt[key] {
                        found = false;
                        continue;
                    }
                }
            }
        }

        if found == true {
            return i + 1;
        }
    }

    0
}
