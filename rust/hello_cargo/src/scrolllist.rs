pub struct ScrollList<T> {
    // generic struct that can hold any type
    items: Vec<T>,
    scroll_offset: usize,
    visible_count: usize,
}

impl<T> ScrollList<T> {
    pub fn new(visible_count: usize) -> Self {
        ScrollList {
            items: Vec::new(),
            scroll_offset: 0,
            visible_count,
        }
    }
    pub fn add(&mut self, item: T) {
        self.items.push(item); // add item to vector
    }
    pub fn scroll_up(&mut self) {
        // don't scroll above 0
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }
    pub fn scroll_down(&mut self) {
        // don't scroll past the end
        if self.scroll_offset + self.visible_count < self.items.len() {
            self.scroll_offset += 1;
        }
    }
    pub fn get_visible(&self) -> &[T] {
        let end = (self.scroll_offset + self.visible_count).min(self.items.len());
        &self.items[self.scroll_offset..end]
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        // Vec::get() returns option -> single item, might !NOT! exist
        self.items.get(index)
    }
}

// Add trait bounds for types that need Display
impl<T> ScrollList<T>
where
    T: std::fmt::Display + Clone + std::fmt::Debug,
{
    pub fn display_visible(&self) {
        for item in self.get_visible() {
            println!("{}", item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_empty() {
        let list: ScrollList<i32> = ScrollList::new(3);
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_count_with_items() {
        let mut list: ScrollList<i32> = ScrollList::new(3);
        list.add(1);
        list.add(2);
        list.add(3);

        assert_ne!(list.count(), 0);
        assert_eq!(list.count(), 3);
    }

    #[test]
    fn test_is_empty() {
        let list: ScrollList<i32> = ScrollList::new(0);

        assert!(list.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut list: ScrollList<i32> = ScrollList::new(2);

        list.add(1);
        list.add(2);
        list.add(3);

        list.clear();

        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_get() {
        let mut list: ScrollList<String> = ScrollList::new(2);

        list.add("abc".to_string());
        list.add("def".to_string());
        list.add("ghi".to_string());

        assert_eq!(list.get(0), Some(&"abc".to_string()));
        assert_eq!(list.get(1), Some(&"def".to_string()));
        assert_eq!(list.get(99), None); // out of bounds
    }
}
