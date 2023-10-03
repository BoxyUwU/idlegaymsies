use std::{env, path};

use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{
    self, Color, DrawParam, Drawable, FillOptions, Image, ImageFormat, Mesh, MeshBuilder, Rect,
    ScreenImage,
};
use ggez::mint;
use ggez::{glam, Context, ContextBuilder, GameResult};

// These are in floating point pixels. idk why, that's how ggez does it, so this means minimal conversion needed.
const REFERENCE_SCREEN_WIDTH: f32 = 1920.0;
const REFERENCE_SCREEN_HEIGHT: f32 = 1080.0;

const DEFAULT_SCREEN_WIDTH: f32 = 1280.0;
const DEFAULT_SCREEN_HEIGHT: f32 = 720.0;

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
            width: DEFAULT_SCREEN_WIDTH,
            height: DEFAULT_SCREEN_HEIGHT,
            // maximized: true,
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

        // let ui_box = UiBox::new(
        //     screen_width,
        //     screen_height,
        //     self.assets.menu_bg.clone(), // TODO: is this clone wise?
        //     AnchorSettings::new().stretch_full(),
        // );

        // let (image_width, image_height) =
        //     (self.assets.menu_bg.width(), self.assets.menu_bg.height());
        // let scalar = Vec2::new(
        //     screen_width / image_width as f32,
        //     screen_height / image_height as f32,
        // );

        // let image1 = graphics::Image::from_path(ctx, "/kenney_ui/dotBlue.png")?;
        // let dst = glam::Vec2::new(200.0, 200.0);
        // canvas.draw(&image1, graphics::DrawParam::new().dest(dst));
        // canvas.draw(
        //     &self.assets.menu_bg,
        //     graphics::DrawParam::new().scale(scalar),
        // );

        // canvas.draw(&ui_box, DrawParam::new());

        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let anchors = get_set_of_all_anchor_points(AnchorPoint::TopLeft);
        let anchors2 = get_set_of_all_anchor_points(AnchorPoint::TopRight);
        let mut button = Button::new(0.0, 0.0, &self.assets.menu_bg, anchors);
        let mut button2 = Button::new(300.0, 300.0, &self.assets.menu_bg, anchors2);
        button.update_image_scale(
            ctx,
            screen_width,
            screen_height,
            REFERENCE_SCREEN_WIDTH,
            REFERENCE_SCREEN_HEIGHT,
        );
        button2.update_image_scale(
            ctx,
            screen_width,
            screen_height,
            REFERENCE_SCREEN_WIDTH,
            REFERENCE_SCREEN_HEIGHT,
        );

        canvas.draw(&button.image, DrawParam::new().scale(button.scalar));
        canvas.draw(&button2.image, DrawParam::new().scale(button2.scalar));

        button.check_if_clicked(ctx);
        button2.check_if_clicked(ctx);

        // let test_image = self.assets.menu_bg.clone();
        // canvas.draw(&test_image, DrawParam::new());

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

// enum Corner {
//     TopLeft,
//     TopRight,
//     BottomLeft,
//     BottomRight,
// }

#[derive(Copy, Clone)]
struct AnchorSettings {
    top_left: AnchorPoint,
    top_right: AnchorPoint,
    bottom_left: AnchorPoint,
    bottom_right: AnchorPoint,
}

impl AnchorSettings {
    fn new() -> AnchorSettings {
        AnchorSettings {
            top_left: AnchorPoint::None,
            top_right: AnchorPoint::None,
            bottom_left: AnchorPoint::None,
            bottom_right: AnchorPoint::None,
        }
    }

    fn no_anchors() -> AnchorSettings {
        AnchorSettings {
            top_left: AnchorPoint::None,
            top_right: AnchorPoint::None,
            bottom_left: AnchorPoint::None,
            bottom_right: AnchorPoint::None,
        }
    }

    // fn set_top_left(&self, anchor: AnchorPoint) -> AnchorSettings {
    //     let top_left = anchor;
    //     let top_right = self.top_right;
    //     let bottom_left = self.bottom_left;
    //     let bottom_right = self.bottom_right;
    //     AnchorSettings {
    //         top_left,
    //         top_right,
    //         bottom_left,
    //         bottom_right,
    //     }
    // }

    // fn set_top_right(&self, anchor: AnchorPoint) -> AnchorSettings {
    //     AnchorSettings {
    //         top_left: self.top_left,
    //         top_right: anchor,
    //         bottom_left: self.bottom_left,
    //         bottom_right: self.bottom_right,
    //     }
    // }

    fn set_all(&self, anchor: AnchorPoint) -> AnchorSettings {
        AnchorSettings {
            top_left: anchor,
            top_right: anchor,
            bottom_left: anchor,
            bottom_right: anchor,
        }
    }

