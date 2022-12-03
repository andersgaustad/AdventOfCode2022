use std::env;

mod dec01;
mod dec02;
mod dec03;

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
        _ => {
            println!("Unrecognized arg {}", arg);
            return Err(());
        }
        
    }

    return Ok(());
}
