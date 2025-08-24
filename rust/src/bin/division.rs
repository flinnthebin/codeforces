use std::io::{self, Read};

struct Scanner<'a> {
    it: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            it: s.split_ascii_whitespace(),
        }
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse().ok().unwrap()
    }
}

fn division(rating: i32) -> u32 {
    match rating {
        i32::MIN..=1399 => 4,
        1400..=1599 => 3,
        1600..=1899 => 2,
        1900..=i32::MAX => 1,
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let n: u32 = sc.next();
    for _ in 0..n {
        let x: i32 = sc.next();
        println!("Division {}", division(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(division(-789), 4);
    }
    #[test]
    fn ex_2() {
        assert_eq!(division(1299), 4);
    }
    #[test]
    fn ex_3() {
        assert_eq!(division(1300), 4);
    }
    #[test]
    fn ex_4() {
        assert_eq!(division(1399), 4);
    }
    #[test]
    fn ex_5() {
        assert_eq!(division(1400), 3);
    }
    #[test]
    fn ex_6() {
        assert_eq!(division(1679), 2);
    }
    #[test]
    fn ex_7() {
        assert_eq!(division(2300), 1);
    }
}
