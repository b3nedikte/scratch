use std::fmt;

pub enum MenuNode {
    Item(MenuItem),
    Submenu {
        name: String,
        children: Vec<Box<MenuNode>>, // box needed for recursive type
    },
}

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

impl MenuNode {
    //helper to create a leaf item
    pub fn item(name: &str, enabled: bool, position: i32) -> Self {
        MenuNode::Item(MenuItem::new(name.to_string(), enabled, position))
    }

    // helper to create a submenu
    pub fn submenu(name: &str, children: Vec<MenuNode>) -> Self {
        MenuNode::Submenu {
            name: name.to_string(),
            children: children.into_iter().map(Box::new).collect(),
        }
    }

    pub fn display(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        match self {
            MenuNode::Item(item) => println!("{}{}", indent, item.name),
            MenuNode::Submenu { name, children } => {
                println!("{}[{}]", indent, name);
                for child in children {
                    child.display(depth + 1);
                }
            }
        }
    }
}
