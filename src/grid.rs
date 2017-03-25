use rand::{Rng, thread_rng};
use rayon::prelude::*;

pub struct Grid {
    cur: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            cur: vec![false;width * height],
            width: width,
            height: height,
        }
    }
    pub fn random(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        let mut cur = vec![false;width * height];
        for e in cur.iter_mut() {
            *e = rng.gen();
        }
        Grid {
            cur: cur,
            width: width,
            height: height,
        }
    }
    #[allow(dead_code)]
    pub fn tick(&mut self) {
        let mut alt = vec![false; self.width * self.height];
        alt.par_iter_mut().enumerate().for_each(|(i, a)| *a = self.will_live(i));
        self.cur = alt;
    }
    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.cur[y * self.width + x]
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    fn will_live(&self, i: usize) -> bool {
        let n = self.neighbours(i);
        (!self.cur[i] && n == 3) || (self.cur[i] && (n == 2 || n == 3))
    }
    fn neighbours(&self, index: usize) -> usize {
        let mut neighbours = 0;
        let width = self.width as isize;
        let len = self.width * self.height;
        let i = index as isize;
        let neigh = [i - width - 1,
                     i - width,
                     i - width + 1,
                     i - 1,
                     i + 1,
                     i + width - 1,
                     i + width,
                     i + width + 1];

        for &j in neigh.iter() {
            if j > 0 && j < len as isize && self.cur[j as usize] ||
               j < 0 && self.cur[(len as isize + j) as usize] ||
               j >= len as isize && self.cur[(j - len as isize) as usize] {
                neighbours += 1
            }
        }
        neighbours
    }
}
