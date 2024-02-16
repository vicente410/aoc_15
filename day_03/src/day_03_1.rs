use std::fs;

fn main(){
    let mut coords: Vec<(i32, i32)> = Vec::new();
    let mut coord: (i32, i32) = (0, 0);
    coords.push((0, 0));
    let mut count = 1;

    let input = fs::read_to_string("input.txt").unwrap();

    for c in input.chars() {
        match c {
            '^' => coord = (coord.0, coord.1 + 1),
            'v' => coord = (coord.0, coord.1 - 1),
            '<' => coord = (coord.0 - 1, coord.1),
            '>' => coord = (coord.0 + 1, coord.1),
            _ => ()
        }

        if !coords.contains(&coord) {
            coords.push(coord);
            count += 1;
        }

    }

    println!("{}", count);
}
