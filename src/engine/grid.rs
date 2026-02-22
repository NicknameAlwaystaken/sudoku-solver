#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grid {
    cells: [u8; 81],
}

#[derive(Debug)]
pub enum GridError {
    OutOfRangeValue(u8),
}

impl Grid {
    pub fn new_empty() -> Self {
        Self { cells: [0; 81] }
    }

    pub fn from_array(cells: [u8; 81]) -> Result<Self, GridError> {
        for &v in &cells {
            if v > 9 {
                return Err(GridError::OutOfRangeValue(v));
            }
        }
        Ok(Self { cells })
    }

    pub fn as_array(&self) -> &[u8; 81] {
        &self.cells
    }

    pub fn as_array_mut(&mut self) -> &mut [u8; 81] {
        &mut self.cells
    }

    #[inline]
    pub fn get(&self, idx: usize) -> u8 {
        self.cells[idx]
    }

    #[inline]
    pub fn set(&mut self, idx: usize, val: u8) {
        self.cells[idx] = val;
    }

    #[inline]
    pub fn idx(row: usize, col: usize) -> usize {
        row * 9 + col
    }

    pub fn is_valid_given(&self) -> bool {
        for idx in 0..81 {
            let v = self.cells[idx];
            if v == 0 {
                continue;
            }
            if !is_placement_valid(&self.cells, idx, v) {
                return false;
            }
        }
        true
    }

    pub fn is_placement_valid(&self, idx: usize, val: u8) -> bool {
        is_placement_valid(&self.cells, idx, val)
    }
}

pub fn is_placement_valid(cells: &[u8; 81], idx: usize, val: u8) -> bool {
    debug_assert!((1..=9).contains(&val));

    let row = idx / 9;
    let col = idx % 9;

    // row
    for c in 0..9 {
        let i = Grid::idx(row, c);
        if i != idx && cells[i] == val {
            return false;
        }
    }

    // col
    for r in 0..9 {
        let i = Grid::idx(r, col);
        if i != idx && cells[i] == val {
            return false;
        }
    }

    // box
    let box_r = (row / 3) * 3;
    let box_c = (col / 3) * 3;
    for r in box_r..box_r + 3 {
        for c in box_c..box_c + 3 {
            let i = Grid::idx(r, c);
            if i != idx && cells[i] == val {
                return false;
            }
        }
    }

    true
}
