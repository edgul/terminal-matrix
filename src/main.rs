use std::{thread, time};
use std::sync::{Arc, Mutex};

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

// reduce this value to increase the frequency of char swapping
static CHAR_SWAP_FACTOR : usize = 5;

fn random_number(n : usize) -> usize {
    rand::thread_rng().gen_range(0..n as u32) as usize
}

fn random_ascii() -> u16 {
    rand::thread_rng().gen_range(260..700)
}

fn main() {
    println!("wake up, neo");
    let (cols, rows) = terminal::size().unwrap();
    // Arc + Mutex for sharing matrix
    // since multi-read threads not used, no need for RwLock
    let matrix = Arc::new(Mutex::new(Matrix::new(rows as usize, cols as usize)));
    let matrix_thread = Arc::clone(&matrix); // this Arc will go to the thread

    let auto_quit_enabled = false;
    let auto_quit_timeout = time::Duration::from_millis(10000);
    let start = time::Instant::now();

    // matrix-mutate thread -- adds characters to the matrix
    thread::spawn(move || {
        let column_fade_enabled = true;
        let blocks_enabled = false; // perf regression
        let char_swapping_enabled = true;
        //
        let mut last_swap_diff = start;

        loop { // mutate-matrix loop
            let diff = time::Instant::now() - start;

            // for each column add character if enough time has passed for next char
            let mut matrix = matrix_thread.lock().unwrap();
            for ci in 0..matrix.num_cols() {
                let next = matrix.col_next_animation(ci);

                // time-based character adding, shouldn't drift
                if diff > time::Duration::from_millis(next) {
                    // slightly better hack than before
                    // we use first_animation bool to always make a new character
                    // the cursor square and then immediately overwrite it in the 
                    // next animation loop
                    let square_char = char::from_u32(0x2588 as u32);
                    if !matrix.col_first_animation(ci) {
                        let new_char = char::from_u32(random_ascii() as u32).unwrap();
                        let h = matrix.lead_index(ci);
                        let second = h.checked_sub(1);
                        if let Some(second_index) = second {
                            if second_index < matrix.num_rows() {
                                matrix.overwrite_char(second_index , ci, new_char);
                            }
                        }
                        matrix.append_char_to_column(ci, square_char.unwrap());
                    } else {
                        matrix.set_col_first_animation(ci, false);
                        matrix.append_char_to_column(ci, square_char.unwrap());
                    }

                    if column_fade_enabled {
                        let h = matrix.lead_index(ci);
                        let tail = h.checked_sub(matrix.tail_length(ci));
                        if let Some(tail_index) = tail {
                            if tail_index < matrix.num_rows() {
                                matrix.overwrite_char(tail.unwrap(), ci, matrix::BCHAR);
                            }
                        }
                    }
                    // we use the stream's animation priority to control the speed
                    // of each stream (ie how often it will animate)
                    let animation_period = 10 * matrix.col_priority(ci) as u64;
                    matrix.set_col_next_animation(ci, next + animation_period);
                }
            }

            // swap chars
            if char_swapping_enabled {
                let now = time::Instant::now();
                let swap_diff = now - last_swap_diff;
                if swap_diff > time::Duration::from_millis(CHAR_SWAP_FACTOR as u64) {
                    let swap_char = char::from_u32(random_ascii() as u32);
                    let swap_col = random_number(matrix.num_cols());
                    let col_lead = matrix.lead_index(swap_col);

                    // -1 gives us a buffer so swapping doesn't result in stray chars
                    let col_tail = col_lead.checked_sub(matrix.tail_length(swap_col) - 1);
                    if let Some(col_tail) = col_tail {
                        let swap_row = rand::thread_rng().gen_range(col_tail as u32..col_lead as u32) as usize;
                        if swap_row < matrix.num_rows() && swap_row > 0 {
                            matrix.overwrite_char(swap_row, swap_col, swap_char.unwrap());
                        }
                    }
                    last_swap_diff = now;
                }
            }  

            // todo: performance problems just being dropped in like this
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
        }
    });

    // main-thread continues into the paint loop
    let mut stdout = stdout(); // Call the function to get the handle
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    loop { // paint loop
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        { // scope the mutex so mutate loop can pickup during main thread sleep
            let matrix = matrix.lock().unwrap();
            let mut count = 0;
            for (_, row) in matrix.rows().into_iter().enumerate() {
                for (_, c) in row.chars().enumerate() {
                    print!("{}", c);
                }
                // todo: but I think we are repainting excessively,
                // doing the whole row when maybe not needed
                stdout.execute(cursor::MoveTo(0, count)).unwrap();
                count += 1;
            }  
        }
        if auto_quit_enabled {
            if start.elapsed() >= auto_quit_timeout {
                // don't join because the mutate thread will never finish
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}
