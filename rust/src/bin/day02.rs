use pest::Parser;
use pest_derive::Parser;
use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("input/day02/input.txt");
    println!("part 1: {}", solve1(input));
    println!("part 2: {}", solve2(input));
    Ok(())
}

fn solve1(lines: &str) -> u64 {
    lines
        .lines()
        .map(|line| GameRecord::try_from(line).unwrap())
        .filter(|record| !GameRecord::invalid(record))
        .map(|record| record.id)
        .sum()
}

fn solve2(lines: &str) -> u64 {
    lines
        .lines()
        .map(|line| GameRecord::try_from(line).unwrap())
        .map(FewestCubes::from)
        .map(FewestCubes::power)
        .sum()
}

#[derive(Debug)]
struct FewestCubes {
    red: u64,
    green: u64,
    blue: u64,
}

impl From<GameRecord> for FewestCubes {
    fn from(game: GameRecord) -> Self {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;
        for reveal in game.reveals {
            for cube in reveal.0 {
                match cube {
                    Red(count) => red = red.max(count),
                    Green(count) => green = green.max(count),
                    Blue(count) => blue = blue.max(count),
                }
            }
        }
        FewestCubes { red, green, blue }
    }
}

impl FewestCubes {
    fn power(self) -> u64 {
        self.red * self.green * self.blue
    }
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

impl Cubes {
    const NUM_RED_CUBES: u64 = 12;
    const NUM_GREEN_CUBES: u64 = 13;
    const NUM_BLUE_CUBES: u64 = 14;

    fn invalid(&self) -> bool {
        match self {
            Red(count) => count > &Self::NUM_RED_CUBES,
            Green(count) => count > &Self::NUM_GREEN_CUBES,
            Blue(count) => count > &Self::NUM_BLUE_CUBES,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Reveal(Vec<Cubes>);

#[derive(Debug, PartialEq)]
struct GameRecord {
    id: u64,
    reveals: Vec<Reveal>,
}

impl GameRecord {
    fn invalid(&self) -> bool {
        self.reveals
            .iter()
            .any(|reveal| reveal.0.iter().any(Cubes::invalid))
    }
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
    let example = include_str!("input/day02/example01.txt");
    assert_eq!(solve1(example), 8);
}
