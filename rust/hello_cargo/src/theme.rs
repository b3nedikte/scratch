use std::rc::Rc;

#[derive(Debug)]
pub struct Theme {
    primary_color: String,
    font_size: u32,
}

struct ThemedButton {
    label: String,
    theme: Rc<Theme>,
}

struct ThemedSlider {
    value: f32,
    theme: Rc<Theme>,
}

impl Theme {
    pub fn new
    (primary: &str, font_size: u32) -> Rc<Self> {
        Rc::new(Theme {
            primary_color: primary.to_string(),
            font_size,
        })
    }
}
