use piston::input::{UpdateArgs};
use opengl_graphics::{GlGraphics};

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
    pub is_accelerating_x: bool,
    pub is_accelerating_y: bool,
    pub rotation: f64,
}

impl Player {

    pub fn render(&self, ctxt: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*; 
        let radius: f64 = 10.0;
        let circle = rectangle::Rectangle::new_round(color::PURPLE, radius);
        let transform = ctxt
            .transform
            .trans(self.x, self.y)
            .rot_rad(self.rotation)
            .trans(-radius, -radius);

        circle.draw([0.0, 0.0, radius*2.0, radius*2.0], &ctxt.draw_state, transform, gl);
        
        let rectangle = rectangle::Rectangle::new(color::WHITE);
        let transform = ctxt
            .transform
            .trans(self.x, self.y)
            .rot_rad(self.rotation)
            .trans(0.0, -radius/10.0);

        rectangle.draw([0.0, 0.0, radius, radius/5.0], &ctxt.draw_state, transform, gl);
    }

    pub fn update(&mut self, cursor_pos: [f64; 2], args: &UpdateArgs) {
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

        self.update_rotation(cursor_pos);
    }

    pub fn update_rotation(&mut self, cursor_pos: [f64; 2]) {
        let delta_x = cursor_pos[0] - self.x;
        let delta_y = cursor_pos[1] - self.y;
        let tan = delta_y / delta_x;
        if delta_x >= 0.0 {
            self.rotation = tan.atan();
        } else {
            self.rotation = tan.atan() + std::f64::consts::PI;
        }
        
        
    }

    pub fn calculate_velocity(&mut self) {
        self.velocity = (self.velocity_x.powi(2) + self.velocity_y.powi(2)).sqrt()
    }

    pub fn reduce_velocity(&mut self) {
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

    pub fn accelerate_left(&mut self) {        
        self.acc_x = -self.acc_value; 
        self.is_accelerating_x = true;
    }

    pub fn accelerate_up(&mut self) {
        self.acc_y = -self.acc_value;
        self.is_accelerating_y = true;
    }

    pub fn accelerate_right(&mut self) {
        self.acc_x = self.acc_value;
        self.is_accelerating_x = true;
    }

    pub fn accelerate_down(&mut self) {
        self.acc_y = self.acc_value;
        self.is_accelerating_y = true;
    }

    pub fn break_x(&mut self) {
        self.is_accelerating_x = false;
        self.acc_x = 0.0;
    }

    pub fn break_y(&mut self) {
        self.is_accelerating_y = false;
        self.acc_y = 0.0;
    }

    pub fn stop_move_x(&mut self) {
        if self.velocity_x < -self.acc_value / 100.0 {
            self.acc_x = self.acc_value;
        } else if self.velocity_x > self.acc_value / 100.0 {
            self.acc_x = -self.acc_value;
        } else {
            self.velocity_x = 0.0;
            self.acc_x = 0.0;
        }
    }

    pub fn stop_move_y(&mut self) {
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