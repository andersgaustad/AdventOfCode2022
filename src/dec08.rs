use std::collections::HashSet;

enum Direction {
    East,
    West,
    North,
    South
}

struct TreeData {
    data: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl TreeData {
    fn get_i_for_row(&self, index: usize) -> Vec<usize> {
        let start = index * self.cols;
        let end = start + self.cols;
        return (start..end).collect::<Vec<usize>>();
    }

    fn get_i_for_col(&self, index: usize) -> Vec<usize> {
        let mut indices = vec![];
        indices.reserve_exact(self.rows);
        
        let start = index;
        let end = self.data.len();

        let mut i = start;
        while i < end {
            indices.push(i);
            i += self.cols;            
        }

        return indices;
    }

    fn get_index_in_direction(&self, index: usize, direction: &Direction) -> Option<usize>
    {
        let result = match direction {
            Direction::East => {
                let max_index = self.cols - 1;
                if index % self.cols == max_index {
                    None
                } else {
                    Some(index + 1)
                }
            },
            Direction::West => {
                let min_index = 0;
                if index % self.cols == min_index {
                    None
                } else {
                    Some(index - 1)
                }

                
            }
            Direction::North => {
                let min_index = 0;
                if index / self.rows == min_index {
                    None
                } else {
                    Some(index - self.cols)
                }
            },
            Direction::South => {
                let max_index = self.cols - 1;
                if index / self.rows == max_index {
                    None
                } else {
                    Some(index + self.cols)
                }
            }
        };

        return result;

    }

    fn get_line_of_sight_score_in_direction(&self, index: usize, direction: &Direction) -> u32 {
        let height = self.data.get(index).unwrap();
        let mut score = 0;
        let mut i = index;
        
        while let Some(next) = self.get_index_in_direction(i, direction) {
            score += 1;
            let height_here = self.data.get(next).unwrap();

            if height_here >= height {
                break;
            }

            i = next;           
        }

        return score;
    }

    fn get_line_of_sight_score(&self, index: usize) -> u32 {
        let dirs = [Direction::North, Direction::East, Direction::South, Direction::West];
        let score = dirs.iter().map(|dir| {
            self.get_line_of_sight_score_in_direction(index, dir)
        }).fold(1, |acc, x| {
            acc * x
        });

        return score;
    }
    
}


fn read_tree_data() -> TreeData {
    let raw = std::fs::read_to_string("res/dec08_input.txt").expect("Failed opening file!");
    let split = raw.split("\n").collect::<Vec<&str>>();
    let first_string = split.get(0).unwrap();
    let rows = split.iter().filter(|string| !string.is_empty()).count();
    let cols = first_string.len();

    const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";
    let data = split.iter().map(|string| string.chars()).flatten().map(|c| {
        c.to_string().parse::<u8>().expect(WRONG_FORMAT_MESSSAGE)
    }).collect::<Vec<u8>>();

    TreeData { data, rows, cols }  
}


fn get_visible_using_range(range: &[usize], trees: &[u8]) -> HashSet<usize> {
    let mut visible = HashSet::new();

    let mut iter = range.iter();
    let first_index = *iter.next().unwrap();
    let mut last_blocking = trees.get(first_index).unwrap();
    visible.insert(first_index);

    for i in iter {
        let i = *i;
        let height = trees.get(i).unwrap();
        if height > last_blocking {
            visible.insert(i);
            last_blocking = height;
        }
    }

    return visible;
}


pub fn main() {
    let data = read_tree_data();
    let trees = &data.data;

    let mut all_visible = HashSet::<usize>::new();

    for r in 0..data.rows {
        let front = data.get_i_for_row(r);
        let visible_front = get_visible_using_range(&front, trees);
        all_visible.extend(visible_front);

        let mut back = front.clone();
        back.reverse();
        let visible_back = get_visible_using_range(&back, trees);
        all_visible.extend(visible_back);
    }

    for c in 0..data.cols {
        let front = data.get_i_for_col(c);
        let visible_front = get_visible_using_range(&front, trees);
        all_visible.extend(visible_front);

        let mut back = front.clone();
        back.reverse();
        let visible_back = get_visible_using_range(&back, trees);
        all_visible.extend(visible_back);

    }

    let visible = all_visible.len();
    println!("Visible trees {} (/{})", &visible, trees.len()-1);

    let max_score = (0..trees.len()).map(|i| {
        data.get_line_of_sight_score(i)
    }).max().unwrap();

    println!("Max score: {}", max_score);
    
}