    fn stretch_top(&self) -> AnchorSettings {
        AnchorSettings {
            top_left: AnchorPoint::TopLeft,
            top_right: AnchorPoint::TopRight,
            bottom_left: self.bottom_left,
            bottom_right: self.bottom_right,
        }
    }

    fn stretch_bottom(&self) -> AnchorSettings {
        AnchorSettings {
            top_left: self.top_left,
            top_right: self.top_right,
            bottom_left: AnchorPoint::BottomLeft,
            bottom_right: AnchorPoint::BottomRight,
        }
    }

    fn stretch_left(&self) -> AnchorSettings {
        AnchorSettings {
            top_left: AnchorPoint::TopLeft,
            top_right: self.top_right,
            bottom_left: AnchorPoint::BottomLeft,
            bottom_right: self.bottom_right,
        }
    }

    fn stretch_right(&self) -> AnchorSettings {
        AnchorSettings {
            top_left: self.top_left,
            top_right: AnchorPoint::TopRight,
            bottom_left: self.bottom_left,
            bottom_right: AnchorPoint::BottomRight,
        }
    }

    fn stretch_full(&self) -> AnchorSettings {
        AnchorSettings {
            top_left: AnchorPoint::TopLeft,
            top_right: AnchorPoint::TopRight,
            bottom_left: AnchorPoint::BottomLeft,
            bottom_right: AnchorPoint::BottomRight,
        }
    }
}

#[derive(Copy, Clone)]
enum AnchorPoint {
    None,
    Custom(f32, f32),
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    CenterCenter,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl AnchorPoint {
    fn to_tuple_fraction(&self) -> (f32, f32) {
        match self {
            AnchorPoint::None => (0.0, 0.0),
            AnchorPoint::Custom(x, y) => (*x, *y),
            AnchorPoint::TopLeft => (0.0, 0.0),
            AnchorPoint::TopCenter => (0.5, 0.0),
            AnchorPoint::TopRight => (1.0, 0.0),
            AnchorPoint::CenterLeft => (0.0, 0.5),
            AnchorPoint::CenterCenter => (0.5, 0.5),
            AnchorPoint::CenterRight => (1.0, 0.5),
            AnchorPoint::BottomLeft => (0.0, 1.0),
            AnchorPoint::BottomCenter => (0.5, 1.0),
            AnchorPoint::BottomRight => (1.0, 1.0),
        }
    }

    fn to_tuple_pixels(&self, ctx: &mut Context) -> (f32, f32) {
        let (width, height) = ctx.gfx.size();
        let fraction = self.to_tuple_fraction();
        (fraction.0 * width, fraction.1 * height)
    }

    fn to_vec2_fraction(&self) -> Vec2 {
        let tuple = self.to_tuple_fraction();
        tuple_to_vec2(tuple)
    }

    fn to_vec2_pixels(&self, ctx: &mut Context) -> Vec2 {
        let tuple = self.to_tuple_pixels(ctx);
        tuple_to_vec2(tuple)
    }
}

fn tuple_to_vec2(tuple: (f32, f32)) -> Vec2 {
    Vec2::new(tuple.0, tuple.1)
}

fn vec2_to_tuple(vec2: Vec2) -> (f32, f32) {
    (vec2.x, vec2.y)
}

fn get_set_of_all_anchor_points(anchor_point: AnchorPoint) -> AnchorSettings {
    AnchorSettings {
        top_left: anchor_point,
        top_right: anchor_point,
        bottom_left: anchor_point,
        bottom_right: anchor_point,
    }
}

struct UiBox {
    width: f32,
    height: f32,
    background: Image,
    anchors: AnchorSettings,
}

trait StretchableImage {
    fn get_original_size(&self) -> Vec2;

    fn get_current_size(&self) -> Vec2;

    fn set_current_size(&mut self, ctx: &mut Context, size: Vec2);

    fn get_scalar(&self) -> Vec2;

    fn set_scalar(&mut self, scalar: Vec2);

    fn get_position(&self) -> Vec2;

    fn get_anchor_settings(&self) -> AnchorSettings;

    fn get_image(&self) -> &Image;

    fn set_image(&mut self, ctx: &mut Context, image: Image);

