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
        .map(|line| dbg!(translate_words(line)))
        .map(|line| find_digits(line.as_str()))
        .map(CalibrationDigits::combine)
        .sum()
}

#[derive(Debug, PartialEq)]
struct CalibrationDigits(char, char);

fn find_digits(line: &str) -> CalibrationDigits {
    let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    dbg!(CalibrationDigits(
        *digits.first().unwrap(),
        *digits.last().unwrap()
    ))
}

impl CalibrationDigits {
    fn combine(self) -> u64 {
        let CalibrationDigits(first, last) = self;
        return format!("{first}{last}").parse::<u64>().unwrap();
    }
}

const REPLACEMENTS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn translate_words<S: AsRef<str>>(line: S) -> String {
    let line = line.as_ref();
    let replacement = REPLACEMENTS.iter().fold(
        None,
        |acc: Option<(usize, &str, &str)>, (find, replace): &(&str, &str)| {
            let idx = match line.find(find) {
                None => return acc,
                Some(idx) => idx,
            };
            match acc {
                None => Some((idx, find, replace)),
                Some((previous_idx, _, _)) if idx < previous_idx => Some((idx, find, replace)),
                Some((_, _, _)) => acc,
            }
        },
    );
    match replacement {
        None => line.to_string(),
        Some((_, find, replace)) => translate_words(line.replacen(find, replace, 1).as_str()),
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
    let translation: Vec<CalibrationDigits> = example
        .lines()
        .map(|line| translate_words(line))
        .map(|line| find_digits(line.as_str()))
        .collect();
    assert_eq!(
        translation,
        vec![
            CalibrationDigits('2', '9'),
            CalibrationDigits('8', '3'),
            CalibrationDigits('1', '3'),
            CalibrationDigits('2', '4'),
            CalibrationDigits('4', '2'),
            CalibrationDigits('1', '4'),
            CalibrationDigits('7', '6'),
        ]
    );
    let result: u64 = translation
        .into_iter()
        .map(CalibrationDigits::combine)
        .sum();
    assert_eq!(result, 281);
}
