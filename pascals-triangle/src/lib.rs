pub struct PascalsTriangle {
    pub row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..(self.row_count + 1)).map(|row| {
            (0..row).scan(1, |num, col| {
                if col != 0 {
                    *num = *num * (row - col) / col;
                }
                Some(*num)
            }).collect()
        }).collect()
    }
}
