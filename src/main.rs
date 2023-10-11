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
    workstations: Vec<Workstation>,
    walls: Vec<Wall>,
    player: Player,

    camera_pos: Vec2,
}

struct Wall {
    mesh: Mesh,
    id: usize,
}

impl Wall {
    pub fn new(mesh: Mesh, id: usize) -> Wall {
        Wall { mesh, id }
    }
}

enum WorkType {
    Nothing,
    GameDesign,
    Programming,
    Art,
    Music,
    SoundEffects,
    Writing,
}

struct Workstation {
    mesh: Mesh,
    id: usize,
    work_type: WorkType,
    progress: f32,
    in_use: bool,
}

impl Workstation {
    pub fn new(mesh: Mesh, id: usize, work_type: WorkType) -> Workstation {
        Workstation {
            mesh,
            id,
            work_type,
            progress: 0.,
            in_use: false,
        }
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
        let mut workstations = vec![];

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

        let programming_desk_polygon = Polygon2D::new([
            Vec2::new(-32., 32.),
            Vec2::new(-32., -32.),
            Vec2::new(32., -32.),
            Vec2::new(32., 32.),
        ])
        .set_trigger(true);
        let programming_desk_mesh = mesh_from_poly(&programming_desk_polygon, TRIGGER_COLOR);

        let id = physics.new_entity(Vec2::new(400., 400.), programming_desk_polygon);

        workstations.push(Workstation::new(
            programming_desk_mesh,
            id,
            WorkType::Programming,
        ));
        // walls.push(Wall::new(trigger_mesh, id));

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
            workstations,
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

        let triggers = self.physics.get_overlapping_triggers(self.player.id);
        for trigger in triggers {
            println!("{}", trigger);
        }

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

        for workstation in &self.workstations {
            let pos = self.physics.position(workstation.id);
            canvas.draw(
                &workstation.mesh,
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
