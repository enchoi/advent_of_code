use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::sync::{Arc, RwLock};
use std::thread;

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

#[derive(Clone, Copy)]
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

#[derive(Debug, Clone)]
struct Mapper {
    pub source: u64,
    pub destination: u64,
    pub range: u64,
}

impl Mapper {
    pub fn is_in(&self, value: u64) -> bool {
        self.source <= value && self.source + self.range > value
    }
}
#[derive(Debug, Clone)]
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
    let seeds = add_seeds(seeds);
    compute_part1(mappers, seeds);
}

fn add_seeds(seeds: Vec<u64>) -> Vec<Range<u64>> {
    let mut new_seeds = Vec::new();
    for i in (0..seeds.len()).filter(|n| n % 2 == 0) {
        new_seeds.push(seeds[i]..seeds[i] + seeds[i + 1])
    }
    new_seeds
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

fn get_mapped_value(mappers: HashMap<String, Mappers>, value: u64, type_: StateMapper) -> u64 {
    let next_type = match type_ {
        StateMapper::SeedToSoil => "seed-to-soil",
        StateMapper::SoilToFertilizer => "soil-to-fertilizer",
        StateMapper::FertilizerToWater => "fertilizer-to-water",
        StateMapper::WaterToLight => "water-to-light",
        StateMapper::LightToTemperature => "light-to-temperature",
        StateMapper::TemperatureToHumidity => "temperature-to-humidity",
        StateMapper::HumidityToLocation => {
            return mappers["humidity-to-location"].get_destination(value)
        }
    };
    return get_mapped_value(
        mappers.clone(),
        mappers[next_type].get_destination(value),
        StateMapper::HumidityToLocation,
    );
}

fn compute_part1(mappers: HashMap<String, Mappers>, seeds: Vec<Range<u64>>) {
    let mut handels = Vec::new();
    let lock: Arc<RwLock<HashMap<u64, u64>>> = Arc::new(RwLock::new(HashMap::new()));

    for range in seeds {
        let copy = mappers.clone();
        let io = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let mut mini = u64::MAX;
            for seed in range {
                let is_here = { io.read().unwrap().get(&seed).and_then(|v| Some(*v)) };
                let location: u64;
                if let Some(x) = is_here {
                    location = x;
                } else {
                    location = get_mapped_value(copy.clone(), seed, StateMapper::SeedToSoil);
                    let mut io = io.write().unwrap();
                    io.insert(seed, location);
                }
                mini = mini.min(location)
            }
            mini
        });
        handels.push(handle);
    }

    let mut results = Vec::new();
    for thread in handels {
        results.push(thread.join().unwrap());
    }
    let closest = results.iter().min().unwrap();

    println!("The closest location is: {}", closest);
}
