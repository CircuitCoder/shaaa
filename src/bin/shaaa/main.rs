use shaaa::*;

fn main() {
    let mut s = Shaaa256::new();
    s.update("abcde".as_bytes());

    let dig = s.digest();

    for c in dig.iter() {
        print!("{:0>2x}", c);
    }

    println!("");
}
