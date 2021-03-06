use std::time::Instant;

pub enum Position {
    Row,
    Column,
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub matrix: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new_empty(row: usize, column: usize) -> Self {
        let mut new_matrix: Vec<Vec<i32>> = Vec::with_capacity(row);
        for _ in 0..row {
            new_matrix.push(vec![0;column]);
        }
        Self {
            rows: row,
            columns: column,
            matrix: new_matrix,
        }
    }
    
    pub fn new(data: Vec<Vec<i32>>) -> Self {
        let mut new_matrix = Self {
            rows: data.len(),
            columns: data[0].len(),
            matrix: data,
        };
        new_matrix.check_row_lengths();
        new_matrix.make_square();
        new_matrix
    }

    fn make_square(&mut self) {
        if self.rows == self.columns { return; }
        if self.rows > self.columns {
            let diff = self.rows - self.columns;
            for _ in 0..diff {
                self.add_column();
                self.columns += 1;
            }
        } else if self.rows < self.columns {
            let diff = self.columns - self.rows;
            for _ in 0..diff {
                self.add_row();
                self.rows += 1;
            }
        }
    }

    fn add_column(&mut self) {
        for row in 0..self.rows {
            self.matrix[row].push(0);
        }
    }

    fn add_row(&mut self) {
        self.matrix.push(vec![0; self.columns]);
    }

    fn check_row_lengths(&mut self) {
        let mut max_row_len = self.matrix.len();
        let mut min_row_len = self.matrix.len();
        
        for row in 0..self.rows {
            if self.matrix[row].len() > max_row_len {
                max_row_len = self.matrix[row].len();
            }
            if self.matrix[row].len() < min_row_len {
                min_row_len = self.matrix[row].len();
            }
        }

        if max_row_len != min_row_len {
            self.columns = max_row_len;
            for row in 0..self.rows {
                if self.matrix[row].len() < max_row_len {
                    for _ in 0..(max_row_len - self.matrix[row].len()) {
                        self.matrix[row].push(0);
                    }
                }
            }
        }
    }

    fn invert_matrix_values(&self) -> Self {
        let mut m = Self {
            rows: self.rows,
            columns: self.columns,
            matrix: self.matrix.clone()
        };

        for row in 0..m.rows {
            for col in 0..m.columns {
                m.matrix[row][col] *= -1;
            }
        }
        m
    }

    fn get(&self, position: Position, index: usize) -> Vec<i32> {
        match position {
            Position::Row => {
                return self.matrix[index].clone();
            },
            Position::Column => {
                let mut temp = Vec::new();
                for i in 0..self.columns {
                    temp.push(self.matrix[i][index]);
                }
                return temp;
            },
        }
    }

    fn find_min_row(&self, index: usize) -> i32 {
        *self.matrix[index].iter().min().unwrap()
    }

    fn find_min_col(&self, index: usize) -> i32 {
        *self.get(Position::Column, index).iter().min().unwrap()
    }
}

#[derive(Debug, Clone)]
struct Backup {
    assignment_mask: Matrix,
    assignment_count: usize,
    crossed_rows: Vec<i32>,
    crossed_columns: Vec<i32>,
}

impl Backup {
    fn new(madarska_metoda_obj: &MadarskaMetoda) -> Self {
        Self {
            assignment_mask: madarska_metoda_obj.assignment_mask.clone(),
            assignment_count: madarska_metoda_obj.assignment_count.clone(),
            crossed_rows: madarska_metoda_obj.crossed_rows.clone(),
            crossed_columns: madarska_metoda_obj.crossed_columns.clone(),
        }
    }
}

pub struct MadarskaMetoda { 
    pub starting_matrix: Matrix,
    pub calculating_matrix: Matrix,
    pub assignment_mask: Matrix,
    assignment_count: usize,
    crossed_rows: Vec<i32>,
    crossed_columns: Vec<i32>,
    backup: Option<Backup>,
    possible_assignments: Vec<(usize, usize)>,
}

impl MadarskaMetoda {

    pub fn new(starting_matrix: &Matrix) -> Self {
        Self {
            starting_matrix: starting_matrix.clone(),
            calculating_matrix: starting_matrix.clone(),
            assignment_mask: Matrix::new_empty(1, 1),
            assignment_count: 0,
            crossed_rows: Vec::new(),
            crossed_columns: Vec::new(),
            backup: None,
            possible_assignments: Vec::new(),
        }
    }

    pub fn solve(&mut self, maximize: Option<bool>) -> i32 {
        let mut do_max = false;
        if let Some(maximize) = maximize {
            do_max = maximize;
        }

        if do_max {
            self.calculating_matrix = self.calculating_matrix.invert_matrix_values();
        }

        self.first_step();
        loop {
            self.reset_assignment();
            self.get_assignment();
            
            if self.assignment_count == self.starting_matrix.rows {
                break;
            }

            self.second_step();


            let mut found_other_optimal_assignment = false;
            if self.third_step().is_err() {
                while self.assignment_count != self.starting_matrix.rows {
                    if self.backup.is_some() {
                        self.load_backup(self.backup.clone());
                        if let Some((row, col)) = self.possible_assignments.pop() {
                            self.make_assignment(row, col);
                        } else {
                            return -1;
                        }
                        self.get_assignment();

                    }
                }
                found_other_optimal_assignment = true;
                // return -1;
            }

            self.backup = None;

            if found_other_optimal_assignment { break; }
        }

        let mut result = 0;
        for row in 0..self.starting_matrix.rows {
            for col in 0..self.starting_matrix.columns {
                if self.assignment_mask.matrix[row][col] == 0 { continue; }
                result += self.starting_matrix.matrix[row][col];
            }
        }

        result
    }

    pub fn solve_timed(starting_matrix: &Matrix, maximize: Option<bool>) -> i32 {
        let timer = Instant::now();
        let res = MadarskaMetoda::new(starting_matrix).solve(maximize);
        println!("{:?}s", timer.elapsed().as_micros() as f64 / 1000000 as f64 );
        res
    }
        
    fn first_step(&mut self) {
        for i in 0..self.calculating_matrix.rows {
            let min = self.calculating_matrix.find_min_row(i);
            for j in 0..self.calculating_matrix.rows {
                self.calculating_matrix.matrix[i][j] -= min;
            }
        }

        for i in 0..self.calculating_matrix.columns {
            let min = self.calculating_matrix.find_min_col(i);
            for j in 0..self.calculating_matrix.columns {
                self.calculating_matrix.matrix[j][i] -= min;
            }
        }
    }

    fn get_assignment(&mut self) {
        
        loop {
            let mut change_occured = false;

            for row in 0..self.calculating_matrix.rows {
                let mut count = 0;
                let mut last_col = 0;
                if self.crossed_rows[row] == 1 { continue; }
                for col in 0..self.calculating_matrix.columns {
                    if self.crossed_columns[col] == 1 { continue; }
                    if self.calculating_matrix.matrix[row][col] == 0 {
                        count += 1;
                        last_col = col;
                    }
                }
                if count == 1 {
                    self.make_assignment(row, last_col);
                    change_occured = true;
                    break;
                }
            }
    
            if change_occured { continue; }
    
            for col in 0..self.calculating_matrix.columns {
                let mut count = 0;
                let mut last_row = 0;
                if self.crossed_columns[col] == 1 { continue; }
                for row in 0..self.calculating_matrix.rows {
                    if self.crossed_rows[row] == 1 { continue; }
                    if self.calculating_matrix.matrix[row][col] == 0 {
                        count += 1;
                        last_row = row;
                    }
                }
                if count == 1 {
                    self.make_assignment(last_row, col);
                    change_occured = true;
                    break;
                }
            }

            if change_occured { continue; }

            ////////////////////////////////////////////////////////////////////

            let mut arbitrary_selection_mask = Matrix::new_empty(self.starting_matrix.rows, self.starting_matrix.columns);

            for col in 0..self.calculating_matrix.columns {
                if self.crossed_columns[col] == 1 { continue; }
                for row in 0..self.calculating_matrix.rows {
                    if self.crossed_rows[row] == 1 { continue; }
                    if self.calculating_matrix.matrix[row][col] == 0 {
                        for r in (row + 1)..self.calculating_matrix.rows {
                            if self.crossed_rows[r] == 1 { continue; }
                            if self.calculating_matrix.matrix[r][col] == 0 {
                                arbitrary_selection_mask.matrix[row][col] += 1;                              
                                arbitrary_selection_mask.matrix[r][col] += 1;
                            }
                        }
                        
                    }
                }
            }

            let mut min_selection = -1;
            let mut selection_row = None;
            let mut selection_col = None;

            for row in 0..self.calculating_matrix.rows {
                if self.crossed_rows[row] == 1 { continue; }
                for col in 0..self.calculating_matrix.columns {
                    if self.crossed_columns[col] == 1 { continue; }
                    if self.calculating_matrix.matrix[row][col] == 0 {
                        for c in (col + 1)..self.calculating_matrix.columns {
                            if self.crossed_columns[c] == 1 { continue; }
                            if self.calculating_matrix.matrix[row][c] == 0 {
                                arbitrary_selection_mask.matrix[row][col] += 1;
                                arbitrary_selection_mask.matrix[row][c] += 1;
                            }
                        }
                        if min_selection == -1 || arbitrary_selection_mask.matrix[row][col] < min_selection {
                            min_selection = arbitrary_selection_mask.matrix[row][col];
                            selection_row = Some(row);
                            selection_col = Some(col);
                        }
                    }
                }
            }
           
            if selection_row.is_some() && selection_col.is_some() {
                self.create_backup(&arbitrary_selection_mask, selection_row.unwrap(), selection_col.unwrap());
                self.make_assignment(selection_row.unwrap(), selection_col.unwrap());
                change_occured = true;
            }

            ////////////////////////////////////////////////////////////////////

            if !change_occured { break; }
        }
    }

    fn make_assignment(&mut self, row: usize, col: usize) {
        self.crossed_rows[row] = 1;
        self.crossed_columns[col] = 1;
        self.assignment_mask.matrix[row][col] = 1;
        self.assignment_count += 1;
    }

    fn second_step(&mut self) {

        self.crossed_rows = vec![1; self.calculating_matrix.rows];
        self.crossed_columns = vec![0; self.calculating_matrix.columns];

        for row in 0..self.assignment_mask.rows {
            for col in 0..self.assignment_mask.columns {
                if self.assignment_mask.matrix[row][col] == 1 { self.crossed_rows[row] = 0; break; }
            }
        }

        loop {

            let mut change_occured = false;

            for row in 0..self.assignment_mask.rows {
                if self.crossed_rows[row] == 1 {
                    for col in 0..self.assignment_mask.columns {
                        if self.crossed_columns[col] == 1 { continue; }
                        if self.calculating_matrix.matrix[row][col] == 0 {
                            self.crossed_columns[col] = 1;
                            change_occured = true;
                        }
                    }
                }
            }

            for col in 0..self.assignment_mask.columns {
                if self.crossed_columns[col] == 1 {
                    for row in 0..self.assignment_mask.rows {
                        if self.crossed_rows[row] == 1 { continue; }
                        if self.assignment_mask.matrix[row][col] == 1 {
                            self.crossed_rows[row] = 1;
                            change_occured = true;
                        }
                    }
                }
            }

            if !change_occured { break; }
        }

        self.crossed_rows.iter_mut().for_each(|val| if *val == 0 { *val = 1} else { *val = 0});
    }

    fn third_step(&mut self) -> Result<bool, String> {
        let min = self.minimum();
        let min = match min {
            Some(m) => m,
            None => return Err("Min is 0".to_owned()),
        };
        for row in 0..self.calculating_matrix.rows {
            if self.crossed_rows[row] == 1 { continue; }
            for col in 0..self.calculating_matrix.columns {
                self.calculating_matrix.matrix[row][col] -= min;
            }
        }

        for col in 0..self.calculating_matrix.columns {
            if self.crossed_columns[col] == 0 { continue; }
            for row in 0..self.calculating_matrix.rows {
                self.calculating_matrix.matrix[row][col] += min;
            }
        }
        Ok(true)
    }

    fn minimum(&mut self) -> Option<i32> {
        let mut non_crossed = Vec::new();
        for row in 0..self.crossed_rows.len() {
            if self.crossed_rows[row] == 1 { continue; }
            for col in 0..self.crossed_columns.len() {
                if self.crossed_columns[col] == 1 { continue; }
                non_crossed.push(self.calculating_matrix.matrix[row][col]);
            }
        }

        non_crossed.into_iter().min()
    }

    fn create_backup(&mut self, arbitrary_selection_mask: &Matrix, row: usize, col: usize) {
        if self.backup.is_none() {
            let mut possible_assignments = Vec::new();
            let min_val = arbitrary_selection_mask.matrix[row][col];
            for row in 0..arbitrary_selection_mask.rows {
                if self.crossed_rows[row] == 1 { continue; }
                for col in 0..arbitrary_selection_mask.columns {
                    if self.crossed_columns[col] == 1 { continue; }
                    if arbitrary_selection_mask.matrix[row][col] == min_val {
                        possible_assignments.push((row, col));
                    }
                }
            }
            possible_assignments.reverse();
            self.backup = Some(Backup::new(&self));
            self.possible_assignments = possible_assignments;
        }
    }

    fn reset_assignment(&mut self) {
        self.assignment_count = 0;
        self.assignment_mask = Matrix::new_empty(self.calculating_matrix.rows, self.calculating_matrix.columns);
        self.crossed_rows = vec![0; self.calculating_matrix.rows];
        self.crossed_columns = vec![0; self.calculating_matrix.columns];
    }

    fn load_backup(&mut self, backup: Option<Backup>) {
        if let Some(backup) = backup {
            self.assignment_count = backup.assignment_count;
            self.assignment_mask = backup.assignment_mask;
            self.crossed_rows = backup.crossed_rows;
            self.crossed_columns = backup.crossed_columns;
        }
    }
}

struct Path {
    path: Vec<[usize;2]>,
    path_count: usize,
    starting_row: usize,
    starting_column: usize,
}

impl Path {
    fn new(row: usize) -> Self {
        Self {
            path: vec![[0;2];row * 2],
            path_count: 0,
            starting_row: 0,
            starting_column: 0,
        }
    }
}

pub struct MadarskaMetodaMunkres {
    pub starting_matrix: Matrix,
    calculating_matrix: Matrix,
    pub assignment_mask: Matrix,
    path: Path,
    crossed_rows: Vec<usize>,
    crossed_columns: Vec<usize>,
    step: usize,
}

impl MadarskaMetodaMunkres {

    pub fn new(matrica: &Matrix) -> Self {
        MadarskaMetodaMunkres {
            starting_matrix: matrica.clone(),
            calculating_matrix: matrica.clone(),
            assignment_mask: Matrix::new_empty(matrica.rows, matrica.columns),
            path: Path::new(matrica.rows),
            crossed_rows: vec![0;matrica.rows],
            crossed_columns: vec![0;matrica.columns],
            step: 1,
        }
    }

    fn first_step(&mut self) {

        for i in 0..self.calculating_matrix.rows {
            let min = self.calculating_matrix.find_min_row(i);
            for j in 0..self.calculating_matrix.rows {
                self.calculating_matrix.matrix[i][j] -= min;
            }
        }

        self.step = 2;
    }

    fn second_step(&mut self) {
        for row in 0..self.calculating_matrix.rows {
            for col in 0..self.calculating_matrix.columns {
                if self.calculating_matrix.matrix[row][col] == 0 && self.crossed_rows[row] == 0 && self.crossed_columns[col] == 0 {
                    self.assignment_mask.matrix[row][col] = 1;
                    self.crossed_rows[row] = 1;
                    self.crossed_columns[col] = 1;
                }
            }
        }

        self.reset_crossed();

        self.step = 3;
    }

    fn third_step(&mut self) {
        for row in 0..self.calculating_matrix.rows {
            for col in 0..self.calculating_matrix.columns {
                if self.assignment_mask.matrix[row][col] == 1 {
                    self.crossed_columns[col] = 1;
                }
            }
        }

        let mut col_count = 0;

        for col in 0..self.calculating_matrix.columns {
            if self.crossed_columns[col] == 1 {
                col_count += 1;
            }
        }

        if col_count >= self.calculating_matrix.columns || col_count >= self.calculating_matrix.rows {
            self.step = 7;
        } else {
            self.step = 4;
        }
    }

