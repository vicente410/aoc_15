use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Gate<'a> {
    Set(&'a str),
    Not(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, u8),
    RShift(&'a str, u8),
}

fn main() {
    let input = fs::read_to_string("../input2.txt").expect("Failed to read file");
    let wires = parse(input.trim());
    let mut memo: HashMap<&str, u16> = HashMap::new();

    let value = calc_wire(&wires, &mut memo, "a").to_string();
    println!("Part 1: the value of a is {}", value);

    let mut wires = wires.clone();
    wires.remove("b");
    wires.insert("b", Gate::Set(&value));
    memo = HashMap::new();
    println!("Part 2: the value of a is {}", calc_wire(&wires, &mut memo, "a"));
}

fn parse(input: &str) -> HashMap<&str, Gate> {
    let mut wires: HashMap<&str, Gate> = HashMap::new();

    for line in input.split("\n") {
        let wire_str: Vec<&str> = line.split_whitespace().collect();

        match wire_str[1] {
            "AND" => wires.insert(wire_str[4], Gate::And(wire_str[0], wire_str[2])),
            "OR" => wires.insert(wire_str[4], Gate::Or(wire_str[0], wire_str[2])),
            "LSHIFT" => wires.insert(wire_str[4], Gate::LShift(wire_str[0], wire_str[2].parse().unwrap())),
            "RSHIFT" => wires.insert(wire_str[4], Gate::RShift(wire_str[0], wire_str[2].parse().unwrap())),
            _ => match wire_str[0] {
                "NOT" => wires.insert(wire_str[3], Gate::Not(wire_str[1])),
                _ => wires.insert(wire_str[2], Gate::Set(wire_str[0])),
            },
        };
    }

    wires
}

fn calc_wire<'a>(wires: &'a HashMap<&str, Gate>, memo: &mut HashMap<&'a str, u16>, input: &'a str) -> u16 {
    if memo.contains_key(input) {
        memo[input]
    } else {
        match input.parse() {
            Ok(num) => num,
            Err(_) => {
                let value = match wires[input] {
                    Gate::Set(a) => calc_wire(wires, memo, a),
                    Gate::Not(a) => !calc_wire(wires, memo, a),
                    Gate::And(a, b) => calc_wire(wires, memo, a) & calc_wire(wires, memo, b),
                    Gate::Or(a, b) => calc_wire(wires, memo, a) | calc_wire(wires, memo, b),
                    Gate::LShift(a, b) => calc_wire(wires, memo, a) << b,
                    Gate::RShift(a, b) => calc_wire(wires, memo, a) >> b,
                };
                memo.insert(input, value);
                value
            }
        }
    }
}
