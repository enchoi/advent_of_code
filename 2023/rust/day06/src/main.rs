const _TRAINING: &str = "Time:      7  15   30
Distance:  9  40  200";

fn main() {
    // let data = _TRAINING;
    let data = include_str!("input.txt");
    let (times, distances) = parse_data(data);
    println!("times: {:?}, distances: {:?}", times, distances);
    let part1 = compute_games(times, distances);
    println!("Part one: {}", part1);
}

fn parse_data(data: &str) -> (Vec<usize>, Vec<usize>) {
    let mut split = data.split("\n");
    let (times, distances) = (
        get_ints(split.next().unwrap()),
        get_ints(split.next().unwrap()),
    );
    (times, distances)
}

fn get_ints(data: &str) -> Vec<usize> {
    data.replace(" ", "")
        .split(":")
        .skip(1)
        .map(|x| {
            if let Ok(int) = x.parse::<usize>() {
                return Some(int);
            }
            None
        })
        .filter(|option| option.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<usize>>()
}

fn compute_games(times: Vec<usize>, distances: Vec<usize>) -> usize {
    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            (0..*time)
                .map(|speed| speed * (*time - speed))
                .filter(|score| score > distance)
                .count()
        })
        .product::<usize>()
}
