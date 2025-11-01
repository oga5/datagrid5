use std::fmt;

/// Data type for column configuration
#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Text,
    Number,
    Date,
    Boolean,
}

/// Represents the value stored in a grid cell
#[derive(Clone, Debug)]
pub enum CellValue {
    Empty,
    Text(String),
    Number(f64),
    Boolean(bool),
    Date(String), // ISO 8601 format: YYYY-MM-DD or YYYY-MM-DD HH:MM:SS
}

impl PartialEq for CellValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CellValue::Empty, CellValue::Empty) => true,
            (CellValue::Text(a), CellValue::Text(b)) => a == b,
            (CellValue::Number(a), CellValue::Number(b)) => a == b,
            (CellValue::Boolean(a), CellValue::Boolean(b)) => a == b,
            (CellValue::Date(a), CellValue::Date(b)) => a == b,
            _ => false,
        }
    }
}

impl CellValue {
    pub fn to_string(&self) -> String {
        match self {
            CellValue::Empty => String::new(),
            CellValue::Text(s) => s.clone(),
            CellValue::Number(n) => n.to_string(),
            CellValue::Boolean(b) => b.to_string(),
            CellValue::Date(d) => d.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, CellValue::Empty)
    }
}

impl Default for CellValue {
    fn default() -> Self {
        CellValue::Empty
    }
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Cell border configuration for individual borders
#[derive(Clone, Debug)]
pub struct CellBorder {
    pub color: u32,      // RGBA color as u32
    pub width: f32,      // Border width in pixels
}

/// Collection of borders for a cell (stored separately from Cell for memory efficiency)
#[derive(Clone, Debug, Default)]
pub struct CellBorders {
    pub top: Option<CellBorder>,
    pub right: Option<CellBorder>,
    pub bottom: Option<CellBorder>,
    pub left: Option<CellBorder>,
}

/// Represents a single cell in the grid
#[derive(Clone, Debug)]
pub struct Cell {
    pub value: CellValue,
    pub editable: bool,
    pub modified: bool, // Track if cell has been edited
    pub bg_color: Option<u32>, // RGBA color as u32
    pub fg_color: Option<u32>,
    pub font_bold: bool,
    pub font_italic: bool,
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Self {
            value,
            editable: true,
            modified: false,
            bg_color: None,
            fg_color: None,
            font_bold: false,
            font_italic: false,
        }
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        Self::new(CellValue::Text(text.into()))
    }

    pub fn with_number(num: f64) -> Self {
        Self::new(CellValue::Number(num))
    }

    pub fn empty() -> Self {
        Self::new(CellValue::Empty)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::empty()
    }
}
