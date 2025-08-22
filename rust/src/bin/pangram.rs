use std::collections::HashSet;
use std::io::{self, Read};

fn pangram(buf: String) -> String {
    let mut charset = HashSet::new();
    for c in buf.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            charset.insert(c);
        }
    }
    if charset.len() == 26 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", pangram(buf));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(pangram("12 toosmallword".to_string()), "NO");
    }
    #[test]
    fn example_2() {
        assert_eq!(
            pangram("35 TheQuickBrownFoxJumpsOverTheLazyDog".to_string()),
            "YES"
        );
    }
}
