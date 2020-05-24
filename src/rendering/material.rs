use std::borrow::Borrow;
use std::rc::Rc;

use glium::uniforms::UniformValue;
use glium::*;

use super::super::program::ProgramsCache;

pub trait Material {
    fn get_program(&self) -> &Program;
    fn get_draw_parameters(&self) -> DrawParameters;
    fn visit_uniforms<'n>(&'n self, visitor: &mut dyn FnMut(&str, UniformValue<'n>));
}

pub struct SimpleMaterial {
    program: Rc<Program>,
}

#[allow(dead_code)]
impl SimpleMaterial {
    pub fn new(program: Rc<Program>) -> SimpleMaterial {
        SimpleMaterial { program }
    }

    pub fn from_cache(
        programs_cache: &ProgramsCache,
        program_name: &str,
    ) -> Option<SimpleMaterial> {
        let program = programs_cache.get_program(program_name);

        match program {
            Some(program) => Some(SimpleMaterial { program }),
            None => None,
        }
    }
}

impl Material for SimpleMaterial {
    fn get_program(&self) -> &Program {
        self.program.borrow()
    }

    fn get_draw_parameters(&self) -> DrawParameters {
        DrawParameters {
            blend: Blend::alpha_blending(),
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        }
    }

    fn visit_uniforms<'n>(&self, _visitor: &mut dyn FnMut(&str, UniformValue<'n>)) {}
}
