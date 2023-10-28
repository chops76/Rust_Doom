use ggez::{
    conf::*,
    event,
    glam::*,
    graphics::{self, DrawMode, Mesh, DrawParam},
    input::keyboard::*,
    Context, GameResult,
};

const RES: usize = 1;
const SCREEN_WIDTH: usize = 160 * RES;
const SCREEN_HEIGHT: usize = 120 * RES;
const HALF_SCREEN_WIDTH: usize = SCREEN_WIDTH / 2;
const HALF_SCREEN_HEIGHT: usize = SCREEN_HEIGHT / 2;
const PIXEL_SCALE: usize = 8 / RES;
const WINDOW_WIDTH: f32 = (SCREEN_WIDTH * PIXEL_SCALE) as f32;
const WINDOW_HEIGHT: f32 = (SCREEN_HEIGHT * PIXEL_SCALE) as f32;

struct MainState {
    pos_x: f32,
    tick: usize
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { pos_x: 0.0, tick: 0 })
    }
}

fn draw_pixel(x: f32, y: f32, c: usize, mb: &mut graphics::MeshBuilder) {
    let color = match c {
        0 => graphics::Color::from_rgb(255, 255, 0),
        1 => graphics::Color::from_rgb(160, 160, 0),
        2 => graphics::Color::from_rgb(0, 255, 0),
        3 => graphics::Color::from_rgb(0, 160, 0),
        4 => graphics::Color::from_rgb(0, 255, 255),
        5 => graphics::Color::from_rgb(0, 160, 160),
        6 => graphics::Color::from_rgb(160, 100, 0),
        7 => graphics::Color::from_rgb(110, 50, 0),
        _ => graphics::Color::from_rgb(0, 60, 130)
    };
    
    let rect = graphics::Rect::new_i32(
        x as i32 * PIXEL_SCALE as i32,
        (SCREEN_HEIGHT as f32 - 1.0 - y) as i32 * PIXEL_SCALE as i32,
        PIXEL_SCALE as i32,
        PIXEL_SCALE as i32,
    );

    _ = mb.rectangle(DrawMode::fill(), rect, color);
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS:u32 = 20;
        while ctx.time.check_update_time(DESIRED_FPS) {
            if ctx.keyboard.is_key_pressed(KeyCode::M) {
                if ctx.keyboard.is_key_pressed(KeyCode::W) {
                    println!("Look Up");
                }
                if ctx.keyboard.is_key_pressed(KeyCode::A) {
                    println!("Look Left");
                }
                if ctx.keyboard.is_key_pressed(KeyCode::D) {
                    println!("Look Right");
                }
                if ctx.keyboard.is_key_pressed(KeyCode::S) {
                    println!("Look Down");
                }
            } else {
                if ctx.keyboard.is_key_pressed(KeyCode::W) {
                    println!("Move Up");
                }
                if ctx.keyboard.is_key_pressed(KeyCode::A) {
                    println!("Move Left");
                }
                if ctx.keyboard.is_key_pressed(KeyCode::D) {
                    println!("Move Right");
                }
                if ctx.keyboard.is_key_pressed(KeyCode::S) {
                    println!("Move Down");
                }
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Comma) {
                println!("Strafe Left");
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Period) {
                println!("Strafe Right");
            }
            

            self.pos_x = self.pos_x % SCREEN_WIDTH as f32 + 1.0;
            self.tick = (self.tick + 1) % 21;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let mut c = 0;
        let mut mb = &mut graphics::MeshBuilder::new();
        for y in 0..HALF_SCREEN_HEIGHT {
            for x in 0..HALF_SCREEN_WIDTH {
                draw_pixel(x as f32, y as f32, c, &mut mb);
                c = (c + 1) % 9;
            }
        }
        draw_pixel(HALF_SCREEN_WIDTH as f32, (HALF_SCREEN_HEIGHT + self.tick) as f32, 0, &mut mb);
        let mesh = mb.build();
        canvas.draw(&Mesh::from_data(ctx, mesh), DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(WindowMode::default()
        .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .maximized(false)
        .fullscreen_type(FullscreenType::Windowed)
        .borderless(false)
        .min_dimensions(0.0, 0.0)
        .max_dimensions(0.0, 0.0)
        .resizable(false));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}