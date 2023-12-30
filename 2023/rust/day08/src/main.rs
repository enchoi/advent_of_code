use num::integer::div_floor;
use std::collections::HashMap;

fn main() {
    let data = include_str!("./input.txt");
    let (sequence, nodes_string) = parse_data(data);
    compute_part2(sequence, nodes_string);
}

fn parse_data(data: &str) -> (Vec<usize>, HashMap<String, Vec<String>>) {
    let to_trim: &[_] = &[' ', '(', ')'];
    let mut split = data.split("\n");
    let sequence = split
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    let mut nodes = HashMap::new();
    for line in split.skip(1) {
        let mut tmp = Vec::new();
        let mut split = line.split("=");
        let key = String::from(split.next().unwrap().trim_end());
        let mut split = split.next().unwrap().split(", ");
        let left = String::from(split.next().unwrap().trim_matches(to_trim));
        let right = String::from(split.next().unwrap().trim_matches(to_trim));
        tmp.push(left);
        tmp.push(right);
        nodes.insert(key, tmp);
    }
    (sequence, nodes)
}

fn compute_part1(sequence: Vec<usize>, nodes: HashMap<String, Vec<String>>) {
    let mut current = "AAA".to_string();
    let mut index = 0;
    while current != "ZZZ".to_string() {
        current = nodes[&current][sequence[index % sequence.len()]].to_string();
        index += 1;
    }
    println!("index: {}", index);
}
fn compute_part2(sequence: Vec<usize>, nodes: HashMap<String, Vec<String>>) {
    let current = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|node| node.to_string())
        .collect::<Vec<String>>();

    let number = current
        .iter()
        .map(|node| {
            let mut test = node.to_string();
            let mut index = 0;
            while !test.ends_with("Z") {
                test = nodes[&test][sequence[index % sequence.len()]].to_string();
                index += 1;
            }
            index as i32
        })
        .collect::<Vec<i32>>();
    println!(
        "lcm: {}",
        lcm(number.iter().map(|int| *int as u64).collect())
    );
}

fn lcm(a: Vec<u64>) -> u64 {
    let mut lcm = a[0];
    for i in a.iter() {
        lcm = div_floor(lcm * *i, num::integer::gcd(lcm, *i));
    }
    lcm
}
