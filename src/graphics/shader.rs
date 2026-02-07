use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::ptr;
use std::ffi::CString;
use nalgebra_glm as glm;

/// A compiled and linked OpenGL shader program.
pub struct Shader {
    pub(crate) id: u32,
    uniform_cache: RefCell<HashMap<String, i32>>,
}

impl Shader {
    /// Compiles vertex and fragment shaders from file paths and links them into a program.
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_code =
            fs::read_to_string(vertex_path).expect("Failed to read vertex shader");
        let fragment_code =
            fs::read_to_string(fragment_path).expect("Failed to read fragment shader");
        Self::from_source(&vertex_code, &fragment_code)
    }

    /// Compiles vertex and fragment shaders from GLSL source strings and links them into a program.
    pub fn from_source(vertex_src: &str, fragment_src: &str) -> Self {
        unsafe {
            let vertex = compile_shader(vertex_src, gl::VERTEX_SHADER);
            let fragment = compile_shader(fragment_src, gl::FRAGMENT_SHADER);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            check_program_link_errors(id);

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            Self { id, uniform_cache: RefCell::new(HashMap::new()) }
        }
    }

    /// Binds this shader program for subsequent draw calls.
    #[inline]
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    fn uniform_location(&self, name: &str) -> i32 {
        if let Some(&loc) = self.uniform_cache.borrow().get(name) {
            return loc;
        }
        let cname = CString::new(name).unwrap();
        let loc = unsafe { gl::GetUniformLocation(self.id, cname.as_ptr()) };
        self.uniform_cache.borrow_mut().insert(name.to_string(), loc);
        loc
    }

    // ---------- Uniform helpers ----------

    /// Sets a `mat4` uniform.
    pub fn set_mat4(&self, name: &str, mat: &glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_location(name),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }

    /// Sets a `vec3` uniform.
    pub fn set_vec3(&self, name: &str, v: &glm::Vec3) {
        unsafe {
            gl::Uniform3f(
                self.uniform_location(name),
                v.x,
                v.y,
                v.z,
            );
        }
    }

    /// Sets a `vec2` uniform.
    pub fn set_vec2(&self, name: &str, v: &glm::Vec2) {
        unsafe {
            gl::Uniform2f(
                self.uniform_location(name),
                v.x,
                v.y,
            );
        }
    }

    /// Sets a `float` uniform.
    pub fn set_f32(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(self.uniform_location(name), value);
        }
    }

    /// Sets a `vec4` uniform.
    pub fn set_vec4(&self, name: &str, v: &glm::Vec4) {
        unsafe {
            gl::Uniform4f(
                self.uniform_location(name),
                v.x,
                v.y,
                v.z,
                v.w,
            );
        }
    }

    /// Sets an `int` uniform.
    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(self.uniform_location(name), value);
        }
    }
}

unsafe fn compile_shader(source: &str, kind: u32) -> u32 {
    let shader = gl::CreateShader(kind);
    let c_str = CString::new(source).unwrap();

    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success == 0 {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

        let mut buffer = vec![0u8; len as usize];
        gl::GetShaderInfoLog(
            shader,
            len,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut _,
        );

        panic!(
            "Shader compilation failed:\n{}",
            String::from_utf8_lossy(&buffer)
        );
    }

    shader
}

unsafe fn check_program_link_errors(program: u32) {
    let mut success = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

    if success == 0 {
        let mut len = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

        let mut buffer = vec![0u8; len as usize];
        gl::GetProgramInfoLog(
            program,
            len,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut _,
        );

        panic!(
            "Program linking failed:\n{}",
            String::from_utf8_lossy(&buffer)
        );
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
