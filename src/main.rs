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
mod dec10;

mod dec12;
mod dec13;
mod dec14;
mod dec15;

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
        "10" => dec10::main(),

        "12" => dec12::main(),
        "13" => dec13::main(),
        "14" => dec14::main(),
        "15" => dec15::main(),
        _ => {
            println!("Unrecognized arg {}", arg);
            return Err(());
        }
        
    }

    return Ok(());
}
