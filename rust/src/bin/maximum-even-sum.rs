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

fn max_even_sum(a: i64, b: i64) -> i64 {
    if b & 1 == 1 {
        if a & 1 == 0 { -1 } else { a * b + 1 }
    } else {
        let mut count = 0;
        let mut x = b;
        while x & 1 == 0 {
            x /= 2;
            count += 1;
        }
        if count == 1 && (a & 1 == 1) {
            -1
        } else {
            a * (b / 2) + 2
        }
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let t: i64 = sc.next();
    for _ in 0..t {
        let a: i64 = sc.next();
        let b: i64 = sc.next();
        println!("{}", max_even_sum(a, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(max_even_sum(8, 1), -1);
    }
    #[test]
    fn ex_2() {
        assert_eq!(max_even_sum(1, 8), 6);
    }
    #[test]
    fn ex_3() {
        assert_eq!(max_even_sum(7, 7), 50);
    }
    #[test]
    fn ex_4() {
        assert_eq!(max_even_sum(2, 6), 8);
    }
    #[test]
    fn ex_5() {
        assert_eq!(max_even_sum(9, 16), 74);
    }
    #[test]
    fn ex_6() {
        assert_eq!(max_even_sum(1, 6), -1);
    }
    #[test]
    fn ex_7() {
        assert_eq!(max_even_sum(4, 6), 14);
    }
}
