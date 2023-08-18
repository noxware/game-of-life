use game_of_life::{cell::Cell, pattern, world::World};
use macroquad::prelude::*;

const SQUARES: i16 = 64;

struct State {
    world: World,
    speed: f64,
    last_update: f64,
}

fn update(state: &mut State) {
    if get_time() - state.last_update > state.speed {
        state.last_update = get_time();
        state.world = state.world.next();
    }
}

fn draw(state: &mut State) {
    clear_background(LIGHTGRAY);

    let game_size = screen_width().min(screen_height());
    let offset_x = (screen_width() - game_size) / 2. + 10.;
    let offset_y = (screen_height() - game_size) / 2. + 10.;
    let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

    draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

    for i in 1..SQUARES {
        draw_line(
            offset_x,
            offset_y + sq_size * i as f32,
            screen_width() - offset_x,
            offset_y + sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }

    for i in 1..SQUARES {
        draw_line(
            offset_x + sq_size * i as f32,
            offset_y,
            offset_x + sq_size * i as f32,
            screen_height() - offset_y,
            2.,
            LIGHTGRAY,
        );
    }

    for Cell { x, y, .. } in state.world.get_alive_cells() {
        draw_rectangle(
            offset_x + x as f32 * sq_size,
            offset_y + y as f32 * sq_size,
            sq_size,
            sq_size,
            LIME,
        );

        // draw_text(format!("GEN: {gen}").as_str(), 10., 20., 20., DARKGRAY);
    }
}

fn init_state() -> State {
    State {
        world: World::parse(pattern::default),
        speed: 0.05,
        last_update: get_time(),
    }
}

#[macroquad::main("Game of Life")]
async fn main() {
    let mut state = init_state();

    loop {
        update(&mut state);
        draw(&mut state);
        next_frame().await;
    }
}
