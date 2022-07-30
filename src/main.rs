extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;
use std::process;

mod play_with_render {
    use glutin_window::GlutinWindow;
    use graphics::glyph_cache::rusttype::GlyphCache;
    use graphics::{clear, line, rectangle, text, Transformed, Context};
    use opengl_graphics::{GlGraphics, OpenGL, TextureSettings};
    use piston::event_loop::Events;
    use piston::input::RenderEvent;
    use piston::{
        Button, ButtonEvent, EventSettings, MouseButton, MouseCursorEvent, PressEvent,
        ReleaseEvent, RenderArgs, UpdateEvent, WindowSettings,
    };
    use piston_window::PistonWindow;

    const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
    const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    pub fn main_loop() {
        let graphic_api = OpenGL::V4_5;
        let mut window: PistonWindow = WindowSettings::new("playing_with_rendering", [800, 600])
            .exit_on_esc(true)
            .build()
            .unwrap();
        let mut graphic_drawer = GlGraphics::new(graphic_api);
        let mut events = Events::new(EventSettings::new());

        let mut sn = StickyNote { x: 500.0, y: 500.0 };

        let mut mouse_button: bool = false;
        while let Some(evt) = events.next(&mut window) {
            if let Some(rdr_a) = evt.render_args() {
                render(&rdr_a, &mut graphic_drawer, &mut sn);
            }
            if let Some(udt_a) = evt.update_args() {}
            if let Some(mus_a) = evt.mouse_cursor_args() {
                // println!("Mouse moved: {}, {}", mus_a[0], mus_a[1]);
                if mouse_button {
                    sn.update_s(mus_a[0], mus_a[1]);
                }
            }
            if let Some(Button::Mouse(btn)) = evt.press_args() {
                println!("Pressed mouse button: {:?}", btn);
                mouse_button = true;
            }
            if let Some(Button::Mouse(btn)) = evt.release_args() {
                println!("Released mouse button : {:?}", btn);
                mouse_button = false;
            }
        }
    }

    struct StickyNote {
        x: f64,
        y: f64,
    }

    impl StickyNote {
        fn render_s(&self, renderer: &mut GlGraphics, c : &Context) {
            rectangle(
                RED,
                rectangle::square(0.0, 0.0, 50.0),
                c.transform.trans(self.x, self.y),
                renderer,
            );
        }

        fn update_s(&mut self, x: f64, y: f64) {
            self.x = x;
            self.y = y;
        }
    }

    fn render(rdr_a: &RenderArgs, gr: &mut GlGraphics, sn: &mut StickyNote) {
        // let's draw some example shapes

        gr.draw(rdr_a.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(
                FOREGROUND,
                rectangle::square(0.0, 0.0, 50.0),
                c.transform.trans(0.0, 0.0),
                gl,
            );

            rectangle(
                FOREGROUND,
                rectangle::square(0.0, 0.0, 50.0),
                c.transform.trans(100.0, 100.0),
                gl,
            );
            let mut cache =
                GlyphCache::new("src/Lemon Days.otf", (), TextureSettings::new()).unwrap();
            text(
                BLACK,
                50,
                "Otf font works better than ttf",
                &mut cache,
                c.transform.trans(500.0, 200.0),
                gl,
            );

            line(
                FOREGROUND,
                10.0,
                [200.0, 200.0, 400.0, 400.0],
                c.transform.trans(0.0, 0.0),
                gl,
            );

            sn.render_s(gl, &c);
        });
    }
}

fn main() {
    play_with_render::main_loop();
}