    fn get_noncrossed_zero(&self) -> Option<(usize, usize)>{
        for _row in 0..self.calculating_matrix.rows {
            if self.crossed_rows[_row] == 1 { continue; }
            for _col in 0..self.calculating_matrix.columns {
                if self.crossed_columns[_col] == 1 { continue; }
                if self.calculating_matrix.matrix[_row][_col] == 0 {
                    return Some((_row, _col));
                }
            }
        }

        None
    }

    fn is_star_in_row(&mut self, row: usize) -> bool {
        let mut output = false;
        for col in 0..self.calculating_matrix.columns {
            if self.assignment_mask.matrix[row][col] == 1 {
                output = true;
            }
        }
        output
    }

    fn get_star_in_row(&mut self, row: usize) -> Option<usize> {
        for col in 0..self.calculating_matrix.columns {
            if self.assignment_mask.matrix[row][col] == 1 {
                return Some(col);
            }
        }
        None
    }

    fn fourth_step(&mut self) {
        loop {
            let opt = self.get_noncrossed_zero();

            if opt.is_none() {
                self.step = 6;
                break;
            } else {
                let (row, mut column) = opt.unwrap();
                self.assignment_mask.matrix[row][column] = 2;
                if self.is_star_in_row(row) {
                    match self.get_star_in_row(row) {
                        Some(c) => column = c,
                        None => panic!("There should be a star in row, is_star_in_row(row) is wrong!"),
                    }
                    self.crossed_rows[row] = 1;
                    self.crossed_columns[column] = 0;
                } else {
                    self.step = 5;
                    self.path.starting_row = row;
                    self.path.starting_column = column;
                    break;
                }
            }
        }
    }

    fn get_star_row_index(&mut self, column: usize) -> Option<usize> {
        for row in 0..self.calculating_matrix.rows {
            if self.assignment_mask.matrix[row][column] == 1 {
                return Some(row);
            }
        }
        None
    }

    fn get_prime_column_index(&mut self, row: usize) -> Option<usize> {
        for col in 0..self.calculating_matrix.columns {
            if self.assignment_mask.matrix[row][col] == 2 {
                return Some(col);
            }
        }
        None
    }

    fn unstar_starred_star_primed(&mut self) {
        for p in 0..self.path.path_count {
            if self.assignment_mask.matrix[self.path.path[p][0]][self.path.path[p][1]] == 1 {
                self.assignment_mask.matrix[self.path.path[p][0]][self.path.path[p][1]] = 0;
            } else {
                self.assignment_mask.matrix[self.path.path[p][0]][self.path.path[p][1]] = 1;
            }
        }
    }

    fn reset_crossed(&mut self) {
        for i in 0..self.calculating_matrix.rows {
            self.crossed_rows[i] = 0;
            self.crossed_columns[i] = 0;
        }
    }

    fn reset_prime(&mut self) {
        for row in 0..self.calculating_matrix.rows {
            for col in 0..self.calculating_matrix.columns {
                if self.assignment_mask.matrix[row][col] == 2 {
                    self.assignment_mask.matrix[row][col] = 0;
                }
            }
        }
    }

    fn fifth_step(&mut self) {
        self.path.path_count = 1;
        self.path.path[0][0] = self.path.starting_row;
        self.path.path[0][1] = self.path.starting_column;

        loop {
            let row_opt = self.get_star_row_index(self.path.path[self.path.path_count - 1][1]);
            if let Some(row) = row_opt {
                self.path.path_count += 1;
                self.path.path[self.path.path_count - 1][0] = row;
                self.path.path[self.path.path_count - 1][1] = self.path.path[self.path.path_count - 2][1];
            } else {
                break;
            }
            let col_opt = self.get_prime_column_index(self.path.path[self.path.path_count - 1][0]);
            if let Some(col) = col_opt {
                self.path.path_count += 1;
                self.path.path[self.path.path_count - 1][0] = self.path.path[self.path.path_count - 2][0];
                self.path.path[self.path.path_count - 1][1] = col;
            }
        }

        self.unstar_starred_star_primed();
        self.reset_crossed();
        self.reset_prime();
        self.step = 3;
    }

    fn get_min_value(&mut self) -> i32 {
        let mut min = None;
        for row in 0..self.calculating_matrix.rows {
            for col in 0..self.calculating_matrix.columns {
                if self.crossed_rows[row] == 0 && self.crossed_columns[col] == 0 {
                    if min.is_none() {
                        min = Some(self.calculating_matrix.matrix[row][col]);
                    } else if min.unwrap() > self.calculating_matrix.matrix[row][col] {
                        min = Some(self.calculating_matrix.matrix[row][col]);
                    }
                }
            }
        }
        min.unwrap()
    }

    fn sixth_step(&mut self) {
        let min = self.get_min_value();
        for row in 0..self.calculating_matrix.rows {
            for col in 0..self.calculating_matrix.columns {
                if self.crossed_rows[row] == 1 {
                    self.calculating_matrix.matrix[row][col] += min;
                }
                if self.crossed_columns[col] == 0 {
                    self.calculating_matrix.matrix[row][col] -= min;
                }
            }
            self.step = 4;
        }
    }

    fn get_result(&mut self) -> i32 {
        let mut result = 0;
        for row in 0..self.calculating_matrix.rows {
            for col in 0..self.calculating_matrix.columns {
                if self.assignment_mask.matrix[row][col] == 1 {
                    result += self.starting_matrix.matrix[row][col];
                }
            }
        }
        result 
    }

    pub fn solve(&mut self, maximize: Option<bool>) -> i32 {
        let mut do_max = false;
        if let Some(maximize) = maximize {
            do_max = maximize;
        }

        if do_max {
            self.calculating_matrix = self.calculating_matrix.invert_matrix_values();
        }

        loop {
            match self.step {
                1 => self.first_step(),
                2 => self.second_step(),
                3 => self.third_step(),
                4 => self.fourth_step(),
                5 => self.fifth_step(),
                6 => self.sixth_step(),
                7 => return self.get_result(),
                _ => panic!("Invalid step"),
            }
        }
    }

}

