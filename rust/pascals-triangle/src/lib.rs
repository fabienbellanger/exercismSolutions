pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.0 {
            0 => Vec::new(),
            1 => vec![vec![1]],
            _ => {
                let mut result = vec![vec![1], vec![1, 1]];

                for _ in 2..self.0 {
                    let mut line = vec![1];
                    let last_line = result.last().unwrap();

                    for j in 0..last_line.len() - 1 {
                        line.push(last_line.get(j).unwrap() + last_line.get(j + 1).unwrap());
                    }

                    line.push(1);
                    result.push(line);
                }

                result
            },
        }
    }
}
