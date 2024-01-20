/* Main */
use macroquad::prelude::*;
use std::time::Duration;
use std::thread;
mod grid;

const CELL_SIZE: f32 = 20.;
const PADDING: f32 = 2.;

fn window_size(columns: usize, rows: usize) -> (f32, f32) {
    let width = columns as f32 * (CELL_SIZE + PADDING) + PADDING;
    let height = rows as f32 * (CELL_SIZE + PADDING) + PADDING;
    (width, height)
}

#[macroquad::main("Matrix Display")]
async fn main() {
    let mut g = grid::grid::Grid::new(50, 70);
    let (width, height) = window_size(g.y, g.x);

    loop {
        clear_background(BEIGE);
        request_new_screen_size(width, height);

        g.display_grid();
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
        let sleep_duration = Duration::from_millis(100);
        thread::sleep(sleep_duration);
        next_frame().await;
    }
}