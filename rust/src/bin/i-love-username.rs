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

macro_rules! chmin {
    ($a:expr, $b:expr) => {{
        let b = $b;
        if b < $a {
            $a = b;
            true
        } else {
            false
        }
    }};
}

macro_rules! chmax {
    ($a:expr, $b:expr) => {{
        let b = $b;
        if b > $a {
            $a = b;
            true
        } else {
            false
        }
    }};
}

fn performance(n: usize, sc: &mut Scanner) -> usize {
    let mut count: usize = 0;
    let first: usize = sc.next();
    let mut worst = first;
    let mut best = first;
    for _ in 0..n - 1 {
        let x: usize = sc.next();
        if chmin!(worst, x) {
            count += 1;
        }
        if chmax!(best, x) {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let n: usize = sc.next();
    println!("{}", performance(n, &mut sc));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        let data = "5 100 50 200 150 200".to_string();
        let mut sc = Scanner::new(&data);
        let n: usize = sc.next();
        assert_eq!(performance(n, &mut sc), 2);
    }
    #[test]
    fn ex_2() {
        let data = "10 4664 6496 5814 7010 5762 5736 6944 4850 3698 7242".to_string();
        let mut sc = Scanner::new(&data);
        let n: usize = sc.next();
        assert_eq!(performance(n, &mut sc), 4);
    }
}
