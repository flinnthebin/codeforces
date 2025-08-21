use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let (first, word) = buf.split_at(1);

    if buf == buf.to_uppercase() {
        println!("{}", buf.to_lowercase());
    } else if first == first.to_lowercase() && word == word.to_uppercase() {
        println!("{}{}", first.to_uppercase(), word.to_lowercase());
    } else {
        println!("{}", buf);
    }
}
