use std::io::{self, Read};

// 0 = .
// 1 = -.
// 2 = --

fn borze(buf: String) -> String {
    let mut chars = buf.chars().peekable();
    let mut res = String::new();

    while let Some(c) = chars.next() {
        match c {
            '.' => res.push('0'),
            '-' => {
                if chars.peek() == Some(&'.') {
                    chars.next();
                    res.push('1');
                } else {
                    chars.next();
                    res.push('2');
                }
            }
            _ => {}
        }
    }
    res
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", borze(buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(borze(".-.--".to_string()), "012")
    }
    #[test]
    fn example_2() {
        assert_eq!(borze("--.".to_string()), "20")
    }
    #[test]
    fn example_3() {
        assert_eq!(borze("-..-.--".to_string()), "1012")
    }
}
