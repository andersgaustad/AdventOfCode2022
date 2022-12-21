use std::{time::Instant, io::BufRead, collections::{HashSet, hash_map::RandomState, HashMap, VecDeque}, str::FromStr};

const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";

// Options
const MINUTES : u8 = 30;

struct VolcanoData {
    flow_rates : Vec<i32>,
    tunnels : Vec<Vec<usize>>
}
fn load_data() -> VolcanoData {
    let mut flow_rates = vec![];
    let mut tunnels = vec![];

    let file = std::fs::File::open("res/dec16_input.txt").expect("Failed opening file!");
    let lines = std::io::BufReader::new(file).lines();
    let mut current_index : usize = 0;
    for line in lines {
        let line = line.expect("Failed reading line");
        if line.is_empty() {
            continue;
        }

        let flow_rate = line
        .split("has flow rate=").nth(1).expect(WRONG_FORMAT_MESSSAGE)
        .split(";").nth(0).expect(WRONG_FORMAT_MESSSAGE)
        .parse::<i32>().expect(WRONG_FORMAT_MESSSAGE);

        let tunnel_tokens = line.split("tunnels lead to valves ").nth(1).expect(WRONG_FORMAT_MESSSAGE).split(", ");
        let tunnel_index_vector = tunnel_tokens.map(|token| {
            let c = token.chars().nth(0).unwrap().to_ascii_lowercase();
            let value = c as usize - 'a' as usize;
            value
        }).collect::<Vec<usize>>();

        // Commit
        flow_rates.push(flow_rate);
        tunnels.push(tunnel_index_vector);

        current_index += 1;
    }

    let data = VolcanoData { flow_rates, tunnels };
    return data;
}

fn distance_to_other_valves(current_index : usize, tunnels : &Vec<Vec<usize>>) -> HashMap<usize, usize> {
    let mut tunnel_to_distance = HashMap::new();
    tunnel_to_distance.insert(current_index, 0);

    let mut next = vec![current_index];
    while !next.is_empty() {
        // Get
        let element = next.pop().unwrap();
        let distance_to_here = *tunnel_to_distance.get(&element).unwrap();
        let distance_to_neighbors = distance_to_here + 1;
        let connected =tunnels.get(element).unwrap();
        for other_valve in connected.iter() {
            let other_valve = *other_valve;
            let distance_to_other = tunnel_to_distance.get(&other_valve);
            if distance_to_other.is_none() || *distance_to_other.unwrap() > distance_to_neighbors {
                tunnel_to_distance.insert(other_valve, distance_to_neighbors);
                if !next.contains(&other_valve) {
                    next.push(other_valve);
                }
            }
        }
    }

    return tunnel_to_distance;

}

pub fn main() {
    // Part a
    let timer_a = Instant::now();
    let data = load_data();
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
