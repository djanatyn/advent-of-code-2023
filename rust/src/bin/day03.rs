use std::collections::BTreeSet;

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

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Self::Number(value),
            '.' => Self::Empty,
            _ => Self::Symbol,
        }
    }
}

#[derive(Debug)]
struct PartNumber(u64);

#[derive(Debug)]
struct Grid(Vec<Vec<Cell>>);

impl From<&str> for Grid {
    fn from(lines: &str) -> Self {
        Self(lines.lines().map(parse_row).collect::<Vec<Vec<Cell>>>())
    }
}

impl Grid {
    /// Bounds checking required.
    fn validate(&self, check: &[Coord]) -> bool {
        adjacent_cells(check).iter().any(|Coord(x, y)| {
            if let Some(row) = self.0.get(*y as usize) {
                if let Some(cell) = row.get(*x as usize) {
                    return matches!(cell, Cell::Symbol);
                }
            }
            return false;
        })
    }

    fn part_numbers(&self) -> Vec<PartNumber> {
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        for (y, row) in self.0.iter().enumerate() {
            let mut potential_part_number: Vec<char> = Vec::new();
            let mut potential_part_number_coords: Vec<Coord> = Vec::new();
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Number(value) => {
                        potential_part_number.push(*value);
                        potential_part_number_coords.push(Coord(x as i64, y as i64));
                    }
                    Cell::Empty | Cell::Symbol if !potential_part_number.is_empty() => {
                        if self.validate(&potential_part_number_coords) {
                            part_numbers.push(PartNumber(
                                potential_part_number
                                    .iter()
                                    .collect::<String>()
                                    .parse::<u64>()
                                    .unwrap(),
                            ));
                        }
                        potential_part_number.clear();
                        potential_part_number_coords.clear();
                    }
                    _ => continue,
                }
            }
            if !potential_part_number.is_empty() && self.validate(&potential_part_number_coords) {
                part_numbers.push(PartNumber(
                    potential_part_number
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap(),
                ));
                potential_part_number.clear();
                potential_part_number_coords.clear();
            }
        }
        part_numbers
    }
}

fn parse_row<S: AsRef<str>>(line: S) -> Vec<Cell> {
    line.as_ref().chars().map(Into::<Cell>::into).collect()
}

/// May return invalid bounds.
fn adjacent_cells(coords: &[Coord]) -> BTreeSet<Coord> {
    let mut adjacent = BTreeSet::new();
    for neighbor in NEIGHBORS {
        for origin in coords {
            adjacent.insert(origin.add(&neighbor));
        }
    }
    adjacent
}

fn main() {
    let input = include_str!("input/day03/input.txt");
    println!("part 1: {}", solve1(input));
}

fn solve1(lines: &str) -> u64 {
    let grid = Grid::from(lines);
    grid.part_numbers()
        .iter()
        .map(|part_number| part_number.0)
        .sum()
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
    assert_eq!(solve1(example), 4361);
}
