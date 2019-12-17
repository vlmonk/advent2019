use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
struct Reaction {
    input: Vec<(usize, String)>,
    to: String,
    to_count: usize,
}

impl Reaction {
    pub fn parse(src: &str) -> Result<Self> {
        lazy_static! {
            // static ref RE: Regex = Regex::new(r"(\d+) ([A-Z]+) => (\d+) ([A-Z]+)").unwrap();
            static ref RE: Regex = Regex::new(r"(.*) => (.*)").unwrap();
        }
        let caps = RE.captures(src).ok_or("input error")?;
        let input_str = caps.get(1).ok_or("input error").map(|r| r.as_str())?;
        let result_str = caps.get(2).ok_or("input error").map(|r| r.as_str())?;

        dbg!(input_str);
        dbg!(result_str);

        let input = Reaction::parse_input(input_str)?;
        let result = Reaction::parse_part(result_str)?;

        Ok(Self {
            input,
            to: result.1,
            to_count: result.0,
        })
    }

    fn parse_input(src: &str) -> Result<Vec<(usize, String)>> {
        src.split(',')
            .map(|part| Reaction::parse_part(part))
            .collect()
    }

    fn parse_part(src: &str) -> Result<(usize, String)> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
        }
        let caps = RE.captures(src).ok_or("input error")?;
        let count = caps
            .get(1)
            .and_then(|input| input.as_str().parse::<usize>().ok())
            .ok_or("input error")?;

        let name = caps
            .get(2)
            .map(|input| input.as_str().to_owned())
            .ok_or("input error")?;

        Ok((count, name))
    }
}

#[derive(Debug)]
struct ReactionInfo {
    reaction: Reaction,
    weight: Option<usize>,
}

impl ReactionInfo {
    pub fn new(reaction: Reaction) -> Self {
        Self {
            reaction: reaction,
            weight: None,
        }
    }
}

#[derive(Debug)]
struct Lab {
    input: HashMap<String, ReactionInfo>,
}

impl Lab {
    pub fn parse(src: &str) -> Result<Self> {
        let mut input = HashMap::new();
        for line in src.lines() {
            let reaction = Reaction::parse(line)?;
            input.insert(reaction.to.clone(), ReactionInfo::new(reaction));
        }

        Ok(Self { input })
    }

    pub fn calculate(&mut self) -> usize {
        // self.set_weight();

        2
    }

    // fn set_weight() {}
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let lab = Lab::parse(&input)?;

    dbg!(lab);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_reaction() {
        let reaction = Reaction::parse(" 10 ORE, 1 C => 10 A \n").unwrap();
        let expected = Reaction {
            input: vec![(10, "ORE".to_owned()), (1, "C".to_owned())],
            to: "A".to_owned(),
            to_count: 10,
        };

        assert_eq!(reaction, expected);
    }

    #[test]
    fn test_lab_a() {
        let mut lab = Lab::parse("1 ORE => 1 FUEL").unwrap();
        let result = lab.calculate();
        assert_eq!(1, result);
    }
}
