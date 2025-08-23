use std::io::{self, Read};

fn winner(s: &str) -> &'static str {
    let mut it = s.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();

    if (n.min(m)) % 2 == 1 {
        "Akshat"
    } else {
        "Malvika"
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", winner(&buf));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(winner("2 2"), "Malvika");
    }
    #[test]
    fn ex_2() {
        assert_eq!(winner("2 3"), "Malvika");
    }
    #[test]
    fn ex_3() {
        assert_eq!(winner("3 3"), "Akshat");
    }
}
