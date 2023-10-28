use ggez::{
    conf::*,
    event,
    glam::*,
    graphics::{self, DrawMode, Mesh, DrawParam},
    input::keyboard::*,
    Context, GameResult,
};

include!(concat!(env!("OUT_DIR"), "/sin_cos.rs"));

const RES: usize = 1;
const SCREEN_WIDTH: usize = 160 * RES;
const SCREEN_HEIGHT: usize = 120 * RES;
const HALF_SCREEN_WIDTH: usize = SCREEN_WIDTH / 2;
const HALF_SCREEN_HEIGHT: usize = SCREEN_HEIGHT / 2;
const PIXEL_SCALE: usize = 8 / RES;
const WINDOW_WIDTH: f32 = (SCREEN_WIDTH * PIXEL_SCALE) as f32;
const WINDOW_HEIGHT: f32 = (SCREEN_HEIGHT * PIXEL_SCALE) as f32;

struct MainState {
    player_x: i64,
    player_y: i64,
    player_z: i64,
    player_h_angle: i64, 
    player_v_angle: i64
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { player_x: 70, player_y: -110, player_z: 20, player_h_angle: 0, player_v_angle: 0 })
    }
}

fn draw_pixel(x: f64, y: f64, c: usize, mb: &mut graphics::MeshBuilder) {
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
        (SCREEN_HEIGHT as f64 - 1.0 - y) as i32 * PIXEL_SCALE as i32,
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
                    self.player_z -= 4;
                }
                if ctx.keyboard.is_key_pressed(KeyCode::A) {
                    self.player_v_angle -= 1;
                }
                if ctx.keyboard.is_key_pressed(KeyCode::D) {
                    self.player_v_angle += 1;
                }
                if ctx.keyboard.is_key_pressed(KeyCode::S) {
                    self.player_z += 4;
                }
            } else {
                if ctx.keyboard.is_key_pressed(KeyCode::W) {
                    self.player_x += (SIN[self.player_h_angle as usize] * 10.0) as i64;
                    self.player_y += (COS[self.player_h_angle as usize] * 10.0) as i64;
                }
                if ctx.keyboard.is_key_pressed(KeyCode::A) {
                    self.player_h_angle -= 4;
                    if self.player_h_angle < 0 {
                        self.player_h_angle += 360;
                    }
                }
                if ctx.keyboard.is_key_pressed(KeyCode::D) {
                    self.player_h_angle += 4;
                    if self.player_h_angle >= 360 {
                        self.player_h_angle -= 360;
                    }
                }
                if ctx.keyboard.is_key_pressed(KeyCode::S) {
                    self.player_x -= (SIN[self.player_h_angle as usize] * 10.0) as i64;
                    self.player_y -= (COS[self.player_h_angle as usize] * 10.0) as i64;
                }
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Comma) {
                self.player_x -= (COS[self.player_h_angle as usize] * 10.0) as i64;
                self.player_y += (SIN[self.player_h_angle as usize] * 10.0) as i64;
            }
            if ctx.keyboard.is_key_pressed(KeyCode::Period) {
                self.player_x += (COS[self.player_h_angle as usize] * 10.0) as i64;
                self.player_y -= (SIN[self.player_h_angle as usize] * 10.0) as i64;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let mut mb = &mut graphics::MeshBuilder::new();

        let x1 = 40 - self.player_x;
        let y1 = 10 - self.player_y;

        let x2 = 40 - self.player_x;
        let y2 = 290 - self.player_y;

        let wx1 = x1 as f64 * COS[self.player_h_angle as usize] - 
            y1 as f64 * SIN[self.player_h_angle as usize];
        let wx2 = x2 as f64 * COS[self.player_h_angle as usize] - 
            y2 as f64 * SIN[self.player_h_angle as usize];

        let wy1 = y1 as f64 * COS[self.player_h_angle as usize] +
            x1 as f64 * SIN[self.player_h_angle as usize];
        let wy2 = y2 as f64 * COS[self.player_h_angle as usize] +
            x2 as f64 * SIN[self.player_h_angle as usize];

        let wz1 = 0.0 - self.player_z as f64 + self.player_v_angle as f64 * wy1 / 32.0;
        let wz2 = 0.0 - self.player_z as f64 + self.player_v_angle as f64 * wy1 / 32.0;

        let sx1 = (wx1 * 200.0 / wy1) + HALF_SCREEN_WIDTH as f64;
        let sy1 = (wz1 * 200.0 / wy1) + HALF_SCREEN_HEIGHT as f64;
        let sx2 = (wx2 * 200.0 / wy2) + HALF_SCREEN_WIDTH as f64;
        let sy2 = (wz2 * 200.0 / wy2) + HALF_SCREEN_HEIGHT as f64;
        
        if sx1 > 0.0 && sx1 < SCREEN_WIDTH as f64 && sy1 > 0.0 && sy1 < SCREEN_HEIGHT as f64 {
            draw_pixel(sx1, sy1, 0, &mut mb);
        }
        if sx2 > 0.0 && sx2 < SCREEN_WIDTH as f64 && sy2 > 0.0 && sy2 < SCREEN_HEIGHT as f64 {
            draw_pixel(sx2, sy2, 0, &mut mb);
        }

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