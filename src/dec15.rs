use std::{time::Instant, io::BufRead, collections::{HashSet, hash_map::RandomState}, str::FromStr};

const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";

// Options
const EVENT_ROW : i32 = 2000000;
const SEARCH_MIN_MAX : (i32, i32) = (0, 4000000);
const PROGRESS_TRACKING : bool = true;
const UPDATE_TRACKING_STEPS : i32 = 100;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cooordinate {
    x : i32,
    y : i32
}

impl std::fmt::Display for Cooordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
    
}

#[derive(Debug, PartialEq, Eq)]
struct CoordinateParseError;

impl FromStr for Cooordinate {
    type Err = CoordinateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Format:
        // x={}, y={}
        let x = s.split("x=").nth(1).ok_or(CoordinateParseError)?.split(",").nth(0).ok_or(CoordinateParseError)?;
        let y = s.split("y=").nth(1).ok_or(CoordinateParseError)?;

        let x_val = x.parse::<i32>().ok().ok_or(CoordinateParseError)?;
        let y_val = y.parse::<i32>().ok().ok_or(CoordinateParseError)?;

        let coordinate = Self {x: x_val, y: y_val};
        return Ok(coordinate);
    }

}

#[derive(PartialEq, Eq, Hash, Debug)]
enum LineAllignment {
    Horizontal,
    Vertical
}

impl Cooordinate {
    fn manhattan_distance_to(&self, other : &Cooordinate) -> u32 {
        let delta_x = self.x.abs_diff(other.x);
        let delta_y = self.y.abs_diff(other.y);

        return delta_x + delta_y;
    }

    fn get_coordinates_in_exact_range(&self, range : u32) -> HashSet<Cooordinate> {
        let mut set = HashSet::new();
        
        let range : i32 = range.try_into().expect("Range too high!");
        let shifted_directions = [(1,1), (1,-1), (-1,1), (-1,-1)];

        for direction in shifted_directions {
            let shift_x = direction.0;
            let shift_y = direction.1;

            for i in 0..range+1 {
                let x = i * shift_x;
                let y = (range - i) * shift_y;

                let coordinate = Cooordinate {x: self.x + x, y: self.y + y};
                set.insert(coordinate);
            }
        }

        return set;
    }

    
    fn on_line(&self, other : &Cooordinate) -> Vec<LineAllignment> {
        let mut allignments = vec![];

        let x = self.x == other.x;
        let y = self.y == other.y;

        if x {
            allignments.push(LineAllignment::Vertical);
        }
        if y {
            allignments.push(LineAllignment::Horizontal)
        }

        return allignments;
    }
}

struct Sensor {
    location : Cooordinate,
    closest_beacon : Cooordinate
}

impl Sensor {
    fn get_exclusion_range(&self) -> u32 {
        let manhattan_distance = self.location.manhattan_distance_to(&self.closest_beacon);
        return manhattan_distance;
    }

    fn get_covered_area(&self) -> HashSet<Cooordinate> {
        let mut set = HashSet::new();

        let max_distance = self.get_exclusion_range();
        for i in 0..max_distance+1 {
            let coordinates_at_range = self.location.get_coordinates_in_exact_range(i);
            set.extend(coordinates_at_range);
        }

        return set;
    }

    fn get_coordinates_in_line(&self, x: Option<i32>, y: Option<i32>) -> Result<Option<Line>, std::fmt::Error> {
        let only_one_set = x.is_some() ^ y.is_some();
        if !only_one_set {
            return Err(std::fmt::Error);
        }

        let exclusion_range : i32 = self.get_exclusion_range().try_into().expect("Too large range");
        let move_x = x.is_some();
        let target = if move_x { x.unwrap() } else { y.unwrap() };
        let current_x = self.location.x;
        let current_y = self.location.y;
        let current = if move_x { current_x } else { current_y };
        let delta : i32 = target.abs_diff(current).try_into().unwrap();

        let line_overflow = exclusion_range - delta;
        if line_overflow < 0 {
            return Ok(None);
        }
               
        let first_point_touched = if move_x { Cooordinate { x: target, y: current_y}} else { Cooordinate { x: current_x, y: target }};
        let add_x = if move_x { 0 } else { line_overflow };
        let add_y = if move_x { line_overflow } else { 0 };

        let start = Cooordinate { x: first_point_touched.x - add_x, y: first_point_touched.y - add_y };
        let end = Cooordinate { x: first_point_touched.x + add_x, y: first_point_touched.y + add_y };

        let line = Line::new(start, end);
        return Ok(Some(line));
    }
}

