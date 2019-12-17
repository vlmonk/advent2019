use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

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
        let caps = RE.captures(src).ok_or(format!("input error: {}", src))?;
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
        for line in src.lines().filter(|l| l.len() > 0) {
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
    let now = Instant::now();

    let input = fs::read_to_string("input.txt")?;
    let mut lab = Lab::parse(&input)?;
    let task_a = lab.calculate(1);

    let total_time = now.elapsed();

    println!("Task I:  {}", task_a);
    println!("Total time: {}μs", total_time.as_micros());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_13312() -> &'static str {
        r#"
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
"#
    }
    fn example_180697() -> &'static str {
        r#"
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
"#
    }

    fn example_2210736() -> &'static str {
        r#"
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
"#
    }

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
        assert_eq!(10, result);
    }
    #[test]
    fn test_lab_13312() {
        let mut lab = Lab::parse(example_13312()).unwrap();
        let result = lab.calculate(1);
        assert_eq!(13312, result);
    }

    #[test]
    fn test_lab_180697() {
        let mut lab = Lab::parse(example_180697()).unwrap();
        let result = lab.calculate(1);
        assert_eq!(180697, result);
    }
    #[test]
    fn test_lab_2210736() {
        let mut lab = Lab::parse(example_2210736()).unwrap();
        let result = lab.calculate(1);
        assert_eq!(2210736, result);
    }
}
