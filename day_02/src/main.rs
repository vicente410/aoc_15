use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let presents = parse(&input);

    println!(
        "Part 1: total area of paper needed is {}",
        calc_paper_size(&presents)
    );
    println!(
        "Part 2: total ribbon lenght needed is {}",
        calc_ribbon_size(&presents)
    );
}

fn parse(input: &String) -> Vec<(u32, u32, u32)> {
    let mut presents = Vec::new();

    for present_str in input.split_whitespace() {
        let p: Vec<u32> = present_str.split("x").map(|x| x.parse().unwrap()).collect();
        presents.push((p[0], p[1], p[2]));
    }

    presents
}

fn calc_paper_size(presents: &Vec<(u32, u32, u32)>) -> u32 {
    let mut size = 0;

    for p in presents {
        size += (2 * p.0 * p.1)
            + (2 * p.1 * p.2)
            + (2 * p.0 * p.2)
            + min(p.0 * p.1, p.0 * p.2, p.1 * p.2);
    }

    size
}

fn calc_ribbon_size(presents: &Vec<(u32, u32, u32)>) -> u32 {
    let mut size = 0;

    for p in presents {
        size += 2 * min(p.0 + p.1, p.0 + p.2, p.1 + p.2) + p.0 * p.1 * p.2;
    }

    size
}

fn min(a: u32, b: u32, c: u32) -> u32 {
    if a < b && a < c {
        a
    } else if b < c {
        b
    } else {
        c
    }
}