struct LoadedData {
    sensors : Vec<Sensor>,
    beacons : HashSet<Cooordinate>
}

#[derive(Debug)]
struct Line {
    start : Cooordinate,
    end : Cooordinate,
    allignments : Vec<LineAllignment>
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", &self.start, &self.end)
    }
    
}

enum LineMergeError {
    LineAllignmentError,
    NoOverlapError
}
impl Line {
    fn new(start : Cooordinate, end : Cooordinate) -> Self {
        let allignments = start.on_line(&end);
        //println!("Created line {} -> {} with allignment {:?}", &start, &end, &allignments);
        Self { start, end, allignments }
    }


    fn merged(&self, other : &Line) -> Result<Line, LineMergeError> {
        //println!("Attempting to merge line {} and line {}", &self, other);
        let my_allignments : HashSet<&LineAllignment, RandomState> = HashSet::from_iter(self.allignments.iter());
        let other_allignments : HashSet<&LineAllignment, RandomState> = HashSet::from_iter(other.allignments.iter());
        let intersection = my_allignments.intersection(&other_allignments).collect::<Vec<&&LineAllignment>>();
        let correct_allignment = intersection.len() == 1;

        if !correct_allignment {
            //println!("Exited with allignment error ({} allignments)", intersection.len());
            return Err(LineMergeError::LineAllignmentError);
        }

        let allignment = **intersection.get(0).unwrap();
        let my_end = if allignment == &LineAllignment::Horizontal { self.end.x } else { self.end.y };
        let other_start = if allignment == &LineAllignment::Horizontal { other.start.x } else { other.start.y };

        if my_end - other_start < -1 {
            //println!("Exiteded with overlap error");
            return Err(LineMergeError::NoOverlapError);
        }

        let min_x = self.start.x.min(other.start.x);
        let min_y = self.start.y.min(other.start.y);
        let max_x = self.end.x.max(other.end.x);
        let max_y = self.end.y.max(other.end.y);

        let start = Cooordinate { x: min_x, y: min_y };
        let end = Cooordinate { x: max_x, y: max_y };

        let line = Line::new(start, end);
        //println!("Merged line {} and line {} to line {}", &self, other, &line);

        return Ok(line);
    }

    fn contains_coordinate(&self, coordinate : &Cooordinate) -> bool {
        let in_x = coordinate.x >= self.start.x && coordinate.x <= self.end.x;
        let in_y = coordinate.y >= self.start.y && coordinate.y <= self.end.y;

        return in_x && in_y;
    }

    fn cut_x(&self, min_x : i32, max_x : i32) -> Self {
        let x_start = self.start.x.max(min_x);
        let x_end = self.end.x.min(max_x);

        let start = Cooordinate { x: x_start, y: self.start.y };
        let end = Cooordinate { x: x_end, y: self.start.y };

        let line = Line::new(start, end);
        return line;
    }

    fn len(&self) -> u32 {
        let x_len = self.start.x.abs_diff(self.end.x) + 1;
        let y_len = self.start.y.abs_diff(self.end.y) + 1;

        return x_len.max(y_len);
    }
}


fn load_sensors_and_beacons() -> LoadedData {
    let mut sensors = vec![];
    let mut beacons = HashSet::new();

    let file = std::fs::File::open("res/dec15_input.txt").expect("Failed opening file!");
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let line = line.expect("Failed reading line");
        if line.is_empty() {
            continue;
        }

        let sensor_str = line.split("Sensor at ").nth(1).expect(WRONG_FORMAT_MESSSAGE).split(":").nth(0).expect(WRONG_FORMAT_MESSSAGE);
        let beacon_str = line.split("closest beacon is at ").nth(1).expect(WRONG_FORMAT_MESSSAGE);

        let sensor_coordinate = Cooordinate::from_str(sensor_str).expect(WRONG_FORMAT_MESSSAGE);
        let beacon_coordinate = Cooordinate::from_str(beacon_str).expect(WRONG_FORMAT_MESSSAGE);

        let sensor = Sensor {location: sensor_coordinate, closest_beacon: beacon_coordinate.clone() };
        
        sensors.push(sensor);
        beacons.insert(beacon_coordinate);
    }

    let data = LoadedData { sensors, beacons };
    return data;

}

