use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("input/day01/input.txt");
    println!("part 1: {}", solve1(input));
    println!("part 2: {}", solve2(input));
    Ok(())
}

fn solve1(lines: &str) -> u64 {
    lines
        .lines()
        .map(|line| find_digits(line))
        .map(CalibrationDigits::combine)
        .sum()
}

fn solve2(lines: &str) -> u64 {
    lines
        .lines()
        .map(|line| find_matches(line))
        .map(CalibrationDigits::combine)
        .sum()
}

#[derive(Debug, PartialEq)]
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

const REPLACEMENTS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

struct Match {
    index: usize,
    value: char,
}

pub struct Matches(Vec<Match>);

fn find_matches<S: AsRef<str>>(line: S) -> CalibrationDigits {
    let line = line.as_ref();
    let mut matches = vec![];
    // find matches for every word
    for (pattern, value) in REPLACEMENTS {
        let mut found: Vec<Match> = line
            .match_indices(pattern)
            .map(|(index, _)| Match { index, value })
            .collect();
        matches.append(&mut found);
    }
    // find matches for all digits
    let mut digit_matches: Vec<Match> = line
        .match_indices(|c: char| c.is_ascii_digit())
        .map(|(index, value)| Match {
            index,
            value: value.chars().nth(0).unwrap(),
        })
        .collect();
    matches.append(&mut digit_matches);
    Matches(matches).calibration_digits()
}

impl Matches {
    fn calibration_digits(&mut self) -> CalibrationDigits {
        self.0
            .sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());
        CalibrationDigits(self.0.first().unwrap().value, self.0.last().unwrap().value)
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

#[test]
fn example02() {
    let example = include_str!("input/day01/example02.txt");
    assert_eq!(solve2(example), 281);
}
