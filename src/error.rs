/// The super type of every error in this crate.
/// If this is used as a return type, the question mark operator can always be used.
///
/// # Examples
///
/// ```rust, no_run
/// use crow::{Context, glutin::{WindowBuilder, EventsLoop}, Texture};
///
/// fn main() -> Result<(), crow::Error> {
///     let mut ctx = Context::new(WindowBuilder::new(), EventsLoop::new())?;
///
///     let image = Texture::load(&mut ctx, "this/path/does/not/exist.png")?;
///
///     ctx.draw(&mut ctx.window_surface(), &image, (0, 0), &Default::default());
///     
///     ctx.finalize_frame();
///     Ok(())
/// }
///
/// ```
#[derive(Debug)]
pub enum Error {
    /// Tried to create a texture with dimensions which are
    /// greater than the maximum allowed texture size or zero.
    InvalidTextureSize { width: u32, height: u32 },
    /// Error created by `image::load`.
    ImageError(image::ImageError),
    /// Error created by `glutin::ContextBuilder::build_windowed`.
    CreationError(glutin::CreationError),
    /// Error created by `glutin::ContextWrapper::make_current`.
    ContextError(glutin::ContextError),
}

#[derive(Debug)]
pub enum NewContextError {
    /// Error created by `glutin::ContextBuilder::build_windowed`.
    CreationError(glutin::CreationError),
    /// Error created by `glutin::ContextWrapper::make_current`.
    ContextError(glutin::ContextError),
}

impl From<NewContextError> for Error {
    fn from(e: NewContextError) -> Self {
        match e {
            NewContextError::CreationError(e) => Error::CreationError(e),
            NewContextError::ContextError(e) => Error::ContextError(e),
        }
    }
}

/// An error returned by `Texture::load`.
#[derive(Debug)]
pub enum LoadTextureError {
    /// Tried to create a texture with dimensions which are
    /// greater than the maximum allowed texture size or zero.
    InvalidTextureSize { width: u32, height: u32 },
    /// Error created by `image::load`.
    ImageError(image::ImageError),
}

impl From<LoadTextureError> for Error {
    fn from(e: LoadTextureError) -> Self {
        match e {
            LoadTextureError::InvalidTextureSize { width, height } => {
                Error::InvalidTextureSize { width, height }
            }
            LoadTextureError::ImageError(e) => Error::ImageError(e),
        }
    }
}

/// An error returned by `Texture::new`.
#[derive(Debug)]
pub enum NewTextureError {
    /// Tried to create a texture with dimensions which are
    /// greater than the maximum allowed texture size or zero.
    InvalidTextureSize { width: u32, height: u32 },
}

impl From<NewTextureError> for LoadTextureError {
    fn from(e: NewTextureError) -> Self {
        match e {
            NewTextureError::InvalidTextureSize { width, height } => {
                LoadTextureError::InvalidTextureSize { width, height }
            }
        }
    }
}

impl From<NewTextureError> for Error {
    fn from(e: NewTextureError) -> Self {
        match e {
            NewTextureError::InvalidTextureSize { width, height } => {
                Error::InvalidTextureSize { width, height }
            }
        }
    }
}
