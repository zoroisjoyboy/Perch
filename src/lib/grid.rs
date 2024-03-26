use core::num;
use std::{collections::HashSet, hash::Hash, vec};
use macroquad::rand::gen_range;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

const OBS_CELL_QUANT: f32 = 100.0;
const OBS_NUM: f32 = 10.0;
const MYST_CELL_QUANT: f32 = 18.0;
const MYST_NUM: f32 = 2.0;
const MIN_SEP: usize = 2.0 as usize;

#[derive(Debug)]
pub enum GridState {
    Empty(i32),
    Obstacle(i32),
    Mystery(i32),
}

#[derive(Debug)]
pub struct Grid {
    pub r: usize,
    pub c: usize,
    pub grid: Vec<Vec<i32>>, 
    myst_matrix: Vec<Vec<i32>>, 
    obs_matrix: Vec<Vec<i32>>,
}

impl Grid {

    pub fn new(row: usize, col: usize) -> Self {
        Grid {
            r: row,
            c: col,
            grid: vec![vec![0; col]; row],
            myst_matrix: vec![vec![0; col]; row],
            obs_matrix: vec![vec![0; col]; row],
        }
    }

    pub fn create_grid(&mut self) {
        self.gen_obs_matrix();
        self.gen_myst_matrix();
    }

    pub fn update_grid(&mut self, mut temp_grid: Vec<Vec<i32>>) {
        if let Some(new_row) = temp_grid.pop() {
            self.grid.insert(0, new_row);
        } else {
            panic!("Empty temp_grid!\n");
        }
        self.grid.pop();
    }

    fn gen_rand_coords(&mut self, num_pairs: usize, min_seperation: usize) -> HashSet<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let mut used_coordinates = HashSet::new();
        let mut count_pairs: i32 = 0;
        let mut attempt_count = 0;
        let mut is_far_enough: bool = false;

        while (count_pairs as usize) < num_pairs {
            let mut r = rng.gen_range(0..self.r);
            let mut c = rng.gen_range(0..self.c);

            while !is_far_enough {
            
                is_far_enough = used_coordinates.iter().all(|&(x, y)| {
                    (r as isize - x as isize).abs() >= min_seperation as isize * MIN_SEP as isize
                        && (c as isize - y as isize).abs() >= min_seperation as isize * MIN_SEP as isize
                });
                
                r = rng.gen_range(0..self.r);
                c = rng.gen_range(0..self.c);

            }

            used_coordinates.insert((r, c));
            count_pairs += 1;
        }
        
        used_coordinates
    }

    fn fill_adjacent_cells(&mut self, env: i32, r: usize, c: usize, mut cells_to_fill: f32) { 
        let mut rng = rand::thread_rng();
        self.grid[r][c] = env; // replace type, cannot just be 1 as it's being used by both obs and myst 
        let moves: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

        let mut moves = moves.clone();
        moves.shuffle(&mut thread_rng());
        
        for (dr, dc) in moves {
            let new_r = r.wrapping_add(dr as usize);
            let new_c = c.wrapping_add(dc as usize);

            cells_to_fill -= 1.0;

            if new_r < self.r && new_c < self.c && cells_to_fill > 0.0 {
                if self.grid[new_r][new_c] == 0 {
                    self.fill_adjacent_cells(env, new_r, new_c, cells_to_fill - 1.0);
                    
                }
            }
        }
    }

    fn gen_obs_matrix(&mut self) {
        let ratio: f32 = OBS_CELL_QUANT / (self.c * self.r) as f32;
        let cells_per_block: f32 = OBS_CELL_QUANT / OBS_NUM;
        let mut current_ratio: f32 = 0.0;      
        let mut count = 0;
        let obs_coordinates: HashSet<(usize, usize)> = self.gen_rand_coords(OBS_NUM as usize,  OBS_CELL_QUANT as usize / OBS_NUM as usize);
        
        while current_ratio < ratio || count < OBS_NUM as i32 {
            for i in 0..self.r {
                for j in 0..self.c {
                    let current_coords = (i, j);
                    let obs_coord_set: HashSet<(usize, usize)> = obs_coordinates.iter().cloned().collect();
                    if obs_coord_set.contains(&current_coords) {
                        let cells_to_fill = rand::thread_rng().gen_range(cells_per_block / 2.0..cells_per_block);
                        self.fill_adjacent_cells(1, i, j, cells_to_fill);                           
                        current_ratio += cells_to_fill / (self.c * self.r) as f32;
                        count += 1; 
                    }
                }        
            }
        }
                
    }

    fn gen_myst_matrix(&mut self) { 
        let cells_per_block: f32 = MYST_CELL_QUANT / MYST_NUM; 
        let mut count = 0;
        let myst_coordinates: HashSet<(usize, usize)> = self.gen_rand_coords(MYST_NUM as usize, MYST_CELL_QUANT as usize / MYST_NUM as usize);
        while count < MYST_NUM as i32 { 
            for i in 0..self.r {
                for j in 0..self.c {
                    let current_coords = (i, j);
                    let myst_coord_set: HashSet<(usize, usize)> = myst_coordinates.iter().cloned().collect();
                    if myst_coord_set.contains(&current_coords) {
                        self.fill_adjacent_cells(2, i, j, cells_per_block); // should not overlap with obs
                        count += 1; 
                    }
                }
            }
        }
    }

    fn myst_gen_rand_asset(&mut self) {} // generate random assets when user touches myst block 
    

}


