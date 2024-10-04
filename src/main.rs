use std::{thread, time};
use rand;
use rand::Rng;

use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

static BCHAR : char = ' ';
static COLUMNS: usize = 80; 
static VIEWPORT_HEIGHT : usize = 25;

fn clear_term() {
    print!("\x1Bc");
}

fn random_number(n : usize) -> u32 {
    rand::thread_rng().gen_range(0..n as u32)
}

fn random_ascii() -> u8 {
    // todo: support unicode
    rand::thread_rng().gen_range(33..126)
}

fn col_height(m: &Vec<Vec<char>>, c: usize) -> usize {
    let mut count = 0;
    for row in m.iter() {
        if row[c] == BCHAR {
            return count;
        }
        count += 1;
    }
    return count
}

fn clear_col(m: &mut Vec<Vec<char>>, c: usize) {
    for row in m.iter_mut() {
        row[c] = BCHAR;
    }
}

fn main() {
    println!("wake up, neo");

    // feature flags
    // todo: switching to crossterm breaks CTRL+C exit; fix it before disabling auto-quit
    let auto_quit_enabled = true;
    let blocks_enabled = true;
    let char_adding_enabled = true;
    let char_swapping_enabled = false;
    let sliding_viewport_enabled = false;

    let frame_period = time::Duration::from_millis(5);
    let animation_length = time::Duration::from_millis(10000);

    let mut matrix : Vec<Vec<char>> = Vec::new();
    matrix.push(vec![BCHAR; COLUMNS]);

    let start = time::Instant::now();

    terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout(); // Call the function to get the handle
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    // paint loop
    loop {

        // add new char to the matrix
        let col = random_number(COLUMNS) as usize;
        let row = col_height(&matrix, col);
        if char_adding_enabled {
            let new_char = random_ascii() as char; 
            if row == matrix.len() {
                matrix.push(vec![BCHAR; COLUMNS]);
            }
            matrix[row][col] = new_char;
        }

        // swap chars
        if char_swapping_enabled {
            let swap_char = random_ascii() as char; 
            let swap_col = random_number(COLUMNS) as usize;
            let swap_row = random_number(col_height(&matrix, col)) as usize;
            //let swap_row = 0;
            matrix[swap_row][swap_col] = swap_char;
        }  

        // divide into blocks
        if blocks_enabled {
            let block_size = COLUMNS / 5;
            let divisible_numbers: Vec<usize> = (0..COLUMNS-1) // -1 to clear two columns
                .filter(|&x| x % block_size == 0)
                .collect();
            for i in divisible_numbers {
                clear_col(&mut matrix, i);
                clear_col(&mut matrix, i+1);
            }
        }

        // sliding viewport
        if sliding_viewport_enabled {
            if row > (VIEWPORT_HEIGHT as u32).try_into().unwrap() {
                matrix.remove(0);
            }
        } 

        // paint it
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        let mut count = 0;
        for row in matrix.iter() {
            let s : String = row.into_iter().collect();
            print!("{}", s);
            stdout.execute(cursor::MoveTo(0, count)).unwrap();
            count += 1;
        }  

        thread::sleep(frame_period); // animation speed

        // auto-quit
        if auto_quit_enabled {
            if start.elapsed() >= animation_length {
                break;
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
}
