use std::{thread, time};
use rand;
use rand::Rng;

static BCHAR : char = ' ';
static COLUMNS: usize = 95; 
static VIEWPORT_HEIGHT : usize = 25;

fn clear_term() {
    print!("\x1Bc");
}

fn random_number(n : u32) -> u32 {
    rand::thread_rng().gen_range(0..n)
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
    let auto_quit_enabled = false;
    let blocks_enabled = true;

    let mut matrix : Vec<Vec<char>> = Vec::new();
    matrix.push(vec![BCHAR; COLUMNS]);

    let frame_period = time::Duration::from_millis(5);
    let animation_length = time::Duration::from_millis(5000); // 5 sec
    let start = time::Instant::now();

    // paint loop
    loop {
        clear_term();

        // add new char to the matrix
        let new_char = random_ascii() as char; 
        let col = random_number(COLUMNS as u32) as usize;
        let row = col_height(&matrix, col);
        if row == matrix.len() {
            matrix.push(vec![BCHAR; COLUMNS]);
        }
        matrix[row][col] = new_char;

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
        if row > VIEWPORT_HEIGHT {
            matrix.remove(0);
        }

        // paint it
        for row in matrix.iter() {
            let s : String = row.into_iter().collect();
            println!("{}", s);
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
