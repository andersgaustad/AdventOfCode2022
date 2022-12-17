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
        return if value == &-1 {
            &0
        } else if value == &-2 {
            &(('z' as i32) - ('a' as i32))
        } else {
            value
        }
    }

    fn in_range(&self, coordinate : &Coordinate) -> bool {
        let x = coordinate.x;
        let y = coordinate.y;

        return 0 <= x && 0 <= y && x < self.cols.try_into().unwrap() && y < self.rows.try_into().unwrap();
    }

    fn get_index(&self, coordinate : &Coordinate) -> usize {
        let value = coordinate.x + coordinate.y * (self.cols as i32);
        return value as usize;
    }

    fn get_coordinate(&self, index : usize) -> Coordinate {
        let x = (index % self.cols) as i32;
        let y = (index / self.cols) as i32;

        Coordinate { x, y }
    }
}

fn load_map() -> Map {
    let raw = std::fs::read_to_string("res/dec12_input.txt").expect(WRONG_FORMAT_MESSSAGE);
    let collected = raw.split("\n").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
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
    //println!("Loaded map: {}x{}", rows, cols);
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

fn herurestic(current: &Coordinate, goal: &Coordinate) -> i32 {
    let delta_x = (goal.x - current.y).abs();
    let delta_y = (goal.y - current.y).abs();

    return delta_x + delta_y;
}

fn fscore(coordinate : &Coordinate, goal: &Coordinate, g_scores: &HashMap<Coordinate, i32>) -> i32 {
    let g_score = g_scores.get(coordinate);
    if let Some(score) = g_score {
        return score + herurestic(coordinate, goal);
    } else {
        return i32::MAX;
    }
}


pub fn main() {
    let map = load_map();

    let start_index = map.map.iter().position(|x| x == &-1).unwrap();
    let end_index = map.map.iter().position(|x| x == &-2).unwrap();

    let start = map.get_coordinate(start_index);
    let end = map.get_coordinate(end_index);

    let mut g_scores = HashMap::new();
    g_scores.insert(start.clone(), 0);

    let mut backtrace = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = vec![start.clone()];

    while !queue.is_empty() {
        queue.sort_by(|a, b| {
            let a_score = fscore(a, &end, &g_scores);
            let b_score = fscore(b, &end, &g_scores);

            b_score.cmp(&a_score)
        });

        let current = queue.pop().unwrap();
        //println!("Current: {},{} ", current.x, current.y);
        if current == end {
            let steps = step_through_backtrace(&start, &current, &backtrace);
            println!("Found path! Steps: {}", steps);
            return;
        }

        let north = Coordinate { x: current.x, y: current.y + 1 };
        let south = Coordinate { x: current.x, y: current.y - 1 };
        let east = Coordinate { x: current.x + 1, y: current.y };
        let west = Coordinate { x: current.x - 1, y: current.y };

        let dirs = vec![north, east, south, west];
        let mut tentative_g = *g_scores.get(&current).unwrap_or(&i32::MAX);
        if tentative_g != i32::MAX {
            tentative_g += 1;
        }
        for dir in dirs {
            let valid = map.in_range(&dir);
            if valid {
                let my_height = map.get_value(&current);
                let height = map.get_value(&dir);

                let delta_height = height - my_height;

                //println!("Height here: ({},{}): {} - Height at neighbor: ({},{}): {}", current.x, current.y, my_height, dir.x, dir.y, height);

                let neighbor_g = *g_scores.get(&dir).unwrap_or(&i32::MAX);
                if delta_height <= 1 && tentative_g < neighbor_g {
                    backtrace.insert(dir.clone(), current.clone());
                    g_scores.insert(dir.clone(), tentative_g);
                    if !queue.contains(&dir) {
                        queue.push(dir);
                    }
                }
            }

        }

        visited.insert(current);        
    }

    println!("Found no path after visiting {} nodes", visited.len());
}
