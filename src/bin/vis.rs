#![allow(unstable)]

extern crate clock_ticks;
extern crate event;
extern crate graphics;
extern crate image;
extern crate input;
extern crate opengl_graphics;
extern crate png;
extern crate sdl2_window;
extern crate search;
extern crate shader_version;

use frame_counter::{ FrameCounter, FrameUpdate };
use opengl_graphics::{ Gl,Texture };
use sdl2_window::Sdl2Window;
use search::map;
use std::cell::RefCell;
use std::io;

macro_rules! errorln {
    ($fmt:expr) => {
        (writeln![&mut io::stdio::stderr(), $fmt]).ok().expect("log failed")
    };
    ($fmt:expr, $($msg:tt)*) => {
        (writeln![&mut io::stdio::stderr(), $fmt, $($msg)*]).ok().expect("log failed")
    };
}

mod frame_counter;

fn main() {
    let img = png::load_png(&Path::new("test/map5.png")).unwrap();
    let map = map::from_png(&img);
    let image = map::to_image_buffer(&map);
    let mut fc = FrameCounter::from_fps(25);
    let opengl = shader_version::OpenGL::_3_2;
    let (width, height) = image.dimensions();
    let window = Sdl2Window::new(
        opengl,
        event::WindowSettings {
            title: "Graph Search".to_string(),
            size: [width, height],
            //fullscreen: true,
            fullscreen: false,
            exit_on_esc: true,
            samples: 0
        }
    );
    let texture = Texture::from_image(&image);
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in event::events(&window) {
        use event::{ RenderEvent };
        if let Some(args) = e.render_args() {
            if let FrameUpdate::NewFrame{skipped_frames, ..} = fc.update() {
                errorln!("new frame: skipped={:?}", skipped_frames);
                gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                    graphics::clear([0.0; 4], gl);
                    graphics::image(&texture, &c, gl);
                });
            }
        };
    }
}
