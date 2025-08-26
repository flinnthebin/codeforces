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

fn games(sc: &mut Scanner) -> usize {
    let n: usize = sc.next();
    let mut v = Vec::<(usize, usize)>::new();
    for _ in 0..n {
        let (x, y) = (sc.next(), sc.next());
        v.push((x, y));
    }
    v.iter()
        .map(|&(home, _)| v.iter().filter(|&&(_, away)| home == away).count())
        .sum()
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    println!("{}", games(&mut sc));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        let data = "3 1 2 2 4 3 4".to_string();
        let mut sc = Scanner::new(&data);
        assert_eq!(games(&mut sc), 1);
    }
    #[test]
    fn ex_2() {
        let data = "4 100 42 42 100 5 42 100 5".to_string();
        let mut sc = Scanner::new(&data);
        assert_eq!(games(&mut sc), 5);
    }
    #[test]
    fn ex_3() {
        let data = "2 1 2 1 2".to_string();
        let mut sc = Scanner::new(&data);
        assert_eq!(games(&mut sc), 0);
    }
}
