use core::fmt::{self, Debug, Display};

pub type ArcResult<T, E = ArcError> = core::result::Result<T, E>;

#[derive(Clone, Copy)]
pub enum ArcError {
    /// An error occurred while creating the layout of an arc
    LayoutError,
    /// An error occurred while allocating memory
    AllocError,
}

impl Debug for ArcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::LayoutError => "LayoutError",
            Self::AllocError => "AllocError",
        };
        f.write_str(message)
    }
}

impl Display for ArcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::LayoutError => "an error ocurred while creating the allocation layout of an arc",
            Self::AllocError => "an error ocurred while allocating memory",
        };
        f.write_str(message)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ArcError {}
