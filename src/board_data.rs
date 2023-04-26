use crate::lrud::LRUD;
use std::iter::ExactSizeIterator;
use std::ops::Index;
use std::ops::IndexMut;
use rand::Rng;

#[derive(Debug)]
pub struct BoardData {
    pub data: [[u32; 4]; 4]
}

impl Default for BoardData {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardData {
    pub fn new() -> Self {
        let data: [[u32; 4]; 4] = [[0; 4]; 4];
        let mut board_data = Self { data };
        board_data.insert();
        board_data.insert();
        board_data
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn insert(&mut self) {
        let mut rng = rand::thread_rng();
        let mut r = rng.gen_range(0..4);
        let mut c = rng.gen_range(0..4);
        while self.data[r][c] != 0 {
            r = rng.gen_range(0..4);
            c = rng.gen_range(0..4);
        }
        let vals = vec![2, 4];
        let val = vals[rng.gen_range(0..2)];
        self.data[r][c] = val;
    }

    pub fn push(&mut self, lrud: LRUD) -> (u32, bool, bool) {
        let (score, moved) = match lrud {
            LRUD::Left => self.push_left(),
            LRUD::Right => self.push_right(),
            LRUD::Up => self.push_up(),
            LRUD::Down => self.push_down(),
        };
        if moved {
            self.insert();
        }
        let over = self.is_full() && !self.can_merge();
        (score, moved, over)
    }

    fn push_left(&mut self) -> (u32, bool) {
        let score = self.merge_left();
        let moved = self._push_left();
        (score, score != 0 || moved)
    }

    fn push_right(&mut self) -> (u32, bool) {
        self.swap_lr();
        let result = self.push_left();
        self.swap_lr();
        result
    }

    fn push_up(&mut self) -> (u32, bool) {
        self.swap_diagnol();
        let result = self.push_left();
        self.swap_diagnol();
        result
    }

    fn push_down(&mut self) -> (u32, bool) {
        self.swap_ud();
        let result = self.push_up();
        self.swap_ud();
        result
    }

    fn merge_left(&mut self) -> u32 {
        let mut score = 0;
        for r in 0..4 {
            let mut i = 0;
            while i < 3 {
                if self.data[r][i] == 0 {
                    i += 1;
                    continue;
                }
                let mut j = i + 1;
                while j < 4 && self.data[r][j] == 0 {
                    j += 1;
                }
                if j == 4 {
                    break;
                }
                if self.data[r][i] == self.data[r][j] {
                    self.data[r][i] *= 2;
                    score += self.data[r][i];
                    self.data[r][j] = 0;
                    i = j + 1;
                } else {
                    i = j;
                }
            }
        }
        score
    }

    fn _push_left(&mut self) -> bool {
        let mut moved = false;
        for r in 0..3 {
            let mut i = 0;
            while i < 4 {
                if self.data[r][i] != 0 {
                    i += 1;
                    continue;
                }
                let mut j = i + 1;
                while j < 4 && self.data[r][j] == 0 {
                    j += 1;
                }
                if j == 4 {
                    break;
                }
                moved = true;
                self.data[r][i] = self.data[r][j];
                self.data[r][j] = 0;
                i += 1;
            }
        }
        moved
    }

    fn swap_lr(&mut self) {
        for r in 0..4 {
            for c in 0..2 {
                self.data[r].swap(c, 3-c);
            }
        }
    }

    fn swap_diagnol(&mut self) {
        for r in 0..4 {
            for c in (r + 1)..4 {
                let tmp = self.data[r][c];
                self.data[r][c] = self.data[c][r];
                self.data[c][r] = tmp;
            }
        }
    }

    fn swap_ud(&mut self) {
        for r in 0..2 {
            for c in 0..4 {
                let tmp = self.data[r][c];
                self.data[r][c] = self.data[3 - r][c];
                self.data[3 - r][c] = tmp;
            }
        }
    }

    fn is_full(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.data[i][j] == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn can_merge(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if i != 3 && self.data[i][j] == self.data[i + 1][j] {
                    return true;
                }
                if j != 3 && self.data[i][j] == self.data[i][j + 1] {
                    return true;
                }
            }
        }
        false
    }   
}

impl Index<usize> for BoardData {
    type Output = [u32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for BoardData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Iterator for BoardData {
    type Item = [u32; 4];

    fn next(&mut self) -> Option<Self::Item> {
        self.data.iter_mut().next().copied()
    }
}

impl ExactSizeIterator for BoardData {
    fn len(&self) -> usize {
        self.data.len()
    }
}