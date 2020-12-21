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

const COORD_BALL_VEL: f32 = G_VEL * 0.25;
const BALL_VEL: [f32; 2] = [COORD_BALL_VEL, COORD_BALL_VEL];

type Pos1 = na::Point1<f32>;
type Pos2 = na::Point2<f32>;
type Vec2 = na::Vector2<f32>;

#[derive(Clone)]
enum Player {
    Left,
    Right,
}

#[derive(Clone)]
struct Board {
    pos: Pos1,
    player: Player,
    score: usize,
}

impl Board {
    fn new(player: Player) -> Self {
        Self {
            pos: Pos1::new(W_HEIGHT * 0.5 - BOARD_HEIGHT * 0.5),
            player,
            score: 0,
        }
    }

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
        let num = graphics::Text::new(format!("{}", self.score));

        let score_pos = Pos2::new(
            match self.player {
                Player::Left => W_WIDTH * 0.5 - LEFT_OFFSET - num.height(ctx) as f32 * 0.5,
                Player::Right => W_WIDTH * 0.5 + LEFT_OFFSET,
            },
            0.0,
        );
        graphics::draw(ctx, &num, graphics::DrawParam::default().dest(score_pos))?;

        let b = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.clone().into(),
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &b, graphics::DrawParam::default())
    }

    fn win(&mut self) {
        self.score += 1;
    }

    fn reset_pos(&mut self) {
        self.pos = Pos1::new(W_HEIGHT * 0.5 - BOARD_HEIGHT * 0.5);
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
    Bounce(Vec2),
    Side(Player),
    Nothing,
}

#[derive(Clone)]
struct Ball {
    pos: Pos2,
    dir: Vec2,
    crossed: Crossed,
}

fn cross(pos: Pos2, dir: Vec2, left_pos: Pos1, right_pos: Pos1) -> Crossed {
    let left_side = dir.x == -BALL_VEL[0] && pos.x == BALL_RADIUS;
    let right_side = dir.x == BALL_VEL[0] && pos.x == W_WIDTH - BALL_RADIUS;
    let wall = dir.y == BALL_VEL[1] && pos.y == W_HEIGHT - BALL_RADIUS
        || dir.y == -BALL_VEL[1] && pos.y == BALL_RADIUS;

    let left_player = dir.x == -BALL_VEL[0]
        && pos.x == LEFT_OFFSET + BOARD_WIDTH
        && pos.y >= left_pos.x
        && pos.y <= left_pos.x + BOARD_HEIGHT;
    let right_player = dir.x == BALL_VEL[0]
        && pos.x == RIGHT_OFFSET
        && pos.y >= right_pos.x
        && pos.y <= right_pos.x + BOARD_HEIGHT;

    let player = left_player || right_player;

    if left_side {
        Crossed::Side(Player::Right)
    } else if right_side {
        Crossed::Side(Player::Left)
    } else if wall {
        Crossed::Bounce([dir.x, -dir.y].into())
    } else if player {
        Crossed::Bounce([-dir.x, dir.y].into())
    } else {
        Crossed::Nothing
    }
}

impl Ball {
    fn new() -> Self {
        let middle = [W_WIDTH * 0.5 - BALL_RADIUS, W_HEIGHT * 0.5 - BALL_RADIUS];
        Self {
            pos: middle.into(),
            dir: BALL_VEL.into(),
            crossed: Crossed::Nothing,
        }
    }

    fn update(&mut self, left_pos: Pos1, right_pos: Pos1) {
        self.crossed = cross(self.pos, self.dir, left_pos, right_pos);
        if let Crossed::Bounce(next_dir) = self.crossed {
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
            0.1,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &c, graphics::DrawParam::default())
    }

    fn reset(&mut self) {
        let middle = [W_WIDTH * 0.5 - BALL_RADIUS, W_HEIGHT * 0.5 - BALL_RADIUS];
        self.pos = middle.into();
    }
}

struct MainState {
    left: Board,
    right: Board,
    ball: Ball,
    game_over: bool,
}

impl MainState {
    fn new() -> Self {
        Self {
            left: Board::new(Player::Left),
            right: Board::new(Player::Right),
            ball: Ball::new(),
            game_over: false,
        }
    }

    fn reset(&mut self) {
        self.left.reset_pos();
        self.right.reset_pos();
        self.ball.reset();
        self.game_over = false;
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.game_over {
            self.ball.update(self.left.pos, self.right.pos);
            if let Crossed::Side(winner) = self.ball.crossed() {
                match winner {
                    Player::Right => self.right.win(),
                    Player::Left => self.left.win(),
                }
                self.game_over = true;
            }
        } else {
            self.reset();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BG_COLOR.into());

        self.left.draw(ctx)?;
        self.right.draw(ctx)?;

        self.ball.draw(ctx)?;

        let mid_line = graphics::Mesh::new_line(
            ctx,
            &[
                Pos2::new(W_WIDTH * 0.5, 0.0),
                Pos2::new(W_WIDTH * 0.5, W_HEIGHT),
            ],
            2.0,
            [0.4; 4].into(),
        )?;

        graphics::draw(ctx, &mid_line, graphics::DrawParam::default())?;

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
            self.left.handle(keycode);
        } else if let KeyCode::Up | KeyCode::Down = keycode {
            self.right.handle(keycode);
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
