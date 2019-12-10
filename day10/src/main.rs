use std::fs;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn parse(input: &str) -> Vec<Self> {
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, point)| (x, y, point))
            })
            .flatten()
            .filter_map(|info| {
                let (x, y, point) = info;
                match point {
                    '#' => Some(Self::new(x as i32, y as i32)),
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let field = Asteroid::parse(&input);

    dbg!(field);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "..#\n#..\n";
        let result = Asteroid::parse(input);
        let expected = vec![Asteroid::new(2, 0), Asteroid::new(0, 1)];

        assert_eq!(expected, result);
    }
}
