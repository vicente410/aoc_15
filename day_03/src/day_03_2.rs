use std::fs;

fn main(){
    let mut coords: Vec<(i32, i32)> = Vec::new();
    let mut coord_santa: (i32, i32) = (0, 0);
    let mut coord_robot: (i32, i32) = (0, 0);
    coords.push((0, 0));
    let mut count = 1;

    let input = fs::read_to_string("input.txt").unwrap();

    for (i, c) in input.chars().enumerate() {
        let change = match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0)
        };

        if i%2 == 0 {
            coord_santa = add_tuple(coord_santa, change);
            if !coords.contains(&coord_santa) {
                coords.push(coord_santa);
                count += 1;
            }
        } else {
            coord_robot = add_tuple(coord_robot, change);
            if !coords.contains(&coord_robot) {
                coords.push(coord_robot);
                count += 1;
            }
        }


    }

    println!("{}", count);
}

fn add_tuple(a: (i32, i32), b: (i32, i32)) -> (i32, i32){
    (a.0 + b.0, a.1 + b.1)
}
