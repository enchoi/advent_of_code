use std::slice::Iter;

#[derive(Debug, Clone)]
struct Point {
    column: usize,
    line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        DIRECTIONS.iter()
    }
}
fn main() {
    let data = include_str!("./input.txt")
        .split("\n")
        .map(|part| part.to_string())
        .collect::<Vec<String>>();
    let start = find_start(&data);
    println!("start: {:?}", start);
    do_maze(&data, start);
}

fn find_start(data: &Vec<String>) -> Point {
    for (line_index, line) in data.iter().enumerate() {
        for (column_index, c) in line.char_indices() {
            if c == 'S' {
                return Point {
                    column: column_index,
                    line: line_index,
                };
            }
        }
    }
    panic!("Nooooooo");
}

fn get_starting_point(s: Point, direction: Direction) -> Option<Point> {
    let closure = match direction {
        Direction::North => |point: Point| {
            let mut p = point.clone();
            p.line = p.line.checked_add(1)?;
            return Some(p);
        },
        Direction::South => |point: Point| {
            let mut p = point.clone();
            p.line = p.line.checked_add_signed(-1)?;
            return Some(p);
        },
        Direction::East => |point: Point| {
            let mut p = point.clone();
            p.column = p.column.checked_add(1)?;
            return Some(p);
        },
        Direction::West => |point: Point| {
            let mut p = point.clone();
            p.column = p.column.checked_add_signed(1)?;
            return Some(p);
        },
    };
    closure(s.clone())
}

fn do_maze(data: &Vec<String>, start: Point) {
    let mut max = Vec::new();
    for direction in Direction::iterator() {
        let mut from = direction.clone();
        let mut current = match get_starting_point(start.clone(), direction.clone()) {
            Some(point) => point,
            None => continue,
        };
        let mut length = 0;
        'outter: while length == 0 || data[current.line].chars().nth(current.column).unwrap() != 'S'
        {
            loop {
                // println!(
                //     "test: {}, point: {:?}, from: {:?}, length: {}",
                //     data[current.line].chars().nth(current.column).unwrap(),
                //     current,
                //     from,
                //     length,
                // );
                match (
                    from.clone(),
                    data[current.line].chars().nth(current.column).unwrap(),
                ) {
                    (Direction::North | Direction::South, '|') => {
                        if from == Direction::North {
                            current.line += 1;
                        } else {
                            current.line -= 1;
                        }
                    }
                    (Direction::East | Direction::West, '-') => {
                        if from == Direction::East {
                            current.column -= 1;
                        } else {
                            current.column += 1;
                        }
                    }
                    (Direction::North | Direction::East, 'L') => {
                        if from == Direction::North {
                            current.column += 1;
                            from = Direction::West;
                        } else {
                            current.line -= 1;
                            from = Direction::South;
                        }
                    }
                    (Direction::North | Direction::West, 'J') => {
                        if from == Direction::North {
                            current.column -= 1;
                            from = Direction::East;
                        } else {
                            current.line -= 1;
                            from = Direction::South;
                        }
                    }
                    (Direction::South | Direction::West, '7') => {
                        if from == Direction::South {
                            current.column -= 1;
                            from = Direction::East;
                        } else {
                            current.line += 1;
                            from = Direction::North;
                        }
                    }
                    (Direction::South | Direction::East, 'F') => {
                        if from == Direction::South {
                            current.column += 1;
                            from = Direction::West;
                        } else {
                            current.line += 1;
                            from = Direction::North;
                        }
                    }
                    (_, 'S') => break,
                    // _ ,'.' => ,
                    (_, _) => break 'outter,
                };
                length += 1;
            }
            max.push(length);
        }
    }
    println!("max: {:?}", max);
    let length = match max.iter().max().unwrap() {
        x if x % 2 == 0 => x / 2,
        x => x / 2 + 1,
    };
    println!("Length: {}", length);
}
