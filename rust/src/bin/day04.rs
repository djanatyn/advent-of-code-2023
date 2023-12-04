use std::collections::HashMap;

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

impl Card {
    fn matches(&self) -> u64 {
        self.yours
            .iter()
            .filter(|yours| self.winning.contains(yours))
            .count() as u64
    }

    fn points(&self) -> u64 {
        let mut points = 0;
        for iteration in 0..self.matches() {
            match iteration {
                0 => points = 1,
                _ => points = points * 2,
            }
        }
        points
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
    cards.0.iter().map(|card| Card::points(card)).sum()
}

fn solve2(lines: &str) -> u64 {
    let cards = Cards::try_from(lines).unwrap();
    let num_cards = cards.0.len();
    // set all copies to 1
    let mut copies: HashMap<u64, u64> = (1..=num_cards)
        .zip(std::iter::repeat(1))
        .map(|(id, copies)| (id as u64, copies as u64))
        .collect();
    for card in cards.0 {
        let current_copies = copies.get(&card.id).unwrap().clone();
        // increase all subsequent cards by number of copies of current card
        for offset in 1..=card.matches() {
            copies
                .entry(card.id + offset)
                .and_modify(|copies| *copies += current_copies);
        }
    }
    copies.values().sum()
}

#[test]
fn example01() {
    let example = include_str!("input/day04/example01.txt");
    assert_eq!(solve1(example), 13)
}

#[test]
fn example02() {
    let example = include_str!("input/day04/example01.txt");
    assert_eq!(solve2(example), 30);
}
