use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("input/day02/input.txt");
    println!("part 1: {}", solve1(input));
    Ok(())
}

fn solve1(lines: &str) -> u64 {
    todo!();
}

#[derive(Debug, PartialEq)]
enum Cubes {
    Red(u64),
    Green(u64),
    Blue(u64),
}

#[derive(Debug, PartialEq)]
struct Reveal(Vec<Cubes>);

#[derive(Debug, PartialEq)]
struct GameRecord {
    id: u64,
    reveals: Vec<Reveal>,
}

impl GameRecord {
    fn new<S: AsRef<str>>(input: S) -> Self {
        let line = input.as_ref();
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Cubes::{Blue, Green, Red};

    #[test]
    fn parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let record = GameRecord::new(line);
        assert_eq!(
            record,
            GameRecord {
                id: 1,
                reveals: vec![
                    Reveal(vec![Blue(3), Red(4),]),
                    Reveal(vec![Red(1), Green(2),]),
                    Reveal(vec![Blue(6), Green(2)])
                ]
            }
        );
    }

    #[test]
    fn example01() {
        let example = include_str!("input/day01/example01.txt");
        assert_eq!(solve1(example), todo!());
    }
}
