// extern crate sdl2;
use sdl2;  // This handles all the windowing, audio, and mouse and keyboard interaction stuff
use gl;
use std::ffi::{CStr, CString};

mod create_and_clear_window;

fn main() {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    //                              Here's what we've done so far                                 //
    ////////////////////////////////////////////////////////////////////////////////////////////////

    // create_and_clear_window::create_and_clear_window();

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //                                    Now, the new stuff                                      //
    ////////////////////////////////////////////////////////////////////////////////////////////////

    // Redo some stuff here to make sure I understand it
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();

    // Set up some things for OpenGL here
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);  // Using OpenGL Core...
    gl_attr.set_context_version(4, 5);            // ...version 4.5

    let window = video_subsystem.window("Window Title", 900, 700)
        .resizable()
        .opengl()
        .build().unwrap();
    let gl_context = window.gl_create_context();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);  // Set up viewport for OpenGL
        gl::ClearColor(0.5, 0.3, 0.3, 1.0);  // Color that window will default to when everything is cleared
    }
    let mut event_pump = _sdl.event_pump().unwrap();

    println!("GL Version: {:}, {:}, {:}, {:}, {:}", gl::MAJOR_VERSION, gl::MINOR_VERSION,
             gl::NUM_SHADING_LANGUAGE_VERSIONS, gl::SHADING_LANGUAGE_VERSION, gl::VERSION);
    println!("Depth test: {:}", gl::DEPTH_TEST);

    // let shader = Shader::from_source()


    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}

// Struct to hold the shader object, simply for convenience
struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    // Since this does not have a `self` parameter, this is basically a static method (doesn't act on individual objects
    // of the `Shader` type, but rather works only with the struct itself.  This is basically a constructor method
    fn from_source(source: &CStr, shader_type: gl::types::GLenum) -> Result<Shader, String> {
        // The `?` does sort of the same thing as a match statement that checks for errors
        let id = shader_from_source(source, shader_type)?;
        Ok(Shader { id })
    }

    fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        let id = shader_from_source(source, gl::VERTEX_SHADER)?;
        Ok(Shader { id })
    }

    fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        let id = shader_from_source(source, gl::FRAGMENT_SHADER)?;
        Ok(Shader { id })
    }

    fn from_compute_source(source: &CStr) -> Result<Shader, String> {
        let id = shader_from_source(source, gl::COMPUTE_SHADER)?;
        Ok(Shader { id })
    }

    fn from_tess_control_source(source: &CStr) -> Result<Shader, String> {
        let id = shader_from_source(source, gl::TESS_CONTROL_SHADER)?;
        Ok(Shader { id })
    }

    fn from_tess_evaluation_source(source: &CStr) -> Result<Shader, String> {
        let id = shader_from_source(source, gl::TESS_EVALUATION_SHADER)?;
        Ok(Shader { id })
    }

    fn from_geometry_source(source: &CStr) -> Result<Shader, String> {
        let id = shader_from_source(source, gl::GEOMETRY_SHADER)?;
        Ok(Shader { id })
    }
}

// Implement `Drop` so that wee aren't leaking memory every time a shader goes out of scope
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

// This will parse a string that contains the shader code.  If it succeeds, then it'll return a shader ID, if it fails,
// it'll return a string with an error message.  Note that we pass in a `CStr` because that's what the underlying
// function that compiles the shader string expects to receive
fn shader_from_source(source: &CStr, shader_type: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    // First, get the shader ID.  This basically creates an empty shader object that we will interact with when doing shader stuff
    let id = unsafe { gl::CreateShader(shader_type) };

    // Now associate the actual shader code (in string form) with the shader object and compile it
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    // Now make sure things worked and if not, create an error message
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // If there was an error compiling, create error message
    if success == 0 {
        // First we must find the length of the error message
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        // Now that we have a buffer of the correct length and type, ask OpenGL to fill it with the message
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        // Return the error, doing a couple of steps to convert it from a `CString` to a (Rust) `String`
        return Err(error.to_string_lossy().into_owned())
    }

    // Otherwise, return the shader object
    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Then we allocate a vector to act as a buffer to hold the message
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);

    // Fill the buffer with spaces, I guess
    buffer.extend(
        [b' ']   // "a single-item stack-allocated array which contains ASCII 'space' byte"
            .iter()   // Obtains an iterator over the array with a single space
            .cycle()  // Cycles over the iterator forever, yielding an infinite number of spaces
            .take(len as usize)  // Limits number of returned items to `len`
    );

    // Convert buffer to a `CString` and return it
    unsafe { CString::from_vec_unchecked(buffer) }
}

