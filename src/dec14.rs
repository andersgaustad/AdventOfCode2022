use std::{time::Instant, io::BufRead, collections::HashSet};

const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Cooordinate {
    x : i32,
    y : i32
}

impl std::fmt::Display for Cooordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
    
}

struct Line {
    a: Cooordinate,
    b: Cooordinate
}

impl Line {
    fn contains_coordinate(&self, coordinate : &Cooordinate) -> bool {
        let min_x = self.a.x.min(self.b.x);
        let min_y = self.a.y.min(self.b.y);
        let max_x = self.a.x.max(self.b.x);
        let max_y = self.a.y.max(self.b.y);

        let x = coordinate.x;
        let y = coordinate.y;

        return x >= min_x && y >= min_y && x <= max_x && y <= max_y;
    }

    fn lowest(&self) -> i32 {
        let max_y = self.a.y.max(self.b.y);
        return max_y;
    }
    
}

fn load_lines() -> Vec<Line> {
    let mut lines = vec![];
    let file = std::fs::File::open("res/dec14_input.txt").expect("Could not open file");
    let buffer = std::io::BufReader::new(file).lines();
    for line in buffer {
        let line = line.expect("Error reading lines");
        if line.is_empty() {
            continue;
        }

        let mut coordinates = vec![];
        let split = line.split(" -> ");
        for pair in split {
            let values = pair.split(",").map(|v| v.parse::<i32>().expect(WRONG_FORMAT_MESSSAGE)).collect::<Vec<i32>>();
            let x = *values.get(0).expect(WRONG_FORMAT_MESSSAGE);
            let y = *values.get(1).expect(WRONG_FORMAT_MESSSAGE);

            let coordiante = Cooordinate {x, y};
            coordinates.push(coordiante);
        }

        for i in 0..coordinates.len()-1 {
            let a = coordinates.get(i).unwrap().clone();
            let b = coordinates.get(i+1).unwrap().clone();

            let structure_line = Line {a, b};
            lines.push(structure_line);
        }
    }

    return lines;
}

fn blocked(coordinate : &Cooordinate, sand : &HashSet<Cooordinate>, lines : &Vec<Line>) -> bool {
    let blocked_by_sand = sand.contains(coordinate);
    if blocked_by_sand {
        return true;
    }

    let blocked_by_line = lines.iter().any(|x| x.contains_coordinate(coordinate));
    return blocked_by_line;
}

pub fn main() {
    // Part a
    let timer_a = Instant::now();
    let lines = load_lines();
    let void_border = lines.iter().map(|l| l.lowest()).max().unwrap();
    //println!("Void border set to {}", void_border);

    let spawn_point = Cooordinate {x: 500, y: 0};
    let mut placed_sand_blocks = HashSet::new();

    let direction_behavior = [Cooordinate {x: 0, y: 1}, Cooordinate {x: -1, y: 1}, Cooordinate {x: 1, y: 1}];

    'outer: loop {
        // Spawn
        let mut sand = spawn_point.clone();

        'inner: loop {
            for physic_dir in direction_behavior.iter() {
                let new_transform = Cooordinate {x: sand.x + physic_dir.x, y: sand.y + physic_dir.y };
                let blocked_there = blocked(&new_transform, &placed_sand_blocks, &lines);
                if !blocked_there {
                    // Move
                    sand = new_transform;

                    // Check event horizon
                    let current_y = sand.y;
                    if current_y > void_border {
                        break 'outer;
                    } else {
                        continue 'inner;
                    }
                }
            }

            // Blocked in all directions
            assert!(!placed_sand_blocks.contains(&sand));
            placed_sand_blocks.insert(sand);
            assert!(!placed_sand_blocks.contains(&spawn_point));
            //println!("Placed sand at {}", &sand);
            continue 'outer;
        }
    }

    println!("Part A: Sand simulation complete: Placed {} sand units (Took {} ms)", placed_sand_blocks.len(), timer_a.elapsed().as_millis());
}
