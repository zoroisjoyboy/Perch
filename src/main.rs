use macroquad::prelude::*;
use lib::{Grid, Ship, Bullet};
use std::time::{Duration, Instant};

const DESIRED_FPS: u64 = 60;
const TARGET_FRAME_TIME: Duration = Duration::from_micros(1_000_000 / DESIRED_FPS);
const CELL_SIZE: f32 = 20.;
const PADDING: f32 = 2.;

enum STATE {
    Boost,
    Health,
}

struct GameState {
    screen_width: f32,
    screen_height: f32,
}

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

fn current_state_of_object(grid: &mut Grid, x: usize, y: usize) -> i32 {
    let (grid_x, grid_y) = window_to_grid(x as f32, y as f32);
    let current_state = grid.grid[grid_y][grid_x];
    return current_state;
}

fn change_state(grid: &mut Grid, x: usize, y: usize, new_state: i32) {
    let (grid_x, grid_y) = window_to_grid(x as f32, y as f32);
    grid.grid[grid_y][grid_x] = new_state;
}

fn draw_grid(grid: &Grid) {
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

fn draw_ship(ship: &Ship) {
    draw_rectangle(ship.x as f32, ship.y as f32, CELL_SIZE, CELL_SIZE, WHITE);
} 

fn draw_panel(grid: &Grid, ship: &Ship) {

    let (width,_) = grid_to_window(grid.x, grid.y);

    if ship.health > 0 {
        draw_text(&ship.health.to_string(), width - 50.0, 20.0, 35.0, BLACK);
    } else {
        draw_text("Dead", width - 70.0, 20.0, 35.0, BLACK);
    }
    
    draw_text(&ship.ammo.to_string(), width - 50.0, 60.0, 35.0, BLACK);

}

fn draw_bullet(grid: &mut Grid, ship: &mut Ship, bullets: &mut Vec<Bullet>) {
    if is_key_down(KeyCode::Space) {
        ship.shoot();
        let new_bullet = Bullet::new (
            ship.x as f32 + CELL_SIZE / 2.0,
            ship.y as f32,
            (CELL_SIZE + PADDING) + 2.0,
        );
        bullets.push(new_bullet);
    }

    for bullet in bullets.iter_mut() {
        bullet.bullet_fired();
        draw_circle(bullet.x, bullet.y, PADDING, BLACK);
        println!("bullet.y: {}, bullet.velocity: {}", bullet.y, bullet.velocity);
        if current_state_of_object(grid, bullet.x as usize, bullet.y as usize) == 1 {
            change_state(grid, bullet.x as usize, bullet.y as usize, 0);
        }
    }

    bullets.retain(|bullet| bullet.y >= 0.0 && !bullet.collided);
}

fn restore_health(grid: &mut Grid, ship: &mut Ship) {
    if current_state_of_object(grid, ship.x, ship.y) == 3 {
        ship.health += 10; 
        if ship.health > 90 {
            ship.health = 100;
        }
    }

}
fn handle_input(width: usize, grid: &mut Grid, ship: &mut Ship) {
    ship.left_move(CELL_SIZE as usize, PADDING as usize);
    ship.right_move(width, CELL_SIZE as usize, PADDING as usize);
}
#[macroquad::main("Perch")]
async fn main() {

    let mut game_state = GameState {
        screen_width: screen_width(),
        screen_height: screen_height(),
    };

    let mut grid = Grid::new(60, 30);
    let (width, height) = grid_to_window(grid.x, grid.y);
    let mut ship = Ship::new(((width / 2.0 - CELL_SIZE) - PADDING + 1.0) as usize, ((height - CELL_SIZE) - PADDING) as usize);
    let mut bullets = Vec::new();

    grid.display_grid();
    if current_state_of_object(&mut grid, ship.x, ship.y) == 1 {
        change_state(&mut grid, ship.x, ship.y, 0);
    }

    let mut last_frame_time = Instant::now();

    loop {
        let delta_time = last_frame_time.elapsed();
        last_frame_time = Instant::now();

        if screen_width() != game_state.screen_width || screen_height() != game_state.screen_height {
            game_state.screen_width = screen_width();
            game_state.screen_height = screen_height();
        }

        if is_mouse_button_down(MouseButton::Left) {
            // Adjust the screen size based on mouse position
            let mouse_pos = mouse_position();
            game_state.screen_width = mouse_pos.0;
            game_state.screen_height = mouse_pos.1;
        }

        clear_background(BEIGE);
        request_new_screen_size(width, height);

        draw_grid(&grid);
        
        draw_ship(&ship); 

        grid.regenerate_top_row();

        handle_input(width as usize, &mut grid, &mut ship);

        if current_state_of_object(&mut grid, ship.x, ship.y) == 1 {
            ship.health -= 50;
            change_state(&mut grid, ship.x, ship.y, 0);
        }

        draw_panel(&mut grid, &mut ship);
        restore_health(&mut grid, &mut ship);
        draw_bullet(&mut grid, &mut ship, &mut bullets);

        let elapsed_frame_time = last_frame_time.elapsed();
        if elapsed_frame_time < TARGET_FRAME_TIME {std::thread::sleep(TARGET_FRAME_TIME - elapsed_frame_time)};

        next_frame().await;
    }
}