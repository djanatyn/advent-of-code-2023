use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("input/day01/input.txt");
    Ok(println!("{}", input))
}

struct CalibrationDigits(char, char);

fn find_digits(line: &str) -> CalibrationDigits {
    let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    CalibrationDigits(*digits.first().unwrap(), *digits.last().unwrap())
}

impl CalibrationDigits {
    fn combine(self) -> u64 {
        let CalibrationDigits(first, last) = self;
        return format!("{first}{last}").parse::<u64>().unwrap();
    }
}

#[test]
fn example01() {
    let example = include_str!("input/day01/example01.txt");
    let result: u64 = example
        .lines()
        .map(|line| find_digits(line))
        .map(CalibrationDigits::combine)
        .sum();
    assert_eq!(result, 142);
}
