use std::fmt;

/// Represents the value stored in a grid cell
#[derive(Clone, Debug)]
pub enum CellValue {
    Empty,
    Text(String),
    Number(f64),
    Boolean(bool),
}

impl CellValue {
    pub fn to_string(&self) -> String {
        match self {
            CellValue::Empty => String::new(),
            CellValue::Text(s) => s.clone(),
            CellValue::Number(n) => n.to_string(),
            CellValue::Boolean(b) => b.to_string(),
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

/// Represents a single cell in the grid
#[derive(Clone, Debug)]
pub struct Cell {
    pub value: CellValue,
    pub editable: bool,
    pub selected: bool,
    pub modified: bool, // Track if cell has been edited
    pub bg_color: Option<u32>, // RGBA color as u32
    pub fg_color: Option<u32>,
    pub font_bold: bool,
    pub font_italic: bool,
    // Custom borders (top, right, bottom, left)
    pub border_top: Option<CellBorder>,
    pub border_right: Option<CellBorder>,
    pub border_bottom: Option<CellBorder>,
    pub border_left: Option<CellBorder>,
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Self {
            value,
            editable: true,
            selected: false,
            modified: false,
            bg_color: None,
            fg_color: None,
            font_bold: false,
            font_italic: false,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
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
