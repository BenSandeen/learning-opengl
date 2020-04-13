use crate::render_gl;  // This is how we get access to the stuff from `render_gl.rs`

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    #[allow(dead_code)]
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn from_shaders(shaders: &[render_gl::Shader]) -> Result<Program, String> {
        // Creates an OpenGL program object.  A program object is an object to which shader objects can be attached.  We
        // need to do this in order to link the shader objects to create the actual program
        let program_id = unsafe { gl::CreateProgram() };

        // Now we have to attach both shaders to our program object.  (I wonder if we can create multiple programs and let
        // them interact with each other?  Maybe one to perform computations and another to render the visuals?)
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        // This linking creates the executable that will run on the appropriate bit of the GPU, depending on whether
        // we've linked vertex, fragment, and/or geometry shaders (apparently the other types of shaders don't need to
        // be linked or something, I don't really know)
        unsafe { gl::LinkProgram(program_id); }

        // Need to handle any errors here.  This process is almost identical to what we do in the
        // `Shader::from_shader_source()` method
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, & mut len);
            }

            let error = render_gl::create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned())
        }

        // Now that we've attached the shaders and linked things, we can detach them so that they can be deleted (this
        // does not actually delete them here)
        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program { id: program_id })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}