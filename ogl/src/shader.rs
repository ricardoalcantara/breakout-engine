use gl::types::*;
use std::mem;
use std::str;
use std::{ffi::CString, ptr};

pub struct Shader {
    pub id: u32,
}

enum ShaderCompileType {
    VERTEX,
    FRAGMENT,
    GEOMETRY,
    PROGRAM,
}

impl Shader {
    pub fn compile(
        vertex_source: &str,
        fragment_source: &str,
        geometry_source: Option<&str>,
    ) -> Self {
        let id = unsafe {
            let (vertex_id, fragment_id, geometry_id): (u32, u32, Option<u32>);
            // vertex Shader
            vertex_id = {
                let vertex_id = gl::CreateShader(gl::VERTEX_SHADER);
                let c_str = CString::new(vertex_source.as_bytes()).unwrap();
                gl::ShaderSource(vertex_id, 1, &c_str.as_ptr(), ptr::null());
                gl::CompileShader(vertex_id);
                Shader::check_compile_errors(vertex_id, ShaderCompileType::VERTEX);
                vertex_id
            };
            // fragment Shader
            fragment_id = {
                let fragment_id = gl::CreateShader(gl::FRAGMENT_SHADER);
                let c_str = CString::new(fragment_source.as_bytes()).unwrap();
                gl::ShaderSource(fragment_id, 1, &c_str.as_ptr(), ptr::null());
                gl::CompileShader(fragment_id);
                Shader::check_compile_errors(fragment_id, ShaderCompileType::FRAGMENT);
                fragment_id
            };
            // if geometry shader source code is given, also compile geometry shader
            geometry_id = if let Some(geometry_source) = geometry_source {
                let geometry_id = gl::CreateShader(gl::GEOMETRY_SHADER);
                let c_str = CString::new(geometry_source.as_bytes()).unwrap();
                gl::ShaderSource(geometry_id, 1, &c_str.as_ptr(), ptr::null());
                gl::CompileShader(geometry_id);
                Shader::check_compile_errors(geometry_id, ShaderCompileType::GEOMETRY);
                Some(geometry_id)
            } else {
                None
            };

            let program_id = {
                let program_id = gl::CreateProgram();

                gl::AttachShader(program_id, vertex_id);
                gl::AttachShader(program_id, fragment_id);
                if let Some(geometry_id) = geometry_id {
                    gl::AttachShader(program_id, geometry_id);
                }

                gl::LinkProgram(program_id);
                Shader::check_compile_errors(program_id, ShaderCompileType::PROGRAM);
                // delete the shaders as they're linked into our program now and no longer necessary
                gl::DeleteShader(vertex_id);
                gl::DeleteShader(fragment_id);
                if let Some(geometry_id) = geometry_id {
                    gl::DeleteShader(geometry_id);
                }

                program_id
            };

            program_id
        };
        Self { id }
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    // Todo: for later https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/shader.cpp

    // pub fn set_float    (const char *name, float value, bool useShader = false);
    pub fn set_integer(&self, name: &str, value: i32, use_shader: bool) {
        if (use_shader) {
            self.use_program();
        }

        let c_name = CString::new(name).unwrap();
        unsafe {
            let uniform_location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform1i(uniform_location, value);
        }
    }
    // pub fn set_vector2f (const char *name, float x, float y, bool useShader = false);
    // pub fn set_vector2f (const char *name, const glm::vec2 &value, bool useShader = false);
    // pub fn set_vector3f (const char *name, float x, float y, float z, bool useShader = false);
    pub fn set_vector3f(&self, name: &str, value: &glam::Vec3, use_shader: bool) {
        if (use_shader) {
            self.use_program();
        }

        let c_name = CString::new(name).unwrap();
        unsafe {
            let uniform_location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform3f(uniform_location, value.x, value.y, value.z);
        }
    }
    // pub fn set_vector4f (const char *name, float x, float y, float z, float w, bool useShader = false);
    // pub fn set_vector4f (const char *name, const glm::vec4 &value, bool useShader = false);

    // pub fn set_matrix4  (&self, const char *name, const glm::mat4 &matrix, use_shader: bool) {
    pub fn set_matrix4(&self, name: &str, matrix: &glam::Mat4, use_shader: bool) {
        if (use_shader) {
            self.use_program();
        }

        let c_name = CString::new(name).unwrap();
        let matrix_array = matrix.to_cols_array_2d();
        unsafe {
            let uniform_location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::UniformMatrix4fv(
                uniform_location,
                1,
                gl::FALSE as GLboolean,
                mem::transmute(&matrix_array[0]),
            );
        }
    }

    fn check_compile_errors(object_id: u32, compile_type: ShaderCompileType) {
        let mut status = gl::FALSE as GLint;
        match compile_type {
            ShaderCompileType::PROGRAM => {
                unsafe {
                    let mut status = gl::FALSE as GLint;
                    gl::GetProgramiv(object_id, gl::LINK_STATUS, &mut status);

                    // Fail on error
                    if status != (gl::TRUE as GLint) {
                        let mut len: GLint = 0;
                        gl::GetProgramiv(object_id, gl::INFO_LOG_LENGTH, &mut len);
                        let mut buf = Vec::with_capacity(len as usize);
                        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                        gl::GetProgramInfoLog(
                            object_id,
                            len,
                            ptr::null_mut(),
                            buf.as_mut_ptr() as *mut GLchar,
                        );
                        panic!(
                            "{}",
                            str::from_utf8(&buf)
                                .ok()
                                .expect("ProgramInfoLog not valid utf8")
                        );
                    }
                }
            }
            _ => {
                unsafe {
                    gl::GetShaderiv(object_id, gl::COMPILE_STATUS, &mut status);
                    if status != (gl::TRUE as GLint) {
                        let mut len = 0;
                        gl::GetShaderiv(object_id, gl::INFO_LOG_LENGTH, &mut len);
                        let mut buf = Vec::with_capacity(len as usize);
                        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                        gl::GetShaderInfoLog(
                            object_id,
                            len,
                            ptr::null_mut(),
                            buf.as_mut_ptr() as *mut GLchar,
                        );
                        panic!(
                            "{}",
                            str::from_utf8(&buf)
                                .ok()
                                .expect("ShaderInfoLog not valid utf8")
                        );
                    }
                }
            }
        }
    }
}
