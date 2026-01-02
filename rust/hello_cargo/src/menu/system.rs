use super::MenuError;
use super::MenuItem;
use std::collections::HashMap;

// The menu system that holds multiple items
pub struct MenuSystem {
    items: Vec<MenuItem>,                  // ordered storage
    name_to_index: HashMap<String, usize>, // fast lookup by name
}

impl MenuSystem {
    // create new empty menusystem
    pub fn new() -> Self {
        MenuSystem {
            items: Vec::new(),
            name_to_index: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: MenuItem) -> Result<(), MenuError> {
        // check if item with this name already exists
        if self.name_to_index.contains_key(&item.name) {
            return Err(MenuError::AlreadyExists(item.name.clone()));
        }

        // step1: get the name before we move the item
        let name = item.name.clone(); // clone the name for the hashmap key

        // step2: get the index where this item will be stored
        let index = self.items.len(); // next available index

        // step3: add item to vec NB! THIS MOVES THE ITEM INTO VEC
        self.items.push(item);

        // step4: add name->index mapping to hashmap
        self.name_to_index.insert(name, index);

        Ok(())
    }

    pub fn remove_item(&mut self, name: &str) -> Result<MenuItem, MenuError> {
        // step1: remove from hashmap, get the index
        let index = self
            .name_to_index
            .remove(name)
            .ok_or_else(|| MenuError::NotFound(String::from(name)))?;

        // step2: remove from vector: swap_remove smaps the last element into the removed position
        // so only one index needs to be updated
        let item = self.items.swap_remove(index);

        // step3: update the HASHMAP index of the item that was swapped into this position
        if index < self.items.len() {
            let swapped_name = self.items[index].name.clone();
            self.name_to_index.insert(swapped_name, index);
        }

        Ok(item)
    }

    pub fn find_item(&self, name: &str) -> Result<&MenuItem, MenuError> {
        // use hashmap to find item
        let index = self
            .name_to_index
            .get(name)
            .ok_or_else(|| MenuError::NotFound(String::from(name)))?;

        // find item in vector
        let item = &self.items[*index];

        Ok(item)
    }

    pub fn list_all(&self) {
        // go through the VECTOR to display
        for item in &self.items {
            item.display();
        }

        // another option
        //self.items.iter().for_each(|item| item.display());
    }

    pub fn get_and_display(&self, name: &str) -> Result<(), MenuError> {
        let item = self.find_item(name)?; // propagates error if not found
        item.display();
        Ok(())
    }
}
