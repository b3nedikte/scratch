use std::fmt;

#[derive(Debug)]
pub struct MenuItem {
    pub name: String,
    pub enabled: bool,
    position: i32,
}

impl MenuItem {
    pub fn new(name: String, enabled: bool, position: i32) -> Self {
        Self {
            name,
            enabled,
            position,
        }
    }

    pub fn display(&self) {
        println!("Showing the menu item from method: {self:?}")
    }
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
