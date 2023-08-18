use game_of_life::{cell::Cell, pattern, world::World};
use macroquad::prelude::*;

const SQUARES: i16 = 64;

struct State {
    world: World,
    speed: f64,
    last_update: f64,
}

fn update(state: &mut State) {
    let current_time = get_time();

    if current_time - state.last_update > state.speed {
        state.last_update = current_time;
        state.world = state.world.next();
    }
}

fn draw(state: &mut State) {
    clear_background(WHITE);

    let sq_size = screen_height() / SQUARES as f32;
    let horizontal_squares = (screen_width() / sq_size).ceil() as i16;

    for i in 1..SQUARES {
        draw_line(
            0.,
            sq_size * i as f32,
            screen_width(),
            sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }

    for i in 1..horizontal_squares {
        draw_line(
            sq_size * i as f32,
            0.,
            sq_size * i as f32,
            screen_height(),
            2.,
            LIGHTGRAY,
        );
    }

    for Cell { x, y, .. } in state.world.get_alive_cells() {
        draw_rectangle(
            x as f32 * sq_size,
            y as f32 * sq_size,
            sq_size,
            sq_size,
            PINK,
        );
    }
}

fn init_state() -> State {
    State {
        world: World::parse(pattern::DEFAULT),
        speed: 0.05,
        last_update: get_time(),
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = init_state();

    while !is_key_down(KeyCode::Escape) {
        update(&mut state);
        draw(&mut state);
        next_frame().await;
    }
}
