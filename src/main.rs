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

fn current_state_of_ship(grid: &mut lib::grid::Grid, ship_x: f32, ship_y: f32) {}

#[macroquad::main("Perch")]
async fn main() {
    let mut grid = lib::grid::Grid::new(30, 30);
    let (width, height) = grid_to_window(grid.x, grid.y);
    let mut ship = lib::ship::Ship::new(((width / 2.0 - CELL_SIZE) - PADDING + 1.0) as usize, ((height - CELL_SIZE) - PADDING) as usize);

    grid.display_grid(); //ensure where the ship is initially placed is not an obstacle 
    let (s_grid_x, s_grid_y) = window_to_grid(ship.x as f32, ship.y as f32);
    let current_state = grid.grid[s_grid_x][s_grid_y];
    if current_state == 1 {
        grid.grid[s_grid_x][s_grid_y] = 0;
    }

    loop {
        clear_background(BEIGE);
        request_new_screen_size(width, height);

        draw_grid(&grid);

        draw_ship(&ship); 

        handle_input(width as usize, &mut grid, &mut ship);

        let (s_grid_x, s_grid_y) = window_to_grid(ship.x as f32, ship.y as f32);
        println!("s.x: {}, s.y: {}, s_grid_x: {}, s_grid_y: {} ", ship.x, ship.y, s_grid_x, s_grid_y);

        let current_state = grid.grid[s_grid_x][s_grid_y]; // this is where right move is breaking.
        println!("{}", current_state);

        ship.health(current_state);
        if ship.health <= 0 {
            println!("Poop");
        }
    
        next_frame().await;
    }
}

fn draw_grid(grid: &lib::grid::Grid) {
    for (i, row) in grid.grid.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            let x = j as f32 * (CELL_SIZE + PADDING) + PADDING;
            let y = i as f32 * (CELL_SIZE + PADDING) + PADDING;

            draw_cell(value, x, y);
        }
    }
}

fn draw_ship(ship: &lib::ship::Ship) {
    draw_rectangle(ship.x as f32, ship.y as f32, CELL_SIZE, CELL_SIZE, WHITE);
}

fn draw_cell(value: i32, x: f32, y: f32) {
    match value {
        1 => draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, BLACK),
        2 => draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, BLUE),
        3 => draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, PINK),
        _ => draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, BEIGE),
    }
}

fn handle_input(width: usize, grid: &mut lib::grid::Grid, ship: &mut lib::ship::Ship) {
    if is_key_pressed(KeyCode::Up) {
        grid.regenerate_top_row();
    }
    ship.left_move(CELL_SIZE as usize, PADDING as usize);
    ship.right_move(width, CELL_SIZE as usize, PADDING as usize);
}
