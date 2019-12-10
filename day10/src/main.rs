use num_rational::Ratio;
use rayon::prelude::*;
use std::fs;
use std::time::Instant;

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

        if dx == 0 && dy == 0 {
            return None;
        }

        let quadrant = match (dx, dy) {
            (dx, dy) if dx >= 0 && dy < 0 => Quadrant::A,
            (dx, dy) if dx > 0 && dy >= 0 => Quadrant::B,
            (dx, dy) if dx <= 0 && dy > 0 => Quadrant::C,
            (_, _) => Quadrant::D,
        };

        let (dx, dy) = match quadrant {
            Quadrant::A | Quadrant::C => (dx.abs(), dy.abs()),
            _ => (dy.abs(), dx.abs()),
        };

        let angel = Ratio::new(dx, dy + dx);
        Some(Vector { quadrant, angel })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Quadrant {
    A,
    B,
    C,
    D,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Vector {
    quadrant: Quadrant,
    angel: Ratio<i32>,
}

struct TaskA<'a> {
    field: &'a [Asteroid],
}

impl<'a> TaskA<'a> {
    pub fn solve(&self) -> Option<(usize, i32, i32)> {
        self.field
            .par_iter()
            .map(|i| {
                let mut total = self
                    .field
                    .iter()
                    .filter_map(|j| i.distance_to(j))
                    .collect::<Vec<_>>();

                total.sort();
                total.dedup();

                (total.len(), i.x, i.y)
            })
            .max_by_key(|(len, _, _)| *len)
    }

    pub fn new(field: &'a [Asteroid]) -> Self {
        Self { field }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Active,
    Vaporized,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AsteroidInfo {
    vector: Vector,
    distance: i32,
    state: State,
    x: i32,
    y: i32,
}

impl AsteroidInfo {
    pub fn active(&self) -> bool {
        self.state == State::Active
    }

    pub fn vaporize(&mut self) {
        self.state = State::Vaporized;
    }
}

struct LaserIter {
    field: Vec<AsteroidInfo>,
    last_vector: Option<Vector>,
}

impl LaserIter {
    pub fn new(input: &[Asteroid], x: i32, y: i32) -> Self {
        let origin = Asteroid { x: x, y: y };
        let mut field = input
            .iter()
            .filter_map(|a| {
                let vector = origin.distance_to(a);
                match vector {
                    Some(vector) => {
                        let distance = (origin.x - a.x).abs() + (origin.y - a.y).abs();

                        Some(AsteroidInfo {
                            vector,
                            distance,
                            state: State::Active,
                            x: a.x,
                            y: a.y,
                        })
                    }
                    None => None,
                }
            })
            .collect::<Vec<_>>();

        field.sort();

        Self {
            field,
            last_vector: None,
        }
    }
}

impl Iterator for LaserIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.last_vector.as_ref() {
            let mut next_from_current = self
                .field
                .iter_mut()
                .filter(|i| i.active())
                .filter(|i| i.vector > *v);

            if let Some(i) = next_from_current.next().as_mut() {
                self.last_vector = Some(i.vector.clone());
                i.vaporize();
                Some((i.x, i.y))
            } else {
                let mut next_from_start = self.field.iter_mut().filter(|i| i.active());
                if let Some(i) = next_from_start.next().as_mut() {
                    self.last_vector = Some(i.vector.clone());
                    i.vaporize();
                    Some((i.x, i.y))
                } else {
                    None
                }
            }
        } else {
            match self.field.iter_mut().next().as_mut() {
                Some(i) => {
                    self.last_vector = Some(i.vector.clone());
                    i.vaporize();
                    Some((i.x, i.y))
                }
                None => None,
            }
        }
    }
}

struct TaskB {
    iter: LaserIter,
}

impl TaskB {
    pub fn new(field: &[Asteroid], x: i32, y: i32) -> Self {
        let iter = LaserIter::new(field, x, y);
        Self { iter }
    }

    pub fn solve(&mut self) -> Option<(i32, i32)> {
        self.iter.nth(199)
    }
}

fn main() -> Result<()> {
    let now = Instant::now();

    let input = fs::read_to_string("input.txt")?;
    let field = Asteroid::parse(&input);
    let field_size = field.len();

    let task_a = TaskA::new(&field).solve().unwrap();
    let task_b = TaskB::new(&field, task_a.1, task_a.2).solve().unwrap();

    let total_time = now.elapsed();

    println!("Total asteroids: {}", field_size);
    println!("Task I :  {} (x: {}, y: {})", task_a.0, task_a.1, task_a.2);
    println!(
        "Task II:  {} (x: {}, y: {})",
        task_b.0 * 100 + task_b.1,
        task_b.0,
        task_b.1
    );
    println!("Total time: {}μs", total_time.as_micros());
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
        assert_eq!(distance_to(3, 3, 4, 2).unwrap(), Vector::new('a', 1, 2));
        assert_eq!(distance_to(3, 3, 7, 3).unwrap(), Vector::new('b', 0, 1));
        assert_eq!(distance_to(3, 3, 0, 1).unwrap(), Vector::new('d', 2, 5));
        assert_eq!(distance_to(2, 2, 2, 2), None);
    }

    #[test]
    fn test_distance_align() {
        assert_eq!(distance_to(0, 0, 0, -10).unwrap(), Vector::new('a', 0, 1));
        assert_eq!(distance_to(0, 0, 10, 0).unwrap(), Vector::new('b', 0, 1));
        assert_eq!(distance_to(0, 0, 0, 10).unwrap(), Vector::new('c', 0, 1));
        assert_eq!(distance_to(0, 0, -10, 0).unwrap(), Vector::new('d', 0, 1));
    }

    #[test]
    fn test_laser_iter_empty() {
        let mut iter = LaserIter::new(&vec![], 0, 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_laser_iter() {
        let field = Asteroid::parse("##.\n.#.\n...\n");
        let mut iter = LaserIter::new(&field, 1, 2);

        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), None);
    }
}
