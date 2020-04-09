// extern crate sdl2;
use sdl2;  // This handles all the windowing, audio, and mouse and keyboard interaction stuff
use gl;
use std::ffi::{CStr, CString};

use crate::render_gl;
use crate::program;

pub fn two_vaos_and_two_vbos() {
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
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);  // Set up viewport for OpenGL
        gl::ClearColor(0.5, 0.3, 0.3, 1.0);  // Color that window will default to when everything is cleared
    }
    let mut event_pump = _sdl.event_pump().unwrap();

    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(
            include_str!("two_vaos_and_two_vbos.vert")
        ).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(
            // This macro effectively tells the compiler to compile the file's contents into this file as a string
            include_str!("two_vaos_and_two_vbos.frag")
        ).unwrap()
    ).unwrap();

    let shader_program = program::Program::from_shaders(
        &vec![vert_shader, frag_shader]
    ).unwrap();

    // Set our program to use our shaders
    shader_program.set_used();

    // Now generate a simple vertex array for a triangle we'll render, and include the colors
    let vertices_1: Vec<f32> = vec![
        // positions        // colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,
         0.0,  0.5, 0.0,    0.0, 0.0, 1.0,
    ];

    let vertices_2: Vec<f32> = vec![
        // positions        // colors
        -1.0, -1.0, 0.1,    1.0, 0.0, 0.0,
        -0.3, -0.5, 0.1,    0.0, 1.0, 0.0,
         0.0,  1.0, 0.1,    0.0, 0.0, 1.0
    ];

    // Create a pointer to that will refer to the array that we can use to hand off to OpenGL.  Note that the way this
    // works is that OpenGL creates this object behind the scenes and when we interact with OpenGL using this integer
    // pointer, it knows that we're referring to the object we created here
    let mut vbo_1: gl::types::GLuint = 0;
    let mut vao_1: gl::types::GLuint = 0;
    unsafe {
        // This tells OpenGL that we'll be using one buffer and gives it the pointer that we'll use to refer to the
        // buffer.  It's crucial that we tell OpenGL the correct number of buffers to create, because otherwise it may
        // overwrite memory that we don't want it to touch
        gl::GenBuffers(1, &mut vbo_1);
        gl::GenVertexArrays(1, &mut vao_1);

        // Need to bind vertex array before binding the buffer to it
        gl::BindVertexArray(vao_1);

        // Binds the `vbo` buffer object and lets OpenGL know that it's an array (vertex) buffer.  Since OpenGL only has
        // one `ARRAY_BUFFER`, whenever we do something with the array buffer, OpenGL knows that it involves `vbo`
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_1);

        // Actually send the data in the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_1.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices_1.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // Send over the positions to the vertex shader
        gl::VertexAttribPointer(
            0,  // index of the generic vertex attribute (corresponds to the `layout(location = 0)` in the shaders)
            3,  // number of components per generic vertex attribute
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,  // stride (byte offset between consecutive attributes)
            0 as *const gl::types::GLvoid  // offset of the first component
        );
        gl::EnableVertexAttribArray(0);  // this is `layout (location = 0)` in vertex shader

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

    let mut vao_2: gl::types::GLuint = 0;
    let mut vbo_2: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_2);
        gl::GenVertexArrays(1, &mut vao_2);

        gl::BindVertexArray(vao_2);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_2);

        // Actually send the data in the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_2.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices_2.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

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
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
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
            gl::BindVertexArray(vao_1);
            gl::DrawArrays(
                // Mode, tells OpenGL how to use the vertices in the VAO.  Can choose from: `GL_POINTS`, `GL_LINE_STRIP`,
                // `GL_LINE_LOOP`, `GL_LINES`, `GL_LINE_STRIP_ADJACENCY`, `GL_LINES_ADJACENCY`, `GL_TRIANGLE_STRIP`,
                // `GL_TRIANGLE_FAN`, `GL_TRIANGLES`, `GL_TRIANGLE_STRIP_ADJACENCY`, `GL_TRIANGLES_ADJACENCY` and
                // `GL_PATCHES`
                gl::TRIANGLES,
                0,  // Starting index in the arrays
                vertices_1.len() as i32   // Number of indices to be rendered
            );
            gl::BindVertexArray(vao_2);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,  // Starting index in the arrays
                vertices_2.len() as i32   // Number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}


