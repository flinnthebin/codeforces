use std::io::{self, Read};

fn diff(n: usize, puzzles: &[u32]) -> u32 {
    let mut v = puzzles.to_vec();
    v.sort();
    v.windows(n).map(|w| w[n - 1] - w[0]).min().unwrap()
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<u32>().unwrap();
    let mut v = Vec::new();
    for _ in 0..m {
        let x = it.next().unwrap().parse::<u32>().unwrap();
        v.push(x);
    }
    println!("{}", diff(n, &v));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(diff(4, &[10, 12, 10, 7, 5, 22]), 5);
    }
}
