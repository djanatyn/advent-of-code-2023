//! TODO: instead of operating on individual seeds, operate on ranges of seeds
//! TODO: when given a range of seeds, first figure out which maps apply
//! TODO: when you understand all maps that apply, fold over them (creating new ranges)
//! TODO: can we collapse the maps? what does the one-pass map look like?

use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "bin/day05.pest"]
struct InputParser;

#[derive(Debug)]
struct Seed(i64);

#[derive(Debug)]
struct Range {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

impl Range {
    fn map(&self, value: i64) -> Option<i64> {
        let start = self.source_start;
        let end = self.source_start + (self.range_length) - 1;

        if start <= value && value <= end {
            let offset = value - self.source_start;
            Some(self.destination_start + offset)
        } else {
            None
        }
    }

    // Check a value range for overlap.
    //
    // Returns None for no overlap (meaning no modification to the range).
    // Returns a new Vec<ValueRange>> for overlap (containing updated ranges,
    // with offset applied).
    fn map_value_range(&self, value_range: &ValueRange, to: Kind) -> Option<Vec<ValueRange>> {
        let range_end: i64 = (self.source_start + self.range_length - 1);
        let values_end: i64 = value_range.start + value_range.length - 1;
        // the values end within the range
        let right_overlap: bool =
            value_range.start <= self.source_start && values_end >= self.source_start;
        // the values start within the range
        let left_overlap: bool = value_range.start <= range_end && values_end >= range_end;
        // the values are a subset of the range
        let within_range: bool = value_range.start > self.source_start && values_end < range_end;
        // the values contain the range
        let total_overlap: bool = right_overlap && left_overlap;
        if total_overlap {
            // TODO: check for left and right remaining
            return Some(vec![ValueRange {
                start: self.destination_start,
                length: value_range.length,
                kind: to,
            }]);
        }
        if within_range {
            let start_offset = value_range.start - self.source_start;
            // TODO: off-by-one?
            return Some(vec![ValueRange {
                start: self.destination_start + start_offset,
                length: range_end - values_end,
                kind: to,
            }]);
        }
        if left_overlap {
            let start_offset = value_range.start - self.source_start;
            // TODO: off-by-one?
            let remaining = ValueRange {
                start: range_end,
                length: values_end - range_end,
                kind: value_range.kind,
            };
            let overlap = ValueRange {
                start: self.destination_start + start_offset,
                length: values_end - value_range.start,
                kind: to,
            };
            return Some(vec![remaining, overlap]);
        }
        if right_overlap {
            let start_offset = value_range.start - self.source_start;
            // TODO: off-by-one?
            let remaining = ValueRange {
                start: value_range.start,
                length: self.source_start - value_range.start,
                kind: value_range.kind,
            };
            let overlap = ValueRange {
                start: self.destination_start + start_offset,
                length: range_end - values_end,
                kind: to,
            };
            return Some(vec![remaining, overlap]);
        };
        None
    }
}

#[test]
fn value_ranges() {
    let range = Range {
        destination_start: 100,
        source_start: 10,
        range_length: 10,
    };
    let value_range = ValueRange {
        start: 5,
        length: 10,
        kind: Kind::Seed,
    };
    assert_eq!(
        range.map_value_range(&value_range, Kind::Soil).unwrap(),
        vec![
            ValueRange {
                start: 5,
                length: 4,
                kind: Kind::Seed
            },
            ValueRange {
                start: 100,
                length: 6,
                kind: Kind::Soil
            }
        ]
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Kind {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for Kind {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Kind::*;

        match value {
            "seed" => Ok(Seed),
            "soil" => Ok(Soil),
            "fertilizer" => Ok(Fertilizer),
            "water" => Ok(Water),
            "light" => Ok(Light),
            "temperature" => Ok(Temperature),
            "humidity" => Ok(Humidity),
            "location" => Ok(Location),
            _ => Err("invalid".into()),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Value(i64, Kind);

impl Value {
    fn quantity(&self) -> i64 {
        match self {
            Value(quantity, _) => *quantity,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ValueRange {
    start: i64,
    length: i64,
    kind: Kind,
}

#[derive(Debug)]
struct Map {
    from: Kind,
    to: Kind,
    ranges: Vec<Range>,
}

impl Map {
    /// Returns None is no values within the range are modified.
    fn translate_range(&self, values: &[ValueRange]) -> Vec<ValueRange> {
        let mut new_ranges: Vec<ValueRange> = Vec::new();
        for value_range in values {
            // fold over all ranges
            let mut results =
                self.ranges
                    .iter()
                    .fold(None, |new: Option<Vec<ValueRange>>, range: &Range| {
                        if let Some(mut result) = range.map_value_range(value_range, self.to) {
                            match new {
                                None => Some(result),
                                Some(mut older_results) => {
                                    result.append(&mut older_results);
                                    Some(result)
                                }
                            }
                        } else {
                            new
                        }
                    });
            if let Some(mut new) = results {
                // if any ranges applied, return the new value ranges
                new_ranges.append(&mut new)
            } else {
                // otherwise, the value range is unmodified
                new_ranges.push(ValueRange {
                    start: value_range.start,
                    length: value_range.length,
                    kind: self.to,
                });
            }
        }
        new_ranges
    }

    fn translate(&self, value: &Value) -> Value {
        let quantity: i64 = match value {
            Value(quantity, kind) if *kind == self.from => *quantity,
            _ => panic!("invalid mapping"),
        };
        let results = self
            .ranges
            .iter()
            .filter_map(|range| range.map(quantity))
            .collect::<Vec<i64>>();

        match results.first() {
            Some(result) => Value(*result, self.to),
            None => Value(quantity, self.to),
        }
    }
}

/// Only calculate locations for each value once.
#[derive(Debug)]
struct LocationCache(HashMap<Value, Value>);

impl LocationCache {
    fn new() -> Self {
        LocationCache(HashMap::new())
    }
}

#[derive(Debug)]
struct Almanac(Vec<Map>);

impl Almanac {
    /// Find a map with a particular source kind.
    fn find_map(&self, kind: Kind) -> Option<&Map> {
        self.0.iter().filter(|map| map.from == kind).next()
    }

    /// Convert a value to a location value.
    fn to_location(&self, value: &Value) -> Value {
        let Value(_, kind) = value;
        let mapped = self
            .find_map(*kind)
            .and_then(|map| Some(map.translate(value)))
            .unwrap();
        match mapped {
            Value(_, Kind::Location) => mapped,
            _ => self.to_location(&mapped),
        }
    }

    /// Convert a value to a location value, using cached results.
    fn cached_to_location(&self, cache: &mut LocationCache, value: &Value) -> Value {
        if let Some(result) = cache.0.get(value) {
            result.clone()
        } else {
            let Value(_, kind) = value;
            let mapped = self
                .find_map(*kind)
                .and_then(|map| Some(map.translate(value)))
                .unwrap();
            match mapped {
                Value(_, Kind::Location) => mapped,
                _ => {
                    let result = self.to_location(&mapped);
                    cache.0.insert(value.clone(), result.clone());
                    result
                }
            }
        }
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<Value>,
    almanac: Almanac,
}

impl Input {
    fn solve1(&self) -> i64 {
        self.seeds
            .iter()
            .map(|seed| match self.almanac.to_location(seed) {
                Value(quantity, _) => quantity,
            })
            .min()
            .unwrap()
    }

    fn part2_seeds(&self) -> Vec<ValueRange> {
        todo!()
    }

    fn solve2(&self) -> i64 {
        let mut cache = LocationCache::new();
        todo!()
    }
}

impl TryFrom<&str> for Input {
    type Error = String;

    fn try_from(lines: &str) -> Result<Self, Self::Error> {
        let mut input = InputParser::parse(Rule::input, lines)
            .map_err(|e| e.to_string())?
            .next()
            .ok_or("no input")?
            .into_inner();
        let seed_tokens = input.next().ok_or("missing seeds")?;
        let seeds = seed_tokens
            .into_inner()
            .map(|number| {
                let quantity = number
                    .as_str()
                    .trim()
                    .parse::<i64>()
                    .map_err(|e| e.to_string())?;
                Ok(Value(quantity, Kind::Seed))
            })
            .collect::<Result<Vec<Value>, String>>()?;
        let almanac = Almanac(
            input
                .map(|map| {
                    let mut map_tokens = map.into_inner();
                    let mut map_type = map_tokens.next().ok_or("missing type")?.into_inner();
                    let from = Kind::try_from(map_type.next().ok_or("missing from")?.as_str())?;
                    let to = Kind::try_from(map_type.next().ok_or("missing to")?.as_str())?;
                    let ranges = map_tokens
                        .map(|range| {
                            let mut range_tokens = range.into_inner();
                            let destination_start = range_tokens
                                .next()
                                .ok_or("missing destination start")?
                                .as_str()
                                .trim()
                                .parse::<i64>()
                                .map_err(|e| e.to_string())?;
                            let source_start = range_tokens
                                .next()
                                .ok_or("missing source start")?
                                .as_str()
                                .trim()
                                .parse::<i64>()
                                .map_err(|e| e.to_string())?;
                            let range_length = range_tokens
                                .next()
                                .ok_or("missing length")?
                                .as_str()
                                .trim()
                                .parse::<i64>()
                                .map_err(|e| e.to_string())?;
                            Ok(Range {
                                destination_start,
                                source_start,
                                range_length,
                            })
                        })
                        .collect::<Result<Vec<Range>, String>>()?;
                    Ok(Map { from, to, ranges })
                })
                .collect::<Result<Vec<Map>, String>>()?,
        );
        Ok(Self { seeds, almanac })
    }
}

#[test]
fn example01_seed14() {
    let example = include_str!("input/day05/example01.txt");
    let input = Input::try_from(example).unwrap();
    let seed = Value(14, Kind::Seed);
    let soil: Value = input
        .almanac
        .find_map(Kind::Seed)
        .and_then(|map| Some(map.translate(&seed)))
        .unwrap();
    assert!(matches!(soil, Value(14, Kind::Soil)));
    let fertilizer: Value = input
        .almanac
        .find_map(Kind::Soil)
        .and_then(|map| Some(map.translate(&soil)))
        .unwrap();
    assert!(matches!(fertilizer, Value(53, Kind::Fertilizer)));
    let water = input
        .almanac
        .find_map(Kind::Fertilizer)
        .and_then(|map| Some(map.translate(&fertilizer)))
        .unwrap();
    assert!(matches!(water, Value(49, Kind::Water)));
    let light = input
        .almanac
        .find_map(Kind::Water)
        .and_then(|map| Some(map.translate(&water)))
        .unwrap();
    assert!(matches!(light, Value(42, Kind::Light)));
    let temperature = input
        .almanac
        .find_map(Kind::Light)
        .and_then(|map| Some(map.translate(&light)))
        .unwrap();
    assert!(matches!(temperature, Value(42, Kind::Temperature)));
    let humidity = input
        .almanac
        .find_map(Kind::Temperature)
        .and_then(|map| Some(map.translate(&temperature)))
        .unwrap();
    assert!(matches!(humidity, Value(43, Kind::Humidity)));
    let location = input
        .almanac
        .find_map(Kind::Humidity)
        .and_then(|map| Some(map.translate(&humidity)))
        .unwrap();
    assert!(matches!(location, Value(43, Kind::Location)));
}

#[test]
fn example01_explanation() {
    let example = include_str!("input/day05/example01.txt");
    let input = Input::try_from(example).unwrap();
    assert_eq!(
        input.almanac.to_location(&Value(79, Kind::Seed)),
        Value(82, Kind::Location)
    );
    assert_eq!(
        input.almanac.to_location(&Value(14, Kind::Seed)),
        Value(43, Kind::Location)
    );
    assert_eq!(
        input.almanac.to_location(&Value(55, Kind::Seed)),
        Value(86, Kind::Location)
    );
    assert_eq!(
        input.almanac.to_location(&Value(13, Kind::Seed)),
        Value(35, Kind::Location)
    );
}

#[test]
fn example01() {
    let example = include_str!("input/day05/example01.txt");
    let input = Input::try_from(example).unwrap();
    assert_eq!(input.solve1(), 35);
}

fn main() {
    let input = Input::try_from(include_str!("input/day05/input.txt")).unwrap();
    dbg!(&input);
    println!("part 1: {}", input.solve1());
    println!("part 1: {}", input.solve2());
}
