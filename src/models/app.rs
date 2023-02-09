use piston::input::{Key, UpdateArgs, RenderArgs, Button};
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};

use crate::models::player::Player;

pub struct App {
    gl: GlGraphics,
    player: Player,
    cursor_pos: [f64; 2],
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

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];        
        
        let player = &self.player;
        
        let x = self.player.x;
        let y = self.player.y;
        let cursor_pos = self.cursor_pos;
        let rotation = self.player.rotation;

        self.gl.draw(args.viewport(), |ctxt, gl| {
            
            clear(BLACK, gl);

            
            let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
            let txt = format!("{}", rotation);
            let size = 20;
            let transform = ctxt.transform.trans(300.0, 400.0);
            Text::new_color(color::WHITE, size)
                .draw(txt.as_str(), &mut glyphs, &DrawState::default(), transform, gl);       
            let transform = ctxt.transform;
            let l = Line{
                color: WHITE,
                radius: 0.5,
                shape: graphics::line::Shape::Square,
            };
            l.draw_from_to(
                [x, y],
                cursor_pos,
                &DrawState::default(),
                transform,
                gl
            );

            player.render(ctxt, gl);
            
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.player.update(args);
    }
}