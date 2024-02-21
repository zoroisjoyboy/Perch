use macroquad::prelude::*;

#[derive(Debug)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub velocity: f32,
    pub collided: bool
}

impl Bullet {
    pub fn new(x: f32, y: f32, velocity: f32) -> Self {
        Bullet {
            x,
            y,
            velocity,
            collided: false,
        }
    } 

    pub fn bullet_fired(&mut self) {
        self.y -= self.velocity;
    }         
}

#[derive(Debug)]
pub struct Ship {
    name: String,
    pub x: usize,
    pub y: usize,
    pub forward: bool,
    pub health: i32,
    pub boost: bool,
    pub ammo: i32,
}

impl Ship {

    pub fn new(x: usize, y: usize) -> Self {
        Ship {
            name: "GoMerry".to_string(),
            x,
            y,
            forward: false,
            health: 100,
            boost: false,
            ammo: 50,
        }
    }    

    pub fn name_plane(&mut self, name: String) {
        self.name = name;       
    }

    pub fn left_move(&mut self, cell_size: usize, padding: usize) {
        if is_key_down(KeyCode::Left) {
            if self.x < cell_size + padding {
                self.x = 2;
            } else {
                self.x -= cell_size + padding;
            }
        }
    }

    pub fn right_move(&mut self, width: usize, cell_size: usize, padding: usize) {
        if is_key_down(KeyCode::Right) {
            if self.x >= (width - cell_size) - padding {
                self.x = (width - cell_size) - padding;
            } else {
                self.x += cell_size + padding;
            }
        }
    }

    pub fn shoot(&mut self) {
        self.ammo -= 1;
        if self.ammo <= 0 {
            self.ammo = 0; 
        }
    }

    pub fn health(&mut self, state: i32) {
        if state == 1 {
            self.health -= 50;
        }
        println!("health: {}", self.health);
    }

    pub fn boost(&mut self, state: i32, count: i32) {
        if state == 3 {
            self.boost = true;
        } else if count > 2 {
            self.boost = false;
        }
    }
}