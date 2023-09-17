use std::{env, path};

use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{glam, Context, ContextBuilder, GameResult};

use ggegui::{egui, Gui};

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
        .window_mode(WindowMode {
            maximized: true,
            resizable: true,
            ..Default::default()
        })
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MainMenuState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MainMenuState {
    // Your state here...
    gui: Gui,
}

impl MainMenuState {
    pub fn new(ctx: &mut Context) -> MainMenuState {
        // Load/create resources such as images here.
        MainMenuState {
            // ...
            gui: Gui::new(ctx),
        }
    }
}

impl EventHandler for MainMenuState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...
        let gui_ctx = self.gui.ctx();

        egui::Window::new("UI").show(&gui_ctx, |ui| {
            ui.label("a very nice gui :3");
            if ui.button("print \"hello world\"").clicked() {
                println!("hello world");
            }
            if ui.button("quit").clicked() {
                ctx.request_quit();
            }
        });
        self.gui.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        // let image1 = graphics::Image::from_path(ctx, "/kenney_ui/dotBlue.png")?;
        // // Draw an image.
        // let dst = glam::Vec2::new(200.0, 200.0);
        // canvas.draw(&image1, graphics::DrawParam::new().dest(dst));
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        canvas.finish(ctx)
    }
}
