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

fn bitset(n: usize, k: usize, s: String) -> String {
    let mut consecutive: usize = 0;
    let mut zeroes = Vec::new();
    let mut ones = Vec::new();
    for (idx, char) in s.chars().enumerate() {
        if char == '0' {
            zeroes.push(idx);
            consecutive = 0;
        } else {
            ones.push(idx);
            consecutive += 1;
            if consecutive >= k {
                return "NO".to_string();
            }
        }
    }
    let mut permutations = vec![0usize; n];
    let mut countdown = n;
    for i in zeroes {
        permutations[i] = countdown;
        countdown -= 1;
    }
    for i in ones {
        permutations[i] = countdown;
        countdown -= 1;
    }
    let mut result = String::new();
    result.push_str("YES\n");
    for val in &permutations {
        result.push_str(&val.to_string());
        result.push(' ');
    }
    result
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut sc = Scanner::new(&buf);
    let t: usize = sc.next();
    for _ in 0..t {
        let n: usize = sc.next();
        let k: usize = sc.next();
        let s: String = sc.next();
        println!("{}", bitset(n, k, s));
    }
}
