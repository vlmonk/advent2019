use std::collections::HashMap;
use std::fs;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
struct OrbitInfo {
    primary: String,
    secondary: String,
}

impl OrbitInfo {
    pub fn parse(input: &str) -> Result<OrbitInfo> {
        let mut parts = input.trim_matches('\n').split(')');
        let primary = parts.next().ok_or("invalid input")?;
        let secondary = parts.next().ok_or("invalid input")?;

        Ok(OrbitInfo::new(primary, secondary))
    }

    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.into(),
            secondary: secondary.into(),
        }
    }
}

#[derive(Debug)]
struct System {
    orbits: HashMap<String, String>,
    distance: HashMap<String, i32>,
}

impl System {
    fn new() -> Self {
        Self {
            orbits: HashMap::new(),
            distance: HashMap::new(),
        }
    }

    fn add(&mut self, info: OrbitInfo) {
        let OrbitInfo { primary, secondary } = info;
        self.orbits.insert(primary, secondary);
    }

    fn update(&mut self, key: String) -> i32 {
        if let Some(d) = self.distance.get(&key) {
            return *d;
        }

        let master = self.orbits.get(&key).map(|k| k.clone());

        if let Some(master) = master {
            let distance = self.update(master.clone()) + 1;
            println!("Set distance {} to {}", master, distance);
            self.distance.insert(key, distance);
            distance
        } else {
            1
        }
    }

    fn update_all(&mut self) {
        let keys = self.orbits.keys().map(|k| k.clone()).collect::<Vec<_>>();
        for k in keys {
            self.update(k);
        }
    }
}

fn main() -> Result<()> {
    let raw = fs::read_to_string("input.txt")?;
    let mut list = raw
        .lines()
        .map(|line| OrbitInfo::parse(line))
        .collect::<Result<Vec<_>>>()?;

    let mut system = System::new();

    list.drain(0..).for_each(|i| {
        system.add(i);
    });

    system.update_all();
    let total: i32 = system.distance.values().sum();
    dbg!(total);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_orbit_info() {
        let input = "COM)B\n";
        let expected = OrbitInfo::new("COM", "B");

        assert_eq!(expected, OrbitInfo::parse(&input).unwrap());
    }
}
