use std::fs;

fn main(){
    let input = fs::read_to_string("input.txt").expect("Failed reading file");
    let mut count = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => ()
        }

        if count == -1 {
            println!("{}", i);
            break;
        }
    }
}
