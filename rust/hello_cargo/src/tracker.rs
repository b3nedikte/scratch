use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ClickTracker {
    counts: RefCell<HashMap<String, u32>>,
}

impl ClickTracker {
    pub fn new() -> Self {
        ClickTracker {
            counts: RefCell::new(HashMap::new()),
        }
    }

    pub fn record_click(&self, widget_name: &str) {
        // can mutate even with &self
        let mut counts = self.counts.borrow_mut();
        *counts.entry(widget_name.to_string()).or_insert(0) += 1;
    }

    pub fn get_clicks(&self, widget_name: &str) -> u32 {
        *self.counts.borrow().get(widget_name).unwrap_or(&0)
    }

    pub fn print_all(&self) {
        println!("Click counts: {:?}", self.counts.borrow());
    }
}
