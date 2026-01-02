use hello_cargo::menu::{MenuError, MenuItem, MenuSystem};

#[test]
fn test_add_and_find_item() -> Result<(), MenuError> {
    let mut menu = MenuSystem::new();

    let list = MenuItem::new(String::from("List"), true, 0);
    let edit = MenuItem::new(String::from("Edit"), false, 1);
    let settings = MenuItem::new(String::from("Settings"), false, 2);

    menu.add_item(list)?;
    menu.add_item(edit)?;
    menu.add_item(settings)?;

    let found = menu.find_item("Edit")?;
    assert_eq!(found.name, "Edit");

    Ok(())
}

#[test]
fn test_duplicate_fails() {
    let mut menu = MenuSystem::new();

    let list = MenuItem::new(String::from("List"), true, 0);
    let edit = MenuItem::new(String::from("Edit"), false, 1);
    let settings = MenuItem::new(String::from("Settings"), false, 2);

    let edit2 = MenuItem::new(String::from("Edit"), true, 1);

    menu.add_item(list).unwrap();
    menu.add_item(edit).unwrap();
    menu.add_item(settings).unwrap();

    let result = menu.add_item(edit2);
    assert!(matches!(result, Err(MenuError::AlreadyExists(_))));
}

#[test]
fn test_remove_item() {
    let mut menu = MenuSystem::new();

    let list = MenuItem::new(String::from("List"), true, 0);
    let edit = MenuItem::new(String::from("Edit"), false, 1);
    let settings = MenuItem::new(String::from("Settings"), false, 2);

    menu.add_item(list).unwrap();
    menu.add_item(edit).unwrap();
    menu.add_item(settings).unwrap();

    let removed = menu.remove_item("Edit").unwrap();
    assert_eq!(removed.name, "Edit");

    assert!(menu.find_item("Edit").is_err());
}
