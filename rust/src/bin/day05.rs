use pest::Parser;
use pest_derive::Parser;

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
    fn map(&self, value: u64) -> u64 {
        let start = self.source_start;
        let end = self.source_start + (self.range_length) - 1;

        dbg!(value, start, end);

        if start <= value && value <= end {
            let offset = value - self.source_start;
            self.destination_start + offset
        } else {
            value
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
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
        dbg!(&self.from, &self.to);
        let result = self
            .ranges
            .iter()
            .fold(quantity, |quantity, range| range.map(quantity));
        Value(result, self.to)
    }
}

#[derive(Debug)]
struct Almanac(Vec<Map>);

impl Almanac {
    /// Find a map with a particular source kind.
    fn find_map(&self, kind: Kind) -> Option<&Map> {
        self.0.iter().filter(|map| map.from == kind).next()
    }

    /// Convert a seed value to a location value.
    fn seed_to_location(&self, seed: &Value) -> Value {
        let soil: Value = self
            .find_map(Kind::Seed)
            .and_then(|map| Some(map.translate(&seed)))
            .unwrap();
        let fertilizer: Value = self
            .find_map(Kind::Soil)
            .and_then(|map| Some(map.translate(&soil)))
            .unwrap();
        let water = self
            .find_map(Kind::Fertilizer)
            .and_then(|map| Some(map.translate(&fertilizer)))
            .unwrap();
        let light = self
            .find_map(Kind::Water)
            .and_then(|map| Some(map.translate(&water)))
            .unwrap();
        let temperature = self
            .find_map(Kind::Light)
            .and_then(|map| Some(map.translate(&light)))
            .unwrap();
        let humidity = self
            .find_map(Kind::Temperature)
            .and_then(|map| Some(map.translate(&temperature)))
            .unwrap();
        let location = self
            .find_map(Kind::Humidity)
            .and_then(|map| Some(map.translate(&humidity)))
            .unwrap();
        location
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<Value>,
    almanac: Almanac,
}

impl Input {
    fn solve1(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| {
                dbg!(&seed);
                let Value(quantity, _) = self.almanac.seed_to_location(seed);
                dbg!(quantity)
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
fn example01_explanation() {
    let example = include_str!("input/day05/example01.txt");
    let input = dbg!(Input::try_from(example).unwrap());
    assert_eq!(
        input.almanac.seed_to_location(&Value(79, Kind::Seed)),
        Value(82, Kind::Location)
    );
    assert_eq!(
        input.almanac.seed_to_location(&Value(14, Kind::Seed)),
        Value(43, Kind::Location)
    );
    assert_eq!(
        input.almanac.seed_to_location(&Value(55, Kind::Seed)),
        Value(86, Kind::Location)
    );
    assert_eq!(
        input.almanac.seed_to_location(&Value(13, Kind::Seed)),
        Value(35, Kind::Location)
    );
}

#[test]
fn example01() {
    let example = include_str!("input/day05/example01.txt");
    let input = dbg!(Input::try_from(example).unwrap());
    assert_eq!(input.solve1(), 35);
}

fn main() {
    let input = Input::try_from(include_str!("input/day05/input.txt")).unwrap();
    println!("part 1: {}", input.solve1());
}
