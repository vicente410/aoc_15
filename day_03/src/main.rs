use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed reading file");

    println!("Santa visited {} houses", calc_num_houses(&input, 1));
    println!(
        "Santa and the robot visited {} houses",
        calc_num_houses(&input, 2)
    );
}

fn calc_num_houses(input: &String, n: usize) -> usize {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut pawns: Vec<(i32, i32)> = vec![(0, 0); n];

    for (i, c) in input.chars().enumerate() {
        if !visited.contains(&pawns[i % n]) {
            visited.push(pawns[i % n]);
        }
        change_pos(&mut pawns[i % n], c);
    }

    visited.len()
}

fn change_pos(pawn: &mut (i32, i32), c: char) {
    let change = match c {
        '^' => (0, 1),
        'v' => (0, -1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => (0, 0),
    };

    pawn.0 += change.0;
    pawn.1 += change.1;
}
