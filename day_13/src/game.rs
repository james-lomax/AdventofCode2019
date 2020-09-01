use std::sync::mpsc::{Receiver, Sender};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run_with},
    input::{Key, ButtonState}
};

const WALL: i32 = 1;
const BLOCK: i32 = 2;
const PADDLE: i32 = 3;
const BALL: i32 = 4;

pub struct DrawGeometry {
    itx: Sender<i32>,
    orx: Receiver<(i32, i32, i32)>,
    screen: HashMap<(i32, i32), i32>,
    score: i32,
    last_in_time: Instant,
    last_input: i32,
    auto: bool,
    update_counter: i32
}

impl DrawGeometry {
    fn with_rx(itx: Sender<i32>, orx: Receiver<(i32, i32, i32)>) -> Result<Self> {
        Ok(Self {
            itx: itx,
            orx: orx,
            screen: HashMap::new(),
            score: 0,
            last_in_time: Instant::now(),
            last_input: 100,
            auto: false,
            update_counter: 0
        })
    }

    fn send_input(&mut self, j: i32) {
        let elapsed = Instant::now() - self.last_in_time;
        if j != self.last_input || elapsed > Duration::from_millis(250) {
            self.last_in_time = Instant::now();
            self.last_input = j;

            self.itx.send(j).unwrap();
        }
    }

    fn find(&mut self, block: i32) -> Vec<(i32, i32)> {
        self.screen.iter()
            .filter(|(_, b)| **b == block)
            .map(|(v, _)| *v)
            .collect()
    }
}

impl State for DrawGeometry {
    fn new() -> Result<Self> {
        Err(quicksilver::Error::ContextError("Ehajd".to_string()))
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Receive updated positions
        while let Ok((x, y, b)) = self.orx.try_recv() {
            if x < 0 {
                self.score = b;
                println!("Score = {}", self.score);
            } else if y < 0 {
                window.close();
            } else {
                self.screen.insert((x, y), b);

                if b == BALL || b == PADDLE {
                    self.update_counter += 1;
                }
            }
        }

        // Check inputs and send to the machine
        if window.keyboard()[Key::Left] == ButtonState::Held {
            self.send_input(-1);
        } else if window.keyboard()[Key::Right] == ButtonState::Held {
            self.send_input(1);
        } else if window.keyboard()[Key::Space] == ButtonState::Held {
            self.send_input(0);
        } else if window.keyboard()[Key::A] == ButtonState::Held {
            self.auto = true;
        }

        // Automatically play the program
        if self.auto && self.update_counter >= 2 {
            let ball = self.find(BALL)[0];
            let paddle = self.find(PADDLE)[0];
            
            if ball.0 > paddle.0 {
                self.itx.send(1).unwrap();
            } else if ball.0 < paddle.0 {
                self.itx.send(-1).unwrap();
            } else {
                self.itx.send(0).unwrap();
            }

            self.update_counter = 0;
        }
        
        // 20 pixel unit (block length) size
        let unit = 15;

        window.clear(Color::BLACK)?;
        
        // Draw screen
        for ((x, y), b) in self.screen.iter() {
            let col = match *b {
                WALL => Color::WHITE,
                BLOCK => Color::GREEN,
                PADDLE => Color::BLUE,
                BALL => Color::RED,
                _ => Color::BLACK
            };

            window.draw(&Rectangle::new((x * unit, y * unit), (unit, unit)), Col(col));
        }
        Ok(())
    }
}

pub fn start(itx: Sender<i32>, orx: Receiver<(i32, i32, i32)>) {
    run_with("Draw", Vector::new(800, 800), Settings::default(), move || {
        DrawGeometry::with_rx(itx, orx)
    });
}