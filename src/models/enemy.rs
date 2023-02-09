enum EnemyClass {
    Circle,
    Square,
}

pub struct Enemy {
    x: f64;
    y: f64;
    class: EnemyClass,
}

impl Enemy {
    pub fn render(&self, ctxt: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*; 
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];            
        let square = rectangle::square(0.0, 0.0, 50.0);
        let transform = ctxt
            .transform
            .trans(self.x, self.y)
            .rot_rad(self.rotation)
            .trans(-25.0, -25.0);

        // Draw a box rotating around the middle of the screen.
        rectangle(RED, square, transform, gl);
    }
}