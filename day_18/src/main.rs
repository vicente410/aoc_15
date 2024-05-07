use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let lights = parse(input.trim());

    println!("Part 1: {} lights are on", run(lights.clone(), 100, false));
    println!("Part 2: {} lights are on", run(lights.clone(), 100, true));
}

fn parse(input: &str) -> [[bool; 102]; 102] {
    let mut lights = [[false; 102]; 102];

    for (i, line) in input.split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                lights[i + 1][j + 1] = true;
            }
        }
    }

    lights
}

fn run(mut lights: [[bool; 102]; 102], steps: u32, stuck_lights: bool) -> u32 {
    for _ in 0..steps {
        let mut next_lights = lights.clone();

        for i in 1..=100 {
            for j in 1..=100 {
                let num_neighbours = num_neighbours_on(&lights, i, j);

                if lights[i][j] {
                    if num_neighbours != 2 && num_neighbours != 3 {
                        next_lights[i][j] = false;
                    }
                } else {
                    if num_neighbours == 3 {
                        next_lights[i][j] = true;
                    }
                }
            }
        }

        if stuck_lights {
            next_lights[1][1] = true;
            next_lights[1][100] = true;
            next_lights[100][1] = true;
            next_lights[100][100] = true;
        }

        lights = next_lights;
    }

    count_lights(lights)
}

fn num_neighbours_on(lights: &[[bool; 102]; 102], x: usize, y: usize) -> u32 {
    let mut total = 0;

    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            if lights[i][j] && (i != x || j != y) {
                total += 1;
            }
        }
    }

    total
}

fn count_lights(lights: [[bool; 102]; 102]) -> u32 {
    let mut total = 0;

    for i in 1..=100 {
        for j in 1..=100 {
            if lights[i][j] {
                total += 1;
            }
        }
    }

    total
}
