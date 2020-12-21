use ggez::nalgebra as na;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{event, Context, ContextBuilder, GameResult};

const W_WIDTH: f32 = 800.0;
const W_HEIGHT: f32 = 600.0;

const BOARD_WIDTH: f32 = 5.0;
const BOARD_HEIGHT: f32 = 50.0;

const G_VEL: f32 = 10.0;

const BALL_RADIUS: f32 = 5.0;
const BG_COLOR: [f32; 4] = [0.1; 4];

const LEFT_OFFSET: f32 = BALL_RADIUS * 2.0;
const RIGHT_OFFSET: f32 = W_WIDTH - BOARD_WIDTH - LEFT_OFFSET;

const BALL_VEL: f32 = G_VEL * 0.25;

#[derive(Clone)]
enum Player {
    Left,
    Right,
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Left => Player::Right,
            Player::Right => Player::Left,
        }
    }
}

#[derive(Clone)]
struct Board {
    pos: na::Point1<f32>,
    player: Player,
}

impl Board {
    fn new(player: Player) -> Self {
        Self {
            pos: na::Point1::new(W_HEIGHT * 0.5 - BOARD_HEIGHT * 0.5),
            player,
        }
    }

    #[allow(illegal_floating_point_literal_pattern)]
    fn handle(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up | KeyCode::W | KeyCode::Z => {
                if self.pos.x > 0.0 {
                    self.pos.x -= G_VEL;
                }
            }
            KeyCode::Down | KeyCode::S => {
                if self.pos.x < W_HEIGHT - BOARD_HEIGHT {
                    self.pos.x += G_VEL;
                }
            }
            _ => (),
        }
    }

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
        let offset = match p.player {
            Player::Left => LEFT_OFFSET,
            Player::Right => RIGHT_OFFSET,
        };
        Self {
            x: offset,
            y: p.pos.x,
            w: BOARD_WIDTH,
            h: BOARD_HEIGHT,
        }
    }
}

#[derive(Clone)]
enum Crossed {
    Wall(na::Vector2<f32>),
    Side(Player),
    Nothing,
}

#[derive(Clone)]
struct Ball {
    pos: na::Point2<f32>,
    dir: na::Vector2<f32>,
    crossed: Crossed,
}

fn cross(pos: na::Point2<f32>, dir: na::Vector2<f32>) -> Crossed {
    if (dir.x == BALL_VEL && pos.x == W_WIDTH - BALL_RADIUS) {
        Crossed::Side(Player::Left)
    } else if (dir.x == -BALL_VEL && pos.x == BALL_RADIUS) {
        Crossed::Side(Player::Right)
    } else if (dir.y == BALL_VEL && pos.y == W_HEIGHT - BALL_RADIUS)
        || (dir.y == -BALL_VEL && pos.y == BALL_RADIUS)
    {
        Crossed::Wall([dir.x, -dir.y].into())
    } else {
        Crossed::Nothing
    }
}

impl Ball {
    fn new() -> Self {
        let middle = [W_WIDTH * 0.5 - BALL_RADIUS, W_HEIGHT * 0.5 - BALL_RADIUS];
        Self {
            pos: middle.into(),
            dir: [BALL_VEL, BALL_VEL].into(),
            crossed: Crossed::Nothing,
        }
    }

    fn update(&mut self) {
        self.crossed = cross(self.pos, self.dir);
        if let Crossed::Wall(next_dir) = self.crossed {
            self.dir = next_dir;
        }

        self.pos += self.dir;
    }

    fn crossed(&self) -> &Crossed {
        &self.crossed
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let c = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos,
            BALL_RADIUS,
            1.0,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &c, graphics::DrawParam::default())
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
            p_1: Board::new(Player::Left),
            p_2: Board::new(Player::Right),
            ball: Ball::new(),
            game_over: false,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.game_over {
            self.ball.update();
            if let Crossed::Side(player) = self.ball.crossed() {
                println!(
                    "{} player won",
                    match player {
                        Player::Left => "left",
                        Player::Right => "right",
                    }
                );
                self.game_over = true;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BG_COLOR.into());

        self.p_1.draw(ctx)?;
        self.p_2.draw(ctx)?;

        self.ball.draw(ctx)?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if let KeyCode::W | KeyCode::Z | KeyCode::S = keycode {
            self.p_1.handle(keycode);
        } else if let KeyCode::Up | KeyCode::Down = keycode {
            self.p_2.handle(keycode);
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
