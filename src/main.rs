use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, Quad};
use ggez::{conf, Context, ContextBuilder, GameResult};
use physics::{PhysicsWorld, Polygon2D};

mod physics;

const WALL_COLOR: Color = Color::BLACK;
const TRIGGER_COLOR: Color = Color::CYAN;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_setup(conf::WindowSetup {
            vsync: true,
            ..Default::default()
        })
        .window_mode(conf::WindowMode {
            resizable: true,
            width: 800.,
            height: 800.,
            ..Default::default()
        })
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
    physics: PhysicsWorld,

    walls: Vec<Wall>,
    player: Player,

    camera_pos: Vec2,
}

struct Wall {
    mesh: Mesh,
    id: usize,
    trigger_zone: Option<Box<Polygon2D>>, // I have no idea if boxing is a good idea here, send halp
}

impl Wall {
    pub fn new(mesh: Mesh, id: usize) -> Wall {
        Wall {
            mesh,
            id,
            trigger_zone: None,
        }
    }

    pub fn set_trigger_zone(&mut self, trigger_zone: Option<Box<Polygon2D>>) -> Wall {
        self.trigger_zone = trigger_zone;
        *self
    }
}

struct Player {
    id: usize,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut physics = PhysicsWorld::new();

        let big_square = Polygon2D::new([
            Vec2::new(-32., 32.),
            Vec2::new(-32., -32.),
            Vec2::new(32., -32.),
            Vec2::new(32., 32.),
        ]);

        let mut walls = vec![];

        let mesh_from_poly = |poly: &Polygon2D, color: Color| {
            Mesh::new_polygon(
                ctx,
                DrawMode::Fill(FillOptions::DEFAULT),
                poly.verts.as_slice(),
                color,
            )
            .unwrap()
        };

        let wall_mesh = mesh_from_poly(&big_square, WALL_COLOR);

        let id = physics.new_entity(Vec2::new(200., 200.), big_square.clone());
        walls.push(Wall::new(wall_mesh.clone(), id));
        let id = physics.new_entity(Vec2::new(500., 150.), big_square.clone());
        walls.push(Wall::new(wall_mesh.clone(), id));

        let trigger_square = Polygon2D::new([
            Vec2::new(-32., 32.),
            Vec2::new(-32., -32.),
            Vec2::new(32., -32.),
            Vec2::new(32., 32.),
        ])
        .set_trigger(true);
        let trigger_mesh = mesh_from_poly(&trigger_square, TRIGGER_COLOR);

        let id = physics.new_entity(Vec2::new(400., 400.), trigger_square);
        walls.push(Wall::new(trigger_mesh, id).set_trigger_zone(Some(Box::new(trigger_square)))); // TODO: wat do here

        // Context:
        // Triggers are implemented by the `is_trigger` boolean on a Polygon2D. Walls and Polygon2Ds currently have no
        // reference to each other, so this is an attempt to add such a thing. However, `trigger_square` is moved into
        // the physics entity. I suspect simply redoing everything to allow the physics entity to make use of references
        // would end up screwing up lots of things, so I'm not doing that. The reason for adding a link between them is
        // so that a Polygon2D can, on trigger, call the linked Wall's relevant function (not currently implemented).
        //
        // Alternative: put the function on the Polygon2D at creation. This feels dumb but simple.
        //
        // Alternative: access the trigger through PhysicsWorld. This feels overly complex, possibly.

        let mut new_wall = |start, end, pos| {
            let wall_col = Polygon2D::new_line(start, end, 8.);
            let wall_mesh = mesh_from_poly(&wall_col, WALL_COLOR);
            let id = physics.new_entity(pos, wall_col);
            walls.push(Wall::new(wall_mesh, id));
        };

        let room_size = 800.;

        new_wall(Vec2::ZERO, Vec2::new(room_size, 0.0), Vec2::ZERO);
        new_wall(
            Vec2::ZERO,
            Vec2::new(room_size, 0.0),
            Vec2::new(0.0, room_size),
        );

        new_wall(Vec2::ZERO, Vec2::new(0.0, room_size), Vec2::ZERO);
        new_wall(
            Vec2::ZERO,
            Vec2::new(0.0, room_size),
            Vec2::new(room_size, 0.0),
        );

        //

        let player = physics.new_entity(
            Vec2::new(100., 100.),
            Polygon2D::new([
                Vec2::new(0., 32.),
                Vec2::new(0., 0.),
                Vec2::new(32., 0.),
                Vec2::new(32., 32.),
            ]),
        );
        MyGame {
            physics,
            walls,
            player: Player { id: player },

            camera_pos: Vec2::ZERO,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let kb = &ctx.keyboard;

        let mut movement = Vec2::ZERO;
        for (sc, offset) in [
            (0x11, Vec2::new(0.0, -1.0)),
            (0x1E, Vec2::new(-1.0, 0.0)),
            (0x1F, Vec2::new(0.0, 1.0)),
            (0x20, Vec2::new(1.0, 0.0)),
        ] {
            if kb.is_scancode_pressed(sc) {
                movement += offset;
            }
        }
        let movement = movement.normalize_or_zero() * 10.0;

        self.physics.move_entity_by(self.player.id, movement);

        self.camera_pos = self.physics.position(self.player.id);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        let window_dim = ctx.gfx.window().inner_size();
        let camera_pos =
            self.camera_pos - (Vec2::new(window_dim.width as f32, window_dim.height as f32) / 2.);

        for wall in &self.walls {
            let pos = self.physics.position(wall.id);
            canvas.draw(
                &wall.mesh,
                DrawParam::new()
                    .dest(pos - camera_pos)
                    .color(Color::new(0.7, 0.7, 0.7, 1.0)),
            );
        }

        canvas.draw(
            &Quad,
            DrawParam::new()
                .color(Color::RED)
                .dest(self.physics.position(self.player.id) - camera_pos)
                .scale(Vec2::new(32., 32.)),
        );
        // Draw code here...
        canvas.finish(ctx)
    }
}
