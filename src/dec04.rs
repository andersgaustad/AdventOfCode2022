use std::{fs::File, io::{BufReader, BufRead}};

fn any_contains_other(a: (u32, u32), b: (u32, u32)) -> bool {
    return a_contains_b(a, b) || a_contains_b(b, a);
}

fn a_contains_b(a: (u32, u32), b: (u32, u32)) -> bool {
    return a.0 <= b.0 && a.1 >= b.1;
}

fn exclusive(a: (u32, u32), b: (u32, u32)) -> bool {
    return a.1 < b.0 || b.1 < a.0;
}

pub fn main() {
    let mut fully_overlapping_pairs = 0;
    let mut partially_overlapping_pairs = 0;

    let file = File::open("res/dec04_input.txt").expect("Failed opening file!");
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.expect("Failed reading line");
        if line.is_empty() {
            continue;
        }

        const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";
        let pair = line.split(",").collect::<Vec<&str>>();
        let a = pair.get(0).expect(WRONG_FORMAT_MESSSAGE).split("-").collect::<Vec<&str>>();
        let b = pair.get(1).expect(WRONG_FORMAT_MESSSAGE).split("-").collect::<Vec<&str>>();

        let a0 = a.get(0).expect(WRONG_FORMAT_MESSSAGE).parse::<u32>().expect(WRONG_FORMAT_MESSSAGE);
        let a1 = a.get(1).expect(WRONG_FORMAT_MESSSAGE).parse::<u32>().expect(WRONG_FORMAT_MESSSAGE);
        let b0 = b.get(0).expect(WRONG_FORMAT_MESSSAGE).parse::<u32>().expect(WRONG_FORMAT_MESSSAGE);
        let b1 = b.get(1).expect(WRONG_FORMAT_MESSSAGE).parse::<u32>().expect(WRONG_FORMAT_MESSSAGE);

        let any_superset = any_contains_other((a0, a1), (b0, b1));
        if any_superset {
            fully_overlapping_pairs += 1;
        }

        let any_overlap = !exclusive((a0, a1), (b0, b1));
        if any_overlap {
            partially_overlapping_pairs += 1;
        }
    }

    println!("Overlapping pairs: {}", fully_overlapping_pairs);
    println!("Partially overlapping pairs: {}", partially_overlapping_pairs);

}
