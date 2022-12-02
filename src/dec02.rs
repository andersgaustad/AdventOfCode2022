use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    pub fn index(&self) -> usize {
        return *self as usize;
    }
    
}

impl TryFrom<u8> for Choice {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Choice::Rock as u8 => Ok(Choice::Rock),
            x if x == Choice::Paper as u8 => Ok(Choice::Paper),
            x if x == Choice::Scissors as u8 => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

fn letter_to_choice(letter: &str) -> Option<Choice> {
    match letter {
        "A" => Some(Choice::Rock),
        "X" => Some(Choice::Rock),
        "B" => Some(Choice::Paper),
        "Y" => Some(Choice::Paper),
        "C" => Some(Choice::Scissors),
        "Z" => Some(Choice::Scissors),
        _ => None
    }
}

fn letter_to_offset(letter: &str) -> i8 {
    match letter {
        "X" => -1,
        "Y" => 0,
        "Z" => 1,
        _ => panic!("Unrecognized letter")
    }

}

fn score_for_matchup(me: &Choice, opponent: &Choice) -> u8 {
    let my_index = me.index() as i8;
    let opponent_index = opponent.index() as i8;
    let mut diff = my_index - opponent_index;
    if diff < 0 {
        diff += 3;
    }
    
    const BASE_SCORE: i8 = 3;
    let score = match diff {
        1 => 2 * BASE_SCORE,
        0 => BASE_SCORE,
        _ => 0
    };
    
    return score as u8;
}

pub fn main() {
    let mut count: u32 = 0;
    let mut score: u32 = 0;
    let mut new_score: u32 = 0;
    let file = File::open("res/dec02_input.txt").expect("Failed opening file!");
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.expect("Failed reading line");
        if line.is_empty() {
            continue;
        }

        const WRONG_FORMAT_MESSSAGE: &str = "File is in wrong format!";
        let split: Vec<&str> = line.split(" ").collect();
        let opponent_choice_string = split.get(0).expect(WRONG_FORMAT_MESSSAGE).to_uppercase();
        let my_choice_string = split.get(1).expect(WRONG_FORMAT_MESSSAGE).to_uppercase();

        let my_choice = letter_to_choice(&my_choice_string).expect(WRONG_FORMAT_MESSSAGE);
        let opponent_choice = letter_to_choice(&opponent_choice_string).expect(WRONG_FORMAT_MESSSAGE);
        let my_decision_index = ((3 + opponent_choice.index() as i8 + letter_to_offset(&my_choice_string)) % 3) as u8;
        let my_new_choice = Choice::try_from(my_decision_index).expect("Error on converting index to choice!");
        
        let choice_score = 1 + my_choice.index() as u32;
        let matchup_score = score_for_matchup(&my_choice, &opponent_choice) as u32;

        let new_choice_score = 1 + my_decision_index as u32;
        let new_matchup_score = score_for_matchup(&my_new_choice, &opponent_choice) as u32;

        let total_score: u32 = choice_score + matchup_score;
        let new_total_score: u32 = new_choice_score + new_matchup_score;

        score += total_score;
        new_score += new_total_score;
        count += 1;

        //println!("Total score: {} ({} + {}) (O: {} vs Me: {}) -> Score is now {}", total_score, choice_score, matchup_score, &opponent_choice_string, &my_choice_string, score);
    }

    println!("Score after {} lines: a: {}, b: {}", count, score, new_score);

}
