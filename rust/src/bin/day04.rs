use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bin/day04.pest"]
struct CardsParser;

#[derive(Debug)]
struct Card {
    id: u64,
    winning: Vec<u64>,
    yours: Vec<u64>,
}

#[derive(Debug)]
struct Cards(Vec<Card>);

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut card = CardsParser::parse(Rule::input, line)
            .map_err(|e| e.to_string())?
            .next()
            .ok_or("no card found")?
            .into_inner();
        let id = card
            .next()
            .ok_or("missing header")?
            .into_inner()
            .next()
            .ok_or("missing id")?
            .as_str()
            .parse::<u64>()
            .map_err(|e| e.to_string())?;
        let winning_numbers = card.next().ok_or("missing winning")?;
        let your_numbers = card.next().ok_or("missing yours")?;
        let winning: Vec<u64> = Vec::new();
        let winning: Vec<u64> = winning_numbers
            .into_inner()
            .map(|number| {
                number
                    .as_str()
                    .trim()
                    .parse::<u64>()
                    .map_err(|e| e.to_string())
            })
            .collect::<Result<Vec<u64>, String>>()?;
        let yours: Vec<u64> = your_numbers
            .into_inner()
            .map(|number| {
                number
                    .as_str()
                    .trim()
                    .parse::<u64>()
                    .map_err(|e| e.to_string())
            })
            .collect::<Result<Vec<u64>, String>>()?;
        Ok(Card { id, winning, yours })
    }
}

impl TryFrom<&str> for Cards {
    type Error = String;

    fn try_from(lines: &str) -> Result<Self, Self::Error> {
        Ok(Cards(
            lines
                .lines()
                .map(|line| Card::try_from(line))
                .collect::<Result<Vec<Card>, String>>()?,
        ))
    }
}

fn main() {
    let input = include_str!("input/day04/input.txt");
    println!("part 1: {}", solve1(input));
    println!("part 2: {}", solve2(input));
}

fn solve1(lines: &str) -> u64 {
    let cards = Cards::try_from(lines).unwrap();
    dbg!(cards);
    todo!()
}

fn solve2(lines: &str) -> u64 {
    todo!()
}

#[test]
fn example01() -> Result<(), String> {
    let example = include_str!("input/day04/example01.txt");
    let cards = Cards::try_from(example)?;
    dbg!(cards);
    Ok(assert_eq!(solve1(example), 13))
}

#[test]
fn example02() {
    let example = include_str!("input/day04/example01.txt");
    // assert_eq!(solve2(example), 467835);
}
