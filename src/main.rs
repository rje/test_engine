
extern crate sdl2;
extern crate gl;
extern crate specs;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::video::GLProfile;

use std::time::Duration;
use std::thread;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let vid_sys = sdl_ctx.video().unwrap();
    let gl_attr = vid_sys.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = vid_sys.window("test_engine", 1280, 720)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

    let _gl_ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| vid_sys.gl_get_proc_address(name) as *const _);

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    'main: loop {
        clear_screen();
        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | 
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                },
                _ => {}
            }
        }
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn clear_screen() {
    unsafe {
        gl::ClearColor(0.6, 0.0, 0.8, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}