fn get_blocked_at_y(y : i32, sensors : &Vec<Sensor>) -> Vec<Line> {
    let mut lines = vec![];
    let mut already_occupied = HashSet::new();

    for sensor in sensors {
        let covered = sensor.get_coordinates_in_line(None, Some(y)).unwrap();
        if let Some(any) = covered {
            lines.push(any);
        }

        already_occupied.insert(sensor.location.clone());
        already_occupied.insert(sensor.closest_beacon.clone());
    }

    for occupied in already_occupied.iter().filter(|c| c.y == y) {
        let line_dot = Line::new(occupied.clone(), occupied.clone());
        lines.push(line_dot);
    }

    lines.sort_by_key(|l| -l.start.x);
    let mut merged_lines = vec![lines.pop().unwrap()];
    while !lines.is_empty() {
        let last_index = merged_lines.len() - 1;
        let fragment = lines.pop().unwrap();
        let merge_attempt = merged_lines.get(last_index).unwrap().merged(&fragment);
        if let Ok(ok) = merge_attempt {
            *merged_lines.get_mut(last_index).unwrap() = ok;
        } else {
            merged_lines.push(fragment);
        }
    }

    return merged_lines;
}


pub fn main() {
    // Part a
    let timer_a = Instant::now();
    let data = load_sensors_and_beacons();
    let sensors = data.sensors;

    let lines = get_blocked_at_y(EVENT_ROW, &sensors);
    let mut ignore = HashSet::new();
    for sensor in sensors.iter() {
        ignore.insert(sensor.location);
        ignore.insert(sensor.closest_beacon);
    }
    let raw = lines.iter().map(|l| l.len() as usize).sum();
    let ignored_entries = ignore.iter().filter(|c| c.y == EVENT_ROW).count();

    let a = if ignored_entries > raw {0} else { raw - ignored_entries};
    //println!("Part a lines: {:?} (sum={})", &lines, a);
    println!("Part A: Found {} excluded positions at row {} (Took {} ms)", a, EVENT_ROW, timer_a.elapsed().as_millis());

    // Part b
    let timer_b = Instant::now();
    let min = SEARCH_MIN_MAX.0;
    let max = SEARCH_MIN_MAX.1;
    if PROGRESS_TRACKING {
        println!("Scanning rows (x: {}-{}) for hidden beacon:", min, max);
    }

    let mut count = 0;
    let checkpoint = max / UPDATE_TRACKING_STEPS;
    assert!(checkpoint != 0);
    if PROGRESS_TRACKING {
        print!("Progress: ->");
    }
    for i in min..max+1 {
        let lines = get_blocked_at_y(i, &sensors).iter().map(|l| l.cut_x(min, max)).collect::<Vec<Line>>();
        count += 1;
        if PROGRESS_TRACKING && count % checkpoint == 0 {
            print!("{} ", i);
        }
        
        //assert!(!check.contains_coordinate(&found));
        

        let sum : u32 = lines.iter().map(|l| l.len()).sum();
        if sum != max as u32 {
            continue;
        }

        let edge = lines.get(0).unwrap().end;
        let found = Cooordinate { x: edge.x + 1, y: edge.y };
        if PROGRESS_TRACKING {
            println!("\nFound solution");
            
        }
        for check in lines.iter() {
            assert!(!check.contains_coordinate(&found));
        }

        let x = found.x as u64;
        let y = found.y as u64;

        let tuning = x * 4000000 + y;
        println!("Part B: Found hidden beacon at {} with tuning frequency {} (Took {} ms)", &found, tuning, timer_b.elapsed().as_millis());
        return;
    }

    if PROGRESS_TRACKING {
        println!();
    }
    println!("No hidden beacon found!");
}
