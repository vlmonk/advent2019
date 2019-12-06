pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
struct OrbitInfo {
    from: String,
    to: String,
}

impl OrbitInfo {
    pub fn parse(input: &str) -> Result<OrbitInfo> {
        Err("foo".into())
    }

    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

fn main() {
    println!("Hello, world!");
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
