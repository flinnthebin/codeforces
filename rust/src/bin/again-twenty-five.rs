use std::io::{self, Read};

fn power(_s: String) -> usize {
    25
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", power(buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex() {
        assert_eq!(power("2".to_string()), 25)
    }
}