/// This one uses a different fragment shader that renders the colors of one of the triangles as all yellow
pub fn one_yellow_triangle() {
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
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);  // Set up viewport for OpenGL
        gl::ClearColor(0.5, 0.3, 0.3, 1.0);  // Color that window will default to when everything is cleared
    }
    let mut event_pump = _sdl.event_pump().unwrap();

    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(
            include_str!("two_vaos_and_two_vbos.vert")
        ).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(
            // This macro effectively tells the compiler to compile the file's contents into this file as a string
            include_str!("two_vaos_and_two_vbos.frag")
        ).unwrap()
    ).unwrap();

    let yellow_frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(
            include_str!("yellow_triangle.frag")
        ).unwrap()
    ).unwrap();

    let normal_shader_program = program::Program::from_shaders(
        &vec![vert_shader.clone(), frag_shader]
    ).unwrap();

    let yellow_shader_program = program::Program::from_shaders(
        &vec![vert_shader.clone(), yellow_frag_shader]
    ).unwrap();

    // Now generate a simple vertex array for a triangle we'll render, and include the colors
    let vertices_1: Vec<f32> = vec![
        // positions        // colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,
         0.0,  0.5, 0.0,    0.0, 0.0, 1.0,
    ];

    let vertices_2: Vec<f32> = vec![
        // positions        // colors
        -1.0, -1.0, 0.1,    1.0, 0.0, 0.0,
        -0.3, -0.5, 0.1,    0.0, 1.0, 0.0,
         0.0,  1.0, 0.1,    0.0, 0.0, 1.0
    ];

    // Create a pointer to that will refer to the array that we can use to hand off to OpenGL.  Note that the way this
    // works is that OpenGL creates this object behind the scenes and when we interact with OpenGL using this integer
    // pointer, it knows that we're referring to the object we created here
    let mut vbo_1: gl::types::GLuint = 0;
    let mut vao_1: gl::types::GLuint = 0;
    unsafe {
        // This tells OpenGL that we'll be using one buffer and gives it the pointer that we'll use to refer to the
        // buffer.  It's crucial that we tell OpenGL the correct number of buffers to create, because otherwise it may
        // overwrite memory that we don't want it to touch
        gl::GenBuffers(1, &mut vbo_1);
        gl::GenVertexArrays(1, &mut vao_1);

        // Need to bind vertex array before binding the buffer to it
        gl::BindVertexArray(vao_1);

        // Binds the `vbo` buffer object and lets OpenGL know that it's an array (vertex) buffer.  Since OpenGL only has
        // one `ARRAY_BUFFER`, whenever we do something with the array buffer, OpenGL knows that it involves `vbo`
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_1);

        // Actually send the data in the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_1.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices_1.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // Send over the positions to the vertex shader
        gl::VertexAttribPointer(
            0,  // index of the generic vertex attribute (corresponds to the `layout(location = 0)` in the shaders)
            3,  // number of components per generic vertex attribute
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,  // stride (byte offset between consecutive attributes)
            0 as *const gl::types::GLvoid  // offset of the first component
        );
        gl::EnableVertexAttribArray(0);  // this is `layout (location = 0)` in vertex shader

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

    let mut vao_2: gl::types::GLuint = 0;
    let mut vbo_2: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_2);
        gl::GenVertexArrays(1, &mut vao_2);

        gl::BindVertexArray(vao_2);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_2);

        // Actually send the data in the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_2.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices_2.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

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
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
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

        // Now draw the triangles
        unsafe {
            normal_shader_program.set_used();
            gl::BindVertexArray(vao_1);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,  // Starting index in the arrays
                vertices_1.len() as i32   // Number of indices to be rendered
            );

            yellow_shader_program.set_used();  // use the yellow one for the second triangle
            gl::BindVertexArray(vao_2);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,  // Starting index in the arrays
                vertices_2.len() as i32   // Number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}
