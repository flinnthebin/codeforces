use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!(
        "{}",
        if buf.chars().any(|c| matches!(c, 'H' | 'Q' | '9')) {
            "YES"
        } else {
            "NO"
        }
    );
}
