use std::error::Error;
use std::fs;

fn calc(input: i64) -> i64 {
    input / 3 - 2
}

fn calc2(input: i64) -> i64 {
    let mut total = calc(input);
    let mut current = total;

    loop {
        let step = calc(current);

        if step <= 0 {
            break;
        }

        total += step;
        current = step;
    }

    total
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("input.txt")?;
    let nums: Vec<i64> = data
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();

    let q01 = nums.iter().map(|e| calc(*e)).fold(0, |a, e| a + e);
    let q02 = nums.iter().map(|e| calc2(*e)).fold(0, |a, e| a + e);

    println!("Q01: {}", q01);
    println!("Q02: {}", q02);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(2, calc(12));
        assert_eq!(33583, calc(100756));
    }

    #[test]
    fn test_calc2() {
        assert_eq!(2, calc2(12));
        assert_eq!(966, calc2(1969));
        assert_eq!(50346, calc2(100756));
    }
}
