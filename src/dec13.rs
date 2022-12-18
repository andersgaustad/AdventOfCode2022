use std::{time::Instant, str::FromStr, cmp::Ordering};

const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";

// Options
const VERBOSE : bool = false; 

#[derive(Debug)]
struct DistressNode {
    content : Vec<DistressMember>
}

#[derive(Debug)]
enum DistressMember {
    Value(i32),
    Node(DistressNode)    
}

#[derive(Debug)]
struct DistressParseError;

#[derive(Eq, PartialEq)]
enum ParseMode {
    None,
    Bracket,
    Value
}

impl FromStr for DistressNode {
    type Err = DistressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("[").ok_or(DistressParseError)?.strip_suffix("]").ok_or(DistressParseError)?;
        //println!("Parsing stripped str {}", stripped);

        let mut parse_mode = ParseMode::None;
        let mut tokens = vec![];

        let mut parse_start = 0;
        let mut bracket_depth = 0;

        let mut vectorized = stripped.chars().collect::<Vec<char>>();
        vectorized.push('\0'); // Adding terminator allowing parsing of e.g. ints to finish inside loop
        for i in 0..vectorized.len() {
            let c = *vectorized.get(i).unwrap();
            match parse_mode {
                // Parse: None
                ParseMode::None => {
                    if c == '[' {
                        parse_mode = ParseMode::Bracket;
                        parse_start = i;
                        bracket_depth = 1;
                    } else if c.is_numeric() {
                        parse_mode = ParseMode::Value;
                        parse_start = i;
                    }
                },
                // Parse: Bracket
                ParseMode::Bracket => {
                    if c == '[' {
                        bracket_depth += 1;
                    }
                    if c == ']' {
                        bracket_depth -= 1;
                        if bracket_depth == 0 {
                            let parse_segment = &vectorized[parse_start..i+1].iter().collect::<String>();
                            //println!("Subset found: {}", parse_segment);
                            let subset = DistressNode::from_str(&parse_segment)?;
                            let wrapped = DistressMember::Node(subset);
                            tokens.push(wrapped);
                            parse_mode = ParseMode::None;
    
                        }
    
                    }
                },
                // Parse: Value
                ParseMode::Value => {

                    if !c.is_numeric() {
                        let parse_segment = &vectorized[parse_start..i].iter().collect::<String>();
                        let value = parse_segment.parse::<i32>().ok().ok_or(DistressParseError)?;
                        //println!("Pushed value {}", value);
                        tokens.push(DistressMember::Value(value));
                        parse_mode = ParseMode::None;
                    }
                },
            }

        }

        let result = DistressNode {content: tokens};
        return Ok(result);        
    }

}

impl std::fmt::Display for DistressNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
    
}

impl DistressNode {
    fn compared_to(&self, other : &DistressNode) -> Ordering {
        let max_items = self.content.len().max(other.content.len());
        for i in 0..max_items {
            let a = self.content.get(i);
            let b = other.content.get(i);

            if a.is_none() && b.is_none() {
                return Ordering::Equal;
            }

            if a.is_none() {
                return Ordering::Less;
            }

            if b.is_none() {
                return Ordering::Greater;
            }

            let a = a.unwrap();
            let b = b.unwrap();

            // Both values
            if let (DistressMember::Value(a_val), DistressMember::Value(b_val)) = (a, b) {
                let value_cmp = a_val.cmp(b_val);
                if value_cmp != Ordering::Equal {
                    return value_cmp;
                }

                continue;
            }

            // Both lists
            if let (DistressMember::Node(a_list), DistressMember::Node(b_list)) = (a, b) {
                let list_cmp = a_list.compared_to(b_list);
                if list_cmp != Ordering::Equal {
                    return list_cmp;
                }

                continue;
            }

            // A value, B list
            if let (DistressMember::Value(a_val), DistressMember::Node(b_list)) = (a, b) {
                let a_list = expand_to_list(a_val.to_owned());
                let list_cmp = a_list.compared_to(b_list);
                if list_cmp != Ordering::Equal {
                    return list_cmp;
                }

                continue;
            }

            // A list, B value
            if let (DistressMember::Node(a_list), DistressMember::Value(b_val)) = (a, b) {
                let b_list = expand_to_list(b_val.to_owned());
                let list_cmp = a_list.compared_to(&b_list);
                if list_cmp != Ordering::Equal {
                    return list_cmp;
                }

                continue;
            }

        }

        return Ordering::Equal;
    }

    fn as_string(&self) -> String {
        let collected = self.content.iter().map(|c| {
            match c {
                DistressMember::Value(val) => val.to_string(),
                DistressMember::Node(node) => node.as_string(),
            }
        }).collect::<Vec<String>>();

        let joined = collected.join(",");
        let string = format!("[{}]", joined);
        return string;
    }

}

fn expand_to_list(value : i32) -> DistressNode {
    let as_vector = vec![DistressMember::Value(value)];
    let node = DistressNode { content: as_vector};
    return node;
}

fn load_pairs() -> Vec<(DistressNode, DistressNode)> {
    let raw = std::fs::read_to_string("res/dec13_input.txt").expect("Failed reading file!");
    let split_into_pairs = raw.split("\n\n").filter(|s| !s.is_empty()).map(|blob| {
        let split_blob = blob.split("\n").collect::<Vec<&str>>();
        let left = split_blob.get(0).expect(WRONG_FORMAT_MESSSAGE);
        let right = split_blob.get(1).expect(WRONG_FORMAT_MESSSAGE);

        let left_node = DistressNode::from_str(&left).expect(WRONG_FORMAT_MESSSAGE);
        let right_node = DistressNode::from_str(&right).expect(WRONG_FORMAT_MESSSAGE);

        (left_node, right_node)
    }).collect::<Vec<(DistressNode, DistressNode)>>();

    return split_into_pairs;
}



pub fn main() {
    // Part a
    let pairs = load_pairs();
    let timer = Instant::now();
    println!("Loaded {} pairs", pairs.len());

    let mut sum = 0;
    for i in 0..pairs.len() {
        let pair = pairs.get(i).unwrap();
        let left = &pair.0;
        let right = &pair.1;

        let cmp = left.compared_to(right);

        if VERBOSE {
            println!("Compared pair {} ({} and {}) -> {:?}", i + 1, &left, &right, cmp);
        }
        if cmp == Ordering::Less {
            sum += i + 1;
        }
    }

    println!("Sum is {} (Took {} ms)", sum, timer.elapsed().as_millis());
}
