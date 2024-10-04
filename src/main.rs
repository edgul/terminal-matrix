use std::{thread, time};

//use rand::prelude::*;

use rand;
use rand::Rng;

fn clear_term() {
    print!("\x1Bc");
}

fn main() {
    println!("Hello, world!");
    clear_term();

    let frame_period = time::Duration::from_millis(100);
    let animation_length = time::Duration::from_millis(5000); // 5 sec
    let start = time::Instant::now();
    loop { // animation loop
        
        // generate random unicode value
        let num = rand::thread_rng().gen_range(0..800); // todo: add exclusion list
        println!("{}", std::char::from_u32(num).unwrap_or('x'));
       
        // frame length control and exit
        thread::sleep(frame_period);
        if start.elapsed() >= animation_length {
            break;
        }
    }
}
