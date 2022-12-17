use std::collections::{HashMap, HashSet};

const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x : i32,
    y : i32,
}

struct Map {
    map : Vec<i32>,
    rows : usize,
    cols : usize,
}

impl Map {
    fn get_value(&self, coordinate : &Coordinate) -> &i32 {
        let in_range = self.in_range(coordinate);
        assert!(in_range);
        let index = self.get_index(coordinate);

        let value = self.map.get(index).unwrap();
        return value;
    }

    fn in_range(&self, coordinate : &Coordinate) -> bool {
        let x = coordinate.x;
        let y = coordinate.y;

        return 0 <= x && 0 <= y && x < self.cols.try_into().unwrap() && y < self.rows.try_into().unwrap();
    }

    fn get_index(&self, coordinate : &Coordinate) -> usize {
        let value = coordinate.x + coordinate.y * (self.rows as i32);
        return value as usize;
    }

    fn get_coordinate(&self, index : usize) -> Coordinate {
        let x = (index % self.cols) as i32;
        let y = (index / self.rows) as i32;

        Coordinate { x, y }
    }
}

fn load_map() -> Map {
    let raw = std::fs::read_to_string("res/dec12_input.txt").expect(WRONG_FORMAT_MESSSAGE);
    let collected = raw.split("\n").collect::<Vec<&str>>();
    let rows = collected.len();
    let cols = collected.get(0).unwrap().len();
    let data = collected.iter().map(|s| s.chars()).flatten().map(|c| {
        match c {
            'S' => -1,
            'E' => -2,
            _ => (c as i32) - ('a' as i32)
        }
    }).collect::<Vec<i32>>();

    let map = Map {
        map: data,
        rows,
        cols,
    };
    return map;
}

fn step_through_backtrace(start : &Coordinate, end : &Coordinate, backtrace : &HashMap<Coordinate, Coordinate>) -> u32 {
    let mut steps : u32 = 0;
    let mut current = end;
    while current != start {
        current = backtrace.get(current).unwrap();
        steps += 1;
    }

    return steps;
}


pub fn main() {
    let map = load_map();

    let start_index = map.map.iter().position(|x| x == &-1).unwrap();
    let end_index = map.map.iter().position(|x| x == &-2).unwrap();

    let start = map.get_coordinate(start_index);
    let end = map.get_coordinate(end_index);

    let mut backtrace = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = vec![start.clone()];

    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if current == end {
            let steps = step_through_backtrace(&start, &current, &backtrace);
            println!("Steps: {}", steps);
            return;
        }

        let north = Coordinate { x: current.x, y: current.y + 1 };
        let south = Coordinate { x: current.x, y: current.y - 1 };
        let east = Coordinate { x: current.x + 1, y: current.y };
        let west = Coordinate { x: current.x - 1, y: current.y };

        let dirs = vec![north, east, south, west];
        for dir in dirs {
            let valid = map.in_range(&dir) && !visited.contains(&dir);
            if valid {
                let my_height = map.get_value(&current);
                let height = map.get_value(&dir);

                if height <= &0 || my_height >= &(height - &1) {
                    backtrace.insert(dir.clone(), current.clone());
                    queue.push(dir);

                }
            }

        }

        visited.insert(current);        
    }

    println!("Found no path?");
}
