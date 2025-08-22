use std::io::{self, Read};

enum Polyhedron {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl Polyhedron {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Tetrahedron" => Some(Self::Tetrahedron),
            "Cube" => Some(Self::Cube),
            "Octahedron" => Some(Self::Octahedron),
            "Dodecahedron" => Some(Self::Dodecahedron),
            "Icosahedron" => Some(Self::Icosahedron),
            _ => None,
        }
    }

    fn faces(&self) -> usize {
        match self {
            Self::Tetrahedron => 4,
            Self::Cube => 6,
            Self::Octahedron => 8,
            Self::Dodecahedron => 12,
            Self::Icosahedron => 20,
        }
    }
}

fn faces(buf: String) -> usize {
    let mut it = buf.split_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut count = 0;
    for _ in 0..n {
        let name = it.next().unwrap();
        let poly = Polyhedron::from_str(name).unwrap();
        count += poly.faces();
    }
    count
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", faces(buf));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(
            faces("4 Icosahedron Cube Tetrahedron Dodecahedron".to_string()),
            42
        );
    }
    #[test]
    fn ex_2() {
        assert_eq!(
            faces("3 Dodecahedron Octahedron Octahedron ".to_string()),
            28
        );
    }
}
