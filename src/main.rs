use ggez::nalgebra as na;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{event, Context, ContextBuilder, GameResult};

const W_WIDTH: f32 = 800.0;
const W_HEIGHT: f32 = 600.0;

const BOARD_WIDTH: f32 = 5.0;
const BOARD_HEIGHT: f32 = 50.0;

const BALL_RADIUS: f32 = 15.0;
const BG_COLOR: [f32; 4] = [0.1; 4];

const P_1_OFFSET: f32 = BOARD_WIDTH;
const P_2_OFFSET: f32 = W_WIDTH - 2.0 * BOARD_WIDTH;

#[derive(Clone)]
struct Board {
    pos: na::Point1<f32>,
    offset: f32,
}

impl Board {
    fn new(offset: f32) -> Self {
        Self {
            pos: na::Point1::new(W_HEIGHT * 0.5 - BOARD_HEIGHT),
            offset,
        }
    }

    fn go_up(&mut self) {}

    fn go_down(&mut self) {}

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let b = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.clone().into(),
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &b, graphics::DrawParam::default())
    }
}

impl From<Board> for graphics::Rect {
    fn from(p: Board) -> Self {
        Self {
            x: p.offset,
            y: p.pos.x,
            w: BOARD_WIDTH,
            h: BOARD_HEIGHT,
        }
    }
}

struct Ball {
    pos: na::Point2<f32>,
}

impl Ball {
    fn new() -> Self {
        Self {
            pos: na::Point2::new(W_WIDTH * 0.5 - BALL_RADIUS, W_HEIGHT * 0.5 - BALL_RADIUS),
        }
    }

    fn update(&mut self) {}

    fn crossed(&self) -> bool {
        false
    }
}

struct MainState {
    p_1: Board,
    p_2: Board,
    ball: Ball,
    game_over: bool,
}

impl MainState {
    fn new() -> Self {
        Self {
            p_1: Board::new(P_1_OFFSET),
            p_2: Board::new(P_2_OFFSET),
            ball: Ball::new(),
            game_over: false,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.game_over {
            self.ball.update();
            self.game_over = self.ball.crossed();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BG_COLOR.into());

        self.p_1.draw(ctx)?;
        self.p_2.draw(ctx)?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::W | KeyCode::Z => self.p_1.go_up(),
            KeyCode::S => self.p_1.go_down(),
            KeyCode::Up => self.p_2.go_up(),
            KeyCode::Down => self.p_2.go_down(),
            _ => (),
        }
    }
}

fn w_mode() -> WindowMode {
    WindowMode {
        width: W_WIDTH,
        height: W_HEIGHT,
        borderless: true,
        ..Default::default()
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ContextBuilder::new("pong", "ahmadb")
        .window_setup(WindowSetup::default().title("Pong!"))
        .window_mode(w_mode())
        .build()?;

    let state = &mut MainState::new();

    event::run(ctx, events_loop, state)
}
