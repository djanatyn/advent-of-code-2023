use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input/day03/input.txt");
    println!("part 1: {}", solve1(input));
}

fn solve1(lines: &str) -> u64 {
    todo!()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coord(i64, i64);

// orthogonal and diagonal neighbor vectors
const NEIGHBORS: [Coord; 8] = [
    Coord(-1, -1),
    Coord(0, -1),
    Coord(1, -1),
    Coord(-1, 0),
    Coord(1, 0),
    Coord(-1, 1),
    Coord(0, 1),
    Coord(1, 1),
];

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Self::Number(value),
            '.' => Self::Empty,
            _ => Self::Symbol,
        }
    }
}

impl Coord {
    fn add(&self, addend: &Coord) -> Self {
        let Coord(x1, y1) = self;
        let Coord(x2, y2) = addend;
        Coord(x1 + x2, y1 + y2)
    }
}

#[derive(Debug, PartialEq)]
enum Cell {
    Number(char),
    Symbol,
    Empty,
}

fn parse_row<S: AsRef<str>>(line: S) -> Vec<Cell> {
    line.as_ref().chars().map(Into::<Cell>::into).collect()
}

#[derive(Debug)]
struct Grid(Vec<Vec<Cell>>);

fn adjacent_cells(coords: &[Coord]) -> BTreeSet<Coord> {
    let mut adjacent = BTreeSet::new();
    for neighbor in NEIGHBORS {
        for origin in coords {
            adjacent.insert(origin.add(&neighbor));
        }
    }
    adjacent
}

#[test]
fn row() {
    let row = "467..114..";
    use Cell::{Empty, Number, Symbol};
    assert_eq!(
        parse_row(row),
        vec![
            Number('4'),
            Number('6'),
            Number('7'),
            Empty,
            Empty,
            Number('1'),
            Number('1'),
            Number('4'),
            Empty,
            Empty
        ]
    );
}

#[test]
fn example01() {
    let example = include_str!("input/day03/example01.txt");
    assert_eq!(solve1(example), 8);
}
