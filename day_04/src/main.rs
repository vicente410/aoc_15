use md5;

fn main() {
    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    println!("{:x}", digest);
    println!("c3fcd3d76192e4007dfb496cca67e13b");
}
