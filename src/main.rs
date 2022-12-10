use std::env;

mod dec01;
mod dec02;
mod dec03;
mod dec04;
// 5 missing in action
mod dec06;
mod dec07;
mod dec08;
mod dec09;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        println!("Give me an argument :)");
        return Err(());

    }

    let arg = args.get(1).unwrap().as_str();
    match arg {
        "1" => dec01::main(),
        "2" => dec02::main(),
        "3" => dec03::main(),
        "4" => dec04::main(),
        // 5 was tragically lost after driver crash
        "6" => dec06::main(),
        "7" => dec07::main(),
        "8" => dec08::main(),
        "9" => dec09::main(),
        _ => {
            println!("Unrecognized arg {}", arg);
            return Err(());
        }
        
    }

    return Ok(());
}
