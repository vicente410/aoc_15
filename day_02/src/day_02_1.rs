use std::fs;

fn main(){
    let input = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut size = 0;

    for present_str in input.split_whitespace() {
        let present: Vec<i32> = present_str.split("x").map(|x| x.parse().unwrap()).collect();
        size += calc_size(&present) + find_min(&present);
    }

    println!("{}", size);
}

fn calc_size(p: &Vec<i32>) -> i32 {
    2*p[0]*p[1] + 2*p[1]*p[2] + 2*p[0]*p[2]
}

fn find_min(p: &Vec<i32>) -> i32 {
    if p[0]*p[1] < p[0]*p[2] && p[0]*p[1] < p[1]*p[2]{
        p[0]*p[1]
    } else if p[0]*p[2] < p[1]*p[2] {
        p[0]*p[2]
    } else {
        p[1]*p[2]
    }
}
