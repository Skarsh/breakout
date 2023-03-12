extern crate glfw;
use self::glfw::{Action, Context, Key};
use glfw::ffi::{glfwGetTime, glfwPollEvents};

extern crate gl;
use self::gl::types::*;

extern crate image;

use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;
use std::sync::mpsc::Receiver;

mod shader;
use shader::Shader;

mod texture;

mod game;
use game::Game;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

#[allow(non_snake_case)]
pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // deltatime vairables
    let mut delta_time = 0.0;
    let mut last_frame = 0.0;

    while !window.should_close() {
        // calculate delta time
        let current_frame = glfw.get_time();
        delta_time = current_frame - last_frame;
        last_frame = current_frame;
        glfw.poll_events();

        // manage user input
        // TODO: This should be done from the game type eventually
        process_events(&mut window, &events);

        // update game state

        // render
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // game.render()
        }

        window.swap_buffers();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
