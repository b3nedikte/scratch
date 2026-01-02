pub trait Touchable {
    // pub to be used from main.rs
    fn on_touch(&mut self, x: f32, y: f32) -> bool;
    fn get_bounds(&self) -> (f32, f32, f32, f32); // x, y, width, height

    // Default implementation
    fn contains_point(&self, x: f32, y: f32) -> bool {
        let (bx, by, width, height) = self.get_bounds();
        x >= bx && x <= bx + width && y >= by && y <= by + height
    }
}

pub struct Button {
    pub label: String,
    pub pressed: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Touchable for Button {
    fn on_touch(&mut self, _x: f32, _y: f32) -> bool {
        self.pressed = !self.pressed;
        println!("Button {} pressed!", self.label);

        self.pressed
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

pub struct Checkbox {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub checked: bool,
}

impl Touchable for Checkbox {
    fn on_touch(&mut self, _x: f32, _y: f32) -> bool {
        self.checked = !self.checked;
        println!("Checkbox toggled: {}", self.checked);

        self.checked
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.size, self.size) // size*size is a square
    }
}

pub struct Slider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub value: f32,
}

impl Touchable for Slider {
    fn on_touch(&mut self, x: f32, y: f32) -> bool {
        self.value = ((x - self.x) / self.width).clamp(0.0, 1.0);
        println!("Slider value: {}", self.value);

        true
    }

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}
