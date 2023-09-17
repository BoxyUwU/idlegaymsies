use std::{env, path};

use ggegui::egui::epaint::Shadow;
use ggegui::egui::{Align2, Color32, ColorImage, Frame, Margin, Resize, Stroke, Vec2};
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{glam, Context, ContextBuilder, GameResult};

use ggegui::{egui, Gui};

use egui_extras::{image, RetainedImage};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // Resource
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };

    // println!("{:#?}", resource_dir);

    // let texture_handle_probably =
    //     RetainedImage::from_color_image("texture_handle", ColorImage::example());

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
    // TODO: Is this necessary? It's in the example at https://github.com/NemuiSen/ggegui/blob/master/examples/egui_demo.rs
    ctx.gfx.window().set_resizable(true);
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MainMenuState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct UiImage {
    texture: Option<egui::TextureHandle>,
}

impl UiImage {
    fn ui(&mut self, ui: &mut egui::Ui) {
        println!("here {}", 1);
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            // println!("about to load");
            ui.ctx().load_texture(
                "example_image",
                egui::ColorImage::example(),
                Default::default(),
            )
            // println!("{:#?}", &ui.ctx());
            // println!("loaded");
        });
        println!("here {}", 2);

        // Show the image:
        // ui.add(egui::Image::new(texture, texture.size_vec2()));

        // Shorter version:
        ui.image(texture, texture.size_vec2());
        println!("here {}", 3);
    }
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
        egui::Window::new("MainMenuUI")
            .title_bar(false)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .constrain(true)
            .auto_sized()
            .frame(Frame {
                inner_margin: Margin {
                    left: 0.0,
                    right: 0.0,
                    top: 0.0,
                    bottom: 0.0,
                },
                outer_margin: Margin {
                    left: 0.0,
                    right: 0.0,
                    top: 0.0,
                    bottom: 0.0,
                },
                rounding: egui::Rounding {
                    nw: 0.0,
                    ne: 0.0,
                    sw: 0.0,
                    se: 0.0,
                },
                shadow: Shadow {
                    extrusion: 0.0,
                    color: Color32::from_black_alpha(0),
                },
                fill: Color32::from_black_alpha(255),
                stroke: Stroke {
                    width: 10.0,
                    color: Color32::from_rgb(155, 0, 255),
                },
            })
            .show(&gui_ctx, |ui| {
                ui.label("Idle Gaym");
                if ui.button("Start Game").clicked() {
                    println!("The game starts! (WIP)");
                }
                if ui.button("Options").clicked() {
                    println!("Behold, options! (WIP)");
                }
                if ui.button("Quit To Desktop").clicked() {
                    ctx.request_quit();
                }

                // ui.image(egui::include_image!("ferris.svg"), Vec2::new(50.0, 50.0));

                // ui.add(
                //     egui::Image::new(
                //         "https://picsum.photos/seed/1.759706314/1024",
                //         Vec2::new(50.0, 50.0),
                //     )
                //     .rounding(egui::Rounding::same(10.0)),
                // );
                println!("here A");
                let mut x = UiImage {
                    texture: Some(ui.ctx().load_texture(
                        "example_image",
                        egui::ColorImage::example(),
                        Default::default(),
                    )),
                };
                println!("here B");
                x.ui(ui);
                println!("here C");
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
