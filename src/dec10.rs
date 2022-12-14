use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

use image::ImageBuffer;

pub fn main() {
    const WIDTH : u32 = 40;
    const HEIGHT : u32 = 6;

    const EARLY_EXIT_AFTER : i32 = 220;
    let mut add_execution_queue = HashMap::new();

    let mut cycle = 1;
    let mut x : i32 = 1;
    let mut sum = 0;

    let mut image_buffer = image::ImageBuffer::new(WIDTH, HEIGHT);

    let file = File::open("res/dec10_input.txt").expect("Failed opening file!");
    let mut lines = BufReader::new(file).lines();
    loop {
        let i = cycle - 1;
        let x_pos = i % WIDTH;
        let y_pos = i / WIDTH;

        if y_pos > HEIGHT {
            break;
        }

        let sprite_delta = x.abs_diff(x_pos.try_into().unwrap());
        let color : u8 = if sprite_delta <= 1 {255} else {0};
        *image_buffer.get_pixel_mut(x_pos, y_pos) = image::Luma([color]);

        let line = lines.next();
        if let Some(line) = line  {
            let line = line.expect("Failed reading line");
            if line.is_empty() {
                continue;
            }

            //println!("-> {}", &line);

            const WRONG_FORMAT_MESSSAGE : &str = "File is in wrong format!";
            let split = line.split(" ").collect::<Vec<&str>>();

            // During
            let cycle_delay = split.len() as u32;
            let value = if cycle_delay == 2 {split.get(1).expect(WRONG_FORMAT_MESSSAGE).parse::<i32>().expect(WRONG_FORMAT_MESSSAGE)} else {0};
            let last_blocking_instruction = add_execution_queue.keys().max().unwrap_or(&cycle);
            add_execution_queue.insert(last_blocking_instruction + cycle_delay, value);

        }

        // After
        let pop = add_execution_queue.remove(&cycle);
        if let Some(pending_add) = pop {
            x += pending_add;
            println!("Add finished (C={}): x+={} -> {}", cycle, pending_add, x);
        }

        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            let signal_strength : i32 = cycle as i32 * x;
            println!("Signal at {}: {} (c={} * x={})", cycle, signal_strength, cycle, x);
            sum += signal_strength;
            if cycle >= EARLY_EXIT_AFTER.try_into().unwrap() {
                break;
            }


        }

        cycle += 1;

    }
    
    println!("Sum of signal strengths: {}", sum);
    image_buffer.save("res/dec10_out.png").expect("Error saving image");
}
