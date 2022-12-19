use std::{time::Instant, io::BufRead, collections::HashSet, str::FromStr};

const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";

// Options
const EVENT_ROW : i32 = 2000000;

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
}

struct LoadedData {
    sensors : Vec<Sensor>,
    beacons : HashSet<Cooordinate>
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


pub fn main() {
    // Part a
    let timer = Instant::now();
    let data = load_sensors_and_beacons();

    let mut event_coordinates : HashSet<Cooordinate> = HashSet::new();

    let sensors = data.sensors;
    let mut already_occupied = HashSet::new();

    for sensor in sensors {
        let covered = sensor.get_covered_area();
        let filtered = covered.iter().filter(|c| c.y == EVENT_ROW);
        event_coordinates.extend(filtered);

        already_occupied.insert(sensor.location.clone());
        already_occupied.insert(sensor.closest_beacon.clone());
    }

    let empty_excluded_spaces = event_coordinates.iter().filter(|c| !already_occupied.contains(c));

    println!("Part A: Found {} excluded positions at row {} (Took {} ms)", empty_excluded_spaces.count(), EVENT_ROW, timer.elapsed().as_millis());
}
