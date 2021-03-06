extern crate ggez;
extern crate rand;
extern crate rayon;

use std::io::Write;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::time::Duration;
use rayon::prelude::*;

mod drop;
use drop::Updatable;
use drop::Drawable;


const DROP_COUNT: usize = 1000;
pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 600.0;

// Scene handles the events of ggez and contains all of the drops. It's the main state
struct Scene {
    drops: Vec<drop::Drop>,
    frames: usize
}

impl Scene {
    fn new(_ctx: &mut Context) -> GameResult<Scene> {
        // Initialize a vec of DROP_COUNT instances of drop::Drop::default()
        let mut drops: Vec<drop::Drop> = (0..DROP_COUNT)
            .map(|_| drop::Drop::default())
            .collect::<Vec<drop::Drop>>();

        // We want values with a lower z-value first, because they're further away
        // This means that drops with a low z-value are located behind drops with a higher z-value
        drops.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        Ok(Scene {
            drops: drops,
            frames: 0
        })
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        // Update all the drops (position etc.) in parallel for better performance.
        self.drops.par_iter_mut().for_each(|drop| drop.update());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Draw background
        let bgrect = graphics::Rect::new(0.0, 0.0, WIDTH*2.0, HEIGHT*2.0); // Why x2? I don't know, it works
        graphics::set_color(ctx, graphics::Color::new(0.902, 0.902, 0.9804, 1.0))?; // Lavender
        graphics::rectangle(ctx, graphics::DrawMode::Fill, bgrect)?;

        //TODO: Pretty sure this part is the bottleneck. Might have to profile
        for drop in self.drops.iter() {
            drop.draw(ctx)?;
        }

        graphics::present(ctx);

        // Print the FPS once every 100 frames
        self.frames += 1;
        if self.frames % 100 == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}

fn main() {
    //TODO: variable window size
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Purple Rain", "ggez", c).unwrap();
    let scene = &mut Scene::new(ctx).unwrap();

    match event::run(ctx, scene) {
        Ok(()) => (),
        Err(e) => {
            writeln!(&mut std::io::stderr(), "error: {}", e).expect("couldn't write error to stderr");
            std::process::exit(1);
        }
    }
}
