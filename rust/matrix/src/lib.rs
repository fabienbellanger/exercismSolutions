pub struct Matrix {
    rows: usize,
    cols: usize,
    items: Vec<u32>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let mut rows = 0;
        let mut cols = 0;

        let items = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                rows += 1;
                let line = line
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                if i == 0 {
                    cols = line.len();
                }
                line
            })
            .collect::<Vec<_>>();

        Self { rows, cols, items }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no > self.rows || row_no == 0 {
            return None;
        }

        Some(self.items[(row_no - 1) * self.cols..row_no * self.cols].to_vec())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no > self.cols || col_no == 0 {
            return None;
        }

        let column = (0..self.rows)
            .map(|i| self.items[i * self.cols + col_no - 1])
            .collect::<Vec<_>>();

        Some(column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_new() {
        let matrix = Matrix::new("1 2\n3 4");
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 2);
        assert_eq!(matrix.items, vec![1, 2, 3, 4]);

        let matrix = Matrix::new("1 2 3\n4 5 6");
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 3);
        assert_eq!(matrix.items, vec![1, 2, 3, 4, 5, 6]);
    }
}
