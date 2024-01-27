/* Main */
use macroquad::prelude::*;
use std::time::Duration;
use std::thread;
use lib::{grid, ship};

const CELL_SIZE: f32 = 20.;
const PADDING: f32 = 2.;

fn window_size(columns: usize, rows: usize) -> (f32, f32) {
    let width = rows as f32 * (CELL_SIZE + PADDING) + PADDING;
    let height = columns as f32 * (CELL_SIZE + PADDING) + PADDING;
    (width, height)
}

#[macroquad::main("Perch")]
async fn main() {
    let mut g = lib::grid::Grid::new(30, 15);
    let (width, height) = window_size(g.y, g.x);
    let mut s = lib::ship::Ship::new(((width / 2.0 - CELL_SIZE) - PADDING + 1.0) as usize, ((height - CELL_SIZE) - PADDING) as usize); 
    
    g.display_grid();

    loop {
        clear_background(BEIGE);
        request_new_screen_size(width, height);

        let g_matrix = &mut g.grid;
        
        for (i, row) in g_matrix.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                let x = j as f32 * (CELL_SIZE + PADDING) + PADDING;
                let y = i as f32 * (CELL_SIZE + PADDING) + PADDING;
                
                match value {
                    1 => draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, BLACK),
                    2 => draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, BLUE),
                    3 => draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, PINK),
                    _ => draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, BEIGE),
                }
            }
        }

        draw_rectangle(s.x as f32, s.y as f32, CELL_SIZE, CELL_SIZE, WHITE);

        // let current_health = g_matrix[s.x][s.y]; 

        // s.health(current_health);

        // if current_health <= 0 {
        //     break;
        // }

        if is_key_down(KeyCode::Up) {
            g.regenerate_top_row();
        }

        s.shoot();
        s.left_move(CELL_SIZE as usize, PADDING as usize);
        s.right_move(width as usize, CELL_SIZE as usize, PADDING as usize);

        let sleep_duration = Duration::from_millis(35);
        thread::sleep(sleep_duration);
        next_frame().await;
    }
}