/**************************************************/
/*                    TESTS                       */
/**************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    const RED: usize = 5;
    const STUPAC: usize = 5;

    #[test]
    fn new_empty_matrix() {
        let m = Matrix::new_empty(RED, STUPAC);
        assert_eq!(vec![vec![0;STUPAC];RED], m.matrix);
        assert_eq!(RED, m.rows);
        assert_eq!(STUPAC, m.columns);
    }
    #[test]
    fn new_matrix() {
        let m = Matrix::new(vec![vec![0;STUPAC];RED]);
        assert_eq!(vec![vec![0;STUPAC];RED], m.matrix);
        assert_eq!(RED, m.rows);
        assert_eq!(STUPAC, m.columns);
    }

    #[test]
    fn min_row() {
        let matrica = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![10, 8,  4, 5],
                vec![ 6, 2, 12, 3],
                vec![ 3, 5,  6, 9],
                vec![ 4, 7,  8, 6],
            ]
        };
        assert_eq!(4, matrica.find_min_row(0));
        assert_eq!(2, matrica.find_min_row(1));
        assert_eq!(3, matrica.find_min_row(2));
        assert_eq!(4, matrica.find_min_row(3));
    }

    #[test]
    fn min_col() {
        let matrica = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![10, 8,  4, 5],
                vec![ 6, 2, 12, 3],
                vec![ 3, 5,  6, 9],
                vec![ 4, 7,  8, 6],
            ]
        };
        assert_eq!(3, matrica.find_min_col(0));
        assert_eq!(2, matrica.find_min_col(1));
        assert_eq!(4, matrica.find_min_col(2));
        assert_eq!(3, matrica.find_min_col(3));
    }

    #[test]
    fn get() {
        let matrica = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![10, 8,  4, 5],
                vec![ 6, 2, 12, 3],
                vec![ 3, 5,  6, 9],
                vec![ 4, 7,  8, 6],
            ]
        };
        assert_eq!(vec![10, 8, 4, 5], matrica.get(Position::Row, 0));
        assert_eq!(vec![ 3, 5, 6, 9], matrica.get(Position::Row, 2));
        assert_eq!(vec![10, 6, 3, 4], matrica.get(Position::Column, 0));
        assert_eq!(vec![ 5, 3, 9, 6], matrica.get(Position::Column, 3));
    }

    #[test]
    fn first_step_test() {
        let matrica = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![10, 8,  4, 5],
                vec![ 6, 2, 12, 3],
                vec![ 3, 5,  6, 9],
                vec![ 4, 7,  8, 6],
            ]
        };
        let after = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![6, 4,  0, 0],
                vec![4, 0, 10, 0],
                vec![0, 2,  3, 5],
                vec![0, 3,  4, 1],
            ]
        };
        let mut mm = MadarskaMetoda::new(&matrica);
        mm.first_step();
        assert_eq!(after.matrix, mm.calculating_matrix.matrix);
    }

    #[test]
    fn get_assignment_test() {
        let matrica = Matrix::new(vec![
            vec![7, 4,  0, 0],
            vec![5, 0, 10, 0],
            vec![0, 1,  2, 4],
            vec![0, 2,  3, 0],
        ]);

        let test_assignment = Matrix::new(vec![
            vec![0, 0, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 1],
        ]);

        let mut mm = MadarskaMetoda::new(&matrica);
        mm.first_step();
        mm.reset_assignment();
        mm.get_assignment();
        assert_eq!(test_assignment.matrix, mm.assignment_mask.matrix);
    }

    #[test]
    fn second_step_test() {
        let matrica = Matrix::new(vec![
            vec![7, 4,  0, 0],
            vec![5, 0, 10, 0],
            vec![0, 1,  2, 4],
            vec![0, 2,  3, 0],
        ]);

        let crossed_rows = vec![1, 1, 1, 1];
        let crossed_cols = vec![0, 0, 0, 0];

        let mut mm = MadarskaMetoda::new(&matrica);
        mm.reset_assignment();
        mm.get_assignment();
        mm.second_step();
        assert_eq!((crossed_rows, crossed_cols), (mm.crossed_rows, mm.crossed_columns));

    }

    #[test]
    fn third_step_test() {
        let matrica = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![6, 4,  0, 0],
                vec![4, 0, 10, 0],
                vec![0, 2,  3, 5],
                vec![0, 3,  4, 1],
            ]
        };
        let after = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![7, 4,  0, 0],
                vec![5, 0, 10, 0],
                vec![0, 1,  2, 4],
                vec![0, 2,  3, 0],
            ]
        };
        let mut mm = MadarskaMetoda::new(&matrica);
        mm.reset_assignment();
        mm.get_assignment();
        mm.second_step();
        let _ = mm.third_step();
        assert_eq!(after.matrix, mm.calculating_matrix.matrix);
    }
    
    #[test]
    fn solve_test() {
        let matrica = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![10, 8,  4, 5],
                vec![ 6, 2, 12, 3],
                vec![ 3, 5,  6, 9],
                vec![ 4, 7,  8, 6],
            ]
        };

        let matrica2 = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![1, 5, 7, 1],
                vec![3, 1, 1, 7],
                vec![2, 1, 2, 4],
                vec![1, 3, 1, 3],
            ]
        };

        let matrica3 = Matrix {
            rows: 4,
            columns: 4,
            matrix: vec![
                vec![1, 5, 7, 1],
                vec![3, 1, 1, 7],
                vec![2, 1, 4, 1],
                vec![1, 3, 1, 3],
            ]
        };

        let matrica4 = Matrix::new(vec![
            vec![60, 59, 71, 15, 82],
            vec![21, 54, 63, 30, 92],
            vec![28,  7, 97,  5, 96],
            vec![70,  5, 95, 75, 31],
            vec![41, 64, 55, 85, 59],
        ]);

        let matrica5 = Matrix::new(vec![
            vec![25, 21, 26, 59, 96, 6, 19, 15, 73, 91],
            vec![74, 64, 13, 10, 10, 49, 87, 33, 91, 47],
            vec![45, 82, 2, 23, 67, 96, 78, 50, 8, 13],
            vec![97, 59, 61, 23, 59, 15, 62, 42, 39, 45],
            vec![74, 95, 78, 44, 86, 11, 25, 34, 86, 15],
            vec![17, 89, 95, 72, 98, 35, 42, 8, 99, 66],
            vec![19, 18, 75, 52, 84, 28, 7, 49, 32, 47],
            vec![53, 52, 46, 34, 80, 5, 70, 34, 29, 27],
            vec![21, 21, 77, 29, 41, 22, 2, 17, 21, 64],
            vec![31, 24, 61, 87, 55, 27, 47, 40, 37, 73],
        ]);

        let matrica6 = Matrix::new(vec![
            vec![28 ,27 ,8 ,15 ,46 ,52 ,55 ,93],
            vec![51 ,6 ,19 ,18 ,97 ,26 ,9 ,31],
            vec![30 ,61 ,40 ,74 ,64 ,34 ,36 ,93],
            vec![31 ,14 ,53 ,40 ,9 ,10 ,57 ,40],
            vec![85 ,21 ,27 ,77 ,78 ,24 ,24 ,84],
            vec![16 ,69 ,21 ,69 ,12 ,39 ,26 ,81],
            vec![57 ,5 ,80 ,65 ,21 ,14 ,68 ,59],
            vec![49 ,13 ,82 ,78 ,46 ,48 ,88 ,48],
        ]);

        let matrica7 = Matrix::new(vec![
            vec![1, 2],
            vec![3, 4],
        ]);

        let matrica8 = Matrix::new(vec![
            vec![167,153,151,194,72,192,45,102,83,146,116,14,51,43,162,143,133,119,155,189],
            vec![11,12,167,75,72,24,152,34,57,99,22,33,145,179,179,185,163,139,83,111],
            vec![70,173,72,19,115,83,164,17,101,90,32,120,128,197,49,196,11,101,122,185],
            vec![92,123,93,91,172,69,77,93,87,195,122,196,77,140,163,110,116,175,121,194],
            vec![190,136,154,12,24,96,32,96,172,61,65,159,97,185,142,11,132,70,100,169],
            vec![113,61,35,62,72,165,104,65,199,49,16,78,132,108,151,34,74,30,139,83],
            vec![141,19,122,17,126,186,172,110,135,85,130,158,197,83,32,190,155,168,172,182],
            vec![77,56,32,43,163,19,118,164,193,21,160,59,17,120,189,80,46,128,123,50],
            vec![109,195,164,166,11,164,151,106,160,111,182,94,97,106,184,17,37,144,25,38],
            vec![181,197,166,145,161,136,159,132,21,198,135,17,146,141,113,167,164,85,103,54],
            vec![66,143,24,124,49,165,90,152,122,197,14,167,101,141,146,102,88,10,17,126],
            vec![176,78,190,97,55,48,125,137,114,184,163,89,24,114,138,68,200,179,69,105],
            vec![175,71,180,56,98,10,111,104,82,120,136,35,88,17,33,44,62,183,91,20],
            vec![37,148,160,167,161,168,30,42,52,134,16,25,18,122,88,178,19,12,54,103],
            vec![55,54,165,110,77,101,64,183,29,64,12,195,104,39,137,148,190,86,181,137],
            vec![57,143,46,145,12,105,83,102,106,119,166,142,137,80,55,60,41,12,32,18],
            vec![71,85,176,186,132,13,115,178,152,124,79,121,94,62,57,148,127,56,193,116],
            vec![52,88,184,185,105,179,78,87,52,91,135,180,124,184,68,117,115,141,108,88],
            vec![51,76,15,137,21,199,182,142,33,26,20,87,148,110,104,120,152,154,74,27],
            vec![83,53,138,37,89,66,103,34,86,23,95,54,51,93,48,168,158,32,191,124],
        ]);
		
		let matrica9 = Matrix::new(vec![
            vec![2, 9, 2, 7, 1],
            vec![6, 8, 7, 6, 1],
			vec![4, 6, 5, 3, 1],
			vec![4, 2, 7, 3, 1],
			vec![5, 3, 9, 5, 1],
        ]);

        let matrica10 = Matrix::new(vec![
            vec![294, 947, 342, 547, 233, 752, 611, 538, 196, 697, 983, 555],
            vec![283, 486, 375, 823, 580, 555, 693, 213, 319, 523, 651, 414],
            vec![882, 255, 518, 793, 989, 919, 449, 744, 412, 268, 939, 221],
            vec![557, 238, 323, 592, 716, 239, 977, 484, 782, 744, 289, 702],
            vec![390, 707, 541, 600, 273, 550, 684, 755, 948, 418, 912, 931],
            vec![639, 392, 376, 774, 758, 630, 718, 363, 978, 805, 612, 490],
            vec![412, 961, 572, 576, 804, 591, 592, 531, 446, 155, 579, 732],
            vec![947, 300, 593, 442, 372, 911, 902, 514, 132, 667, 628, 120],
            vec![629, 114, 159, 798, 956, 103, 369, 739, 511, 332, 127, 229],
            vec![583, 284, 852, 248, 144, 990, 385, 307, 535, 150, 741, 158],
            vec![379, 932, 670, 383, 935, 653, 717, 482, 767, 760, 284, 346],
            vec![193, 720, 698, 867, 237, 617, 493, 413, 928, 889, 761, 132],
        ]);

        let matrica11 = Matrix::new(vec![
            vec![21, 10, 13, 25, 16, 16, 5],
            vec![16, 12, 23, 25, 16, 4, 24],
            vec![14, 13, 10, 23, 22, 24, 28],
            vec![11, 23, 16, 28, 25, 11, 24],
            vec![16, 9, 23, 20, 13, 29, 20],
            vec![4, 17, 9, 14, 11, 12, 24],
        ]);

        let matrica12 = Matrix::new(vec![
            vec![20, 0, 0, 0, 0],
            vec![0, 24, 0, 0, 0],
            vec![0, 0, 38, 0, 0],
            vec![0, 0, 0, 50, 0],
            vec![0, 0, 0, 0, 48],
        ]);

        let matrica13 = Matrix::new(vec![
            vec![5, 2, 5, 4, 2, 2, 4, 3, 2, 2],
            vec![4, 2, 2, 3, 2, 4, 3, 5, 2, 2],
            vec![2, 3, 5, 3, 2, 2, 4, 2, 3, 3],
            vec![3, 5, 2, 2, 4, 2, 2, 5, 2, 3],
            vec![5, 2, 4, 4, 4, 4, 3, 5, 5, 2],
            vec![5, 2, 3, 4, 3, 4, 2, 5, 3, 4],
            vec![5, 2, 5, 4, 4, 3, 4, 3, 3, 2],
            vec![4, 4, 3, 2, 2, 4, 3, 2, 3, 4],
            vec![5, 2, 3, 4, 5, 2, 5, 4, 4, 3],
            vec![3, 5, 2, 5, 3, 4, 3, 5, 3, 2],
        ]);

        let matrica14 = Matrix::new(vec![
            vec![2, 0, 1, 4, 4], 
            vec![1, 1, 3, 2, 1], 
            vec![2, 1, 3, 4, 1], 
            vec![3, 4, 4, 4, 4], 
            vec![4, 0, 4, 2, 0],
        ]);

        let matrica15 = Matrix::new(vec![
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 0, 0, 2],
            vec![1, 0, 2, 2, 0],
            vec![0, 1, 1, 2, 0],
            vec![1, 2, 2, 2, 1],
        ]);

        let matrica16 = Matrix::new(vec![
            vec![2, 1, 0, 0, 0],
            vec![2, 2, 1, 2, 1],
            vec![0, 1, 1, 2, 2],
            vec![2, 2, 2, 0, 1],
            vec![0, 1, 1, 0, 1],
        ]);

        let mut mm = MadarskaMetoda::new(&matrica);
        assert_eq!(15, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica2);
        assert_eq!(4, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica3);
        assert_eq!(4, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica4);
        assert_eq!(129, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica5);
        assert_eq!(138, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica6);
        assert_eq!(155, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica7);
        assert_eq!(5, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica8);
        assert_eq!(459, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica9);
        assert_eq!(13, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica10);
        assert_eq!(2848, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica11);
        assert_eq!(50, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica12);
        assert_eq!(0, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica13);
        assert_eq!(20, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica14);
        assert_eq!(7, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica15);
        assert_eq!(1, mm.solve(None));
        let mut mm = MadarskaMetoda::new(&matrica16);
        assert_eq!(2, mm.solve(None));

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        assert_eq!(15, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica2);
        assert_eq!(4, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica3);
        assert_eq!(4, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica4);
        assert_eq!(129, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica5);
        assert_eq!(138, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica6);
        assert_eq!(155, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica7);
        assert_eq!(5, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica8);
        assert_eq!(459, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica9);
        assert_eq!(13, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica10);
        assert_eq!(2848, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica11);
        assert_eq!(50, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica12);
        assert_eq!(0, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica13);
        assert_eq!(20, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica14);
        assert_eq!(7, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica15);
        assert_eq!(1, mm.solve(None));
        let mut mm = MadarskaMetodaMunkres::new(&matrica16);
        assert_eq!(2, mm.solve(None));
    }

    #[test]
    fn performance() {
        let matrica = Matrix::new(
        vec![
                vec![85, 78, 4, 47, 78, 46, 100, 62, 2, 49, 57, 4, 19, 95, 57, 29, 86, 67, 95, 65, 8, 73, 92, 24, 92, 81, 10, 69, 81, 99, 40, 83, 46, 49, 65, 34, 35, 38, 16, 92, 5, 16, 37, 42, 50, 45, 8, 3, 87, 7, 44, 52, 55, 89, 96, 93, 52, 17, 82, 26, 90, 38, 66, 76, 67, 62, 32, 71, 2, 51, 88, 70, 26, 69, 72, 8, 55, 99, 87, 78, 3, 60, 13, 84, 73, 52, 3, 88, 54, 48, 62, 95, 50, 60, 92, 4, 18, 24, 26, 97],
                vec![92, 88, 76, 45, 59, 46, 27, 5, 89, 14, 21, 64, 30, 93, 66, 75, 95, 74, 59, 75, 80, 96, 48, 60, 22, 8, 56, 4, 81, 41, 27, 60, 16, 19, 15, 2, 9, 99, 46, 95, 43, 96, 32, 94, 34, 28, 33, 74, 19, 40, 58, 43, 65, 40, 85, 73, 83, 6, 33, 56, 88, 97, 55, 91, 2, 80, 25, 98, 8, 28, 8, 84, 13, 30, 90, 43, 65, 88, 30, 72, 90, 36, 36, 56, 56, 47, 88, 97, 37, 43, 39, 91, 82, 42, 37, 95, 16, 8, 80, 33],
                vec![71, 15, 77, 52, 35, 30, 60, 31, 73, 37, 63, 84, 14, 34, 73, 56, 50, 48, 49, 85, 65, 15, 96, 24, 38, 43, 80, 65, 15, 31, 16, 16, 100, 92, 46, 4, 56, 78, 46, 3, 96, 54, 47, 44, 70, 84, 95, 89, 40, 8, 14, 69, 68, 36, 74, 45, 35, 61, 35, 38, 93, 43, 74, 21, 76, 58, 86, 22, 42, 72, 45, 62, 69, 11, 63, 26, 96, 16, 29, 32, 2, 84, 64, 5, 83, 16, 67, 81, 84, 7, 19, 5, 77, 22, 72, 1, 55, 19, 18, 74],
                vec![4, 38, 47, 68, 37, 64, 69, 73, 24, 41, 38, 15, 8, 13, 58, 33, 73, 32, 3, 20, 32, 86, 61, 38, 98, 83, 92, 77, 26, 85, 31, 14, 37, 65, 81, 31, 9, 81, 27, 65, 60, 17, 98, 62, 95, 90, 42, 48, 38, 41, 72, 71, 15, 21, 14, 22, 19, 7, 13, 11, 20, 21, 19, 98, 27, 31, 39, 41, 41, 74, 68, 14, 34, 11, 34, 38, 44, 58, 5, 80, 40, 88, 92, 74, 72, 40, 84, 42, 22, 80, 34, 46, 21, 11, 22, 39, 21, 41, 75, 22],
                vec![85, 5, 85, 51, 53, 6, 78, 75, 6, 100, 3, 83, 75, 17, 36, 62, 6, 80, 19, 99, 89, 11, 80, 17, 20, 67, 28, 37, 63, 88, 62, 83, 71, 4, 81, 3, 90, 48, 100, 8, 1, 31, 73, 67, 26, 69, 45, 82, 83, 41, 51, 85, 96, 69, 64, 15, 54, 70, 65, 95, 22, 78, 63, 14, 14, 25, 23, 19, 97, 41, 28, 10, 28, 38, 39, 67, 32, 84, 7, 37, 57, 93, 23, 28, 99, 58, 48, 65, 28, 59, 91, 72, 64, 86, 16, 89, 98, 88, 60, 27],
                vec![1, 88, 2, 85, 9, 7, 72, 56, 34, 19, 98, 30, 18, 41, 21, 5, 43, 4, 81, 49, 77, 41, 68, 99, 50, 47, 66, 6, 91, 68, 64, 69, 17, 58, 17, 55, 86, 27, 50, 56, 43, 57, 20, 47, 22, 74, 92, 96, 60, 88, 99, 66, 100, 31, 41, 41, 60, 36, 88, 42, 98, 83, 50, 31, 5, 42, 63, 34, 47, 11, 74, 41, 62, 75, 39, 36, 51, 91, 47, 36, 96, 68, 24, 100, 5, 43, 6, 77, 26, 17, 53, 57, 13, 71, 78, 54, 5, 32, 98, 77],
                vec![79, 77, 9, 29, 60, 38, 59, 74, 21, 16, 47, 82, 11, 72, 97, 63, 49, 74, 74, 1, 37, 79, 73, 42, 35, 27, 63, 89, 34, 61, 94, 51, 51, 43, 92, 5, 7, 94, 39, 6, 63, 14, 45, 21, 60, 43, 99, 45, 43, 25, 88, 40, 59, 90, 19, 61, 49, 24, 58, 29, 20, 81, 18, 93, 84, 66, 15, 23, 88, 94, 93, 17, 39, 94, 9, 86, 92, 1, 22, 87, 23, 48, 63, 48, 42, 12, 25, 30, 48, 33, 59, 73, 27, 99, 71, 60, 30, 57, 94, 15],
                vec![5, 28, 78, 38, 70, 52, 42, 60, 60, 29, 75, 45, 71, 50, 13, 93, 43, 85, 59, 84, 25, 77, 54, 37, 71, 33, 22, 13, 24, 2, 5, 84, 6, 17, 82, 92, 23, 7, 28, 93, 47, 20, 7, 94, 100, 11, 74, 11, 98, 8, 30, 62, 50, 31, 96, 55, 63, 42, 75, 92, 86, 36, 92, 77, 97, 67, 61, 49, 79, 46, 62, 12, 28, 79, 43, 95, 11, 74, 87, 76, 11, 58, 49, 99, 81, 47, 15, 66, 48, 46, 94, 21, 49, 60, 66, 26, 61, 51, 2, 25],
                vec![85, 99, 21, 17, 7, 64, 77, 7, 35, 6, 35, 68, 37, 2, 38, 100, 76, 95, 42, 86, 4, 70, 24, 69, 61, 31, 40, 24, 91, 38, 100, 49, 72, 39, 73, 89, 98, 78, 54, 83, 84, 71, 25, 99, 16, 74, 66, 50, 98, 94, 52, 24, 25, 26, 66, 72, 66, 53, 95, 56, 88, 79, 84, 88, 50, 9, 50, 58, 38, 85, 8, 3, 36, 55, 9, 13, 42, 23, 23, 41, 3, 1, 11, 99, 98, 84, 20, 42, 60, 78, 24, 25, 75, 100, 51, 36, 43, 18, 64, 99],
                vec![18, 61, 4, 79, 63, 32, 22, 55, 23, 66, 92, 89, 96, 57, 15, 32, 13, 82, 41, 34, 82, 73, 53, 70, 46, 85, 41, 42, 45, 41, 14, 78, 28, 67, 89, 37, 17, 5, 31, 83, 95, 24, 36, 51, 42, 76, 18, 84, 78, 2, 49, 63, 13, 62, 76, 88, 35, 31, 39, 14, 34, 15, 54, 96, 61, 27, 41, 32, 24, 88, 53, 12, 57, 71, 100, 85, 56, 17, 39, 55, 23, 81, 64, 70, 3, 22, 11, 21, 59, 73, 74, 42, 36, 1, 69, 76, 63, 21, 59, 75],
                vec![74, 52, 31, 83, 16, 64, 2, 23, 100, 83, 11, 71, 88, 33, 49, 100, 3, 10, 8, 88, 84, 91, 30, 18, 62, 37, 63, 38, 81, 16, 68, 2, 43, 100, 28, 84, 83, 2, 38, 31, 20, 67, 64, 38, 85, 99, 24, 30, 60, 60, 43, 73, 26, 42, 79, 27, 77, 78, 28, 41, 16, 88, 55, 69, 71, 99, 56, 5, 98, 23, 94, 5, 56, 58, 80, 84, 67, 85, 47, 17, 58, 69, 98, 9, 99, 37, 10, 19, 79, 96, 77, 43, 35, 22, 92, 65, 18, 71, 51, 48],
                vec![31, 37, 62, 75, 87, 27, 68, 1, 96, 21, 15, 90, 71, 85, 61, 71, 96, 26, 85, 63, 46, 68, 35, 64, 20, 44, 81, 94, 16, 82, 60, 62, 53, 93, 46, 94, 60, 69, 28, 62, 23, 29, 90, 51, 68, 46, 13, 85, 78, 51, 27, 31, 49, 68, 29, 68, 75, 61, 96, 24, 51, 53, 15, 12, 68, 24, 15, 8, 28, 53, 97, 92, 3, 98, 74, 89, 27, 74, 57, 33, 43, 87, 24, 85, 27, 10, 98, 81, 66, 28, 82, 82, 22, 65, 4, 77, 74, 86, 77, 8],
                vec![45, 24, 4, 66, 70, 9, 93, 36, 36, 64, 2, 70, 68, 54, 17, 3, 85, 27, 20, 26, 23, 57, 53, 2, 14, 48, 92, 65, 32, 8, 77, 85, 55, 66, 44, 50, 20, 39, 75, 28, 3, 59, 25, 35, 24, 72, 57, 90, 39, 82, 51, 34, 73, 26, 53, 50, 80, 51, 98, 63, 5, 72, 51, 84, 63, 61, 18, 37, 87, 68, 46, 99, 26, 85, 43, 38, 78, 59, 74, 33, 87, 68, 97, 54, 36, 54, 100, 53, 61, 79, 21, 98, 86, 84, 54, 83, 46, 84, 15, 85],
                vec![43, 17, 83, 59, 93, 43, 51, 7, 48, 97, 2, 72, 76, 64, 38, 87, 18, 55, 26, 4, 83, 35, 91, 49, 87, 84, 88, 67, 4, 5, 54, 52, 55, 96, 34, 49, 27, 7, 43, 94, 7, 14, 41, 27, 62, 24, 23, 25, 21, 45, 8, 57, 57, 83, 70, 17, 58, 48, 4, 94, 59, 12, 24, 46, 21, 19, 84, 99, 37, 40, 37, 83, 69, 22, 53, 48, 35, 78, 68, 51, 25, 29, 72, 36, 7, 92, 3, 88, 6, 64, 98, 11, 38, 79, 65, 74, 78, 68, 23, 19],
                vec![16, 5, 19, 54, 6, 65, 29, 50, 82, 77, 86, 83, 44, 56, 88, 61, 36, 7, 30, 20, 9, 65, 18, 97, 71, 13, 6, 38, 35, 76, 11, 16, 14, 17, 42, 100, 67, 79, 74, 67, 74, 39, 12, 92, 18, 37, 28, 15, 98, 15, 80, 28, 3, 44, 82, 34, 70, 26, 57, 17, 100, 73, 64, 35, 63, 93, 60, 37, 40, 96, 73, 43, 45, 47, 94, 16, 86, 99, 52, 82, 15, 69, 23, 26, 89, 2, 93, 87, 92, 7, 77, 57, 33, 25, 86, 43, 98, 31, 43, 99],
                vec![74, 3, 16, 55, 31, 72, 40, 83, 7, 59, 55, 33, 34, 86, 60, 91, 73, 79, 17, 24, 21, 55, 17, 78, 89, 61, 89, 17, 61, 16, 94, 52, 23, 66, 58, 71, 17, 48, 50, 80, 33, 62, 55, 81, 33, 93, 60, 7, 72, 32, 21, 46, 56, 27, 59, 57, 66, 23, 82, 75, 51, 59, 49, 29, 28, 72, 92, 52, 95, 7, 60, 38, 11, 36, 44, 81, 38, 99, 23, 27, 68, 8, 100, 38, 29, 52, 8, 75, 50, 97, 4, 2, 60, 90, 95, 33, 84, 98, 26, 77],
                vec![53, 79, 63, 25, 95, 10, 43, 12, 41, 37, 28, 69, 7, 55, 40, 70, 58, 81, 45, 30, 47, 66, 5, 84, 35, 74, 99, 46, 81, 7, 65, 25, 12, 33, 41, 28, 90, 13, 78, 15, 82, 17, 93, 31, 13, 72, 85, 92, 35, 97, 59, 92, 87, 55, 10, 67, 49, 73, 96, 17, 24, 22, 86, 6, 2, 39, 2, 48, 44, 49, 31, 33, 88, 6, 72, 72, 37, 72, 27, 45, 90, 88, 39, 68, 76, 5, 36, 49, 48, 25, 61, 60, 17, 1, 24, 65, 47, 17, 26, 66],
                vec![80, 32, 48, 64, 29, 70, 7, 18, 86, 78, 72, 69, 49, 14, 62, 4, 88, 67, 50, 63, 65, 87, 39, 36, 65, 52, 83, 39, 81, 88, 82, 56, 7, 6, 20, 56, 60, 93, 2, 22, 99, 73, 83, 90, 22, 51, 63, 55, 54, 40, 42, 98, 38, 68, 10, 53, 18, 95, 94, 7, 50, 30, 68, 40, 9, 57, 3, 46, 52, 84, 35, 89, 65, 37, 53, 79, 90, 69, 21, 87, 33, 48, 88, 7, 43, 88, 25, 64, 33, 27, 71, 20, 49, 13, 93, 39, 59, 66, 48, 96],
                vec![15, 12, 91, 72, 40, 43, 80, 11, 59, 12, 45, 16, 35, 45, 48, 2, 82, 23, 8, 14, 59, 23, 49, 61, 17, 24, 74, 50, 65, 60, 56, 41, 93, 62, 95, 10, 17, 69, 61, 23, 86, 11, 38, 23, 1, 92, 17, 35, 26, 10, 43, 77, 42, 46, 79, 59, 94, 45, 8, 52, 16, 37, 6, 42, 4, 52, 12, 96, 71, 27, 29, 73, 92, 81, 95, 70, 1, 69, 9, 20, 65, 40, 92, 55, 97, 55, 9, 12, 13, 63, 15, 52, 32, 19, 28, 80, 15, 92, 56, 12],
                vec![12, 21, 18, 88, 21, 60, 27, 63, 38, 90, 72, 61, 24, 18, 30, 39, 29, 100, 56, 5, 41, 50, 15, 76, 67, 89, 38, 20, 67, 62, 69, 61, 81, 20, 4, 69, 51, 93, 14, 27, 24, 100, 64, 26, 12, 67, 52, 98, 25, 47, 80, 73, 61, 99, 100, 28, 24, 82, 91, 30, 39, 40, 65, 23, 20, 95, 51, 85, 25, 64, 68, 85, 7, 5, 8, 83, 34, 39, 5, 89, 85, 43, 32, 46, 7, 83, 10, 13, 70, 21, 8, 7, 8, 10, 37, 64, 17, 38, 3, 48],
                vec![12, 85, 92, 56, 42, 39, 48, 80, 3, 40, 31, 82, 7, 73, 54, 23, 84, 46, 18, 50, 14, 8, 13, 37, 14, 61, 18, 97, 2, 93, 52, 30, 58, 65, 9, 30, 3, 5, 53, 49, 29, 34, 58, 71, 87, 90, 44, 83, 68, 83, 21, 35, 2, 67, 35, 44, 60, 91, 24, 87, 75, 98, 37, 71, 16, 41, 96, 98, 42, 57, 46, 59, 80, 10, 77, 22, 16, 51, 90, 40, 50, 15, 30, 23, 43, 20, 54, 56, 21, 65, 64, 76, 52, 60, 83, 38, 69, 96, 50, 35],
                vec![83, 41, 5, 71, 15, 26, 96, 39, 81, 65, 98, 93, 23, 75, 62, 24, 25, 40, 35, 77, 73, 65, 51, 93, 69, 16, 63, 95, 66, 90, 45, 89, 71, 81, 19, 2, 27, 10, 58, 70, 39, 6, 78, 3, 39, 23, 26, 82, 86, 32, 64, 98, 95, 59, 19, 68, 88, 50, 6, 10, 29, 87, 5, 94, 91, 69, 22, 89, 2, 21, 83, 71, 39, 29, 69, 50, 98, 51, 50, 78, 66, 65, 43, 6, 73, 26, 46, 53, 43, 19, 37, 80, 76, 97, 25, 6, 41, 36, 76, 42],
                vec![91, 2, 4, 2, 34, 69, 50, 75, 40, 38, 58, 98, 19, 25, 49, 14, 49, 21, 53, 72, 85, 67, 76, 80, 85, 79, 91, 77, 39, 33, 59, 80, 78, 18, 60, 59, 3, 60, 11, 88, 15, 90, 20, 16, 70, 89, 97, 96, 90, 10, 49, 12, 58, 20, 63, 12, 16, 19, 53, 86, 45, 18, 20, 52, 22, 54, 37, 56, 45, 70, 61, 1, 80, 75, 95, 39, 87, 95, 92, 67, 94, 32, 52, 84, 83, 76, 24, 71, 84, 17, 65, 89, 90, 97, 92, 85, 80, 36, 79, 100],
                vec![54, 14, 24, 88, 92, 48, 28, 68, 64, 20, 61, 79, 78, 49, 44, 36, 12, 78, 38, 5, 26, 35, 49, 17, 61, 72, 1, 100, 41, 3, 67, 41, 55, 15, 19, 16, 65, 93, 76, 35, 37, 5, 37, 81, 52, 45, 4, 14, 34, 41, 36, 37, 52, 58, 69, 23, 36, 60, 79, 80, 86, 73, 77, 2, 25, 44, 75, 68, 37, 55, 86, 55, 97, 39, 2, 55, 52, 49, 48, 88, 77, 53, 24, 35, 37, 55, 29, 78, 11, 83, 5, 62, 52, 52, 82, 64, 23, 92, 20, 84],
                vec![26, 58, 93, 18, 4, 22, 50, 98, 44, 10, 68, 78, 19, 18, 100, 53, 72, 33, 47, 85, 92, 9, 89, 16, 63, 61, 2, 63, 12, 72, 77, 36, 61, 32, 8, 12, 88, 79, 91, 33, 49, 63, 32, 58, 43, 80, 77, 47, 16, 92, 62, 95, 21, 25, 78, 18, 94, 66, 39, 34, 93, 63, 62, 1, 43, 8, 13, 28, 49, 74, 20, 66, 54, 69, 27, 17, 32, 22, 1, 63, 2, 59, 100, 27, 89, 56, 15, 75, 53, 75, 24, 72, 74, 42, 29, 55, 95, 46, 70, 50],
                vec![50, 81, 15, 27, 60, 52, 65, 34, 20, 74, 27, 78, 83, 28, 10, 33, 73, 99, 8, 94, 56, 11, 21, 15, 58, 88, 23, 92, 24, 59, 74, 4, 49, 57, 6, 15, 41, 34, 31, 16, 4, 26, 24, 24, 89, 34, 87, 8, 20, 92, 47, 9, 22, 78, 72, 77, 74, 35, 20, 3, 71, 36, 34, 3, 16, 67, 38, 57, 87, 14, 65, 16, 48, 78, 82, 8, 16, 100, 88, 32, 15, 63, 40, 95, 34, 21, 98, 56, 46, 48, 44, 44, 51, 89, 81, 20, 35, 98, 83, 40],
                vec![23, 54, 64, 89, 63, 55, 30, 73, 70, 54, 13, 53, 76, 65, 5, 72, 24, 25, 53, 78, 61, 93, 66, 27, 60, 74, 17, 92, 26, 96, 35, 44, 91, 98, 26, 84, 46, 23, 91, 51, 54, 37, 53, 72, 84, 20, 84, 7, 17, 31, 55, 46, 40, 10, 73, 40, 61, 47, 69, 6, 24, 90, 78, 90, 55, 97, 81, 32, 59, 41, 67, 59, 75, 48, 13, 31, 27, 51, 92, 98, 66, 16, 91, 71, 67, 84, 42, 28, 71, 58, 91, 100, 24, 45, 27, 66, 18, 23, 57, 60],
                vec![67, 8, 71, 90, 36, 87, 15, 42, 24, 95, 1, 56, 6, 43, 98, 31, 23, 84, 63, 97, 40, 31, 5, 57, 11, 22, 28, 18, 96, 34, 90, 20, 68, 30, 3, 41, 4, 15, 94, 91, 20, 43, 51, 87, 31, 11, 99, 63, 56, 55, 38, 36, 80, 17, 62, 57, 78, 18, 45, 45, 60, 78, 80, 96, 46, 85, 79, 60, 56, 64, 27, 39, 67, 14, 59, 44, 3, 58, 57, 69, 79, 41, 41, 45, 16, 19, 100, 29, 44, 90, 53, 16, 30, 23, 45, 71, 10, 99, 94, 95],
                vec![99, 69, 69, 30, 60, 64, 54, 75, 83, 83, 100, 36, 51, 77, 96, 91, 26, 65, 4, 85, 55, 47, 79, 84, 89, 7, 10, 29, 71, 29, 8, 65, 35, 80, 73, 58, 33, 81, 91, 10, 78, 22, 50, 20, 49, 97, 52, 76, 99, 68, 95, 3, 38, 17, 41, 17, 58, 68, 31, 99, 94, 45, 62, 83, 57, 66, 34, 69, 6, 27, 18, 6, 7, 58, 15, 75, 52, 81, 42, 23, 79, 5, 15, 19, 93, 17, 23, 21, 9, 39, 40, 39, 34, 44, 96, 53, 55, 49, 8, 30],
                vec![88, 42, 35, 47, 85, 74, 33, 24, 67, 50, 41, 30, 85, 70, 92, 23, 42, 37, 78, 24, 86, 98, 57, 5, 53, 92, 60, 52, 65, 49, 76, 20, 95, 27, 3, 10, 61, 73, 85, 34, 81, 41, 50, 86, 48, 35, 92, 57, 45, 44, 80, 75, 91, 85, 31, 29, 60, 99, 85, 26, 40, 56, 55, 86, 90, 17, 56, 84, 30, 85, 90, 29, 29, 2, 51, 80, 82, 35, 8, 34, 63, 76, 97, 21, 16, 1, 22, 59, 76, 89, 56, 57, 12, 75, 40, 32, 29, 10, 9, 11],
                vec![95, 52, 67, 43, 2, 33, 100, 46, 89, 97, 43, 67, 96, 8, 69, 33, 9, 87, 63, 3, 59, 2, 47, 59, 23, 51, 94, 32, 39, 69, 26, 28, 72, 14, 52, 67, 19, 10, 31, 16, 84, 38, 6, 75, 74, 3, 51, 14, 98, 50, 52, 31, 18, 59, 89, 57, 93, 21, 100, 98, 6, 88, 43, 16, 37, 5, 50, 10, 90, 51, 70, 43, 80, 56, 98, 14, 29, 70, 43, 81, 20, 22, 69, 59, 49, 30, 34, 62, 20, 53, 8, 69, 27, 98, 82, 27, 76, 45, 42, 17],
                vec![83, 78, 37, 27, 71, 99, 7, 95, 39, 66, 5, 62, 36, 85, 29, 64, 51, 65, 54, 61, 63, 36, 57, 57, 54, 32, 71, 32, 91, 75, 48, 66, 18, 71, 61, 87, 72, 97, 49, 98, 58, 92, 95, 61, 47, 92, 79, 50, 31, 32, 15, 44, 53, 31, 17, 9, 1, 6, 97, 93, 80, 17, 47, 17, 43, 45, 13, 27, 53, 11, 26, 19, 18, 82, 18, 64, 20, 13, 83, 76, 71, 56, 45, 91, 52, 92, 10, 27, 15, 48, 72, 12, 16, 57, 63, 65, 32, 41, 91, 25],
                vec![84, 67, 26, 77, 4, 52, 83, 45, 74, 83, 82, 41, 37, 44, 91, 23, 84, 7, 61, 83, 29, 17, 10, 60, 3, 10, 35, 18, 2, 52, 22, 47, 95, 22, 52, 19, 9, 26, 2, 39, 17, 1, 82, 90, 85, 78, 22, 5, 15, 14, 35, 73, 12, 88, 9, 86, 69, 40, 51, 13, 9, 55, 79, 90, 94, 29, 16, 98, 39, 14, 28, 24, 66, 73, 2, 62, 95, 42, 5, 46, 14, 99, 58, 38, 65, 14, 7, 95, 7, 17, 8, 19, 87, 21, 55, 27, 18, 56, 31, 64],
                vec![53, 23, 92, 8, 90, 5, 96, 75, 93, 65, 77, 33, 71, 35, 68, 45, 15, 49, 7, 76, 67, 80, 72, 83, 80, 6, 86, 78, 36, 16, 19, 95, 37, 76, 57, 23, 93, 54, 2, 64, 68, 12, 63, 42, 66, 20, 73, 44, 65, 12, 91, 99, 3, 98, 70, 48, 10, 40, 15, 16, 73, 33, 94, 8, 52, 22, 33, 6, 16, 92, 81, 70, 19, 45, 29, 93, 50, 73, 90, 22, 18, 85, 82, 90, 8, 31, 53, 21, 20, 69, 11, 81, 48, 93, 92, 86, 58, 40, 87, 70],
                vec![81, 19, 3, 52, 45, 79, 5, 8, 13, 2, 8, 6, 79, 58, 87, 32, 50, 12, 32, 91, 14, 64, 84, 15, 11, 42, 100, 44, 4, 56, 44, 45, 15, 18, 47, 99, 43, 85, 5, 53, 47, 79, 86, 73, 30, 63, 44, 45, 26, 66, 99, 79, 32, 96, 66, 27, 10, 37, 55, 76, 40, 14, 100, 67, 18, 47, 28, 1, 39, 53, 60, 57, 42, 98, 61, 31, 53, 73, 100, 64, 67, 32, 52, 86, 98, 34, 73, 96, 95, 29, 55, 58, 83, 60, 69, 63, 7, 7, 67, 88],
                vec![83, 40, 90, 82, 11, 44, 93, 30, 8, 53, 77, 72, 73, 53, 44, 43, 11, 44, 71, 72, 32, 62, 26, 45, 89, 42, 94, 16, 17, 1, 18, 48, 43, 64, 10, 38, 89, 41, 67, 40, 75, 75, 82, 60, 20, 77, 28, 40, 5, 38, 16, 51, 66, 65, 70, 46, 93, 81, 88, 25, 10, 15, 5, 54, 75, 97, 50, 24, 81, 36, 96, 6, 26, 29, 52, 70, 15, 61, 50, 95, 46, 7, 92, 79, 72, 12, 22, 49, 57, 62, 60, 24, 92, 71, 11, 77, 54, 80, 37, 100],
                vec![60, 17, 15, 29, 33, 29, 35, 4, 79, 42, 71, 82, 95, 94, 73, 6, 75, 80, 76, 32, 5, 56, 23, 6, 92, 5, 37, 90, 32, 56, 60, 77, 95, 37, 70, 76, 56, 76, 36, 47, 74, 33, 27, 55, 15, 49, 68, 49, 42, 8, 97, 15, 39, 19, 24, 53, 13, 69, 33, 93, 81, 75, 48, 26, 42, 30, 29, 67, 10, 10, 44, 39, 35, 85, 76, 8, 32, 44, 50, 8, 21, 11, 56, 53, 47, 98, 75, 57, 47, 29, 39, 96, 8, 51, 89, 4, 83, 46, 57, 80],
                vec![14, 32, 46, 38, 29, 98, 78, 69, 99, 81, 5, 20, 85, 84, 4, 21, 17, 97, 36, 89, 92, 20, 24, 88, 71, 18, 10, 62, 20, 15, 83, 60, 58, 41, 83, 36, 47, 84, 67, 84, 7, 14, 51, 43, 2, 51, 30, 10, 97, 74, 78, 97, 18, 87, 41, 55, 71, 83, 52, 32, 62, 96, 62, 13, 76, 65, 13, 78, 74, 59, 13, 77, 80, 43, 44, 7, 70, 17, 29, 68, 51, 12, 7, 35, 41, 5, 73, 52, 79, 45, 44, 70, 99, 21, 15, 37, 55, 28, 32, 30],
                vec![89, 48, 6, 86, 51, 19, 66, 39, 88, 65, 99, 96, 16, 77, 75, 38, 29, 17, 88, 48, 70, 61, 6, 79, 71, 25, 12, 8, 96, 25, 60, 13, 49, 56, 41, 71, 39, 70, 8, 29, 57, 67, 18, 92, 9, 36, 5, 100, 22, 64, 77, 84, 44, 64, 43, 30, 4, 86, 36, 40, 80, 28, 98, 75, 51, 76, 31, 55, 44, 38, 79, 59, 16, 46, 72, 59, 61, 55, 7, 29, 81, 27, 41, 33, 17, 90, 49, 63, 94, 48, 77, 4, 8, 97, 18, 72, 93, 45, 1, 47],
                vec![93, 36, 12, 41, 68, 41, 35, 44, 68, 87, 100, 100, 25, 94, 59, 38, 52, 34, 47, 19, 77, 52, 31, 55, 6, 93, 31, 94, 32, 57, 56, 62, 72, 31, 23, 14, 53, 71, 31, 55, 11, 95, 27, 79, 66, 39, 45, 55, 98, 84, 81, 75, 89, 14, 54, 20, 10, 81, 74, 65, 99, 64, 61, 47, 13, 52, 83, 18, 49, 9, 18, 46, 26, 59, 59, 41, 56, 48, 20, 80, 66, 49, 88, 85, 84, 62, 10, 89, 78, 70, 45, 3, 84, 50, 74, 6, 28, 97, 3, 30],
                vec![51, 92, 91, 53, 26, 29, 7, 50, 83, 10, 42, 76, 40, 91, 72, 78, 13, 72, 93, 41, 74, 32, 75, 57, 58, 4, 72, 44, 12, 61, 12, 85, 24, 17, 22, 8, 41, 97, 50, 42, 87, 50, 85, 70, 56, 31, 100, 4, 46, 85, 88, 61, 60, 6, 5, 41, 68, 79, 81, 56, 57, 1, 32, 78, 3, 2, 94, 86, 94, 52, 58, 94, 65, 7, 31, 6, 4, 13, 80, 79, 1, 60, 58, 59, 49, 41, 82, 51, 87, 82, 15, 65, 84, 71, 17, 100, 18, 94, 10, 8],
                vec![70, 2, 23, 6, 64, 37, 30, 36, 38, 49, 43, 32, 67, 33, 11, 6, 74, 1, 35, 17, 91, 33, 40, 100, 80, 91, 47, 18, 50, 90, 99, 74, 2, 100, 42, 40, 77, 53, 5, 4, 53, 74, 9, 27, 44, 26, 79, 8, 18, 74, 13, 45, 59, 36, 81, 92, 53, 55, 67, 75, 7, 52, 37, 9, 3, 10, 62, 48, 73, 28, 56, 27, 67, 98, 40, 38, 20, 16, 29, 43, 44, 46, 52, 90, 45, 55, 48, 54, 86, 53, 41, 91, 20, 65, 12, 86, 11, 66, 84, 42],
                vec![81, 85, 49, 53, 39, 72, 42, 69, 82, 1, 18, 90, 70, 84, 11, 56, 78, 5, 28, 60, 98, 33, 78, 29, 57, 29, 17, 54, 58, 41, 70, 54, 7, 61, 88, 92, 25, 12, 14, 39, 91, 19, 88, 78, 24, 43, 10, 47, 54, 42, 35, 3, 14, 86, 26, 31, 14, 35, 81, 78, 49, 69, 1, 68, 54, 36, 11, 28, 13, 51, 2, 64, 45, 27, 93, 95, 87, 32, 54, 73, 51, 45, 53, 53, 47, 83, 59, 68, 84, 72, 94, 22, 4, 3, 91, 52, 51, 3, 54, 89],
                vec![53, 91, 83, 51, 19, 86, 98, 40, 13, 57, 54, 21, 18, 17, 18, 40, 56, 97, 14, 5, 92, 56, 49, 89, 73, 85, 7, 94, 50, 45, 10, 5, 49, 95, 42, 29, 2, 37, 31, 61, 58, 62, 85, 53, 15, 8, 22, 8, 94, 4, 25, 20, 31, 8, 16, 65, 45, 1, 37, 93, 34, 51, 1, 94, 39, 41, 25, 38, 96, 37, 73, 50, 6, 46, 64, 48, 22, 1, 74, 62, 93, 52, 11, 35, 68, 25, 40, 40, 88, 81, 52, 95, 4, 85, 1, 60, 91, 90, 18, 47],
                vec![87, 25, 98, 65, 70, 5, 44, 41, 75, 50, 23, 91, 91, 51, 85, 89, 53, 48, 12, 65, 6, 61, 41, 5, 38, 68, 84, 6, 97, 23, 71, 25, 85, 97, 69, 50, 43, 89, 34, 20, 58, 61, 48, 42, 97, 10, 9, 78, 8, 88, 50, 24, 87, 70, 11, 96, 49, 11, 63, 59, 68, 73, 48, 29, 71, 15, 11, 4, 48, 15, 10, 66, 88, 85, 99, 27, 34, 4, 40, 50, 95, 50, 20, 77, 86, 90, 51, 47, 13, 37, 1, 49, 36, 6, 81, 48, 41, 20, 60, 55],
                vec![6, 100, 83, 7, 5, 70, 81, 36, 32, 56, 58, 50, 27, 80, 50, 50, 44, 16, 84, 99, 15, 32, 29, 50, 18, 49, 65, 94, 57, 38, 42, 25, 6, 14, 26, 14, 23, 55, 25, 78, 20, 16, 10, 35, 68, 99, 26, 100, 82, 68, 91, 27, 62, 68, 19, 12, 61, 24, 23, 85, 59, 50, 42, 99, 1, 92, 31, 84, 26, 70, 36, 7, 80, 9, 2, 89, 22, 54, 22, 98, 78, 46, 87, 77, 41, 52, 61, 39, 71, 86, 18, 58, 55, 83, 94, 4, 35, 71, 31, 65],
                vec![24, 18, 3, 93, 25, 81, 34, 31, 92, 40, 46, 53, 92, 82, 25, 40, 73, 61, 84, 70, 97, 71, 62, 74, 34, 59, 10, 65, 10, 79, 8, 72, 41, 66, 76, 71, 87, 18, 67, 95, 86, 30, 75, 32, 33, 69, 53, 24, 99, 55, 20, 36, 2, 74, 46, 4, 97, 74, 38, 85, 45, 84, 45, 39, 2, 55, 45, 8, 14, 69, 78, 19, 68, 33, 22, 39, 38, 29, 15, 24, 98, 8, 85, 20, 31, 22, 3, 71, 83, 33, 21, 47, 89, 49, 57, 78, 41, 94, 27, 27],
                vec![46, 11, 38, 67, 45, 77, 45, 70, 19, 81, 30, 35, 58, 93, 27, 45, 56, 54, 4, 65, 90, 97, 82, 58, 72, 76, 9, 15, 58, 100, 45, 1, 71, 34, 12, 57, 26, 67, 69, 46, 46, 26, 51, 24, 7, 66, 43, 27, 50, 71, 97, 39, 43, 84, 63, 82, 74, 15, 3, 27, 21, 77, 53, 18, 18, 66, 97, 73, 44, 46, 42, 94, 29, 10, 11, 90, 23, 89, 96, 55, 2, 50, 9, 88, 64, 63, 58, 21, 24, 54, 18, 10, 25, 73, 79, 62, 64, 9, 10, 63],
                vec![62, 43, 38, 48, 76, 81, 100, 55, 44, 30, 67, 72, 9, 76, 76, 38, 94, 41, 88, 51, 46, 93, 100, 50, 32, 69, 52, 20, 96, 52, 49, 93, 34, 80, 70, 70, 49, 2, 57, 46, 36, 28, 29, 6, 44, 27, 12, 66, 97, 84, 23, 86, 90, 26, 76, 6, 94, 26, 33, 44, 26, 66, 7, 12, 89, 5, 50, 21, 7, 89, 90, 75, 5, 89, 26, 66, 87, 79, 36, 47, 11, 27, 24, 8, 48, 1, 80, 82, 62, 73, 34, 68, 66, 45, 91, 84, 90, 54, 40, 29],
                vec![48, 38, 90, 84, 29, 39, 4, 63, 98, 2, 6, 31, 75, 6, 92, 38, 31, 48, 13, 82, 43, 58, 13, 34, 71, 45, 93, 31, 31, 3, 3, 51, 97, 45, 62, 42, 85, 34, 33, 14, 45, 2, 18, 41, 27, 26, 59, 53, 28, 49, 26, 79, 37, 19, 65, 96, 55, 98, 72, 72, 98, 46, 46, 10, 24, 71, 33, 99, 18, 60, 54, 13, 81, 46, 92, 13, 77, 75, 55, 93, 72, 78, 32, 36, 48, 42, 26, 48, 26, 97, 82, 59, 80, 47, 96, 40, 6, 33, 84, 31],
                vec![93, 39, 83, 4, 77, 60, 52, 26, 31, 26, 27, 14, 72, 92, 63, 26, 59, 21, 57, 12, 9, 60, 13, 52, 87, 65, 91, 16, 56, 5, 59, 7, 23, 98, 91, 76, 80, 74, 80, 61, 40, 81, 6, 46, 77, 57, 29, 88, 58, 1, 82, 83, 49, 45, 71, 74, 80, 91, 47, 66, 76, 41, 67, 53, 20, 45, 7, 96, 88, 15, 75, 99, 20, 71, 43, 26, 58, 58, 72, 40, 11, 83, 89, 71, 39, 80, 4, 1, 95, 85, 44, 55, 6, 49, 24, 41, 16, 26, 65, 12],
                vec![49, 49, 100, 37, 82, 41, 1, 59, 85, 10, 89, 46, 53, 62, 11, 70, 44, 92, 39, 11, 64, 3, 36, 84, 11, 9, 21, 1, 66, 19, 51, 10, 91, 6, 64, 66, 82, 50, 73, 68, 74, 13, 57, 90, 76, 46, 69, 66, 48, 87, 1, 49, 37, 78, 87, 15, 97, 92, 11, 98, 27, 29, 15, 8, 20, 50, 17, 92, 32, 18, 10, 76, 92, 34, 100, 61, 25, 18, 62, 61, 70, 18, 49, 9, 82, 86, 22, 68, 58, 43, 82, 36, 84, 41, 26, 48, 97, 37, 47, 80],
                vec![98, 94, 53, 32, 58, 58, 12, 66, 71, 72, 4, 83, 69, 25, 13, 25, 90, 71, 34, 4, 77, 86, 59, 81, 64, 13, 15, 100, 6, 89, 40, 61, 16, 93, 5, 10, 43, 56, 83, 22, 38, 14, 58, 77, 39, 72, 93, 32, 26, 6, 11, 3, 47, 73, 86, 43, 88, 17, 15, 54, 81, 46, 61, 91, 58, 83, 53, 93, 43, 2, 73, 53, 57, 7, 2, 54, 16, 63, 80, 42, 90, 74, 17, 11, 36, 47, 78, 72, 63, 13, 53, 80, 33, 64, 28, 54, 27, 78, 17, 74],
                vec![69, 55, 88, 63, 83, 35, 68, 97, 41, 31, 43, 51, 77, 89, 23, 23, 97, 56, 7, 36, 31, 80, 1, 26, 50, 55, 59, 73, 50, 25, 11, 29, 60, 19, 76, 23, 82, 36, 54, 13, 22, 99, 89, 61, 64, 15, 72, 1, 38, 43, 90, 29, 37, 27, 20, 45, 9, 60, 8, 87, 42, 74, 96, 15, 24, 64, 97, 61, 40, 26, 31, 71, 65, 18, 71, 46, 11, 17, 61, 12, 44, 30, 22, 94, 17, 100, 43, 3, 99, 46, 5, 83, 81, 45, 100, 20, 54, 39, 64, 53],
                vec![13, 76, 17, 45, 59, 79, 75, 26, 60, 78, 23, 11, 51, 84, 88, 41, 58, 30, 54, 93, 9, 24, 62, 92, 7, 84, 30, 89, 1, 77, 30, 21, 38, 86, 93, 70, 79, 99, 77, 33, 98, 34, 42, 99, 45, 58, 82, 2, 68, 86, 65, 77, 58, 80, 80, 30, 71, 46, 56, 84, 36, 72, 40, 88, 34, 53, 61, 56, 85, 26, 88, 53, 37, 23, 60, 100, 49, 29, 65, 8, 60, 51, 52, 17, 88, 75, 95, 78, 51, 95, 69, 6, 73, 52, 51, 60, 18, 50, 4, 25],
                vec![23, 94, 72, 22, 81, 90, 64, 64, 49, 42, 21, 58, 80, 19, 19, 17, 37, 88, 97, 76, 89, 24, 24, 47, 13, 65, 26, 93, 76, 63, 37, 19, 17, 69, 37, 37, 89, 75, 74, 69, 81, 13, 34, 59, 96, 86, 8, 61, 5, 54, 53, 93, 87, 61, 71, 92, 86, 39, 75, 26, 75, 56, 46, 56, 55, 35, 4, 88, 51, 51, 62, 58, 90, 85, 20, 97, 64, 19, 53, 59, 7, 34, 3, 47, 10, 53, 96, 42, 12, 55, 62, 94, 32, 62, 29, 15, 97, 96, 73, 50],
                vec![66, 33, 21, 24, 52, 43, 34, 3, 7, 38, 32, 36, 92, 12, 94, 98, 37, 29, 98, 43, 70, 41, 16, 33, 64, 89, 43, 68, 19, 47, 48, 95, 87, 56, 31, 24, 33, 93, 6, 8, 74, 88, 51, 74, 61, 28, 5, 55, 90, 60, 4, 84, 63, 16, 14, 93, 43, 29, 69, 76, 89, 16, 23, 12, 83, 18, 3, 40, 39, 82, 66, 62, 19, 33, 70, 97, 13, 96, 80, 82, 72, 45, 36, 98, 56, 82, 75, 68, 23, 50, 77, 7, 21, 26, 30, 3, 81, 63, 94, 50],
                vec![78, 81, 40, 88, 17, 68, 100, 19, 47, 4, 10, 52, 10, 32, 5, 49, 59, 62, 41, 76, 54, 25, 42, 6, 77, 85, 34, 19, 11, 85, 45, 34, 10, 71, 39, 20, 87, 51, 35, 53, 51, 90, 28, 56, 94, 67, 18, 56, 34, 86, 58, 33, 20, 54, 57, 100, 45, 35, 59, 85, 55, 10, 82, 96, 57, 69, 39, 25, 49, 48, 54, 69, 98, 70, 59, 17, 59, 40, 12, 60, 44, 18, 54, 30, 92, 84, 51, 83, 9, 34, 56, 32, 94, 22, 84, 40, 79, 30, 73, 74],
                vec![9, 1, 19, 49, 29, 7, 8, 16, 88, 9, 24, 54, 87, 69, 23, 93, 58, 58, 38, 93, 66, 26, 67, 20, 3, 1, 56, 6, 77, 78, 23, 5, 14, 12, 15, 37, 68, 73, 69, 96, 60, 64, 64, 87, 39, 90, 93, 94, 84, 6, 57, 77, 45, 69, 81, 30, 63, 21, 84, 29, 62, 37, 16, 81, 66, 27, 37, 21, 39, 99, 65, 38, 84, 48, 96, 47, 22, 61, 44, 51, 36, 43, 91, 22, 19, 66, 40, 42, 19, 90, 72, 94, 25, 31, 35, 50, 47, 60, 55, 65],
                vec![48, 70, 80, 59, 19, 81, 46, 80, 83, 71, 79, 77, 72, 13, 50, 47, 59, 48, 20, 99, 80, 81, 23, 85, 31, 15, 32, 93, 30, 27, 32, 72, 72, 3, 44, 4, 54, 91, 60, 100, 53, 1, 31, 17, 78, 8, 83, 83, 27, 80, 44, 32, 100, 88, 100, 48, 30, 10, 43, 1, 95, 71, 4, 33, 87, 92, 68, 63, 99, 21, 62, 71, 35, 21, 17, 80, 21, 92, 73, 79, 43, 48, 77, 38, 6, 76, 3, 1, 73, 33, 39, 60, 39, 81, 56, 13, 24, 68, 82, 55],
                vec![93, 36, 90, 19, 12, 38, 11, 22, 8, 63, 16, 55, 25, 3, 66, 76, 62, 31, 25, 97, 50, 87, 39, 82, 64, 74, 39, 37, 7, 59, 58, 58, 40, 47, 43, 89, 48, 72, 75, 22, 10, 8, 77, 34, 48, 56, 72, 31, 56, 70, 26, 4, 71, 48, 91, 98, 57, 64, 92, 5, 52, 59, 61, 17, 35, 87, 92, 89, 91, 95, 66, 31, 13, 53, 40, 91, 61, 44, 36, 95, 47, 24, 61, 7, 50, 82, 94, 56, 71, 22, 62, 84, 11, 80, 79, 44, 43, 9, 47, 24],
                vec![56, 91, 4, 49, 88, 39, 17, 68, 19, 6, 26, 23, 67, 23, 1, 24, 77, 95, 33, 33, 63, 14, 63, 86, 84, 19, 65, 70, 98, 97, 34, 77, 19, 76, 1, 92, 60, 71, 78, 38, 48, 70, 74, 82, 2, 96, 6, 92, 83, 80, 25, 89, 95, 83, 83, 65, 6, 29, 20, 67, 77, 88, 27, 54, 95, 78, 68, 94, 55, 82, 24, 65, 10, 23, 60, 46, 72, 33, 32, 55, 13, 43, 43, 76, 22, 69, 65, 92, 99, 90, 46, 50, 71, 62, 9, 99, 8, 18, 57, 57],
                vec![18, 1, 79, 13, 9, 57, 41, 67, 63, 11, 63, 97, 6, 91, 48, 23, 86, 42, 39, 19, 6, 22, 63, 56, 94, 23, 64, 24, 17, 96, 18, 81, 3, 9, 28, 69, 53, 67, 90, 29, 86, 25, 37, 18, 96, 39, 19, 63, 77, 29, 31, 44, 92, 51, 28, 79, 98, 48, 21, 5, 43, 29, 3, 94, 75, 16, 1, 10, 92, 12, 87, 6, 90, 21, 58, 42, 57, 92, 39, 61, 99, 79, 92, 71, 72, 53, 24, 94, 65, 44, 68, 35, 49, 72, 40, 99, 37, 83, 47, 91],
                vec![15, 72, 93, 60, 78, 80, 54, 40, 60, 67, 20, 2, 57, 71, 6, 36, 5, 46, 55, 15, 93, 83, 61, 50, 87, 75, 93, 54, 6, 72, 96, 27, 99, 50, 67, 50, 41, 72, 66, 75, 67, 1, 19, 71, 47, 61, 52, 21, 30, 35, 88, 77, 83, 10, 39, 87, 36, 99, 95, 5, 96, 72, 89, 75, 37, 28, 78, 86, 8, 68, 52, 72, 85, 9, 75, 31, 26, 20, 47, 30, 63, 30, 75, 53, 100, 37, 96, 91, 9, 6, 70, 10, 88, 66, 10, 90, 18, 12, 32, 27],
                vec![44, 81, 8, 19, 26, 40, 17, 8, 60, 85, 67, 24, 57, 12, 53, 27, 27, 93, 57, 13, 44, 15, 53, 84, 15, 100, 4, 19, 62, 99, 99, 5, 45, 27, 1, 34, 35, 75, 81, 78, 91, 44, 82, 82, 23, 47, 12, 69, 68, 66, 31, 39, 49, 35, 62, 23, 73, 79, 53, 42, 89, 53, 74, 55, 94, 6, 72, 45, 64, 2, 83, 50, 90, 37, 52, 97, 76, 57, 25, 71, 88, 7, 26, 26, 71, 38, 1, 49, 53, 74, 97, 66, 32, 93, 59, 29, 45, 94, 17, 27],
                vec![91, 6, 44, 40, 62, 71, 80, 18, 71, 48, 59, 77, 83, 33, 92, 13, 59, 65, 4, 59, 83, 9, 19, 1, 25, 4, 17, 57, 94, 53, 58, 75, 20, 21, 4, 10, 26, 97, 85, 54, 87, 54, 18, 31, 42, 93, 8, 23, 15, 50, 39, 8, 25, 65, 31, 79, 81, 65, 30, 48, 30, 88, 22, 41, 36, 54, 63, 42, 85, 50, 73, 37, 40, 26, 77, 19, 1, 38, 39, 99, 53, 31, 69, 73, 40, 94, 19, 61, 36, 39, 85, 50, 98, 19, 31, 2, 27, 26, 27, 40],
                vec![70, 45, 31, 16, 77, 21, 10, 13, 65, 53, 34, 11, 37, 8, 60, 52, 90, 13, 38, 60, 73, 89, 95, 100, 33, 44, 37, 47, 10, 7, 16, 81, 59, 7, 42, 15, 24, 76, 49, 33, 18, 90, 17, 88, 54, 67, 45, 6, 34, 30, 95, 21, 55, 27, 8, 71, 87, 33, 85, 89, 12, 69, 30, 57, 57, 91, 77, 5, 100, 87, 95, 4, 13, 79, 37, 15, 86, 2, 44, 8, 97, 43, 50, 35, 91, 92, 81, 24, 86, 88, 26, 80, 27, 94, 52, 32, 34, 85, 75, 10],
                vec![82, 24, 63, 10, 61, 59, 74, 18, 29, 39, 65, 85, 77, 16, 69, 11, 96, 61, 25, 86, 45, 25, 23, 79, 84, 37, 51, 2, 6, 67, 72, 98, 7, 85, 19, 97, 33, 88, 9, 77, 50, 44, 68, 99, 99, 42, 31, 42, 55, 93, 97, 2, 89, 20, 96, 86, 83, 76, 19, 96, 16, 85, 17, 58, 41, 88, 33, 31, 44, 96, 38, 5, 69, 13, 4, 79, 98, 74, 98, 5, 91, 90, 84, 100, 51, 85, 33, 4, 74, 31, 65, 47, 30, 10, 97, 71, 4, 34, 44, 60],
                vec![98, 87, 43, 4, 3, 49, 91, 22, 47, 11, 2, 99, 42, 83, 89, 4, 68, 97, 2, 71, 86, 39, 67, 42, 40, 87, 65, 43, 15, 31, 37, 66, 46, 57, 11, 84, 57, 68, 2, 3, 59, 89, 17, 41, 50, 85, 83, 3, 86, 44, 34, 31, 100, 48, 8, 72, 36, 20, 29, 23, 87, 6, 49, 16, 16, 87, 66, 17, 24, 68, 59, 100, 78, 52, 45, 83, 9, 1, 76, 63, 36, 18, 61, 94, 82, 36, 36, 14, 3, 46, 100, 7, 90, 47, 23, 84, 32, 12, 65, 63],
                vec![28, 31, 16, 66, 39, 99, 89, 48, 71, 31, 24, 55, 95, 48, 93, 32, 46, 58, 84, 84, 70, 65, 40, 91, 79, 8, 19, 49, 25, 92, 37, 74, 18, 5, 56, 79, 33, 18, 81, 55, 45, 1, 82, 22, 15, 71, 70, 10, 26, 7, 60, 71, 60, 91, 48, 7, 47, 44, 6, 58, 80, 11, 51, 85, 23, 79, 88, 86, 80, 94, 63, 87, 32, 45, 77, 86, 73, 90, 34, 15, 8, 64, 98, 80, 2, 69, 61, 67, 64, 81, 43, 30, 96, 41, 8, 8, 100, 24, 84, 36],
                vec![80, 48, 24, 97, 61, 6, 5, 98, 87, 12, 3, 15, 33, 77, 72, 59, 91, 55, 73, 8, 100, 57, 31, 45, 78, 71, 70, 93, 25, 85, 40, 55, 25, 95, 56, 17, 30, 45, 18, 50, 39, 59, 8, 53, 81, 40, 21, 50, 16, 33, 43, 22, 52, 29, 55, 83, 51, 6, 50, 19, 61, 95, 3, 63, 3, 66, 19, 68, 80, 64, 24, 45, 20, 54, 8, 25, 15, 53, 39, 100, 9, 13, 17, 24, 85, 7, 42, 30, 80, 15, 60, 13, 59, 86, 80, 3, 44, 32, 27, 17],
                vec![53, 66, 17, 22, 91, 44, 21, 32, 7, 82, 68, 65, 84, 87, 62, 81, 37, 91, 34, 100, 2, 82, 19, 68, 11, 98, 67, 95, 89, 85, 18, 86, 55, 20, 96, 68, 8, 89, 44, 78, 81, 45, 42, 84, 13, 87, 61, 18, 43, 16, 63, 65, 13, 17, 96, 28, 39, 21, 64, 57, 55, 58, 68, 14, 19, 33, 77, 75, 72, 79, 92, 66, 64, 70, 93, 66, 59, 1, 19, 22, 94, 92, 26, 65, 11, 45, 31, 53, 13, 77, 52, 37, 9, 28, 3, 90, 23, 91, 66, 40],
                vec![97, 61, 13, 19, 57, 73, 20, 96, 78, 25, 60, 13, 66, 96, 76, 17, 10, 54, 6, 15, 51, 51, 4, 60, 60, 49, 78, 28, 36, 30, 57, 27, 64, 85, 75, 34, 20, 52, 60, 36, 48, 12, 94, 94, 98, 77, 14, 35, 76, 50, 27, 27, 19, 43, 98, 4, 94, 100, 23, 13, 60, 30, 73, 99, 27, 83, 65, 77, 7, 4, 1, 90, 86, 85, 98, 100, 93, 42, 67, 78, 85, 9, 1, 40, 78, 75, 16, 74, 7, 75, 14, 18, 38, 55, 62, 65, 50, 49, 79, 4],
                vec![50, 79, 7, 6, 35, 91, 8, 25, 2, 68, 58, 68, 81, 7, 3, 20, 48, 16, 98, 49, 60, 68, 90, 7, 74, 87, 86, 21, 28, 14, 71, 31, 99, 74, 76, 56, 53, 77, 42, 67, 20, 86, 21, 22, 18, 82, 37, 53, 16, 67, 58, 99, 48, 35, 28, 44, 37, 42, 76, 59, 10, 64, 78, 73, 49, 78, 6, 78, 2, 98, 42, 84, 69, 58, 78, 77, 12, 41, 97, 58, 13, 69, 67, 20, 96, 43, 36, 45, 63, 96, 50, 16, 90, 16, 68, 33, 94, 78, 41, 6],
                vec![86, 90, 91, 96, 88, 4, 98, 90, 70, 52, 17, 57, 78, 98, 81, 45, 2, 88, 100, 4, 79, 49, 100, 56, 67, 17, 38, 18, 84, 37, 79, 78, 44, 41, 50, 58, 46, 42, 42, 6, 64, 33, 93, 72, 41, 69, 35, 31, 98, 62, 32, 52, 46, 89, 21, 17, 12, 55, 74, 28, 47, 10, 56, 20, 19, 18, 15, 33, 85, 71, 75, 2, 35, 37, 81, 3, 26, 34, 69, 37, 21, 34, 94, 65, 42, 61, 94, 32, 79, 62, 54, 32, 59, 37, 84, 76, 69, 21, 18, 47],
                vec![70, 90, 73, 91, 9, 18, 48, 34, 3, 6, 49, 83, 80, 32, 99, 95, 76, 77, 16, 51, 83, 83, 89, 29, 77, 13, 83, 94, 55, 81, 14, 57, 44, 88, 86, 59, 97, 68, 98, 64, 70, 90, 79, 1, 62, 25, 4, 61, 35, 59, 50, 51, 27, 23, 23, 26, 92, 64, 78, 31, 7, 86, 18, 91, 21, 95, 97, 94, 88, 36, 98, 60, 81, 9, 24, 2, 77, 15, 37, 99, 53, 95, 47, 89, 1, 34, 27, 35, 7, 50, 18, 87, 1, 38, 84, 41, 54, 13, 40, 45],
                vec![100, 9, 78, 24, 3, 2, 52, 89, 22, 58, 17, 64, 88, 96, 16, 7, 55, 45, 30, 60, 56, 22, 93, 2, 81, 28, 70, 46, 69, 47, 48, 23, 41, 63, 26, 49, 58, 88, 48, 27, 41, 35, 12, 46, 47, 2, 23, 40, 82, 55, 40, 20, 70, 6, 29, 91, 11, 83, 1, 49, 10, 1, 24, 65, 44, 74, 85, 87, 4, 81, 5, 56, 59, 13, 2, 94, 57, 84, 39, 3, 15, 61, 5, 14, 6, 78, 59, 74, 55, 3, 30, 17, 62, 10, 55, 52, 82, 33, 89, 91],
                vec![12, 34, 13, 29, 74, 29, 63, 2, 86, 76, 13, 74, 63, 11, 36, 12, 69, 91, 64, 91, 42, 44, 17, 29, 6, 93, 47, 63, 80, 72, 39, 87, 18, 45, 77, 72, 53, 63, 97, 37, 96, 73, 19, 1, 77, 25, 74, 51, 19, 89, 12, 78, 88, 2, 19, 7, 31, 6, 56, 87, 16, 84, 59, 70, 41, 38, 7, 63, 27, 26, 12, 96, 39, 33, 6, 14, 36, 49, 76, 16, 63, 42, 46, 17, 38, 76, 11, 41, 31, 5, 63, 14, 13, 49, 75, 16, 44, 24, 14, 100],
                vec![65, 28, 3, 66, 11, 7, 95, 22, 33, 24, 26, 7, 47, 72, 80, 21, 58, 89, 81, 93, 17, 45, 83, 77, 98, 32, 53, 27, 66, 13, 60, 52, 94, 41, 77, 26, 6, 48, 80, 79, 83, 36, 72, 26, 65, 73, 86, 87, 43, 73, 91, 74, 87, 26, 26, 12, 38, 7, 12, 59, 13, 48, 4, 94, 17, 23, 1, 44, 3, 59, 67, 46, 25, 45, 6, 43, 47, 21, 28, 64, 51, 34, 75, 18, 18, 97, 38, 31, 34, 35, 62, 7, 38, 48, 13, 94, 93, 98, 96, 7],
                vec![87, 58, 85, 11, 22, 59, 91, 84, 72, 9, 77, 46, 65, 61, 69, 48, 46, 32, 88, 83, 15, 56, 98, 87, 24, 90, 32, 87, 79, 92, 9, 72, 44, 1, 21, 17, 65, 19, 86, 8, 95, 29, 38, 58, 79, 89, 67, 32, 33, 46, 29, 29, 54, 87, 14, 38, 36, 57, 43, 46, 70, 43, 85, 76, 88, 21, 27, 74, 73, 41, 37, 16, 12, 8, 36, 12, 18, 95, 11, 74, 53, 74, 56, 86, 91, 69, 53, 94, 99, 35, 94, 19, 79, 95, 65, 89, 52, 43, 69, 70],
                vec![18, 68, 32, 32, 54, 57, 16, 98, 13, 87, 18, 21, 38, 77, 100, 79, 13, 41, 72, 53, 56, 90, 10, 14, 81, 43, 8, 72, 73, 42, 57, 43, 81, 38, 79, 50, 72, 44, 70, 70, 80, 76, 1, 20, 85, 95, 31, 25, 35, 24, 22, 36, 62, 65, 15, 60, 51, 59, 16, 81, 95, 76, 87, 85, 32, 57, 71, 54, 29, 32, 20, 41, 82, 68, 7, 89, 68, 79, 73, 66, 70, 60, 80, 16, 38, 13, 45, 66, 21, 59, 75, 2, 7, 86, 34, 47, 29, 36, 98, 84],
                vec![81, 40, 77, 82, 81, 75, 84, 92, 33, 80, 12, 3, 27, 6, 98, 93, 97, 92, 17, 21, 52, 92, 84, 93, 26, 78, 94, 29, 82, 40, 81, 68, 72, 61, 73, 16, 32, 77, 73, 22, 43, 55, 25, 6, 84, 73, 69, 50, 33, 45, 79, 22, 73, 83, 85, 91, 59, 23, 62, 50, 35, 54, 69, 7, 34, 89, 69, 35, 22, 64, 70, 88, 24, 49, 9, 90, 70, 16, 3, 95, 66, 23, 96, 23, 51, 85, 100, 31, 49, 49, 56, 4, 68, 52, 50, 81, 64, 87, 33, 63],
                vec![73, 79, 49, 79, 72, 16, 77, 84, 47, 39, 7, 31, 72, 58, 35, 76, 99, 77, 68, 28, 96, 22, 25, 79, 15, 98, 9, 60, 83, 99, 66, 50, 88, 45, 98, 73, 44, 4, 50, 77, 40, 99, 89, 95, 32, 98, 8, 23, 64, 93, 14, 14, 39, 29, 68, 90, 13, 42, 62, 8, 10, 30, 47, 57, 73, 97, 81, 18, 9, 49, 48, 93, 58, 30, 46, 89, 64, 44, 51, 37, 21, 39, 66, 64, 15, 3, 38, 70, 90, 24, 32, 58, 6, 72, 55, 84, 87, 16, 17, 7],
                vec![87, 23, 5, 72, 64, 78, 4, 5, 15, 7, 70, 92, 48, 23, 80, 89, 52, 68, 19, 33, 25, 61, 83, 48, 48, 89, 82, 4, 48, 19, 58, 30, 31, 67, 19, 7, 32, 90, 94, 78, 84, 17, 91, 48, 24, 84, 73, 45, 63, 63, 76, 97, 63, 29, 77, 50, 82, 18, 66, 77, 94, 5, 92, 87, 51, 36, 38, 25, 25, 35, 31, 12, 42, 87, 81, 62, 34, 14, 79, 25, 78, 23, 45, 66, 16, 22, 40, 42, 16, 88, 79, 23, 12, 64, 37, 2, 28, 22, 6, 59],
                vec![52, 43, 31, 82, 26, 74, 72, 5, 62, 6, 61, 90, 46, 80, 63, 80, 53, 22, 47, 56, 89, 18, 51, 98, 67, 44, 41, 71, 34, 88, 24, 84, 10, 66, 39, 15, 42, 27, 55, 65, 73, 1, 79, 39, 40, 6, 20, 53, 80, 46, 21, 92, 68, 7, 69, 84, 38, 45, 91, 72, 84, 38, 23, 42, 50, 28, 37, 99, 38, 3, 39, 93, 93, 17, 37, 22, 16, 59, 56, 81, 35, 43, 17, 4, 23, 7, 56, 2, 93, 82, 36, 84, 53, 62, 23, 92, 11, 24, 17, 1],
                vec![21, 39, 39, 16, 57, 30, 34, 39, 58, 34, 25, 95, 100, 71, 43, 81, 90, 31, 26, 42, 40, 84, 38, 39, 74, 98, 49, 63, 7, 20, 60, 73, 92, 2, 4, 83, 73, 27, 85, 88, 59, 25, 67, 82, 55, 78, 83, 20, 66, 84, 50, 77, 92, 86, 51, 54, 84, 38, 37, 54, 8, 20, 87, 8, 44, 28, 90, 77, 9, 56, 5, 64, 5, 91, 64, 22, 92, 30, 51, 95, 79, 18, 46, 60, 56, 95, 42, 100, 23, 89, 27, 60, 6, 85, 80, 26, 88, 49, 53, 16],
                vec![34, 67, 72, 51, 7, 89, 92, 67, 15, 24, 72, 62, 57, 22, 48, 39, 5, 16, 94, 46, 57, 38, 42, 94, 42, 86, 61, 11, 42, 8, 7, 99, 47, 5, 73, 65, 10, 86, 72, 21, 1, 5, 75, 79, 61, 34, 3, 7, 72, 33, 49, 53, 38, 88, 96, 67, 47, 82, 63, 53, 33, 28, 97, 12, 88, 24, 67, 50, 21, 61, 70, 70, 100, 57, 92, 98, 98, 39, 81, 62, 97, 26, 81, 95, 16, 100, 49, 26, 71, 9, 97, 66, 65, 35, 51, 16, 23, 100, 97, 55],
                vec![63, 11, 69, 99, 19, 89, 37, 34, 54, 65, 53, 72, 7, 76, 83, 22, 22, 81, 11, 80, 77, 31, 6, 31, 80, 88, 10, 11, 78, 12, 88, 98, 15, 14, 30, 8, 13, 10, 22, 26, 46, 35, 77, 18, 25, 71, 2, 37, 83, 6, 10, 8, 29, 35, 30, 64, 38, 91, 74, 74, 41, 73, 91, 78, 19, 9, 66, 19, 53, 45, 74, 89, 79, 31, 92, 32, 61, 78, 30, 99, 68, 5, 95, 35, 57, 73, 33, 97, 64, 1, 64, 45, 52, 22, 90, 38, 92, 93, 70, 14],
                vec![23, 60, 85, 1, 22, 83, 61, 89, 93, 1, 6, 72, 24, 73, 51, 29, 67, 3, 51, 43, 18, 8, 13, 63, 80, 6, 17, 51, 3, 92, 29, 41, 79, 90, 51, 5, 11, 50, 17, 43, 45, 67, 73, 93, 13, 38, 23, 66, 46, 19, 30, 44, 93, 39, 2, 2, 7, 2, 76, 96, 5, 94, 89, 13, 18, 5, 99, 96, 36, 89, 91, 69, 83, 7, 33, 81, 17, 86, 2, 93, 65, 10, 83, 48, 7, 94, 61, 49, 64, 43, 28, 99, 88, 65, 60, 36, 74, 96, 17, 37],
                vec![61, 30, 99, 43, 90, 93, 79, 36, 41, 51, 54, 6, 49, 46, 41, 58, 42, 44, 64, 75, 26, 90, 82, 46, 81, 44, 25, 33, 65, 15, 27, 89, 13, 24, 18, 90, 88, 65, 42, 12, 64, 56, 26, 51, 40, 11, 14, 82, 84, 84, 94, 10, 4, 26, 8, 13, 63, 22, 48, 37, 26, 4, 86, 39, 50, 67, 10, 6, 47, 80, 98, 84, 13, 31, 68, 20, 29, 25, 94, 57, 88, 92, 31, 83, 8, 70, 70, 18, 50, 62, 57, 25, 89, 75, 85, 69, 10, 42, 63, 29],
                vec![34, 23, 78, 78, 15, 55, 71, 98, 57, 23, 21, 50, 17, 95, 13, 11, 81, 77, 69, 90, 2, 23, 14, 9, 76, 2, 18, 79, 19, 3, 5, 17, 10, 60, 75, 91, 12, 46, 14, 11, 20, 3, 28, 79, 73, 78, 81, 81, 34, 49, 42, 48, 60, 75, 89, 72, 49, 91, 40, 60, 78, 81, 85, 100, 45, 12, 22, 2, 96, 37, 5, 77, 52, 4, 37, 66, 26, 45, 4, 7, 67, 95, 53, 46, 21, 8, 26, 30, 88, 19, 43, 69, 41, 24, 35, 4, 64, 36, 30, 15],
                vec![31, 67, 12, 35, 73, 30, 32, 91, 70, 86, 18, 36, 31, 76, 8, 54, 40, 59, 44, 94, 33, 52, 95, 35, 87, 63, 20, 86, 54, 99, 57, 49, 63, 7, 26, 91, 40, 66, 80, 89, 24, 54, 41, 80, 72, 23, 40, 100, 10, 46, 57, 19, 40, 43, 88, 97, 84, 59, 37, 6, 4, 90, 35, 70, 42, 90, 65, 69, 41, 25, 23, 32, 74, 53, 98, 100, 19, 85, 44, 58, 40, 9, 24, 17, 79, 35, 74, 35, 34, 58, 88, 42, 82, 52, 48, 86, 21, 27, 42, 2],
                vec![52, 82, 42, 27, 11, 71, 83, 34, 4, 26, 53, 64, 51, 62, 99, 52, 55, 34, 79, 48, 4, 16, 17, 94, 25, 3, 80, 33, 37, 28, 37, 71, 40, 65, 89, 40, 26, 7, 98, 84, 12, 9, 32, 51, 5, 56, 97, 7, 84, 50, 3, 41, 2, 11, 59, 88, 20, 61, 12, 19, 80, 79, 8, 68, 81, 78, 94, 57, 48, 77, 39, 80, 40, 2, 39, 74, 50, 81, 46, 7, 64, 73, 40, 22, 52, 34, 32, 5, 18, 2, 53, 15, 54, 37, 5, 85, 15, 13, 84, 76],
                vec![34, 50, 62, 78, 55, 58, 14, 86, 72, 73, 77, 99, 41, 91, 27, 34, 3, 11, 63, 48, 66, 1, 70, 32, 90, 7, 41, 57, 44, 5, 1, 18, 56, 40, 2, 15, 37, 56, 41, 50, 49, 17, 21, 50, 49, 56, 85, 27, 15, 91, 72, 2, 9, 63, 78, 8, 13, 2, 63, 30, 54, 86, 17, 82, 47, 31, 20, 29, 67, 55, 98, 68, 22, 34, 25, 82, 1, 76, 16, 72, 10, 86, 64, 95, 4, 74, 26, 78, 72, 73, 74, 59, 88, 74, 36, 32, 11, 12, 13, 95],
                vec![32, 82, 94, 51, 62, 86, 46, 43, 98, 78, 13, 16, 23, 24, 35, 19, 9, 84, 61, 67, 35, 42, 91, 47, 50, 61, 70, 50, 93, 77, 83, 28, 92, 61, 81, 86, 68, 54, 54, 42, 100, 33, 94, 15, 87, 76, 71, 5, 80, 3, 53, 23, 42, 89, 94, 87, 95, 88, 57, 65, 74, 21, 82, 64, 41, 53, 44, 22, 48, 55, 57, 69, 32, 5, 28, 47, 94, 4, 24, 25, 77, 36, 38, 55, 51, 83, 47, 57, 18, 2, 51, 13, 78, 38, 7, 71, 76, 93, 79, 71],
                vec![6, 94, 4, 48, 90, 25, 47, 89, 58, 84, 42, 16, 49, 66, 66, 6, 77, 91, 38, 56, 95, 24, 4, 30, 74, 75, 83, 46, 56, 67, 2, 10, 40, 21, 29, 4, 18, 89, 78, 56, 100, 21, 58, 96, 26, 17, 10, 2, 57, 15, 92, 62, 9, 28, 27, 2, 7, 58, 90, 90, 75, 16, 16, 82, 69, 26, 44, 32, 17, 33, 80, 60, 13, 21, 50, 77, 56, 59, 99, 54, 38, 71, 100, 91, 20, 17, 78, 54, 29, 69, 57, 32, 49, 37, 31, 5, 4, 69, 15, 75],
                vec![86, 96, 57, 97, 76, 43, 59, 31, 30, 12, 52, 88, 34, 53, 98, 20, 83, 16, 39, 22, 75, 75, 4, 41, 75, 73, 85, 92, 42, 79, 25, 7, 59, 96, 100, 21, 94, 50, 30, 68, 50, 37, 45, 66, 35, 22, 89, 90, 62, 81, 84, 22, 96, 60, 73, 64, 12, 8, 20, 99, 73, 91, 67, 6, 29, 91, 14, 62, 10, 71, 28, 86, 33, 58, 67, 85, 38, 12, 33, 8, 84, 23, 8, 73, 91, 86, 82, 80, 34, 10, 33, 67, 82, 15, 10, 24, 66, 65, 27, 61],
                vec![100, 18, 85, 79, 99, 10, 84, 55, 95, 80, 71, 96, 39, 17, 11, 87, 5, 37, 3, 86, 98, 40, 68, 27, 31, 49, 7, 98, 28, 67, 6, 42, 50, 4, 25, 63, 46, 74, 8, 67, 18, 50, 70, 78, 18, 25, 41, 63, 48, 99, 20, 68, 71, 81, 74, 83, 3, 56, 9, 50, 66, 28, 69, 11, 19, 79, 53, 12, 28, 72, 44, 78, 80, 76, 18, 63, 83, 42, 89, 82, 89, 89, 62, 26, 3, 26, 61, 97, 44, 28, 66, 54, 32, 10, 29, 89, 62, 99, 53, 95],
                vec![76, 27, 79, 62, 54, 19, 61, 85, 88, 89, 64, 34, 76, 55, 8, 11, 28, 8, 73, 77, 32, 84, 41, 15, 99, 76, 97, 12, 6, 4, 17, 32, 38, 91, 89, 9, 14, 96, 78, 69, 51, 31, 4, 48, 62, 97, 70, 99, 77, 68, 48, 52, 54, 40, 25, 53, 37, 81, 36, 50, 9, 17, 91, 97, 62, 88, 11, 55, 29, 39, 37, 75, 58, 81, 3, 8, 43, 6, 60, 37, 62, 45, 66, 98, 3, 23, 40, 72, 11, 32, 84, 36, 92, 74, 76, 4, 71, 95, 27, 24],
                vec![31, 67, 59, 3, 13, 36, 67, 82, 99, 63, 11, 26, 51, 25, 68, 34, 43, 29, 88, 57, 95, 1, 84, 57, 31, 47, 70, 17, 19, 78, 100, 18, 24, 24, 76, 14, 21, 57, 44, 23, 55, 89, 67, 68, 56, 99, 14, 62, 88, 3, 5, 77, 70, 79, 1, 69, 69, 53, 49, 95, 80, 20, 3, 19, 73, 51, 24, 84, 97, 38, 1, 99, 83, 15, 29, 38, 60, 99, 44, 18, 94, 50, 38, 99, 34, 9, 17, 46, 92, 26, 43, 40, 9, 3, 29, 18, 49, 50, 64, 74],
            ]);
		
        let mut mm = MadarskaMetoda::new(&matrica);
        assert_eq!(236, mm.solve(Some(false)));

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        assert_eq!(236, mm.solve(Some(false)));
    }

    #[test]
    fn square_test() {
        let matrica = Matrix::new(vec![
            vec![1, 2],
        ]);

        let matrica2 = Matrix::new(vec![
            vec![1],
            vec![2],
        ]);

        assert_eq!(vec![vec![1, 2], vec![0, 0]], matrica.matrix);
        assert_eq!(vec![vec![1, 0], vec![2, 0]], matrica2.matrix);
    }

    #[test]
    fn row_len_test() {
        let matrica = Matrix::new(vec![
            vec![1],
            vec![1, 2],
        ]);

        let matrica2 = Matrix::new(vec![
            vec![1, 2],
            vec![1],
            vec![]
        ]);


        assert_eq!(vec![vec![1, 0], vec![1, 2]], matrica.matrix);
        assert_eq!(vec![vec![1, 2, 0], vec![1, 0, 0], vec![0, 0, 0]], matrica2.matrix);
    }

    #[test]
    fn invert_matrix_values_text() {
        let matrica = Matrix::new(vec![
            vec![1, 2],
            vec![2, 4],
        ]);

        let matrica = matrica.invert_matrix_values();

        assert_eq!(vec![vec![-1, -2], vec![-2, -4]], matrica.matrix);
    }

    #[test]
    fn solve_max() {
        let matrica = Matrix::new(vec![
            vec![1, 2],
            vec![2, 4],
        ]);

        let mut mm = MadarskaMetoda::new(&matrica);
        assert_eq!(5, mm.solve(Some(true)));
        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        assert_eq!(5, mm.solve(Some(true)));
    }
    
    #[test]
    fn min_max_solve_tests() {
        let matrica = Matrix::new(vec![
            vec![6, 7, 8, 8, 4, 6, 0, 4, 1, 5, 6, 7, 3, 2, 8, 0, 9, 5, 8, 6, 0, 1, 7, 1, 4, 2, 2, 1, 4, 3, 8, 3, 6, 9, 6, 7, 8, 8],
            vec![2, 1, 4, 0, 0, 8, 8, 3, 6, 0, 3, 2, 2, 8, 5, 3, 2, 1, 5, 4, 6, 4, 8, 5, 9, 0, 1, 7, 7, 9, 4, 8, 5, 8, 3, 7, 1, 0],
            vec![7, 7, 7, 9, 0, 1, 1, 0, 4, 3, 1, 9, 2, 3, 7, 7, 7, 7, 9, 0, 9, 7, 7, 6, 0, 2, 5, 9, 6, 3, 7, 9, 3, 2, 5, 6, 6, 5],
            vec![4, 6, 9, 3, 8, 1, 9, 9, 5, 2, 3, 5, 4, 9, 0, 9, 3, 0, 5, 0, 2, 5, 5, 5, 0, 2, 5, 8, 2, 7, 1, 7, 9, 7, 5, 5, 3, 8],
            vec![1, 9, 8, 7, 1, 3, 7, 1, 9, 8, 6, 3, 6, 1, 0, 9, 6, 4, 4, 2, 7, 0, 4, 0, 9, 6, 3, 5, 7, 4, 0, 7, 8, 7, 3, 8, 5, 8],
            vec![1, 9, 3, 3, 8, 9, 7, 1, 7, 2, 5, 5, 7, 3, 8, 3, 3, 2, 3, 2, 1, 8, 8, 4, 4, 2, 2, 2, 9, 2, 8, 3, 9, 0, 0, 6, 3, 3],
            vec![5, 3, 9, 0, 8, 7, 0, 3, 2, 2, 5, 0, 8, 1, 2, 8, 4, 6, 2, 3, 5, 2, 5, 4, 4, 5, 7, 6, 3, 5, 1, 2, 8, 8, 3, 9, 2, 3],
            vec![2, 8, 0, 8, 7, 1, 5, 7, 4, 1, 0, 8, 2, 0, 1, 4, 1, 8, 3, 1, 3, 1, 1, 3, 5, 0, 9, 3, 6, 0, 8, 7, 2, 5, 4, 7, 8, 2],
            vec![8, 8, 4, 7, 9, 0, 9, 5, 1, 1, 8, 8, 3, 7, 6, 7, 5, 8, 7, 9, 0, 9, 5, 5, 7, 6, 7, 5, 8, 1, 5, 6, 9, 5, 4, 6, 6, 6],
            vec![2, 2, 5, 6, 6, 9, 2, 1, 0, 4, 3, 3, 7, 2, 2, 6, 6, 9, 7, 2, 2, 1, 0, 6, 8, 8, 4, 7, 3, 1, 0, 3, 4, 9, 2, 0, 1, 8],
            vec![7, 7, 0, 0, 5, 4, 2, 4, 5, 3, 3, 6, 4, 0, 0, 6, 5, 3, 6, 0, 3, 1, 3, 4, 4, 5, 9, 0, 3, 0, 4, 2, 1, 1, 9, 6, 8, 5],
            vec![6, 2, 3, 7, 2, 8, 1, 8, 7, 9, 1, 6, 0, 8, 9, 3, 4, 1, 7, 8, 0, 9, 4, 4, 5, 0, 9, 5, 8, 7, 6, 6, 1, 0, 6, 5, 3, 9],
            vec![1, 3, 4, 1, 1, 3, 9, 9, 5, 3, 5, 8, 2, 0, 7, 1, 8, 2, 6, 9, 8, 7, 7, 2, 2, 1, 9, 7, 3, 5, 2, 3, 6, 5, 7, 5, 7, 7],
            vec![6, 7, 1, 4, 3, 8, 9, 6, 5, 8, 1, 6, 6, 5, 2, 5, 1, 3, 6, 7, 0, 0, 2, 9, 1, 4, 9, 8, 8, 0, 6, 9, 7, 4, 1, 9, 0, 8],
            vec![2, 0, 1, 2, 5, 4, 2, 4, 6, 8, 6, 0, 9, 4, 4, 6, 0, 1, 8, 0, 4, 2, 3, 0, 7, 6, 5, 8, 6, 4, 4, 2, 5, 2, 0, 1, 9, 3],
            vec![5, 6, 7, 6, 5, 8, 4, 0, 9, 4, 2, 9, 5, 8, 5, 1, 7, 4, 7, 8, 0, 2, 1, 4, 0, 4, 3, 3, 7, 7, 8, 1, 4, 2, 5, 7, 7, 7],
            vec![9, 4, 5, 4, 0, 6, 8, 8, 0, 7, 5, 9, 9, 7, 7, 0, 8, 4, 1, 3, 8, 1, 9, 8, 9, 5, 5, 8, 7, 1, 4, 1, 5, 9, 3, 3, 4, 3],
            vec![4, 5, 5, 5, 4, 0, 2, 9, 4, 7, 4, 7, 6, 6, 8, 1, 3, 5, 6, 7, 0, 1, 3, 9, 3, 8, 2, 3, 7, 5, 1, 2, 3, 2, 9, 4, 0, 9],
            vec![1, 6, 1, 0, 2, 9, 2, 8, 7, 6, 9, 4, 1, 5, 4, 2, 3, 2, 0, 9, 9, 1, 2, 3, 2, 0, 0, 3, 4, 5, 5, 5, 8, 4, 6, 6, 6, 2],
            vec![3, 4, 9, 7, 1, 8, 6, 6, 1, 4, 5, 7, 6, 6, 0, 1, 8, 0, 3, 4, 2, 0, 0, 1, 2, 3, 9, 8, 8, 0, 9, 7, 2, 4, 5, 3, 2, 1],
            vec![4, 6, 9, 2, 7, 0, 2, 2, 4, 3, 3, 2, 4, 9, 5, 7, 4, 9, 6, 6, 3, 4, 5, 9, 2, 8, 2, 1, 0, 8, 6, 7, 3, 9, 8, 1, 7, 8],
            vec![2, 0, 0, 8, 0, 3, 5, 4, 6, 1, 3, 1, 8, 0, 8, 9, 3, 5, 0, 1, 3, 4, 8, 2, 4, 4, 3, 5, 1, 3, 4, 8, 6, 0, 7, 8, 5, 6],
            vec![8, 8, 3, 1, 4, 0, 0, 3, 9, 1, 2, 8, 8, 3, 3, 3, 1, 9, 4, 0, 4, 0, 0, 2, 6, 0, 6, 0, 5, 1, 7, 9, 0, 4, 4, 1, 9, 6],
            vec![6, 4, 6, 9, 1, 1, 1, 4, 0, 2, 7, 7, 4, 2, 3, 3, 7, 1, 8, 4, 3, 4, 2, 1, 9, 4, 1, 6, 7, 0, 2, 8, 3, 2, 5, 8, 4, 8],
            vec![4, 0, 0, 7, 5, 4, 9, 2, 5, 9, 4, 3, 5, 7, 2, 2, 1, 2, 9, 9, 8, 8, 4, 1, 0, 9, 6, 2, 6, 1, 2, 8, 7, 3, 4, 0, 6, 5],
            vec![9, 3, 5, 3, 8, 7, 3, 0, 6, 8, 8, 5, 3, 5, 2, 7, 1, 5, 6, 5, 1, 3, 8, 4, 6, 7, 8, 0, 7, 0, 2, 8, 2, 1, 3, 0, 3, 2],
            vec![6, 2, 6, 6, 1, 1, 0, 6, 9, 0, 0, 7, 0, 9, 7, 9, 2, 9, 7, 4, 2, 5, 4, 9, 2, 8, 1, 8, 6, 4, 4, 4, 9, 1, 3, 5, 0, 2],
            vec![3, 3, 6, 1, 7, 1, 5, 2, 2, 9, 1, 4, 8, 1, 5, 4, 3, 1, 0, 4, 3, 2, 5, 9, 8, 5, 1, 4, 9, 6, 6, 8, 1, 0, 7, 8, 6, 1],
            vec![6, 7, 0, 4, 1, 5, 6, 5, 3, 2, 7, 3, 1, 3, 6, 1, 4, 0, 8, 3, 8, 7, 4, 0, 4, 9, 3, 7, 2, 6, 3, 4, 7, 1, 2, 4, 7, 0],
            vec![3, 7, 1, 5, 3, 6, 4, 4, 3, 8, 5, 3, 1, 2, 8, 7, 1, 4, 4, 7, 2, 4, 8, 5, 5, 7, 4, 5, 6, 9, 9, 0, 4, 2, 1, 9, 3, 1],
            vec![2, 0, 0, 9, 0, 5, 1, 2, 2, 6, 6, 5, 3, 2, 0, 0, 0, 7, 0, 0, 7, 3, 2, 8, 4, 2, 9, 7, 0, 3, 9, 4, 4, 6, 1, 7, 4, 1],
            vec![5, 5, 1, 2, 4, 1, 8, 1, 4, 9, 8, 3, 5, 0, 9, 5, 8, 3, 8, 2, 9, 4, 6, 2, 8, 7, 2, 6, 4, 9, 4, 8, 1, 3, 9, 8, 3, 9],
            vec![7, 8, 4, 1, 2, 4, 4, 8, 2, 5, 7, 1, 0, 8, 7, 7, 9, 0, 3, 1, 0, 0, 0, 2, 6, 9, 6, 5, 0, 7, 6, 2, 4, 2, 0, 1, 8, 4],
            vec![3, 0, 6, 8, 0, 0, 8, 9, 7, 4, 4, 3, 0, 1, 1, 4, 3, 0, 0, 2, 3, 2, 4, 6, 7, 9, 7, 7, 8, 6, 4, 1, 2, 8, 3, 0, 8, 9],
            vec![8, 9, 6, 3, 1, 8, 1, 0, 9, 4, 9, 8, 8, 2, 9, 7, 4, 2, 8, 2, 4, 7, 2, 0, 0, 9, 0, 3, 4, 2, 4, 9, 4, 6, 5, 0, 0, 2],
            vec![9, 2, 4, 8, 1, 2, 7, 8, 4, 9, 8, 4, 7, 3, 6, 8, 1, 5, 1, 3, 8, 4, 8, 7, 5, 1, 2, 5, 8, 0, 6, 5, 1, 5, 3, 8, 8, 7],
            vec![1, 9, 7, 5, 6, 0, 6, 5, 1, 5, 5, 2, 7, 1, 6, 9, 0, 9, 0, 7, 0, 6, 5, 6, 1, 8, 1, 6, 1, 3, 8, 3, 2, 1, 1, 7, 2, 6],
            vec![7, 2, 9, 7, 0, 6, 3, 1, 8, 2, 0, 8, 3, 3, 3, 7, 5, 1, 0, 1, 0, 3, 7, 1, 1, 4, 2, 3, 5, 0, 7, 7, 4, 2, 2, 1, 3, 5],
        ]);
            
        let matrica2 = Matrix::new(vec![
            vec![1, 3, 1, 1, 2, 2, 2, 1, 3, 3],
            vec![3, 1, 3, 3, 1, 3, 3, 3, 2, 3],
            vec![1, 2, 1, 1, 2, 3, 2, 2, 1, 1],
            vec![3, 3, 3, 2, 1, 3, 2, 3, 3, 1],
            vec![1, 2, 3, 2, 3, 2, 2, 3, 2, 2],
            vec![1, 2, 3, 1, 1, 2, 3, 2, 2, 3],
            vec![3, 1, 1, 1, 2, 3, 1, 1, 3, 2],
            vec![3, 2, 2, 2, 2, 3, 2, 1, 3, 2],
            vec![3, 2, 3, 3, 1, 2, 2, 1, 3, 2],
            vec![1, 3, 2, 3, 2, 2, 3, 2, 2, 3],
        ]);
        
        let mut mm = MadarskaMetoda::new(&matrica);
        assert_eq!(1, mm.solve(Some(false)));
        let mut mm = MadarskaMetoda::new(&matrica);
        assert_eq!(341, mm.solve(Some(true)));
        let mut mm = MadarskaMetoda::new(&matrica2);
        assert_eq!(11, mm.solve(Some(false)));
        let mut mm = MadarskaMetoda::new(&matrica2);
        assert_eq!(30, mm.solve(Some(true)));

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        assert_eq!(1, mm.solve(Some(false)));
        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        assert_eq!(341, mm.solve(Some(true)));
        let mut mm = MadarskaMetodaMunkres::new(&matrica2);
        assert_eq!(11, mm.solve(Some(false)));
        let mut mm = MadarskaMetodaMunkres::new(&matrica2);
        assert_eq!(30, mm.solve(Some(true)));
    }

    #[test]
    fn munkres_first_step() {
        let matrica = Matrix::new(vec![
            vec![1, 2, 3],
            vec![2, 4, 6],
            vec![3, 6, 9],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.first_step();

        let expected_result = vec![
            vec![0, 1, 2],
            vec![0, 2, 4],
            vec![0, 3, 6],
        ];

        assert_eq!(expected_result, mm.calculating_matrix.matrix);
    }

    #[test]
    fn munkres_second_step() {
        let matrica = Matrix::new(vec![
            vec![0, 1, 2],
            vec![0, 2, 4],
            vec![0, 3, 6],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.second_step();

        let expected_result = vec![
            vec![1, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];

        assert_eq!(expected_result, mm.assignment_mask.matrix);
    }

    #[test]
    fn munkres_get_noncrossed_zero() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.crossed_columns[0] = 1;

        let opt = mm.get_noncrossed_zero();
        let (row, col) = opt.unwrap();
        assert_eq!((0, 1), (row, col));
        assert_ne!((0, 0), (row, col));
    }

    #[test]
    fn munkres_is_star_in_row() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.assignment_mask.matrix[0][0] = 1;

        assert_eq!(true, mm.is_star_in_row(0));
        assert_eq!(false, mm.is_star_in_row(1));
        assert_eq!(false, mm.is_star_in_row(2));
    }

    #[test]
    fn munkres_get_star_in_row() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.assignment_mask.matrix[0][0] = 1;
        
        let star = mm.get_star_in_row(0).unwrap();
        assert_eq!(0, star);
    }

    #[test]
    fn munkres_fourth_step() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.assignment_mask.matrix[0][0] = 1;
        mm.crossed_columns[0] = 1;
        mm.fourth_step();

        let expected_assignment_mask = vec![
            vec![1, 2, 0],
            vec![2, 0, 0],
            vec![0, 0, 0],
        ];

        let expected_crossed_rows = vec![1, 0, 0];
        let expected_crossed_columns = vec![0, 0, 0];

        assert_eq!(expected_assignment_mask, mm.assignment_mask.matrix);
        assert_eq!(expected_crossed_rows, mm.crossed_rows);
        assert_eq!(expected_crossed_columns, mm.crossed_columns);
    }

    #[test]
    fn munkres_get_star_row_index() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.assignment_mask.matrix[0][0] = 1;

        assert_eq!(0, mm.get_star_row_index(0).unwrap());
        assert_eq!(None, mm.get_star_row_index(1));
        assert_eq!(None, mm.get_star_row_index(2));
    }

    #[test]
    fn munkres_get_prime_column_index() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.assignment_mask.matrix[0][0] = 1;
        mm.assignment_mask.matrix[0][1] = 2;
        mm.assignment_mask.matrix[1][0] = 2;

        assert_eq!(1, mm.get_prime_column_index(0).unwrap());
        assert_eq!(0, mm.get_prime_column_index(1).unwrap());
        assert_eq!(None, mm.get_prime_column_index(2));
    }

    #[test]
    fn munkres_unstar_starred_star_primed() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.assignment_mask.matrix[0][0] = 1;
        mm.assignment_mask.matrix[0][1] = 2;
        mm.assignment_mask.matrix[1][0] = 2;

        mm.path.path_count = 3;
        mm.path.path[0][0] = 1;
        mm.path.path[0][1] = 0;
        mm.path.path[2][0] = 0;
        mm.path.path[2][1] = 1;

        mm.unstar_starred_star_primed();

        let expected_assignment = vec![
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ];

        assert_eq!(expected_assignment, mm.assignment_mask.matrix);
    }

    #[test]
    fn munkres_fift_step() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.assignment_mask.matrix[0][0] = 1;
        mm.assignment_mask.matrix[0][1] = 2;
        mm.assignment_mask.matrix[1][0] = 2;
        mm.path.starting_row = 1;
        mm.path.starting_column = 0;

        mm.fifth_step();

        let expected_assignment = vec![
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ];

        assert_eq!(expected_assignment, mm.assignment_mask.matrix);
    }

    #[test]
    fn munkres_get_min_value() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.crossed_columns = vec![1, 1, 0];

        assert_eq!(1, mm.get_min_value());
    }

    #[test]
    fn munkres_sixth_step() {
        let matrica = Matrix::new(vec![
            vec![0, 0, 1],
            vec![0, 1, 3],
            vec![0, 2, 5],
        ]);

        let mut mm = MadarskaMetodaMunkres::new(&matrica);
        mm.crossed_columns = vec![1, 1, 0];

        let expected_matrix = vec![
            vec![0, 0, 0],
            vec![0, 1, 2],
            vec![0, 2, 4],
        ];

        mm.sixth_step();

        assert_eq!(expected_matrix, mm.calculating_matrix.matrix);
    }
}