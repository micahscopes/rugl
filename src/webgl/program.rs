/// WebGl Program

use crate::webgl::Shader;
use web_sys::{WebGlProgram, WebGlRenderingContext};

pub struct Program {
    internal: WebGlProgram
}

impl Program {
    pub fn new<'a, T: IntoIterator<Item = &'a Shader>>(
        context: &WebGlRenderingContext,
        shaders: T
    ) -> Result<Program, String> {

        // Create a webgl program
        let program = context
            .create_program()
            .ok_or_else(|| { log!("Unable to create program"); String::from("Unable to create shader object") })?;

        // Iterate through shaders attaching them to the program
        for shader in shaders {
            context.attach_shader(&program, shader.as_ref());
        }

        // Link program with webgl
        context.link_program(&program);

        // See if link failed
        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(Program{ internal: program })
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| { String::from("Unknown error creating program object") }))
        }
    }

    pub fn inner(&self) -> &WebGlProgram {
        &self.internal
    }
}
