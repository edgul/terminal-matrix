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

fn random_number(n : usize) -> u32 {
    rand::thread_rng().gen_range(0..n as u32)
}

fn random_ascii() -> u8 {
    // todo: support unicode
    rand::thread_rng().gen_range(33..126)
}

fn clear_col(m: &mut Vec<Vec<char>>, c: usize) {
    for row in m.iter_mut() {
        row[c] = BCHAR;
    }
}

fn main() {
    println!("wake up, neo");

    let (col, row) = terminal::size().unwrap();
    let term_columns: usize = col as usize;
    let term_height : usize = row as usize;

    // feature flags
    let auto_quit_enabled = false;
    let blocks_enabled = true;
    let char_adding_enabled = true;
    let char_swapping_enabled = true;
    let sliding_viewport_enabled = true;
    let column_fade_enabled = true;

    let frame_period = time::Duration::from_millis(1);
    let animation_length = time::Duration::from_millis(10000);

    let mut matrix : Vec<Vec<char>> = Vec::new();
    matrix.push(vec![BCHAR; term_columns]);
    let mut recent_rows = vec![0; term_columns];

    let start = time::Instant::now();

    let mut stdout = stdout(); // Call the function to get the handle
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    // paint loop
    loop {
        // add new char to the matrix
        let col = random_number(term_columns) as usize;
        let row = recent_rows[col];
        if char_adding_enabled {
            let new_char = random_ascii() as char; 
            if row >= matrix.len() {
                matrix.push(vec![BCHAR; term_columns]);
            }
            matrix[row][col] = new_char;
            recent_rows[col] = row + 1;
        }

        // swap chars
        if char_swapping_enabled {
            let swap_char = random_ascii() as char; 
            let swap_col = random_number(term_columns) as usize;
            let swap_row = random_number(recent_rows[col]) as usize;
            matrix[swap_row][swap_col] = swap_char;
        }  

        // divide into blocks
        if blocks_enabled {
            let block_size = term_columns / 5;
            let divisible_numbers: Vec<usize> = (0..term_columns-1) // -1 to clear two columns
                .filter(|&x| x % block_size == 0)
                .collect();
            for i in divisible_numbers {
                clear_col(&mut matrix, i);
                clear_col(&mut matrix, i+1);
            }
        }

        // column "fade"
        if column_fade_enabled {
            let stream_height = 15;
            if recent_rows[col] > stream_height {
                matrix[recent_rows[col]-stream_height][col] = BCHAR;
            }
        }

        // sliding viewport
        if sliding_viewport_enabled {
            if row > (term_height as u32).try_into().unwrap() {
                matrix.remove(0);
                for i in recent_rows.iter_mut() {
                    if *i  > 0 {
                        *i -= 1;
                    }
                }
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
}
