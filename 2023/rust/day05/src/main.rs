use std::collections::HashMap;
use std::fs;

const _TRAINING: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

enum StateMapper {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Clone, Eq, PartialEq, Copy)]
enum StateParser {
    Seeds,
    Waiting,
    Mapping,
}

#[derive(Debug)]
struct Mapper {
    pub source: u64,
    pub destination: u64,
    pub range: u64,
}

impl Mapper {
    pub fn is_in(&self, value: u64) -> bool {
        (self.source..self.source + self.range).contains(&value)
    }
}
#[derive(Debug)]
struct Mappers {
    mappers: Vec<Mapper>,
}
impl Mappers {
    fn new() -> Self {
        Self {
            mappers: Vec::new(),
        }
    }
    fn get_destination(&self, value: u64) -> u64 {
        for mapper in &self.mappers {
            if mapper.is_in(value) {
                return mapper.destination + value - mapper.source;
            }
        }
        return value;
    }
    fn add_mapper(&mut self, source: u64, destination: u64, range: u64) {
        self.mappers.push(Mapper {
            source,
            destination,
            range,
        })
    }
}

fn main() {
    // parse mapper
    // let data = _TRAINING;
    let data = get_file_content("src/input.txt");
    let (seeds, mappers) = parse_data(&data);
    // println!("seeds: {:?}", seeds);
    // println!("mappers: {:#?}", mappers);
    compute_part1(&mappers, &seeds);
}

fn get_file_content(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}

fn get_seeds(line: &str, seeds: &mut Vec<u64>) {
    let split = line.split(" ");
    split
        .skip(1)
        .for_each(|number| seeds.push(number.parse::<u64>().unwrap()));
}

fn populate_mapper(mappers: &mut Mappers, line: &str) {
    let mut split = line.split(" ");
    let destination = split.next().unwrap().parse::<u64>().unwrap();
    let source = split.next().unwrap().parse::<u64>().unwrap();
    let range = split.next().unwrap().parse::<u64>().unwrap();

    mappers.add_mapper(source, destination, range);
}

fn parse_data(data: &str) -> (Vec<u64>, HashMap<String, Mappers>) {
    let mut last_mapper: Option<&mut Mappers> = None;
    let mut mappers = HashMap::new();
    let mut seeds = Vec::new();

    let mut state = StateParser::Seeds;

    for line in data.split("\n") {
        match (line, state) {
            // new mapper icoming
            ("", _) => state = StateParser::Waiting,

            // begining of the file
            (text, StateParser::Seeds) => get_seeds(text, &mut seeds),

            // create the new mapper
            (text, StateParser::Waiting) => {
                let name = text.split(" ").next().unwrap();
                mappers.insert(String::from(name), Mappers::new());
                last_mapper = mappers.get_mut(name);
                state = StateParser::Mapping;
            }

            // parse data
            (text, StateParser::Mapping) => {
                if let Some(mapper) = last_mapper.as_mut() {
                    populate_mapper(mapper, text);
                } else {
                    panic!("What the Puck 🏒")
                }
            }
        };
    }

    (seeds, mappers)
}

fn get_mapped_value(mappers: &HashMap<String, Mappers>, value: u64, type_: StateMapper) -> u64 {
    match type_ {
        StateMapper::SeedToSoil => get_mapped_value(
            mappers,
            mappers["seed-to-soil"].get_destination(value),
            StateMapper::SoilToFertilizer,
        ),
        StateMapper::SoilToFertilizer => get_mapped_value(
            mappers,
            mappers["soil-to-fertilizer"].get_destination(value),
            StateMapper::FertilizerToWater,
        ),
        StateMapper::FertilizerToWater => get_mapped_value(
            mappers,
            mappers["fertilizer-to-water"].get_destination(value),
            StateMapper::WaterToLight,
        ),
        StateMapper::WaterToLight => get_mapped_value(
            mappers,
            mappers["water-to-light"].get_destination(value),
            StateMapper::LightToTemperature,
        ),
        StateMapper::LightToTemperature => get_mapped_value(
            mappers,
            mappers["light-to-temperature"].get_destination(value),
            StateMapper::TemperatureToHumidity,
        ),
        StateMapper::TemperatureToHumidity => get_mapped_value(
            mappers,
            mappers["temperature-to-humidity"].get_destination(value),
            StateMapper::HumidityToLocation,
        ),
        StateMapper::HumidityToLocation => mappers["humidity-to-location"].get_destination(value),
    }
}

fn compute_part1(mappers: &HashMap<String, Mappers>, seeds: &Vec<u64>) {
    let mut closest = u64::MAX;
    for seed in seeds {
        let location = get_mapped_value(mappers, *seed, StateMapper::SeedToSoil);
        // println!("Seed: {} => location: {}", seed, location);
        closest = closest.min(location)
    }
    println!("The closest location is: {}", closest);
}
