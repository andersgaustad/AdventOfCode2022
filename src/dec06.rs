use std::collections::HashSet;

pub fn main() {
    const UNIQUE_NUMBER : usize = 14;

    let string = std::fs::read_to_string("res/dec06_input.txt").expect("Failed opening file!").chars().into_iter().collect::<Vec<char>>();
    let length = string.len();
    let mut index = 4;
    while index <= length {
        // Tail index
        let tail_index = if index >= UNIQUE_NUMBER {index- UNIQUE_NUMBER} else {0};
        let slice = &string[tail_index..index];
        let hashed : HashSet<&char> = HashSet::from_iter(slice.iter().clone());

        // Check
        if hashed.len() == UNIQUE_NUMBER {
            break;
        }

        index += 1;
    }

    println!("Unique sequence found after {} chars", index);
}
