use std::fmt;

#[derive(Debug)]
pub enum MenuError {
    NotFound(String),
    AlreadyExists(String),
    Empty,
}

impl fmt::Display for MenuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuError::NotFound(name) => write!(f, "Item '{}' not found", name),
            MenuError::AlreadyExists(name) => write!(f, "Item '{}' already exists", name),
            MenuError::Empty => write!(f, "Menu is empty"),
        }
    }
}
