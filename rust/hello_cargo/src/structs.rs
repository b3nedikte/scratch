#[derive(Debug)]

struct MenuItem {
    name: String,
    enabled: bool,
    position: i32,
}

impl MenuItem {
    fn new(name: String, enabled: bool, position: i32) -> Self {
        Self {
            name,
            enabled,
            position,
        }
    }
    fn display(&self) {
        println!("Showing the menu item from method: {self:?}")
    }
    fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled
    }
}

fn main() {
    // use constructor new()
    let mut menu = MenuItem::new(String::from("Alarm"), true, 0);

    // print the item using display()
    menu.display(); // call as method of MenuItem

    // flip the enabled state and display it
    menu.toggle_enabled(); // call as method of MenuItem
    menu.display();
}
