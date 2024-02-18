use std::fs;

enum Mode {
    PartOne,
    PartTwo,
}

struct Instruction {
    begin: (usize, usize),
    end: (usize, usize),
    command: Command,
}

#[derive(PartialEq)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let instructions = parse(input.trim());

    println!(
        "Part 1: {} lights are on",
        run(&instructions, Mode::PartOne)
    );
    println!(
        "Part 2: the total brightness is {}",
        run(&instructions, Mode::PartTwo)
    );
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input.split("\n") {
        let mut instr_str: Vec<&str> = line.split_whitespace().collect();
        if instr_str[0] == "turn" {
            instr_str.remove(0);
        }

        let begin: Vec<&str> = instr_str[1].split(",").collect();
        let end: Vec<&str> = instr_str[3].split(",").collect();

        let instruction = Instruction {
            begin: (begin[0].parse().unwrap(), begin[1].parse().unwrap()),
            end: (end[0].parse().unwrap(), end[1].parse().unwrap()),
            command: match instr_str[0] {
                "on" => Command::TurnOn,
                "off" => Command::TurnOff,
                _ => Command::Toggle,
            },
        };

        instructions.push(instruction);
    }

    instructions
}

fn run(instructions: &Vec<Instruction>, mode: Mode) -> u32 {
    let mut lights: [[u32; 1000]; 1000] = [[0; 1000]; 1000];

    for instruction in instructions {
        run_instruction(&mut lights, instruction, &mode);
    }

    total_brightness(lights)
}

fn run_instruction(lights: &mut [[u32; 1000]; 1000], instruction: &Instruction, mode: &Mode) {
    for i in instruction.begin.0..=instruction.end.0 {
        for j in instruction.begin.1..=instruction.end.1 {
            match mode {
                Mode::PartOne => part_one_rules(&mut lights[i][j], &instruction.command),
                Mode::PartTwo => part_two_rules(&mut lights[i][j], &instruction.command),
            }
        }
    }
}

fn part_one_rules(light: &mut u32, command: &Command) {
    match command {
        Command::TurnOn => *light = 1,
        Command::TurnOff => *light = 0,
        Command::Toggle => {
            if *light == 1 {
                *light = 0;
            } else {
                *light = 1;
            }
        }
    }
}

fn part_two_rules(light: &mut u32, command: &Command) {
    match command {
        Command::TurnOn => *light += 1,
        Command::TurnOff => {
            if *light > 0 {
                *light -= 1
            }
        }
        Command::Toggle => *light += 2,
    }
}

fn total_brightness(lights: [[u32; 1000]; 1000]) -> u32 {
    let mut total = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            total += lights[i][j];
        }
    }

    total
}
