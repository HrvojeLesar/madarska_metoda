
enum Position {
    Row,
    Column,
}

#[derive(Debug)]
struct Matrix {
    rows: usize,
    columns: usize,
    matrix: Vec<Vec<i32>>,
}

impl Matrix {
    fn new_empty(row: usize, column: usize) -> Self {
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
    fn new(data: Vec<Vec<i32>>) -> Self {
        Self {
            rows: data.len(),
            columns: data[0].len(),
            matrix: data,
        }
    }

    // fn find_min_row(red: &Vec<i32>) -> i32 {
    //     let mut min = red[0];
    //     for broj in red.iter() {
    //         if min == 0 {
    //             return 0;
    //         }
    //         if broj < &min {
    //             min = *broj;
    //         }
    //     }
    //     return min;
    // }

    fn find_min_row(&self, index: usize) -> i32 {
        let row = &self.matrix[index];
        let mut min = row[0];
        for num in row.iter() {
            if min == 0 {
                return 0;
            }
            if num < &min {
                min = *num;
            }
        }
        return min;
    }

    // fn find_min_col(stupac: &Vec<Vec<i32>>, indeks: usize) -> i32 {
    //     let mut min: i32 = stupac[0][indeks];
    //     for i in 0..stupac.len() {
    //         if min == 0 {
    //             return 0;
    //         }
    //         if stupac[i][indeks] < min {
    //             min = stupac[i][indeks];
    //         }
    //     }
    //     return min;
    // }

    fn find_min_col(&self, index: usize) -> i32 {
        let column = &self.matrix;
        let mut min: i32 = column[0][index];
        for i in 0..column.len() {
            if min == 0 {
                return 0;
            }
            if column[i][index] < min {
                min = column[i][index];
            }
        }
        return min;
    }

    fn count(&self, index: usize, position: Position, value: i32) -> usize {
        match position {
            Position::Row => {
                return self.matrix[index].iter().filter(|x| **x == value).count();
            },
            Position::Column => {
                let mut count: usize = 0;
                let matrix = &self.matrix;
                for i in 0..self.columns {
                    if matrix[i][index] == value {
                        count += 1;
                    }
                }
                return count;
            },
        }
    }

    fn get_matrix(&self) -> &Vec<Vec<i32>> {
        &self.matrix
    }
}

struct MadarskaMetoda { }

impl MadarskaMetoda {
    // 1. oduzmi minimum z sakoga reda, oduzmi minimum z sakoga stupca
    // 2. prekriziti redove koji imaju 0, prekriziti stupce koji imaju 0 (zbrojiti broj prekrizenih stupaca i redova)
    //    ako je zbroj prekrizenih redova i stupaca == redovi ili stupci, gotovo inace korak 3
    // 3. najdi minimum od ne oznacenih, dodaj minimuma oznacenima oduzmi neoznacenima
    pub fn solve(starting_matrix: &Matrix) {
        let mut calculating_matrix = Matrix::new(starting_matrix.matrix.clone());

        calculating_matrix = Self::first_step(calculating_matrix);
        Self::second_step(&calculating_matrix);
    }
    
    fn first_step(mut matrix: Matrix) -> Matrix {
        for i in 0..matrix.rows {
            let min = matrix.find_min_row(i);
            for j in 0..matrix.rows {
                matrix.matrix[i][j] -= min;
            }
        }

        for i in 0..matrix.columns {
            let min = matrix.find_min_col(i);
            for j in 0..matrix.columns {
                matrix.matrix[j][i] -= min;
            }
        }
        matrix
    }

    fn second_step(matrix: &Matrix) {
        let mut selected_rows: Vec<usize> = Vec::new();
        let mut selected_columns: Vec<usize> = Vec::new();

        for i in 0..matrix.rows {
            if selected_rows.contains(&i) { continue; }
            for j in 0..matrix.columns {
                if selected_columns.contains(&j) { continue; }
                if matrix.matrix[i][j] == 0 {
                    let zero_in_row = matrix.count(i, Position::Row, 0);
                    let zero_in_column = matrix.count(j, Position::Column, 0);
                    if zero_in_row >= zero_in_column {
                        selected_rows.push(i);
                        break;
                    } else {
                        selected_columns.push(j);
                        break;
                    }
                }
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: usize = 5;
    const STUPAC: usize = 6;



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
    #[should_panic]
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
        assert_ne!(4, matrica.find_min_row(3));
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
    fn solve() {
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
        MadarskaMetoda::solve(&matrica);
    }
}
