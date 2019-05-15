use shaaa::Shaaa512;

fn main() {
    let mut s = Shaaa512::new();
    s.update("abcde".as_bytes());

    let dig = s.digest();

    for c in dig.iter() {
        print!("{:x}", c);
    }

    println!("");
}
