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

fn permutation(v: &[i64], n: usize) -> String {
    let n = n as i64;
    let w: Vec<i64> = v.iter().map(|&x| n + 1 - x).collect();
    w.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let t: i64 = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            let x: i64 = sc.next();
            v.push(x);
        }
        println!("{}", permutation(&v, n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(permutation(&[1, 3, 2], 3), "2 3 1");
    }
    #[test]
    fn ex_2() {
        assert_eq!(permutation(&[5, 1, 2, 4, 3], 5), "4 5 1 2 3");
    }
    #[test]
    fn ex_3() {
        assert_eq!(permutation(&[6, 7, 1, 5, 4, 3, 2], 7), "2 1 3 7 5 6 4");
    }
}
