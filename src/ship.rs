/* Ship */

use macroquad::prelude::*;

#[derive(Debug)]
struct UserPlane {
    name: String,
    health: i32,
    speed: i32,
    boost: bool,
    ammo: i32,
}

impl UserPlane {

    pub fn new() -> Self {
        UserPlane {
            name: "MyPlane".to_string(),
            health: 100,
            speed: 50,
            boost: false,
            ammo: 50,
        }
    }    

    // fn name_plane(&mut self, name: String) {
    //     self.name = name;       
    // }

    fn ammo(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.ammo -= 1;
            if self.ammo <= 0 {
                self.ammo = 0; 
            }
        }
    }

    fn speed(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.speed = 70; 
        } else if is_key_released(KeyCode::Up) {
            self.speed = 50;
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
    
    let mut plane = UserPlane::new();

    loop {
        request_new_screen_size(640.0, 360.0);
        clear_background(BLACK); 
        plane.ammo();
        plane.speed();
        plane.boost();
        draw_text(&plane.name, 50.0, 50.0, 20.0, WHITE);
        draw_text(&plane.health.to_string(), 50.0, 70.0, 20.0, WHITE);
        draw_text(&plane.speed.to_string(), 50.0, 90.0, 20.0, WHITE);
        draw_text(&plane.boost.to_string(), 50.0,110.0, 20.0, WHITE);
        draw_text(&plane.ammo.to_string(), 50.0, 130.0, 20.0, WHITE);
        

        next_frame().await
    }
}