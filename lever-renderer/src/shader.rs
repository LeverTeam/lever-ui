use crate::error::RendererError;
use glow::HasContext;

pub unsafe fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, RendererError> {
    let shader = gl
        .create_shader(shader_type)
        .map_err(|_| RendererError::GlAllocation("Shader"))?;

    gl.shader_source(shader, source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        let info_log = gl.get_shader_info_log(shader);
        gl.delete_shader(shader);
        return Err(RendererError::ShaderCompile(info_log));
    }

    Ok(shader)
}

pub unsafe fn link_program(
    gl: &glow::Context,
    vert_shader: glow::Shader,
    frag_shader: glow::Shader,
) -> Result<glow::Program, RendererError> {
    let program = gl
        .create_program()
        .map_err(|_| RendererError::GlAllocation("Program"))?;

    gl.attach_shader(program, vert_shader);
    gl.attach_shader(program, frag_shader);
    gl.link_program(program);

    if !gl.get_program_link_status(program) {
        let info_log = gl.get_program_info_log(program);
        gl.delete_program(program);
        return Err(RendererError::ProgramLink(info_log));
    }

    Ok(program)
}
