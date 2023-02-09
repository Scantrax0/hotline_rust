use opengl_graphics::{GlGraphics};

pub enum EnemyClass {
    Circle,
    Square,
}

#[derive(PartialEq)]
pub enum EnemyState {
    Alive,
    Dead,
}

pub struct Enemy {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub class: EnemyClass,
    pub state: EnemyState,
}

impl Enemy {
    pub fn render(&self, ctxt: graphics::Context, gl: &mut GlGraphics) {
        if self.state == EnemyState::Alive {
            use graphics::*;          
            let square = rectangle::square(0.0, 0.0, self.size);
            let transform = ctxt
                .transform
                .trans(self.x, self.y)
                .trans(-self.size/2.0, -self.size/2.0);

            rectangle(color::RED, square, transform, gl);
        }
    }

    pub fn update() {
        ();
    }
}