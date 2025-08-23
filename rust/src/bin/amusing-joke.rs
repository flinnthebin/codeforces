use std::io::{self, Read};

fn joke(a: &str, b: &str, c: &str) -> &'static str {
    let mut v: Vec<char> = a.chars().chain(b.chars()).collect();
    let mut w: Vec<char> = c.chars().collect();
    v.sort_unstable();
    w.sort_unstable();
    if v == w { "YES" } else { "NO" }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    let c = it.next().unwrap();
    println!("{}", joke(a, b, c));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(joke("SANTACLAUS", "DEDMOROZ", "SANTAMOROZDEDCLAUS"), "YES");
    }
    #[test]
    fn ex_2() {
        assert_eq!(joke("PAPAINOEL", "JOULUPUKKI", "JOULNAPAOILELUPUKKI"), "NO");
    }
    #[test]
    fn ex_3() {
        assert_eq!(
            joke(
                "BABBONATALE",
                "FATHERCHRISTMAS",
                "BABCHRISTMASBONATALLEFATHER"
            ),
            "NO"
        );
    }
}
