// extern crate sdl2;
use sdl2;  // This handles all the windowing, audio, and mouse and keyboard interaction stuff
use gl;
use std::ffi::CString;
use resources::Resources;
use std::path::Path;


mod create_and_clear_window;
mod render_gl;
mod program;
mod two_vaos_and_two_vbos;
pub mod resources;

fn main() {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    //                              Here's what we've done so far                                 //
    ////////////////////////////////////////////////////////////////////////////////////////////////

    // create_and_clear_window::create_and_clear_window();

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //                    Now, some stuff playing around with stuff to learn it                   //
    ////////////////////////////////////////////////////////////////////////////////////////////////

    two_vaos_and_two_vbos::two_vaos_and_two_vbos();
    two_vaos_and_two_vbos::one_yellow_triangle();
    two_vaos_and_two_vbos::vertex_shader_coloring();
    two_vaos_and_two_vbos::coloring_with_uniforms();

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //                                    Now, the new stuff                                      //
    ////////////////////////////////////////////////////////////////////////////////////////////////

    // Redo some stuff here to make sure I understand it
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();

    // Set up some things for OpenGL here
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);  // Using OpenGL Core...
    gl_attr.set_context_version(3, 3);            // ...version 3.3

    let window = video_subsystem.window("Window Title", 900, 700)
        .resizable()
        .opengl()
        .build().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);  // Set up viewport for OpenGL
        gl::ClearColor(0.5, 0.3, 0.3, 1.0);  // Color that window will default to when everything is cleared
    }
    let mut event_pump = _sdl.event_pump().unwrap();

    println!("GL Version: {:}, {:}, {:}, {:}, {:}", gl::MAJOR_VERSION, gl::MINOR_VERSION,
             gl::NUM_SHADING_LANGUAGE_VERSIONS, gl::SHADING_LANGUAGE_VERSION, gl::VERSION);
    println!("Depth test: {:}", gl::DEPTH_TEST);

    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(
            include_str!("triangle.vert")
        ).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(
            // This macro effectively tells the compiler to compile the file's contents into this file as a string
            include_str!("triangle.frag")
        ).unwrap()
    ).unwrap();

    let shader_program = program::Program::from_shaders(
        &vec![vert_shader, frag_shader]
    ).unwrap();

    // Set our program to use our shaders
    shader_program.set_used();

    // Now generate a simple vertex array for a triangle we'll render, and include the colors
    let vertices: Vec<f32> = vec![
        // positions        // colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,
         0.0,  0.5, 0.0,    0.0, 0.0, 1.0
    ];

    // Create a pointer to that will refer to the array that we can use to hand off to OpenGL.  Note that the way this
    // works is that OpenGL creates this object behind the scenes and when we interact with OpenGL using this integer
    // pointer, it knows that we're referring to the object we created here
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        // This tells OpenGL that we'll be using one buffer and gives it the pointer that we'll use to refer to the
        // buffer.  It's crucial that we tell OpenGL the correct number of buffers to create, because otherwise it may
        // overwrite memory that we don't want it to touch
        gl::GenBuffers(1, &mut vbo);

        // Binds the `vbo` buffer object and lets OpenGL know that it's an array (vertex) buffer.  Since OpenGL only has
        // one `ARRAY_BUFFER`, whenever we do something with the array buffer, OpenGL knows that it involves `vbo`
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Actually send the data in the buffer
        gl::BufferData(
            // Vertex buffer type
            gl::ARRAY_BUFFER,

            // Get size of array in bytes and convert to an OpenGL type
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,

            // Pointer to the actual vertex array
            vertices.as_ptr() as *const gl::types::GLvoid,

            // How we want to use the array.  Options are: GL_STREAM_DRAW, `GL_STREAM_READ`, `GL_STREAM_COPY`,
            // `GL_STATIC_DRAW`, `GL_STATIC_READ`, `GL_STATIC_COPY`, `GL_DYNAMIC_DRAW`, `GL_DYNAMIC_READ`, or
            // `GL_DYNAMIC_COPY`
            gl::STATIC_DRAW,
        );

        // Unbind the buffer now that we've sent the data to the GPU
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // Now we must create a Vertex Array Object (VAO) to tell OpenGL how to interpret the data in `vertices`.  VAO is
    // basically a wrapper around VBOs to facilitate easier descriptions of how to interact with a VBO
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    unsafe {
        // Re-bind the vertex buffer object for this step so that we can "configure the relation between the VBO and the
        // VAO".  Normally we wouldn't unbind and then re-bind this, but doing it here makes it clear that we actually
        // need it for this step to work
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Send over the positions to the vertex shader
        gl::EnableVertexAttribArray(0);  // this is `layout (location = 0)` in vertex shader

        gl::VertexAttribPointer(
            0,  // index of the generic vertex attribute (corresponds to the `layout(location = 0)` in the shaders)
            3,  // number of components per generic vertex attribute
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,  // stride (byte offset between consecutive attributes)
            0 as *const gl::types::GLvoid  // offset of the first component
        );

        // Now send over the colors to the vertex shader
        gl::EnableVertexAttribArray(1);  // this is `layout (location = 1)` in vertex shader

        gl::VertexAttribPointer(
            1,  // index of the generic vertex attribute (corresponds to the `layout(location = 1)` in the shaders)
            3,  // number of components per generic vertex attribute
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,  // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid  // offset of initial element of the array
        );

        // Unbind both VAO and VBO, just like we did previously
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

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

        // Now draw the triangle
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                // Mode, tells OpenGL how to use the vertices in the VAO.  Can choose from: `GL_POINTS`, `GL_LINE_STRIP`,
                // `GL_LINE_LOOP`, `GL_LINES`, `GL_LINE_STRIP_ADJACENCY`, `GL_LINES_ADJACENCY`, `GL_TRIANGLE_STRIP`,
                // `GL_TRIANGLE_FAN`, `GL_TRIANGLES`, `GL_TRIANGLE_STRIP_ADJACENCY`, `GL_TRIANGLES_ADJACENCY` and
                // `GL_PATCHES`
                gl::TRIANGLES,
                0,  // Starting index in the arrays
                3   // Number of indices to be rendered
            )
        }

        window.gl_swap_window();
    }
}
