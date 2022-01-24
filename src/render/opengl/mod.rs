use crate::error::{BreakoutError, BreakoutResult};

pub mod renderer2d;
pub mod texture;

mod render2d_pipeline;
mod shader;

#[inline]
pub(crate) fn check_gl_ok() -> BreakoutResult {
    let err = unsafe { gl::GetError() };
    match err {
        //"No error has been recorded. The value of this symbolic constant is guaranteed to be 0."
        gl::NO_ERROR => Ok(()),
        gl::INVALID_ENUM => Err(BreakoutError::RenderError("An unacceptable value is specified for an enumerated argument. The offending command is ignored and has no other side effect than to set the error flag.")),
        gl::INVALID_VALUE => Err(BreakoutError::RenderError("A numeric argument is out of range. The offending command is ignored and has no other side effect than to set the error flag.")),
        gl::INVALID_OPERATION => Err(BreakoutError::RenderError("The specified operation is not allowed in the current state. The offending command is ignored and has no other side effect than to set the error flag.")),
        gl::INVALID_FRAMEBUFFER_OPERATION => Err(BreakoutError::RenderError("The framebuffer object is not complete. The offending command is ignored and has no other side effect than to set the error flag.")),
        gl::OUT_OF_MEMORY => Err(BreakoutError::RenderError("There is not enough memory left to execute the command. The state of the GL is undefined, except for the state of the error flags, after this error is recorded.")),
        gl::STACK_UNDERFLOW => Err(BreakoutError::RenderError("An attempt has been made to perform an operation that would cause an internal stack to underflow.")),
        gl::STACK_OVERFLOW => Err(BreakoutError::RenderError("An attempt has been made to perform an operation that would cause an internal stack to overflow. ")),
        _ => Err(BreakoutError::RenderError("Unknown error")),
    }
}
