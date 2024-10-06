
pub static BCHAR : char = ' ';

pub struct Matrix {
    matrix : Vec<Vec<char>>,
    last_rows : Vec<usize>,
}

impl Matrix {
    pub fn new(rows : usize, cols : usize) -> Self {
        let matrix = vec![vec![BCHAR; cols]; rows];
        let last_rows = vec![0; cols]; 
        Self{ matrix, last_rows }
    }

    pub fn num_cols(&self) -> usize {
        self.matrix[0].len()
    }

    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    pub fn append_char_to_stream(&mut self, col : usize, c : char) {
        let row = self.last_rows[col];
        if row >= self.matrix.len() {
            self.matrix.push(vec![BCHAR; self.num_cols()]);
        }
        self.matrix[row][col] = c;
        self.last_rows[col] = row + 1;
    }

    pub fn overwrite_char(&mut self, row : usize, col : usize, c : char) {
        self.matrix[row][col] = c;
    }

    pub fn clear_col(&mut self, col : usize) {
        for row in self.matrix.iter_mut() {
            row[col] = BCHAR;
        }
    }

    pub fn remove_row(&mut self, num : u8) {
        self.matrix.remove(num as usize);
        self.matrix.push(vec![BCHAR; self.num_cols()]);
        for j in self.last_rows.iter_mut() {
            if *j > 0 {
                *j -= 1;
            }
        }
    }

    pub fn column_height(&self, col : usize) -> usize {
        self.last_rows[col]
    }

    pub fn rows(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for row in self.matrix.iter() {
            res.push(row.into_iter().collect());
        }
        res 
    }
}
