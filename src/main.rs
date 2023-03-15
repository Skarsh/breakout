extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;

extern crate image;

use std::sync::mpsc::Receiver;

mod graphics;
use graphics::Graphics;

mod macros;

mod shader;

mod texture;

mod game;
use game::Game;

mod game_object;

mod game_level;

mod shader_manager;

mod sprite_renderer;

mod texture_manager;

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
            "Breakout",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // setup game
    let shader_manager = shader_manager::ShaderManager::new();
    let texture_manager = texture_manager::TextureManager::new();
    let graphics = Graphics::new(SCR_WIDTH, SCR_HEIGHT, shader_manager, texture_manager);
    let mut game = Game::new(graphics);
    game.init();

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
        game.process_input(delta_time);

        // update game state
        game.update(delta_time);

        // render
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        game.render();

        // delete all resources
        game.clear();
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

//fn key_callback(window: &mut glfw::Window, game: &mut Game, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action) {
//    if key == glfw::Key::Escape && action == glfw::Action::Press {
//        window.set_should_close(true);
//    }
//
//    if action == glfw::Action::Press {
//        game.keys[key.get_scancode().unwrap() as usize] = true;
//    }
//}
