use std::collections::HashSet;

use ggez::event;
use ggez::glam::*;
use ggez::graphics::Image;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};

use crate::cell::Cell;
use crate::world::World;

struct State {
    world: World,
}

impl State {
    fn new() -> State {
        State {
            world: World::new(HashSet::from([(4, 5), (5, 5), (6, 5)])),
        }
    }
}

impl event::EventHandler<ggez::GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 10;

        while ctx.time.check_update_time(DESIRED_FPS) {
            self.world = self.world.next();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        let alive_cell = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, 16.0, 16.0),
            Color::BLACK,
        )?;

        let dead_cell = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            Rect::new(0.0, 0.0, 16.0, 16.0),
            Color::BLACK,
        )?;

        for x in 0..100 {
            for y in 0..100 {
                let x_dest = x as f32 * 16.0;
                let y_dest = y as f32 * 16.0;

                let Cell { is_alive, .. } = self.world.get_by_xy(x, y);

                if is_alive {
                    canvas.draw(
                        &alive_cell,
                        graphics::DrawParam::new().dest(Vec2::new(x_dest, y_dest)),
                    );
                } else {
                    canvas.draw(
                        &dead_cell,
                        graphics::DrawParam::new().dest(Vec2::new(x_dest, y_dest)),
                    );
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn run() {
    let window_mode = ggez::conf::WindowMode::default().dimensions(500.0, 500.0);
    let window_setup = ggez::conf::WindowSetup::default().title("Game of Life");

    let cb = ggez::ContextBuilder::new("nxgol", "noxware")
        .window_mode(window_mode)
        .window_setup(window_setup);

    let (ctx, event_loop) = cb.build().unwrap();
    let state = State::new();

    event::run(ctx, event_loop, state)
}
