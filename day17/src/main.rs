mod vm;
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Item {
    Scaffold,
    Empty,
    Robot(Direction),
}

trait Storage {
    fn add(&mut self, item: Item);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

struct Field {}


impl Field {

}

'foo'

impl Storage for Field {
    fn new() -> Self {
        Self {}
    }
    fn add(&mut self, item: Item) {}
    fn width(&self) -> usize {
        0
    }
    fn height(&self) -> usize {
        0
    }
}

fn main() {
    println!("hi man!!!!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_size_empty() {
        let field = Field::new()
    }
}
