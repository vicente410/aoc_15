pub mod lib;

fn main() {
    //let mut a = String::new();
    //io::stdin().read_line(&mut a).unwrap();

    //md5("AAA".to_string());
}

/*fn md5 (input: String) {
    let mut vec = convert_str_to_bytes(input);
    add_padding(&mut vec);
    let vec = vec8_to_vec32(vec);

    for n in vec {
        print!("{}\t", n);
    }

}

fn convert_str_to_bytes (input: String) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    vec.extend(input.as_bytes());
    vec
}

fn add_padding (vec: &mut Vec<u8>) {
    let length: u8 = vec.len() as u8; 
    let mut zeroes: Vec<u8> = vec![0; 62-vec.len()];

    vec.push(128);
    vec.append(&mut zeroes);
    vec.push(length);
}

fn vec8_to_vec32(vec8: Vec<u8>) -> Vec<u32> {
    let mut vec32: Vec<u32> = vec![0; 16];
    
    for i in 0..64 {
        vec32[i/4] += (vec8[i] as u32) << (3-(i%4))*8;
    }

    vec32
}

fn f()*/
