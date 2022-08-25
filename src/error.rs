/*!
One-line description.

More detailed description, with

# Example

 */

use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum Error {
    InvalidValue {
        type_name: String,
        value_as_string: String,
    },
    IoError {
        source: std::io::Error,
    },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn invalid_value(type_name: &str, value: &impl Debug) -> Error {
    Error::InvalidValue {
        type_name: type_name.to_string(),
        value_as_string: format!("{:?}", value),
    }
}

#[inline]
pub fn io_error(source: std::io::Error) -> Error {
    Error::IoError { source }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidValue {
                    type_name,
                    value_as_string,
                } => format!("Invalid value `{}` for type {}", value_as_string, type_name),
                Error::IoError { source } =>
                    format!("An I/O error occurred, source: `{:?}`", source),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        io_error(source)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ---------------------------------------------------------------------------------------
