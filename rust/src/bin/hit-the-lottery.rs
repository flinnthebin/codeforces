use std::io::{self, Read};

fn num_bills(mut n: usize) -> usize {
    let mut count = 0;
    for d in [100, 20, 10, 5, 1] {
        count += n / d;
        n %= d;
    }
    count
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    println!("{}", num_bills(n));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(num_bills(125), 3);
    }
    #[test]
    fn ex_2() {
        assert_eq!(num_bills(43), 5);
    }
    #[test]
    fn ex_3() {
        assert_eq!(num_bills(1000000000), 10000000);
    }
}
