use std::{collections::HashSet, fs::File, io::{BufReader, BufRead}};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn coordine_in_direction(&self, x_dir: i32, y_dir: i32) -> Coordinate {
        let new_coordinate = Coordinate { x: self.x + x_dir, y: self.y + y_dir };
        return new_coordinate;
    }

    fn touching(&self, other: &Coordinate) -> bool {
        let x_delta = (self.x - other.x).abs();
        let y_delta = (self.y - other.y).abs();
        let max_delta = x_delta.max(y_delta);

        return max_delta <= 1;
    }
    
}

pub fn main() {
    const ROPE_LENGTH : usize = 10;
    let mut rope = vec![Coordinate {x: 0, y: 0}; ROPE_LENGTH];

    let mut visited = HashSet::new();
    visited.insert(Coordinate {x: 0, y: 0});

    let file = File::open("res/dec09_input.txt").expect("Failed opening file!");
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.expect("Failed reading line");
        if line.is_empty() {
            continue;
        }

        //println!("-> {}", &line);

        const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";
        let pair = line.split(" ").collect::<Vec<&str>>();
        let direction = pair.get(0).expect(WRONG_FORMAT_MESSSAGE);
        let steps = pair.get(1).expect(WRONG_FORMAT_MESSSAGE).parse::<i32>().expect(WRONG_FORMAT_MESSSAGE);

        let mut x_dir = 0;
        let mut y_dir = 0;

        match direction {
            &"U" => y_dir = 1,
            &"D" => y_dir = -1,
            &"R" => x_dir = 1,
            &"L" => x_dir = -1,
            _ => panic!("Match error: {}", WRONG_FORMAT_MESSSAGE)
        }

        for _ in 0..steps {
            let mut new_rope = vec![];
            new_rope.reserve_exact(ROPE_LENGTH);
            for knot in 0..ROPE_LENGTH {
                //println!("Checking knot index {}", &knot);
                let current = rope.get(knot).unwrap();
                if knot == 0 {
                    let head = current.coordine_in_direction(x_dir, y_dir);
                    new_rope.push(head);
                }
                else {
                    let knot_in_front_after_move = new_rope.get(knot - 1).unwrap();
                    if !knot_in_front_after_move.touching(current) {
                        let delta_x = knot_in_front_after_move.x - current.x;
                        let delta_y = knot_in_front_after_move.y - current.y;

                        let abs_x = delta_x.abs();
                        let abs_y = delta_y.abs();

                        let move_x = if delta_x == 0 {0} else {delta_x / abs_x};
                        let move_y = if delta_y == 0 {0} else {delta_y / abs_y};

                        let new_location = current.coordine_in_direction(move_x, move_y);
                        new_rope.push(new_location);
                    }
                    else {
                        new_rope.push(current.clone());
                    }
                }

            }

            let tail = new_rope.get(ROPE_LENGTH - 1).unwrap();
            visited.insert(tail.clone());
            rope = new_rope;
        }

    }

    println!("Unique locations visited: {}", visited.len());
    
}
