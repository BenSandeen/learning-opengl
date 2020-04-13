// No need to do `use sdl2` and `use gl` here, this file has access to that stuff

#[allow(dead_code)]
pub fn create_and_clear_window() {
    let sdl = sdl2::init().unwrap();  // Need to initialize the SDL2 library before doing anything else with it

    // Get SDL2's video system handler
    let video_subsystem = sdl.video().unwrap();

    // Initializes the window (an actual window should pop up)
    let window = video_subsystem
        .window("Poopy", 900, 700)
        .resizable()
        .opengl()  // Tells SDL we'll use this window for handling OpenGL stuff.  Without this, the `let gl_context` line fails
        .build()
        .unwrap();

    // Get the OpenGL context from the window for OpenGL to actually use and do things
    let _gl_context = window.gl_create_context().unwrap();

    // This initializes the gl library or something, so that we can now use it to do stuff on the window we've created
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Sets the color to which OpenGL will clear the window
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // This "event pump" is what handles window events for us
    let mut event_pump = sdl.event_pump().unwrap();

    // In order to not end the program immediately and kill the window we just opened, we loop indefinitely
    'main: loop {  // Label loop `main` so we can break out of it in the nested for loop
        // For each event as they come in, handle them appropriately
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main,
                _ => {},
            }
        }
        // render to window here

        // Tells it to actually clear the window to the given color that we set previously
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // We need to swap the currently displayed window with the onee whose contents we just cleared
        window.gl_swap_window();
    }
}
