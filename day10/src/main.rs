use num_rational::Ratio;
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

    pub fn distance_to(&self, other: &Asteroid) -> Option<Vector> {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if (dx == 0 && dy == 0) {
            return None;
        }

        let quadrant = match (dx > 0, dy >= 0) {
            (true, true) => Quadrant::A,
            (false, true) => Quadrant::B,
            (false, false) => Quadrant::C,
            (true, false) => Quadrant::D,
        };

        let (dx, dy) = match quadrant {
            Quadrant::A | Quadrant::C => (dx.abs(), dy.abs()),
            _ => (dy.abs(), dx.abs()),
        };

        let angel = Ratio::new(dy, dy + dx);
        Some(Vector { quadrant, angel })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Quadrant {
    A,
    B,
    C,
    D,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vector {
    quadrant: Quadrant,
    angel: Ratio<i32>,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let field = Asteroid::parse(&input);

    let task_a = field
        .iter()
        .map(|i| {
            let mut total = field
                .iter()
                .filter_map(|j| i.distance_to(j))
                .collect::<Vec<_>>();

            total.sort();
            total.dedup();

            total.len()
        })
        .max();

    dbg!(task_a);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    impl Quadrant {
        fn from_char(input: char) -> Self {
            match input {
                'a' => Self::A,
                'b' => Self::B,
                'c' => Self::C,
                'd' => Self::D,
                _ => panic!("invalid input for Quadrant::from_char"),
            }
        }
    }

    impl Vector {
        fn new(quadrant: char, a: i32, b: i32) -> Self {
            let quadrant = Quadrant::from_char(quadrant);
            Self {
                quadrant,
                angel: Ratio::new(a, b),
            }
        }
    }

    fn distance_to(a_x: i32, a_y: i32, b_x: i32, b_y: i32) -> Option<Vector> {
        let a = Asteroid::new(a_x, a_y);
        let b = Asteroid::new(b_x, b_y);

        a.distance_to(&b)
    }

    #[test]
    fn test_parse() {
        let input = "..#\n#..\n";
        let result = Asteroid::parse(input);
        let expected = vec![Asteroid::new(2, 0), Asteroid::new(0, 1)];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_distance_to() {
        assert_eq!(distance_to(1, 1, 2, 2).unwrap(), Vector::new('a', 1, 2));
        assert_eq!(distance_to(1, 1, 5, 5).unwrap(), Vector::new('a', 1, 2));
        assert_eq!(distance_to(3, 3, 7, 2).unwrap(), Vector::new('d', 4, 5));
        assert_eq!(distance_to(0, 0, 0, 1).unwrap(), Vector::new('b', 0, 1));
        assert_eq!(distance_to(2, 2, 2, 2), None);
    }
}
