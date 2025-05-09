macro_rules! read_stdin {
    ($t:ty) => {{
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

fn main() {
    let mut v = read_stdin!(usize);
    let mut it = v.drain(..);

    let mut s = it.next().unwrap();
    let n = it.next().unwrap();

    let mut dragons: Vec<(usize, usize)> = it
        .by_ref()
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    dragons.sort_by_key(|&(x, _)| x);

    for (dragon_str, bonus) in dragons {
        if s > dragon_str {
            s += bonus;
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
