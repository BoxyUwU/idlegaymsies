use std::{env, path};

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{glam, Context, ContextBuilder, GameResult};

fn main() {
    // Resource
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };

    println!("{:#?}", resource_dir);

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        let image1 = graphics::Image::from_path(ctx, "/kenney_ui/dotBlue.png")?;
        // Draw an image.
        let dst = glam::Vec2::new(200.0, 200.0);
        canvas.draw(&image1, graphics::DrawParam::new().dest(dst));
        canvas.finish(ctx)
    }
}
