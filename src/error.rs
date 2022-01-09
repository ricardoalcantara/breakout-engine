use image::ImageError;

pub type BreakoutResult<T = ()> = Result<T, BreakoutError>;

#[derive(Debug)]
pub enum BreakoutError {
    FontError(freetype::Error),
    GenericError(&'static str),
    RenderError(&'static str),

    ImageError(ImageError),
    IOError(std::io::Error),
}
