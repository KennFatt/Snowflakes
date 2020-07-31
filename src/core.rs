use piston::input::{
    self,
    Button,
    ButtonArgs,
    RenderArgs,
    UpdateArgs,
};
use piston::window::WindowSettings;
use opengl_graphics::GlGraphics;
use graphics::Context;

use super::entity::Snowflakes;


pub struct Core {
    window_settings: WindowSettings,
    snowflakes: Snowflakes,
}


impl Core {
    pub fn new(window_settings: WindowSettings) -> Self {
        Self {
            window_settings,
            snowflakes: Snowflakes::new(100)
        }
    }

    pub fn setup(mut self) -> Self {
        /* Randomize the snowflakes */
        self.snowflakes.randomize_vectors(
            self.window_settings.get_size().width,
            self.window_settings.get_size().height
        );

        self
    }

    pub fn keypress(&mut self, args: &ButtonArgs) {
        if args.state != input::ButtonState::Press { return () }

        match args.button {
            Button::Keyboard(key) => {
                match key {
                    input::Key::R => {
                        self.snowflakes.randomize_vectors(
                            self.window_settings.get_size().width,
                            self.window_settings.get_size().height
                        );
                    }

                    _ => ()
                }
            }

            _ => ()
        }
    }

    pub fn render(&self, args: &RenderArgs, c: Context, g: &mut GlGraphics) {
        // Render the background canvas
        graphics::clear(graphics::color::hex("373472"), g);

        // Render the snowflakes
        for i in 0..self.snowflakes.get_capacity() {
            if let Some(s) = self.snowflakes.get_snowflake(i) {
                let rect: [f64; 4] = s.into();
                graphics::ellipse(
                    graphics::color::hex("ffffff"),
                    rect, c.transform, g
                );
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for i in 0..self.snowflakes.get_capacity() {
            if let Some(s) = self.snowflakes.get_snowflake(i) {
                if s.y > self.window_settings.get_size().height + s.radius {
                    self.snowflakes.update_pair(i, (s.x, 0. - s.radius)).ok();
                    continue;
                }

                self.snowflakes.update_pair(i, (s.x, s.y + s.velocity)).ok();
            }
        }
    }
}