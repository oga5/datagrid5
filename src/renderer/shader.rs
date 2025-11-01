use crate::GridError;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

/// Vertex shader for grid rendering
pub const GRID_VERTEX_SHADER: &str = r#"
    attribute vec2 a_position;
    attribute vec4 a_color;

    uniform vec2 u_resolution;
    uniform vec2 u_translation;

    varying vec4 v_color;

    void main() {
        // Convert from pixel coordinates to clip space
        vec2 position = a_position + u_translation;
        vec2 clipSpace = (position / u_resolution) * 2.0 - 1.0;

        // Flip Y axis
        gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        v_color = a_color;
    }
"#;

/// Fragment shader for grid rendering
pub const GRID_FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    varying vec4 v_color;

    void main() {
        gl_FragColor = v_color;
    }
"#;

/// Compile a shader
pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, GridError> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| GridError::ShaderError {
            error: "Unable to create shader object".to_string(),
        })?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let error = context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string());
        Err(GridError::ShaderError { error })
    }
}

/// Link a shader program
pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, GridError> {
    let program = context
        .create_program()
        .ok_or_else(|| GridError::ShaderError {
            error: "Unable to create shader program".to_string(),
        })?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let error = context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string());
        Err(GridError::ShaderError { error })
    }
}

/// Shader program wrapper
pub struct ShaderProgram {
    pub program: WebGlProgram,
}

impl ShaderProgram {
    /// Create a new shader program
    pub fn new(context: &WebGlRenderingContext) -> Result<Self, GridError> {
        let vert_shader = compile_shader(
            context,
            WebGlRenderingContext::VERTEX_SHADER,
            GRID_VERTEX_SHADER,
        )?;

        let frag_shader = compile_shader(
            context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            GRID_FRAGMENT_SHADER,
        )?;

        let program = link_program(context, &vert_shader, &frag_shader)?;

        Ok(Self { program })
    }
}
