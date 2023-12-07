use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "bin/day05.pest"]
struct InputParser;

#[derive(Debug)]
struct Seed(u64);

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

impl Range {
    fn map(&self, value: u64) -> Option<u64> {
        let start = self.source_start;
        let end = self.source_start + (self.range_length) - 1;

        if start <= value && value <= end {
            let offset = value - self.source_start;
            Some(self.destination_start + offset)
        } else {
            None
        }
    }
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
struct Value(u64, Kind);

#[derive(Debug)]
struct Map {
    from: Kind,
    to: Kind,
    ranges: Vec<Range>,
}

impl Map {
    fn translate(&self, value: &Value) -> Value {
        let quantity: u64 = match value {
            Value(quantity, kind) if *kind == self.from => *quantity,
            _ => panic!("invalid mapping"),
        };
        let results = self
            .ranges
            .iter()
            .filter_map(|range| range.map(quantity))
            .collect::<Vec<u64>>();

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
    fn solve1(&self) -> u64 {
        let mut cache = LocationCache::new();
        self.seeds
            .iter()
            .map(|seed| {
                let Value(quantity, _) = self.almanac.cached_to_location(&mut cache, seed);
                quantity
            })
            .min()
            .unwrap()
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
                    .parse::<u64>()
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
                                .parse::<u64>()
                                .map_err(|e| e.to_string())?;
                            let source_start = range_tokens
                                .next()
                                .ok_or("missing source start")?
                                .as_str()
                                .trim()
                                .parse::<u64>()
                                .map_err(|e| e.to_string())?;
                            let range_length = range_tokens
                                .next()
                                .ok_or("missing length")?
                                .as_str()
                                .trim()
                                .parse::<u64>()
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
    println!("part 1: {}", input.solve1());
}
