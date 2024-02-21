
use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;
use std::collections::HashMap;
use std::{collections::HashSet, vec};

#[derive(Debug)]
pub struct Grid {
    pub x: usize,
    pub y: usize,
    pub grid: Vec<Vec<i32>>, 
    obstacle_row: Vec<bool>,
    boost_row: Vec<bool>,
    heal_row: Vec<bool>,
    obstacle_gen: f64,
    boost_gen: f64,
    heal_gen: f64,
}

impl Grid {

    pub fn new(x: usize, y: usize) -> Self {
        Grid {
            x,
            y,
            grid: vec![vec![0; x]; y], 
            obstacle_row: vec![false; x],
            boost_row: vec![false; x], 
            heal_row: vec![false; x],
            obstacle_gen: 0.02,
            boost_gen: 0.02,
            heal_gen: 0.02,
        }
    }
    
    pub fn display_grid(&mut self) {
        for i in 0..self.grid.len() {
            for (&index, value) in &self.generate_row() {
                self.grid[i][index] = *value;
            }
        }
    }

    pub fn regenerate_top_row(&mut self) {
        let new_chunk_elements = self.generate_row();
        let mut new_top_row: Vec<i32> = vec![0; self.x + 1];
            
        for (&index, value) in &new_chunk_elements {
            new_top_row[index] = *value; 
        }
        self.grid.insert(0, new_top_row);
        self.grid.pop();
    }

    fn generate_obstacles_row(&mut self) {
        let row = &mut self.obstacle_row; 
        let chunk_generated = rand_chunk_generate(row, self.obstacle_gen); 
        for i in 0..row.len() {
            if chunk_generated.contains(&i) {
                row[i] = true;
            }
        }
    }

    fn generate_boosts_row(&mut self) {
        let row = &mut self.boost_row;
        let chunk_generated = rand_chunk_generate(row, self.boost_gen);  
        for i in 0..row.len() {
            if chunk_generated.contains(&i) {
                if self.obstacle_row[i] {
                    let mut pos_ref = i;
                    while self.obstacle_row[pos_ref] || pos_ref == i {
                        pos_ref = thread_rng().gen_range(0..row.len());
                    }
                    row[pos_ref] = true;
                } else {
                    row[i] = true;
                }
            }
        }
    }       

    fn generate_heals_row(&mut self) {
        let row: &mut Vec<bool> = &mut self.heal_row;
        let chunk_generated = rand_chunk_generate(row, self.heal_gen);
        for i in 0..row.len() {
            if chunk_generated.contains(&i) {
                if self.obstacle_row[i] || self.boost_row[i] {
                    let mut pos_ref = i;
                    while self.obstacle_row[pos_ref] || self.boost_row[pos_ref] || pos_ref == i {
                        pos_ref = thread_rng().gen_range(0..row.len());
                    }
                    row[pos_ref] = true;
                } else {
                    row[i] = true;
                }
            } 
        }
    }

    fn generate_row(&mut self) -> HashMap<usize, i32> {
        self.generate_obstacles_row();
        self.generate_boosts_row();
        self.generate_heals_row();
        
        let mut generated_row: HashMap<usize, i32> = HashMap::new();
        let n: usize = self.obstacle_row.len();
        for i in 0..n {
            if self.obstacle_row[i] {
                generated_row.insert(i, 1);
            } else if self.boost_row[i] {
                generated_row.insert(i, 2);
            } else if self.heal_row[i] {
                generated_row.insert(i,3);
            } else {
                generated_row.insert(i, 0);
            }
        }

        self.clear_rows();
        return generated_row;
    }

    fn clear_rows(&mut self) {
        self.obstacle_row = vec![false; self.x as usize];
        self.boost_row = vec![false; self.x as usize];
        self.heal_row = vec![false; self.x as usize];
    }

}

fn rand_chunk_generate(row: &mut Vec<bool>, gen_perc: f64) -> HashSet<usize> {
    let num_items = (row.len() as f64 * gen_perc).round() as i32; 
    let mut items_hashset = HashSet::new();
    while (items_hashset.len() as i32) < num_items {
        let items_pos = thread_rng().gen_range(0..row.len());
        items_hashset.insert(items_pos);
    }
    items_hashset
}
