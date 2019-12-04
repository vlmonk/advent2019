#[derive(Debug)]
struct Num([u8; 6]);

impl Num {
    fn from_i32(input: i32) -> Self {
        let d0 = ((input / 100_000) % 10) as u8;
        let d1 = ((input / 10_000) % 10) as u8;
        let d2 = ((input / 1_000) % 10) as u8;
        let d3 = ((input / 100) % 10) as u8;
        let d4 = ((input / 10) % 10) as u8;
        let d5 = (input % 10) as u8;

        Self {
            0: [d0, d1, d2, d3, d4, d5],
        }
    }
}

fn is_2_digit_same(input: &Num) -> bool {
    (0..5).any(|i| input.0[i] == input.0[i + 1])
}

fn is_2_digit_same_advanced(input: &Num) -> bool {
    let input = input.0;

    (0..5).any(|i| match i {
        0 => (input[0] == input[1]) && (input[0] != input[2]),
        4 => (input[4] == input[5]) && (input[4] != input[3]),
        n => (input[n] == input[n + 1]) && (input[n] != input[n - 1]) && (input[n] != input[n + 2]),
    })
}

fn is_increase(input: &Num) -> bool {
    (0..5).all(|i| input.0[i] <= input.0[i + 1])
}

fn main() {
    let from = 137683;
    let to = 596253;

    let total = (from..to + 1)
        .map(Num::from_i32)
        .filter(is_2_digit_same)
        .filter(is_increase)
        .count();

    let total2 = (from..to + 1)
        .map(Num::from_i32)
        .filter(is_2_digit_same_advanced)
        .filter(is_increase)
        .count();

    println!("Q1: {}", total);
    println!("Q2: {}", total2);
}
