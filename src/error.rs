use image::ImageError;

pub type BreakoutResult<T = ()> = Result<T, BreakoutError>;

#[derive(Debug)]
pub enum BreakoutError {
    RenderError(&'static str),
    GenericError(&'static str),
    ImageError(ImageError),
    IOError(std::io::Error),
    // InvalidFont(InvalidFont),
}
