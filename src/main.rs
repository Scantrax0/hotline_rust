extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::text::Text;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::input::*;

pub struct App {
    gl: GlGraphics,
    player: Player,
    cursor_pos: [f64; 2],
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity: f64,
    pub acc_x: f64,
    pub acc_y: f64,
    pub acc_value: f64,
    pub max_velocity: f64,
    is_accelerating_x: bool,
    is_accelerating_y: bool,
    pub rotation: f64,
}

impl Player {
    fn update(&mut self, args: &UpdateArgs) {
        if !self.is_accelerating_x {
            self.stop_move_x();
        }
        if !self.is_accelerating_y {
            self.stop_move_y();
        }

        self.velocity_x += self.acc_x * args.dt;
        self.velocity_y += self.acc_y * args.dt;

        self.calculate_velocity();

        if self.velocity > self.max_velocity {
            self.reduce_velocity();
        }

        self.x += self.velocity_x * args.dt;
        self.y += self.velocity_y * args.dt;
    }

    fn calculate_velocity(&mut self) {
        self.velocity = (self.velocity_x.powi(2) + self.velocity_y.powi(2)).sqrt()
    }

    fn reduce_velocity(&mut self) {
        if self.velocity > self.max_velocity {
            let n: f64 = 2.0;
            let delta = self.velocity - self.max_velocity;

            if self.velocity_x > self.max_velocity * 0.7 {
                self.velocity_x -= delta / (n.sqrt());
            } else if self.velocity_x < -self.max_velocity * 0.7 {
                self.velocity_x += delta / (n.sqrt());
            }

            if self.velocity_y > self.max_velocity * 0.7 {
                self.velocity_y -= delta / (n.sqrt());
            } else if self.velocity_y < -self.max_velocity * 0.7 {
                self.velocity_y += delta / (n.sqrt());
            }
        }
         
    }

    fn accelerate_left(&mut self) {        
        self.acc_x = -self.acc_value; 
        self.is_accelerating_x = true;
    }

    fn accelerate_up(&mut self) {
        self.acc_y = -self.acc_value;
        self.is_accelerating_y = true;
    }

    fn accelerate_right(&mut self) {
        self.acc_x = self.acc_value;
        self.is_accelerating_x = true;
    }

    fn accelerate_down(&mut self) {
        self.acc_y = self.acc_value;
        self.is_accelerating_y = true;
    }

    fn break_x(&mut self) {
        self.is_accelerating_x = false;
        self.acc_x = 0.0;
    }

    fn break_y(&mut self) {
        self.is_accelerating_y = false;
        self.acc_y = 0.0;
    }

    fn stop_move_x(&mut self) {
        if self.velocity_x < -self.acc_value / 100.0 {
            self.acc_x = self.acc_value;
        } else if self.velocity_x > self.acc_value / 100.0 {
            self.acc_x = -self.acc_value;
        } else {
            self.velocity_x = 0.0;
            self.acc_x = 0.0;
        }
    }

    fn stop_move_y(&mut self) {
        if self.velocity_y < -self.acc_value / 100.0 {
            self.acc_y = self.acc_value;
        } else if self.velocity_y > self.acc_value / 100.0 {
            self.acc_y = -self.acc_value;
        } else {
            self.velocity_y = 0.0;
            self.acc_y = 0.0;
        }
    }
}

impl App {
    pub fn new(opengl: OpenGL) -> App {
        let gl = GlGraphics::new(opengl);

        let player = Player {
            x: 50.0,
            y: 50.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity: 0.0,
            acc_value: 2000.0,
            max_velocity: 200.0,
            acc_x: 0.0,
            acc_y: 0.0,
            is_accelerating_x: false,
            is_accelerating_y: false,
            rotation: 0.0,
        };

        App {
            gl,  
            player,
            cursor_pos: [0.0, 0.0],
        }

    }

    pub fn cursor_move(&mut self, pos: [f64; 2]) {
        let delta_x = pos[0] - self.player.x;
        let delta_y = pos[1] - self.player.y;
        let tan = delta_y / delta_x;
        self.player.rotation = tan.atan();
        self.cursor_pos = pos;
    }

    pub fn input(&mut self, button: &Button, is_press: bool) {
        match (&button, is_press) {
            (Button::Keyboard(key), true) => {
                match key {
                    Key::W => self.player.accelerate_up(),
                    Key::S => self.player.accelerate_down(),
                    Key::A => self.player.accelerate_left(),
                    Key::D => self.player.accelerate_right(),
                    _ => (),
                }
            },
            (Button::Keyboard(key), false) => {
                match key {
                    Key::W => self.player.break_y(),
                    Key::S => self.player.break_y(),
                    Key::A => self.player.break_x(),
                    Key::D => self.player.break_x(),
                    _ => (),
                }
            }
            _ => {}
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.player.rotation;
        
        let x = self.player.x;
        let y = self.player.y;
        let cursor_pos = self.cursor_pos;
        let velocity = self.player.velocity;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Render velocity
            // let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
            // let txt = format!("Velocity: {}", velocity as i64);
            // let size = 30;
            // let transform = c.transform.trans(300.0, 400.0);
            // Text::new_color(color::WHITE, size)
            //     .draw(txt.as_str(), &mut glyphs, &DrawState::default(), transform, gl);       
            let transform = c.transform;
            let l = Line{
                color: WHITE,
                radius: 0.2,
                shape: graphics::line::Shape::Square,
            };
            l.draw_from_to(
                [x, y],
                cursor_pos,
                &DrawState::default(),
                transform,
                gl
            );

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.player.update(args);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut app = App::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(pos) = e.mouse_cursor_args() {
            app.cursor_move(pos);
        }

        if let Some(i) = e.press_args() {
            app.input(&i, true);
        }

        if let Some(i) = e.release_args() {
            app.input(&i, false);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
