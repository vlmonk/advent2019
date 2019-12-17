use lazy_static::lazy_static;
use regex::Regex;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
struct Reaction {
    from: String,
    to: String,
    from_count: usize,
    to_count: usize,
}

impl Reaction {
    pub fn new<T>(from_count: usize, from: T, to_count: usize, to: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            from_count,
            to_count,
            from: from.into(),
            to: to.into(),
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) ([A-Z]+) => (\d+) ([A-Z]+)").unwrap();
        }

        let caps = RE.captures(input).ok_or("input error")?;
        let parts = (1..5)
            .map(|i| caps.get(i).map(|p| p.as_str()).ok_or("input_error".into()))
            .collect::<Result<Vec<_>>>()?;

        let from_count = parts[0].parse::<usize>()?;
        let to_count = parts[2].parse::<usize>()?;

        Ok(Self::new(from_count, parts[1], to_count, parts[3]))
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_reaction() {
        let reaction = Reaction::parse(" 10 ORE => 10 A \n").unwrap();
        let expected = Reaction::new(10, "ORE", 10, "A");

        assert_eq!(reaction, expected);
    }
}
