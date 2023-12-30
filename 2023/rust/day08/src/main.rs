use std::collections::HashMap;

fn main() {
    let data = include_str!("./input.txt");
    let (sequence, nodes_string) = parse_data(data);
    compute_part1(sequence, nodes_string);
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
