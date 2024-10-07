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

mod matrix;
use matrix::Matrix;

static PRIORITY_COUNTER_INIT : usize = 3;
static CHAR_SWAP_FACTOR : usize = 5;

fn random_number(n : usize) -> usize {
    rand::thread_rng().gen_range(0..n as u32) as usize
}

fn random_ascii() -> u8 {
    rand::thread_rng().gen_range(33..126)
}

fn main() {
    println!("wake up, neo");

    let (cols, rows) = terminal::size().unwrap();
    let mut matrix = Matrix::new(rows as usize, cols as usize);

    // feature flags
    let auto_quit_enabled = false;
    let blocks_enabled = true;
    let char_swapping_enabled = true;
    let column_fade_enabled = true;

    let frame_period = time::Duration::from_millis(5);
    let animation_length = time::Duration::from_millis(10000);
    let mut priority_counter = PRIORITY_COUNTER_INIT;

    let start = time::Instant::now();

    let mut stdout = stdout(); // Call the function to get the handle
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    // paint loop
    loop {
        // add new char to the matrix
        for ci in 0..matrix.num_cols() {
            if priority_counter % matrix.col_priority(ci) == 0 {
                let new_char = random_ascii() as char; 
                matrix.append_char_to_column(ci, new_char);
            } 

            // column "fade"
            if column_fade_enabled {
                let h = matrix.lead_index(ci);
                let tail = h.checked_sub(matrix.tail_length(ci));
                if let Some(tail_index) = tail {
                    if tail_index < matrix.num_rows() {
                        matrix.overwrite_char(tail.unwrap(), ci, matrix::BCHAR);
                    }
                }
            }
        }

        // swap chars
        if char_swapping_enabled {
            for _ in 0..CHAR_SWAP_FACTOR {
                let swap_char = random_ascii() as char; 
                let swap_col = random_number(matrix.num_cols());
                let col_lead = matrix.lead_index(swap_col);

                // -1 gives us a buffer so swapping doesn't result in stray chars
                let col_tail = col_lead.checked_sub(matrix.tail_length(swap_col) - 1);
                if let Some(col_tail) = col_tail {
                    let swap_row = rand::thread_rng().gen_range(col_tail as u32..col_lead as u32) as usize;
                    if swap_row < matrix.num_rows() && swap_row > 0 {
                        matrix.overwrite_char(swap_row, swap_col, swap_char);
                    }
                }
            }
        }  

        // divide into blocks
        if blocks_enabled {
            let block_size = matrix.num_cols() / 5;
            if block_size > 4 {
                let divisible_numbers: Vec<usize> = (0..matrix.num_cols()-1) // -1 to clear two columns
                    .filter(|&x| x % block_size == 0)
                    .collect();
                for i in divisible_numbers {
                    matrix.clear_col(i);
                    matrix.clear_col(i+1);
                }
            }
        }

        // paint it
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        let mut count = 0;
        for row in matrix.rows() {
            print!("{}", row);
            stdout.execute(cursor::MoveTo(0, count)).unwrap();
            count += 1;
        }  

        priority_counter += 1;
        if priority_counter > 9999 {
            priority_counter = PRIORITY_COUNTER_INIT;
        }

        thread::sleep(frame_period); // animation speed

        // auto-quit
        if auto_quit_enabled {
            if start.elapsed() >= animation_length {
                break;
            }
        }
    }
}
