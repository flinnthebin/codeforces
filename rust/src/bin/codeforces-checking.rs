use std::io::{self, Read};

fn check(s: String) -> String {
    let mut it = s.split_whitespace();
    let t = it.next().unwrap().parse::<usize>().unwrap();
    let mut x = String::new();
    for _ in 0..t {
        let c = it.next().unwrap().parse::<char>().unwrap();
        if matches!(c, 'c' | 'o' | 'd' | 'e' | 'f' | 'r' | 's') {
            x.push_str("YES\n");
        } else {
            x.push_str("NO\n");
        }
    }
    x
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", check(buf));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            check("10 a b c d e f g h i j".to_string()),
            "NO\nNO\nYES\nYES\nYES\nYES\nNO\nNO\nNO\nNO\n"
        );
    }
}
