use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

#[derive(Debug, Clone)]
struct PartNumber {
    value: u64,
    symbols: Vec<Coord>,
}

#[derive(Debug)]
struct Grid(Vec<Vec<Cell>>);

impl From<&str> for Grid {
    fn from(lines: &str) -> Self {
        Self(lines.lines().map(parse_row).collect::<Vec<Vec<Cell>>>())
    }
}

impl Grid {
    fn adjacent_symbols(&self, check: &[Coord]) -> Option<Vec<Coord>> {
        let mut gears: Vec<Coord> = Vec::new();
        // Bounds checking required.
        for Coord(x, y) in adjacent_cells(check) {
            if let Some(row) = self.0.get(y as usize) {
                if let Some(cell) = row.get(x as usize) {
                    if let Cell::Symbol = cell {
                        gears.push(Coord(x, y));
                    }
                }
            }
        }
        if gears.is_empty() {
            None
        } else {
            Some(gears)
        }
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
                        if let Some(symbols) = self.adjacent_symbols(&potential_part_number_coords)
                        {
                            part_numbers.push(PartNumber {
                                value: potential_part_number
                                    .iter()
                                    .collect::<String>()
                                    .parse::<u64>()
                                    .unwrap(),
                                symbols,
                            });
                        }
                        potential_part_number.clear();
                        potential_part_number_coords.clear();
                    }
                    _ => continue,
                }
            }
            if !potential_part_number.is_empty() {
                if let Some(symbols) = self.adjacent_symbols(&potential_part_number_coords) {
                    part_numbers.push(PartNumber {
                        value: potential_part_number
                            .iter()
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap(),
                        symbols,
                    });
                }
            }
            potential_part_number.clear();
            potential_part_number_coords.clear();
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
    println!("part 2: {}", solve2(input));
}

fn solve1(lines: &str) -> u64 {
    let grid = Grid::from(lines);
    grid.part_numbers()
        .iter()
        .map(|part_number| part_number.value)
        .sum()
}

fn solve2(lines: &str) -> u64 {
    let grid = Grid::from(lines);
    let part_numbers: Vec<PartNumber> = grid.part_numbers();
    let potential_gears: BTreeSet<Coord> = part_numbers
        .iter()
        .cloned()
        .flat_map(|part_number| part_number.symbols)
        .collect();
    let mut gear_ratios: Vec<u64> = Vec::new();
    for gear in potential_gears {
        let matching_part_numbers: Vec<&PartNumber> = part_numbers
            .iter()
            .filter(|part_number| part_number.symbols.contains(&gear))
            .collect();
        if let &[one, two] = matching_part_numbers.as_slice() {
            gear_ratios.push(one.value * two.value);
        }
    }
    gear_ratios.iter().sum()
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

#[test]
fn example02() {
    let example = include_str!("input/day03/example01.txt");
    assert_eq!(solve2(example), 467835);
}
