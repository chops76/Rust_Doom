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

fn clip_behind_player(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> (f64, f64, f64) {
    let mut d = y1 as i64 - y2 as i64;
    if d == 0 {
        d = 1;
    }
    let s = y1 / (y1 - y2);
    let x1_clipped = x1 + s * (x2 - x1);
    let mut y1_clipped = y1 + s * (y2 - y1);
    if y1_clipped == 0.0 {
        y1_clipped = 1.0;
    }
    let z1_clipped = z1 + s * (z2 - z1);
    (x1_clipped, y1_clipped, z1_clipped)
}

fn draw_wall(x1: f64, x2: f64, b1: f64, b2: f64, t1: f64, t2: f64, mb: &mut graphics::MeshBuilder) {
    let x1 = x1.max(1.0).min(SCREEN_WIDTH as f64 - 1.0);
    let x2 = x2.max(1.0).min(SCREEN_WIDTH as f64 - 1.0);
    let dyb = b2 - b1;
    let dyt = t2 - t1;
    let mut dx = x2 - x1;
    if dx < 1.0 {
        dx = 1.0;
    }
    let xs = x1;
    let mut x = x1;
    while x < x2 {
        let y1 = dyb * (x - xs + 0.5) / dx + b1;
        let y2 = dyt * (x - xs + 0.5) / dx + t1;
        let y1 = y1.max(1.0).min(SCREEN_HEIGHT as f64 - 1.0);
        let y2 = y2.max(1.0).min(SCREEN_HEIGHT as f64 - 1.0);
        let mut y = y1;
        while y < y2 {
            draw_pixel(x, y, 0, mb);
            y += 1.0;
        }
        x += 1.0;
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

fn to_world_coords(x: i64, y: i64, z: i64, h_angle: i64, v_angle: i64) -> (f64, f64, f64) {
    let wx = x as f64 * COS[h_angle as usize] - 
        y as f64 * SIN[h_angle as usize];

    let wy = y as f64 * COS[h_angle as usize] +
        x as f64 * SIN[h_angle as usize];

    let wz = z as f64 + v_angle as f64 * wy / 32.0;

    (wx, wy, wz)
}

fn to_screen_coords(wx: f64, wy: f64, wz: f64) -> (f64, f64) {
    let sx = (wx * 200.0 / wy) + HALF_SCREEN_WIDTH as f64;
    let sy = (wz * 200.0 / wy) + HALF_SCREEN_HEIGHT as f64;

    (sx, sy)
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
        let mut mb = graphics::MeshBuilder::new();

        let (mut wx1, mut wy1, mut wz1) = to_world_coords(40 - self.player_x, 10 - self.player_y, 
            0 - self.player_z, self.player_h_angle, self.player_v_angle);
        let (mut wx2, mut wy2, mut wz2) = to_world_coords(40 - self.player_x, 290 - self.player_y, 
            0 - self.player_z, self.player_h_angle, self.player_v_angle);
        let (mut wx3, mut wy3, mut wz3) = to_world_coords(40 - self.player_x, 10 - self.player_y, 
            40 - self.player_z, self.player_h_angle, self.player_v_angle);
        let (mut wx4, mut wy4, mut wz4) = to_world_coords(40 - self.player_x, 290 - self.player_y, 
            40 - self.player_z, self.player_h_angle, self.player_v_angle);

        if !(wy1 < 1.0 && wy2 < 1.0) {
            if wy1 < 1.0 {
                (wx1, wy1, wz1) = clip_behind_player(wx1, wy1, wz1, wx2, wy2, wz2);
                (wx3, wy3, wz3) = clip_behind_player(wx3, wy3, wz3, wx4, wy4, wz4);
            }

            if wy2 < 1.0 {
                (wx2, wy2, wz2) = clip_behind_player(wx2, wy2, wz2, wx1, wy1, wz1);
                (wx4, wy4, wz4) = clip_behind_player(wx4, wy4, wz4, wx3, wy3, wz3);
            }

            let (sx1, sy1) = to_screen_coords(wx1, wy1, wz1);
            let (sx2, sy2) = to_screen_coords(wx2, wy2, wz2);
            let (sx3, sy3) = to_screen_coords(wx3, wy3, wz3);
            let (sx4, sy4) = to_screen_coords(wx4, wy4, wz4);
            
            draw_wall(sx1, sx2, sy1, sy2, sy3, sy4, &mut mb);
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