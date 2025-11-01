use std::fmt;
use wasm_bindgen::JsValue;

/// Error types for DataGrid operations
#[derive(Debug, Clone)]
pub enum GridError {
    /// Cell coordinates out of bounds
    OutOfBounds { row: usize, col: usize },

    /// Cell is not editable
    CellNotEditable { row: usize, col: usize },

    /// Invalid regular expression pattern
    InvalidRegex { pattern: String, error: String },

    /// Paste operation failed
    PasteFailed { reason: String },

    /// Invalid JSON data
    InvalidJson { error: String },

    /// WebGL/Canvas initialization failed
    RenderInitFailed { error: String },

    /// Shader compilation failed
    ShaderError { error: String },

    /// Generic operation error
    OperationError { message: String },
}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridError::OutOfBounds { row, col } => {
                write!(f, "Cell ({}, {}) is out of bounds", row, col)
            }
            GridError::CellNotEditable { row, col } => {
                write!(f, "Cell ({}, {}) is not editable", row, col)
            }
            GridError::InvalidRegex { pattern, error } => {
                write!(f, "Invalid regex pattern '{}': {}", pattern, error)
            }
            GridError::PasteFailed { reason } => {
                write!(f, "Paste operation failed: {}", reason)
            }
            GridError::InvalidJson { error } => {
                write!(f, "Invalid JSON data: {}", error)
            }
            GridError::RenderInitFailed { error } => {
                write!(f, "Renderer initialization failed: {}", error)
            }
            GridError::ShaderError { error } => {
                write!(f, "Shader error: {}", error)
            }
            GridError::OperationError { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl std::error::Error for GridError {}

// Conversion to JsValue for WASM bindings
impl From<GridError> for JsValue {
    fn from(err: GridError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

// Conversion from String for backward compatibility
impl From<String> for GridError {
    fn from(message: String) -> Self {
        GridError::OperationError { message }
    }
}
