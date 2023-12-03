use pest::Parser;
use pest_derive::Parser;
use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("input/day02/input.txt");
    println!("part 1: {}", solve1(input));
    Ok(())
}

fn solve1(lines: &str) -> u64 {
    todo!();
}

#[derive(Parser)]
#[grammar = "bin/day02.pest"]
struct GameParser;

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

impl TryFrom<&str> for GameRecord {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut game = GameParser::parse(Rule::game, line)
            .map_err(|e| e.to_string())?
            .next()
            .ok_or("no games found")?
            .into_inner();
        let id = game
            .next()
            .ok_or("missing id")?
            .as_str()
            .parse::<u64>()
            .unwrap();
        let mut reveals = Vec::new();
        for reveal in game {
            let mut cubes = Vec::new();
            for show in reveal.into_inner() {
                let mut children = show.into_inner();
                let count = children
                    .next()
                    .ok_or("missing count")?
                    .as_str()
                    .parse::<u64>()
                    .map_err(|e| e.to_string())?;
                let color = match children.next().unwrap().as_str() {
                    "red" => Cubes::Red(count),
                    "green" => Cubes::Green(count),
                    "blue" => Cubes::Blue(count),
                    _ => Err("invalid color")?,
                };
                cubes.push(color);
            }
            reveals.push(Reveal(cubes));
        }
        Ok(Self { id, reveals })
    }
}

use Cubes::{Blue, Green, Red};

#[test]
fn parse_line() {
    let line = "Game 123: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let record = GameRecord::try_from(line).unwrap();
    assert_eq!(
        record,
        GameRecord {
            id: 123,
            reveals: vec![
                Reveal(vec![Blue(3), Red(4)]),
                Reveal(vec![Red(1), Green(2), Blue(6)]),
                Reveal(vec![Green(2)])
            ]
        }
    );
}

#[test]
fn example01() {
    let example = include_str!("input/day01/example01.txt");
    assert_eq!(solve1(example), todo!());
}
