use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, Quad};
use ggez::{conf, Context, ContextBuilder, GameResult};
use physics::{PhysicsWorld, Polygon2D};

mod physics;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_setup(conf::WindowSetup {
            vsync: true,
            ..Default::default()
        })
        .window_mode(conf::WindowMode {
            resizable: true,
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
}

struct Wall {
    mesh: Mesh,
    id: usize,
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

        let wall_mesh = {
            Mesh::new_polygon(
                ctx,
                DrawMode::Fill(FillOptions::DEFAULT),
                big_square.verts.as_slice(),
                Color::WHITE,
            )
            .unwrap()
        };

        let id = physics.new_entity(Vec2::new(200., 200.), big_square.clone());
        walls.push(Wall {
            id,
            mesh: wall_mesh.clone(),
        });
        let id = physics.new_entity(Vec2::new(300., 40.), big_square.clone());
        walls.push(Wall {
            id,
            mesh: wall_mesh,
        });

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

        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        canvas.draw(
            &Quad,
            DrawParam::new()
                .color(Color::RED)
                .dest(self.physics.position(self.player.id))
                .scale(Vec2::new(32., 32.)),
        );

        for wall in &self.walls {
            let pos = self.physics.position(wall.id);
            canvas.draw(
                &wall.mesh,
                DrawParam::new()
                    .dest(pos)
                    .color(Color::new(0.7, 0.7, 0.7, 1.0)),
            );
        }

        // Draw code here...
        canvas.finish(ctx)
    }
}
