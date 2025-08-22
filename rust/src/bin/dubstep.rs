use std::io::{self, Read};

fn dubstep(s: String) -> String {
    let mut res = String::new();

    for word in s.split("WUB") {
        if !word.is_empty() {
            if !res.is_empty() {
                res.push(' ');
            }
            res.push_str(word);
        }
    }

    res
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let res = dubstep(buf);
    println!("{res}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(dubstep("WUBWUBABCWUB".to_string()), "ABC");
    }
    #[test]
    fn example_2() {
        assert_eq!(
            dubstep("WUBWEWUBAREWUBWUBTHEWUBCHAMPIONSWUBMYWUBFRIENDWUB".to_string()),
            "WE ARE THE CHAMPIONS MY FRIEND"
        );
    }
}