    fn update_image_scale(
        &mut self,
        ctx: &mut Context,
        screen_width: f32,
        screen_height: f32,
        reference_screen_width: f32,
        reference_screen_height: f32,
    ) {
        let anchor = self.get_anchor_settings().top_left;
        let (anchor_offset_width, anchor_offset_height) = anchor.to_tuple_pixels(ctx);

        let (x, y) = vec2_to_tuple(self.get_position());

        let x = x + anchor_offset_width;
        let y = y + anchor_offset_height;

        let scale_width = screen_width / reference_screen_width;
        let scale_height = screen_height / reference_screen_height;

        let (width, height) = vec2_to_tuple(self.get_original_size());

        self.set_scalar(Vec2::new(scale_width, scale_height));

        let new_image = get_image_at_new_size(ctx, self.get_image(), width, height);

        self.set_image(ctx, new_image);
    }
}

fn get_image_at_new_size(
    ctx: &mut Context,
    old_image: &Image,
    new_width: f32,
    new_height: f32,
) -> Image {
    Image::new_canvas_image(
        &ctx.gfx,
        old_image.format(),
        new_width as u32,
        new_height as u32,
        old_image.samples(),
    )
}

trait Clickable {
    fn check_if_clicked(&self, ctx: &mut Context) {
        if ctx.mouse.button_just_released(event::MouseButton::Left) {
            if is_mouse_in_rect(ctx, self.get_area()) {
                self.on_just_released(ctx);
            }
        }
        if ctx.mouse.button_just_pressed(event::MouseButton::Left) {
            if is_mouse_in_rect(ctx, self.get_area()) {
                self.on_just_pressed(ctx);
            }
        }
        if ctx.mouse.button_pressed(event::MouseButton::Left) {
            if is_mouse_in_rect(ctx, self.get_area()) {
                self.on_pressed(ctx);
            }
        }
    }

    fn get_area(&self) -> Rect;

    fn on_pressed(&self, ctx: &mut Context) {
        ()
    }

    fn on_just_pressed(&self, ctx: &mut Context) {
        ()
    }

    fn on_just_released(&self, ctx: &mut Context) {
        ()
    }
}

fn is_mouse_in_rect(ctx: &Context, area: Rect) -> bool {
    is_point_in_rect(area, ctx.mouse.position().into())
}

fn is_point_in_rect(area: Rect, point: Vec2) -> bool {
    let bounds = RectBounds::from_rect(area);

    if point.x >= bounds.left_bound
        && point.x <= bounds.right_bound
        && point.y >= bounds.top_bound
        && point.y <= bounds.bottom_bound
    {
        return true;
    }
    false
}
struct RectBounds {
    top_bound: f32,
    bottom_bound: f32,
    left_bound: f32,
    right_bound: f32,
}

impl RectBounds {
    fn new(top_bound: f32, bottom_bound: f32, left_bound: f32, right_bound: f32) -> RectBounds {
        RectBounds {
            top_bound,
            bottom_bound,
            left_bound,
            right_bound,
        }
    }

    fn from_rect(rect: Rect) -> RectBounds {
        RectBounds {
            top_bound: rect.y,
            bottom_bound: rect.y + rect.h,
            left_bound: rect.x,
            right_bound: rect.x + rect.w,
        }
    }
}

struct Button {
    x: f32,
    y: f32,
    original_x: f32,
    original_y: f32,
    original_width: f32,
    original_height: f32,
    image: Image,
    scalar: Vec2,
    anchor_settings: AnchorSettings,
}

impl Button {
    fn new(x: f32, y: f32, image: &Image, anchor_settings: AnchorSettings) -> Button {
        Button {
            x,
            y,
            original_x: x,
            original_y: y,
            original_width: image.width() as f32,
            original_height: image.height() as f32,
            image: image.clone(),
            scalar: Vec2::new(1.0, 1.0),
            anchor_settings,
        }
    }

    fn draw(&self, ctx: &mut Context) {}
}

impl Clickable for Button {
    fn get_area(&self) -> Rect {
        Rect::new(
            self.x,
            self.y,
            self.image.width() as f32 * self.scalar.x,
            self.image.height() as f32 * self.scalar.y,
        )
    }

    fn on_just_pressed(&self, ctx: &mut Context) {
        println!("Button clicked!");
        println!("Clicked at: {}", self.get_position().to_string());
    }
}

impl StretchableImage for Button {
    fn get_scalar(&self) -> Vec2 {
        self.scalar
    }

    fn get_position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    fn get_anchor_settings(&self) -> AnchorSettings {
        self.anchor_settings
    }

    fn get_image(&self) -> &Image {
        &self.image
    }

    fn get_original_size(&self) -> Vec2 {
        Vec2::new(self.original_width, self.original_height)
    }

    fn get_current_size(&self) -> Vec2 {
        Vec2::new(self.image.width() as f32, self.image.height() as f32)
    }

    fn set_current_size(&mut self, ctx: &mut Context, size: Vec2) {
        self.image = get_image_at_new_size(ctx, &self.image, size.x, size.y);
    }

    fn set_scalar(&mut self, scalar: Vec2) {
        self.scalar = scalar;
    }

    fn set_image(&mut self, ctx: &mut Context, image: Image) {
        self.image = image;
    }
}
