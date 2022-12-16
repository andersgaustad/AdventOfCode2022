const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";

const NUMBER_OF_ROUNDS : u64 = 10_000;
const STRESS_MODE : bool = true;

struct Monkey {
    items : Vec<u64>,
    operation : Box<dyn OperationLogic>,
    const_arg_a : Option<u64>,
    const_arg_b : Option<u64>,
    divisible_test_val : u64,
    true_toss_index : usize,
    false_toss_index : usize,
    inspection_counter : u64,
    common_multiplum : u64,
}

impl Monkey {
    fn inspect(&mut self) -> Option<InspectionResult> {
        let item = self.items.pop();
        if item.is_none() {
            return None;
        }

        self.inspection_counter += 1;

        let item = item.unwrap();
        let a = self.const_arg_a.unwrap_or(item);
        let b = self.const_arg_b.unwrap_or(item);
        let stressed_item = self.operation.perform_operation(a, b);

        // Destress
        //println!("CMP = {}", self.common_multiplum);
        let destressed = if STRESS_MODE {stressed_item % self.common_multiplum} else {stressed_item / 3};
        let test = destressed % self.divisible_test_val == 0; 

        let new_monkey_index = if test {self.true_toss_index} else {self.false_toss_index};

        let result = InspectionResult {
            new_value: destressed,
            index: new_monkey_index
        };

        return Some(result);
    }
    
}

struct InspectionResult {
    new_value : u64,
    index : usize,
}

trait OperationLogic {
    fn perform_operation(&self, a: u64, b: u64) -> u64;
}

struct AddOperation {}
struct MulOperation {}

impl OperationLogic for AddOperation {
    fn perform_operation(&self, a: u64, b: u64) -> u64 {
        a + b
    }
}

impl OperationLogic for MulOperation {
    fn perform_operation(&self, a: u64, b: u64) -> u64 {
        a * b
    }
}

fn load_monkeys() -> Vec<Monkey> {
    let mut monkeys = vec![];

    let raw = std::fs::read_to_string("res/dec11_input.txt").expect("Failed opening file!");
    let segments = raw.split("\n\n");
    for segment in segments {
        let data = segment.split("\n").collect::<Vec<&str>>();

        // Starting items
        let starting_items_tokens = data.get(1).expect(WRONG_FORMAT_MESSSAGE).trim().split(" ").collect::<Vec<&str>>();
        let starting_item_slice = &starting_items_tokens[2..];
        let starting_items = starting_item_slice.iter().map(|string| {
            let sanitized = string.replace(",", "");
            let parsed = sanitized.parse::<u64>().expect(WRONG_FORMAT_MESSSAGE);
            parsed
        }).collect::<Vec<u64>>();
        //println!("Starting items: {:?}", starting_items);

        // Operation
        let operation_tokens = data.get(2).expect(WRONG_FORMAT_MESSSAGE).trim().split(" ").collect::<Vec<&str>>();
        let token_a = operation_tokens.get(3).expect(WRONG_FORMAT_MESSSAGE).parse::<u64>();
        let a = if token_a.is_ok() { Some(token_a.unwrap()) } else { None };
        let token_b = operation_tokens.get(5).expect(WRONG_FORMAT_MESSSAGE).parse::<u64>();
        let b = if token_b.is_ok() { Some(token_b.unwrap()) } else { None };
        let operation_token = operation_tokens.get(4).expect(WRONG_FORMAT_MESSSAGE);
        let operation : Box<dyn OperationLogic> = match operation_token {
            &"+" => Box::new(AddOperation {}),
            &"*" => Box::new(MulOperation {}),
            _ => panic!("{}", WRONG_FORMAT_MESSSAGE),
        };

        // Division
        let divison_tokens = data.get(3).expect(WRONG_FORMAT_MESSSAGE).trim().split(" ").collect::<Vec<&str>>();
        let division = divison_tokens.get(3).expect(WRONG_FORMAT_MESSSAGE).parse::<u64>().expect(WRONG_FORMAT_MESSSAGE);

        // True
        let true_tokens = data.get(4).expect(WRONG_FORMAT_MESSSAGE).trim().split(" ").collect::<Vec<&str>>();
        let true_index = true_tokens.get(5).expect(WRONG_FORMAT_MESSSAGE).parse::<usize>().expect(WRONG_FORMAT_MESSSAGE);


        // False
        let false_tokens = data.get(5).expect(WRONG_FORMAT_MESSSAGE).trim().split(" ").collect::<Vec<&str>>();
        let false_index = false_tokens.get(5).expect(WRONG_FORMAT_MESSSAGE).parse::<usize>().expect(WRONG_FORMAT_MESSSAGE);

        // Monkey
        let monkey = Monkey { 
            items: starting_items, 
            operation, 
            const_arg_a: a, 
            const_arg_b: b, 
            divisible_test_val: division, 
            true_toss_index: true_index, 
            false_toss_index: false_index ,
            inspection_counter : 0,
            common_multiplum : 1
        };

        monkeys.push(monkey);

    }

    let common_multiplum = monkeys.iter().map(|monkey| monkey.divisible_test_val).fold(1, |acc, x| acc * x);
    for m in monkeys.iter_mut() {
        m.common_multiplum = common_multiplum
    }

    return monkeys;

}

pub fn main() {
    let mut monkeys = load_monkeys();

    for _ in 1..NUMBER_OF_ROUNDS+1 {
        for i in 0..monkeys.len() {
            'inner: loop
            {
                let monkey = monkeys.get_mut(i).unwrap();
                let inspection = monkey.inspect();
                if let Some(result) = inspection {
                    let item = result.new_value;
                    let index = result.index;
                    assert!(i != index);

                    monkeys.get_mut(index).unwrap().items.push(item);
                }
                else {
                    break 'inner;
                }

            }
            
        }
    }

    let mut monkey_inspections = monkeys.iter().map(|monkey| monkey.inspection_counter).collect::<Vec<u64>>();
    println!("Monkey inspections: {:?}", monkey_inspections);
    monkey_inspections.sort_unstable_by(|a, b| b.cmp(a));

    let result : u64 = monkey_inspections.iter().take(2).product();
    println!("Monkey buisness: {}", result);

}
