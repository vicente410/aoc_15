use md5;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Part1: {}", solve(&args[1], "00000"));
    println!("Part1: {}", solve(&args[1], "000000"));
}

fn solve(key: &str, search: &str) -> u32 {
    let mut num = 1;

    loop {
        let key = format!("{}{}", key, num);
        let digest = format!("{:x}", md5::compute(key.as_bytes()));

        if &digest[0..search.len()] == search {
            return num;
        }

        num += 1;
    }
}
