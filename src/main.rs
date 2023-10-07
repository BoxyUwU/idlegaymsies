use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, Quad};
use ggez::{conf, Context, ContextBuilder, GameResult};
use physics::{PhysicsWorld, Polygon2D, Trigger};

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
}

struct TriggerZone {
    mesh: Mesh,
    id: usize,
    overlapping_entities: Vec<usize>,
}

impl Trigger for TriggerZone {
    fn on_trigger_enter(&self, triggering_entity: usize) {
        println!("Trigger {} entered!", self.id.to_string())
    }

    fn on_trigger_exit(&self, triggering_entity: usize) {
        println!("Trigger {} exited!", self.id.to_string())
    }

    fn try_add_overlapping_entity(&mut self, other_entity: usize) {
        if self.overlapping_entities.contains(&other_entity) {
            // Entity is already in the list, so do nothing
            return;
        }
        self.overlapping_entities.push(other_entity);
    }

    fn try_remove_overlapping_entity(&mut self, other_entity: usize) {
        if !self.overlapping_entities.contains(&other_entity) {
            // Entity is already not in the list, so do nothing
            return;
        }
        let index = self
            .overlapping_entities
            .iter()
            .position(|&e| e == other_entity)
            .unwrap();
        let _ = self.overlapping_entities.swap_remove(index);
    }

    fn get_overlapping_entity_list(&self) -> Vec<usize> {
        self.overlapping_entities.clone()
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

        let mesh_from_poly = |poly: &Polygon2D| {
            Mesh::new_polygon(
                ctx,
                DrawMode::Fill(FillOptions::DEFAULT),
                poly.verts.as_slice(),
                Color::WHITE,
            )
            .unwrap()
        };

        let wall_mesh = mesh_from_poly(&big_square);

        let id = physics.new_entity(Vec2::new(200., 200.), big_square.clone());
        walls.push(Wall {
            id,
            mesh: wall_mesh.clone(),
        });
        let id = physics.new_entity(Vec2::new(500., 150.), big_square.clone());
        walls.push(Wall {
            id,
            mesh: wall_mesh.clone(),
        });
        let id = physics.new_entity(Vec2::new(400., 400.), big_square.clone());
        walls.push(Wall {
            id,
            mesh: wall_mesh,
        });

        //

        let mut new_wall = |start, end, pos| {
            let wall_col = Polygon2D::new_line(start, end, 8.);
            let wall_mesh = mesh_from_poly(&wall_col);
            let id = physics.new_entity(pos, wall_col);
            walls.push(Wall {
                id,
                mesh: wall_mesh,
            });
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

        canvas.draw(
            &Quad,
            DrawParam::new()
                .color(Color::RED)
                .dest(self.physics.position(self.player.id) - camera_pos)
                .scale(Vec2::new(32., 32.)),
        );

        for wall in &self.walls {
            let pos = self.physics.position(wall.id);
            canvas.draw(
                &wall.mesh,
                DrawParam::new()
                    .dest(pos - camera_pos)
                    .color(Color::new(0.7, 0.7, 0.7, 1.0)),
            );
        }

        // Draw code here...
        canvas.finish(ctx)
    }
}
