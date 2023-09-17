use std::{env, path};

use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawParam, Image, ImageFormat, ScreenImage};
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
        .window_mode(WindowMode {
            // Uncomment w/h and set maximized to false for consistent testing
            // See https://docs.rs/ggez/latest/ggez/conf/struct.WindowMode.html
            // width: 1280.0,
            // height: 720.0,
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

struct Assets {
    menu_bg: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> Assets {
        Assets {
            // menu_bg: ScreenImage::new(&ctx.gfx, None, 1.0, 1.0, 1),
            // menu_bg: graphics::ScreenImage::new(
            //     ctx,
            //     Image::from_color(ctx, 32, 32, Some(Color::CYAN)),
            //     1.0,
            //     1.0,
            //     1,
            // ),
            menu_bg: graphics::Image::from_path(&ctx.gfx, "/kenney_ui/glassPanel.png").unwrap(),
        }
    }
}

struct MainMenuState {
    // Your state here...
    assets: Assets,
}

impl MainMenuState {
    pub fn new(ctx: &mut Context) -> MainMenuState {
        // Load/create resources such as images here.
        MainMenuState {
            // ...
            assets: Assets::new(ctx),
        }
    }
}

impl EventHandler for MainMenuState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // let mut canvas =
        // graphics::Canvas::from_screen_image(ctx, &self.assets.menu_bg, Color::WHITE);
        // let mut canvas = graphics::Canvas::from_image(
        //     &ctx.gfx,
        //     graphics::Image::new_canvas_image(
        //         &ctx.gfx,
        //         ctx.gfx.surface_format(),
        //         ctx.gfx.frame().width(),
        //         ctx.gfx.frame().height(),
        //         1,
        //     ),
        //     Some(Color::WHITE),
        // ); // TODO: Figure out why this doesn't work

        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let (image_width, image_height) =
            (self.assets.menu_bg.width(), self.assets.menu_bg.height());
        let scalar = Vec2::new(
            screen_width / image_width as f32,
            screen_height / image_height as f32,
        );

        let image1 = graphics::Image::from_path(ctx, "/kenney_ui/dotBlue.png")?;
        let dst = glam::Vec2::new(200.0, 200.0);
        canvas.draw(&image1, graphics::DrawParam::new().dest(dst));
        canvas.draw(
            &self.assets.menu_bg,
            graphics::DrawParam::new().scale(scalar),
        );
        canvas.finish(ctx)

        // let mut canvas = graphics::Canvas::from_screen_image(
        //     &ctx.gfx,
        //     ScreenImage::new(&ctx.gfx, None, 0.5, 0.5, 1),
        //     Some(Color::WHITE),
        // );
        // let mut canvas = graphics::Canvas::from_image(
        //     &ctx.gfx,
        //     graphics::Image::new_canvas_image(&ctx.gfx, ctx.gfx.surface_format(), 200, 200, 1),
        //     Some(Color::BLACK),
        // );
        // canvas.draw(
        //     &image1,
        //     graphics::DrawParam::new().dest(Vec2::new(300.0, 300.0)),
        // );
        // canvas.finish(ctx)
    }
}
