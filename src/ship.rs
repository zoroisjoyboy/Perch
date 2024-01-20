/* Ship */

use macroquad::prelude::*;

#[derive(Debug)]
struct UserPlane {
    name: String,
    x: i32,
    y: i32,
    health: i32,
    walk: i32,
    boost: bool,
    ammo: i32,
}

impl UserPlane {

    pub fn new(x: i32, y: i32) -> Self {
        UserPlane {
            name: "GoMerry".to_string(),
            x,
            y,
            health: 100,
            walk: 1,
            boost: false,
            ammo: 50,
        }
    }    

    fn name_plane(&mut self, name: String) {
        self.name = name;       
    }

    fn left_move(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            self.x -= 1;
            if self.x < 0 {
                self.x = 0;
            }
        }
    }

    fn right_move(&mut self, max_y: i32) {
        if is_key_pressed(KeyCode::Right) {
            self.x += 1;
            if self.x > max_y {
                self.x = max_y;
            }
        }
    }

    fn ammo(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.ammo -= 1;
            if self.ammo <= 0 {
                self.ammo = 0; 
            }
        }
    }

    fn boost(&mut self) {
        if is_key_down(KeyCode::RightShift) {
            self.boost = true;
        } else if is_key_released(KeyCode::RightShift) {
            self.boost = false;
        }
    }

}

#[macroquad::main("InstrumentPanel")]
async fn main() {

}