extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod models;

use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use opengl_graphics::{OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;

use crate::models::app::App;

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
