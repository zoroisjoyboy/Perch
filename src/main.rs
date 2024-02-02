/* Main */
use macroquad::prelude::*;
use std::time::Duration;
use std::thread;
use lib::{grid, ship};

const CELL_SIZE: f32 = 20.;
const PADDING: f32 = 2.;

fn grid_to_window(grid_x: usize, grid_y: usize) -> (f32, f32) {
    let x = grid_x as f32 * (CELL_SIZE + PADDING) + PADDING;
    let y = grid_y as f32 * (CELL_SIZE + PADDING) + PADDING;
    (x, y)
}

fn window_to_grid(window_x: f32, window_y: f32) -> (usize, usize) {
    let grid_x = ((window_x - PADDING) / (CELL_SIZE + PADDING)).floor() as usize;
    let grid_y = ((window_y - PADDING) / (CELL_SIZE + PADDING)).floor() as usize;
    (grid_x, grid_y)
}

fn current_state_of_ship(grid: &mut grid::Grid, ship_x: usize, ship_y: usize) -> i32 {
    let (s_grid_x, s_grid_y) = window_to_grid(ship_x as f32, ship_y as f32);
    let current_state = grid.grid[s_grid_y][s_grid_x];
    return current_state;
}

fn change_state(grid: &mut grid::Grid, ship_x: usize, ship_y: usize, new_state: i32) {
    let (s_grid_x, s_grid_y) = window_to_grid(ship_x as f32, ship_y as f32);
    grid.grid[s_grid_y][s_grid_x] = new_state;
}

#[macroquad::main("Perch")]
async fn main() {
    let mut grid = grid::Grid::new(30, 16);
    let (width, height) = grid_to_window(grid.x, grid.y);
    let mut ship = ship::Ship::new(((width / 2.0 - CELL_SIZE) - PADDING + 1.0) as usize, ((height - CELL_SIZE) - PADDING) as usize);

    grid.display_grid();
    if current_state_of_ship(&mut grid, ship.x, ship.y) == 1 {
        change_state(&mut grid, ship.x, ship.y, 0);
    }

    loop {
        clear_background(BEIGE);
        request_new_screen_size(width, height);

        draw_grid(&grid);
        
        draw_ship(&ship); 

        grid.regenerate_top_row();

        handle_input(width as usize, &mut grid, &mut ship);

        if current_state_of_ship(&mut grid, ship.x, ship.y) == 1 {
            ship.health -= 50;
            change_state(&mut grid, ship.x, ship.y, 0);
        }

        if ship.health > 0 {
            draw_text(&ship.health.to_string(), 600.0, 20.0, 35.0, BLACK);
        } else {
            draw_text("Dead", 600.0, 20.0, 35.0, BLACK);
        }
        
        thread::sleep(Duration::from_millis(75));
        next_frame().await;
    }
}

fn draw_grid(grid: &grid::Grid) {
    for (i, row) in grid.grid.iter().enumerate() {
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
}

fn draw_ship(ship: &ship::Ship) {
    draw_rectangle(ship.x as f32, ship.y as f32, CELL_SIZE, CELL_SIZE, WHITE);
}

fn handle_input(width: usize, grid: &mut grid::Grid, ship: &mut ship::Ship) {
    ship.left_move(CELL_SIZE as usize, PADDING as usize);
    ship.right_move(width, CELL_SIZE as usize, PADDING as usize);
}
