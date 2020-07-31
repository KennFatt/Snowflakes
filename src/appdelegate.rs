use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow;
use piston::input::{RenderEvent, UpdateEvent, ButtonEvent};
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};

use super::core::Core;


pub struct AppDelegate {
    events: Events,
    core: Core,
    gl: GlGraphics,
    window: GlutinWindow,
}


impl AppDelegate {

    /// Setup the driver and instantiate all its fields and states.
    /// 
    /// This method is required since you can't run the driver before initializing it
    /// by using this `setup` method.
    ///
    /// ```rust
    /// let driver = AppDelegate::setup();
    /// ```
    pub fn setup() -> Self {
        /* Setting up the window */
        let opengl = OpenGL::V3_1;
        let window_settings = 
            WindowSettings::new("Snowflakes", [640, 480])
                .samples(8)
                .vsync(true)
                .resizable(false)
                .exit_on_esc(true)
                .graphics_api(opengl);
        let window: GlutinWindow = window_settings.build().unwrap();

        /* Instantiate new GL back-end */
        let gl = GlGraphics::new(opengl);

        /* Instantiate events controller */
        let mut event_settings = EventSettings::new();
        event_settings.max_fps = 60;
        let events = Events::new(event_settings);

        /* Instantiate our core state */
        let core = Core::new(window_settings).setup();

        Self { events, core, gl, window }
    }

    /// Starting the main-loop of the application.
    /// It will listen for some events in order:
    /// 1. Button
    /// 2. Render
    /// 3. Update
    /// 
    /// And after that, it will continue the event to the Core to do
    /// more advance task.
    ///
    /// ```rust
    /// let driver = AppDelegate::setup().start();
    /// ```
    ///
    pub fn start(&mut self) {
        while let Some(ev) = self.events.next(&mut self.window) {
            /* Button event */
            if let Some(args) = ev.button_args() {
                self.core.keypress(&args);
            }

            /* Render event */
            if let Some(args) = ev.render_args() {
                /* 
                    I have to call gl.draw_begin() and gl.draw_end() manually.
                    This is basically how gl.draw() works.
                */
                let c = self.gl.draw_begin(args.viewport());

                /* --- Render start --- */
                self.core.render(&args, c, &mut self.gl);
                
                /* --- Render end --- */

                self.gl.draw_end();
            }

            /* Update event */
            if let Some(args) = ev.update_args() {
                self.core.update(&args);
            }
        }
    }
}
