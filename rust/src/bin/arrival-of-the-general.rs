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

fn line_up(n: usize, sc: &mut Scanner) -> usize {
    let mut v = Vec::<usize>::new();
    for _ in 0..n {
        v.push(sc.next());
    }
    let short = *v.iter().min().unwrap();
    let tall = *v.iter().max().unwrap();

    let short_pos = v.iter().rposition(|&x| x == short).unwrap();
    let tall_pos = v.iter().position(|&x| x == tall).unwrap();

    let mut res = tall_pos + (n - 1 - short_pos);
    if tall_pos > short_pos {
        res -= 1;
    }
    res
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let n: usize = sc.next();
    println!("{}", line_up(n, &mut sc));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        let data = "4 33 44 11 22".to_string();
        let mut sc = Scanner::new(&data);
        let n: usize = sc.next();
        assert_eq!(line_up(n, &mut sc), 2);
    }
    #[test]
    fn ex_2() {
        let data = "7 10 10 58 31 63 40 76".to_string();
        let mut sc = Scanner::new(&data);
        let n: usize = sc.next();
        assert_eq!(line_up(n, &mut sc), 10);
    }
}
