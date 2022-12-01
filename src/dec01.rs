use std::fs;

fn parse_from_sep(seperated: &str) -> u32 {
    let mut sum: u32 = 0;
    let split = seperated.split("\n");
    split.for_each(|string| {
        if !string.is_empty() {
            //println!("{}", string);
            let number: u32 = string.parse().unwrap();
            sum += number;
        }
    });

    return sum;
}

fn insert_highscores(check: u32, scores: &mut Vec<u32>)
{
    const LIMIT: u8 = 3;
    if scores.is_empty() {
        scores.push(check);
    }

    let mut under = 0;
    for highscore in scores.iter() {
        if check > *highscore {
            under += 1;
        }
    }

    scores.insert(under, check);
    while scores.len() > LIMIT.into() {
        scores.remove(0);
    }
}

pub fn main() {
    let mut scores: Vec<u32> = vec![];
    let content = fs::read_to_string("src/dec01.txt").expect("Failed reading file");
    let split = content.split("\n\n");
    split.for_each(|segment| {
        let value = parse_from_sep(segment);
        insert_highscores(value, &mut scores);
    });

    let sum = scores.iter().fold(0, |acc, x| {
        acc + x
    });

    
    println!("{}", sum);
}
