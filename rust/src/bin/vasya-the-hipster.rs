use std::io::{self, Read};

fn socks(a: usize, b: usize) -> (usize, usize) {
    if a > b {
        (b, (a - b) / 2)
    } else {
        (a, (b - a) / 2)
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();

    let a = it.next().unwrap().parse::<usize>().unwrap();
    let b = it.next().unwrap().parse::<usize>().unwrap();

    let (uniq, pair) = socks(a, b);
    println!("{uniq} {pair}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(socks(3, 1), (1, 1));
    }
    #[test]
    fn example_2() {
        assert_eq!(socks(2, 3), (2, 0));
    }
    #[test]
    fn example_3() {
        assert_eq!(socks(7, 3), (3, 2));
    }
}
