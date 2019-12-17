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
            static ref RE: Regex = Regex::new(r"(.*) => (.*)").unwrap();
        }
        let caps = RE.captures(src).ok_or("input error")?;
        let input_str = caps.get(1).ok_or("input error").map(|r| r.as_str())?;
        let result_str = caps.get(2).ok_or("input error").map(|r| r.as_str())?;

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
    wants: HashMap<String, usize>,
}

impl Lab {
    pub fn parse(src: &str) -> Result<Self> {
        let mut input = HashMap::new();
        for line in src.lines() {
            let reaction = Reaction::parse(line)?;
            input.insert(reaction.to.clone(), ReactionInfo::new(reaction));
        }

        Ok(Self {
            input,
            wants: HashMap::new(),
        })
    }

    pub fn calculate(&mut self, fuel: usize) -> usize {
        self.set_weight("FUEL");
        self.wants.insert("FUEL".into(), fuel);

        loop {
            let (i, total) = self.get_wants();

            if i == "ORE" {
                return total;
            }
            self.wants.remove(&i);
            let info = &self.input.get(&i).expect("A").reaction;
            let multi = (total as f32 / info.to_count as f32).ceil() as usize;
            let more_wants: Vec<_> = info
                .input
                .iter()
                .map(|i| (i.0 * multi, i.1.clone()))
                .collect();
            self.add_wants(more_wants);
        }
    }

    fn set_weight(&mut self, part: &str) -> usize {
        if part == "ORE" {
            return 0;
        }

        let current: Option<usize>;

        let info = self.input.get(part).expect("B");
        if let Some(weight) = info.weight {
            return weight;
        } else {
            let inputs: Vec<_> = info.reaction.input.iter().map(|i| i.1.to_owned()).collect();
            let i: Vec<_> = inputs.into_iter().map(|i| self.set_weight(&i)).collect();
            let max = i.into_iter().max().unwrap_or(0);
            current = Some(max + 1);
        }

        self.input.get_mut(part).map(|v| v.weight = current);
        current.expect("C")
    }

    fn get_wants(&self) -> (String, usize) {
        let mut unsorted: Vec<_> = self
            .wants
            .keys()
            .map(|k| k.to_owned())
            .map(|name| {
                let weight = self
                    .input
                    .get(&name)
                    .and_then(|info| info.weight)
                    .unwrap_or(0);
                (name, weight)
            })
            .collect();

        unsorted.sort_by_key(|i| i.1);
        unsorted.reverse();
        let wants = unsorted.into_iter().next().expect("A3").0;
        let value = *self.wants.get(&wants).expect("A4");
        (wants, value)
    }

    fn add_wants(&mut self, wants: Vec<(usize, String)>) {
        for (a, b) in wants {
            if let Some(c) = self.wants.get_mut(&b) {
                *c += a;
            } else {
                self.wants.insert(b, a);
            }
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut lab = Lab::parse(&input)?;
    let task_a = lab.calculate(1);

    println!("Task I:  {}", task_a);

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
        let result = lab.calculate(1);
        assert_eq!(1, result);
    }
    #[test]
    fn test_lab_b() {
        let mut lab = Lab::parse("10 ORE => 1 FUEL").unwrap();
        let result = lab.calculate(1);
        assert_eq!(10, result);
    }
    #[test]
    fn test_lab_c() {
        let mut lab = Lab::parse("10 ORE => 1 A\n10 A => 1 FUEL").unwrap();
        let result = lab.calculate(1);
        assert_eq!(100, result);
    }
    #[test]
    fn test_lab_d() {
        let mut lab = Lab::parse("10 ORE => 1 A\n10 A => 10 B\n1 A, 1 B => 1 FUEL").unwrap();
        let result = lab.calculate(1);
        assert_eq!(110, result);
    }
    #[test]
    fn test_lab_e() {
        let mut lab = Lab::parse("10 ORE => 10 A\n5 A => 5 B\n1 A, 1 B => 1 FUEL").unwrap();
        let result = lab.calculate(1);
        assert_eq!(11, result);
    }
}
