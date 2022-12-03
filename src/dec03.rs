use std::{fs::File, io::{BufReader, BufRead}};

fn create_ascii_vec() -> Vec<char> {
    const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z'
    ];

    let mut vector: Vec<char> = vec![];
    vector.reserve_exact(2 * ASCII_LOWER.len());
    for c in ASCII_LOWER {
        vector.push(c);
    }

    for c in ASCII_LOWER {
        vector.push(c.to_ascii_uppercase());
    }

    return vector;
}

fn part_a() {
    let alphabet: String = create_ascii_vec().into_iter().collect();

    let mut sum = 0;

    let file = File::open("res/dec03_input.txt").expect("Failed opening file!");
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.expect("Failed reading line");
        if line.is_empty() {
            continue;
        }

        let length = line.len();
        assert_eq!(length % 2, 0);

        let half = length >> 1;
        let a = &line[0..half];
        let b = &line[half..length];

        let mut found = false;
        for c in a.chars() {
            if b.contains(c) {
                found = true;
                let number = alphabet.find(c).expect("Could not find char in alphabet?") + 1;
                sum += number;
                break;
            }
        }

        assert!(found);
    }

    println!("Part a: {}", sum);

}

fn part_b() {
    let alphabet: String = create_ascii_vec().into_iter().collect();

    let mut sum = 0;

    let mut possible_chars = create_ascii_vec();

    let mut group_container : Vec<String> = vec![];
    group_container.reserve_exact(3);

    let file = File::open("res/dec03_input.txt").expect("Failed opening file!");
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.expect("Failed reading line");
        if line.is_empty() {
            continue;
        }

        let filtered: Vec<char> = possible_chars.iter().filter(|&x| line.contains(*x)).map(|c| *c).collect();
        possible_chars = filtered;

        if possible_chars.len() == 1 {
            let remaining_char = possible_chars.get(0).unwrap();
            //println!("Found char {}", &remaining_char);
            let number = alphabet.find(*remaining_char).expect("Could not find char in alphabet?") + 1;
            sum += number;

            possible_chars = create_ascii_vec();

        }
    }

    println!("Part b: {}", sum);


}

pub fn main() {
    part_a();
    part_b();

}
