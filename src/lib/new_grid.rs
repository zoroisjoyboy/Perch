use std::vec;


enum Generate {
    ObstacleGen,
    MysteryGen,
}

#[derive(Debug)]
pub enum GridState {
    Empty,
    Obstacle,
    Mystery,
    PlayerX,
}

#[derive(Debug)]
pub struct Grid {
    pub x: usize,
    pub y: usize,
    pub grid: Vec<Vec<i32>>, 
    state: GridState,
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        Grid {
            x,
            y,
            grid: vec![vec![0; x]; y],
            state: GridState::Empty,
        }
    }
    

}