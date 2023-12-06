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

        if start >= value && value >= end {
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

#[derive(Debug)]
struct Value(u64, Kind);

#[derive(Debug)]
struct Map {
    from: Kind,
    to: Kind,
    ranges: Vec<Range>,
}

impl Map {
    fn translate(&self, value: Value) -> Value {
        let quantity: u64 = match value {
            Value(quantity, kind) if kind == self.from => quantity,
            _ => panic!("invalid mapping"),
        };
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
    fn seed_to_location(&self, seed: Value) -> Value {
        let soil: Value = self
            .find_map(Kind::Seed)
            .and_then(|map| Some(map.translate(seed)))
            .unwrap();
        let fertilizer: Value = self
            .find_map(Kind::Soil)
            .and_then(|map| Some(map.translate(soil)))
            .unwrap();
        let water = self
            .find_map(Kind::Fertilizer)
            .and_then(|map| Some(map.translate(fertilizer)))
            .unwrap();
        let light = self
            .find_map(Kind::Water)
            .and_then(|map| Some(map.translate(water)))
            .unwrap();
        let temperature = self
            .find_map(Kind::Light)
            .and_then(|map| Some(map.translate(light)))
            .unwrap();
        let humidity = self
            .find_map(Kind::Temperature)
            .and_then(|map| Some(map.translate(temperature)))
            .unwrap();
        let location = self
            .find_map(Kind::Humidity)
            .and_then(|map| Some(map.translate(humidity)))
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
        todo!()
    }
}

impl TryFrom<&str> for Input {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
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
