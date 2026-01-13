use std::rc::Rc;

mod app_state;
mod menu;
mod scrolllist;
mod theme;
mod tracker;
mod widgets;

use app_state::{AppState, ScoreButton, ScoreDisplay};
use menu::{MenuItem, MenuNode, MenuSystem};
use scrolllist::ScrollList;
use theme::Theme;
use tracker::ClickTracker;
use widgets::{Button, Checkbox, Slider, Touchable};

fn main() {
    let mut main = MenuSystem::new();

    let list = MenuItem::new(String::from("List"), true, 0);
    let edit = MenuItem::new(String::from("Edit"), false, 1);
    let settings = MenuItem::new(String::from("Settings"), false, 2);

    main.add_item(list).unwrap();
    main.add_item(edit).unwrap();
    main.add_item(settings).unwrap();

    // try to add duplicate
    let duplicate = MenuItem::new(String::from("Edit"), true, 3);
    main.add_item(duplicate).unwrap_or_else(|e| {
        println!("Error: {}", e);
        //panic!("Failed to add item");
    });

    match main.remove_item(&String::from("Edit")) {
        Ok(item) => println!("Removed menu item: {:?}", item),
        Err(e) => println!("Error: {}", e),
    }

    match main.find_item(&String::from("Edit")) {
        Ok(item) => println!("Found this item: {:?}", item),
        Err(e) => println!("Error: {}", e),
    }

    main.list_all();

    let _ = main.get_and_display(&String::from("Settings"));

    let theme = Theme::new("#007bff", 13); //15A-2
    // Create three touchable widgets
    let mut widgets: Vec<Box<dyn Touchable>> = vec![
        Box::new(Button {
            x: 10.0,
            y: 10.0,
            width: 100.0,
            height: 40.0,
            label: String::from("Submit"),
            pressed: false,
            theme: Rc::clone(&theme),
        }),
        Box::new(Checkbox {
            x: 10.0,
            y: 60.0,
            size: 20.0,
            checked: false,
            theme: Rc::clone(&theme),
        }),
        Box::new(Slider {
            x: 10.0,
            y: 90.0,
            width: 150.0,
            height: 20.0,
            value: 0.5,
            theme: Rc::clone(&theme),
        }),
    ];

    let touch_x = 50.0;
    let touch_y = 50.0;

    // iterate and touch each
    for widget in &mut widgets {
        if widget.contains_point(touch_x, touch_y) {
            widget.on_touch(touch_x, touch_y);
        }
    }

    let mut list: ScrollList<i32> = ScrollList::new(3);
    list.add(10);
    list.add(23);
    list.add(17);
    list.add(98);

    let mut menulist: ScrollList<MenuItem> = ScrollList::new(2);
    menulist.add(MenuItem::new(String::from("List"), true, 0));
    menulist.add(MenuItem::new(String::from("Edit"), true, 0));
    menulist.add(MenuItem::new(String::from("Settings"), true, 0));

    let mut stringlist: ScrollList<String> = ScrollList::new(2);
    stringlist.add(String::from("a"));
    stringlist.add(String::from("abc"));
    stringlist.add(String::from("abcd"));

    stringlist.display_visible();
    stringlist.scroll_down();
    stringlist.display_visible();
    stringlist.scroll_up();
    stringlist.display_visible();

    let enabled_items = menulist.filter(|item| item.enabled);
    let names = menulist.map(|item| item.name.clone());

    // 15A-1
    let menu_node = MenuNode::submenu(
        "Main Menu",
        vec![
            MenuNode::item("New Game", true, 0),
            MenuNode::item("Load Game", true, 1),
            MenuNode::submenu(
                "Settings",
                vec![
                    MenuNode::item("Audio", true, 0),
                    MenuNode::item("Video", true, 1),
                ],
            ),
            MenuNode::item("Quit", true, 2),
        ],
    );
    menu_node.display(0);

    // 15A-2

    for widget in &mut widgets {
        println!("theme: {:?}", widget.get_theme());
    }
    println!("reference count: {:?}", Rc::strong_count(&theme));

    // 15A-3 -> click counter can be modified through shared references
    let tracker = Rc::new(ClickTracker::new());

    // simulate clicks and record them
    for widget in &mut widgets {
        if widget.contains_point(touch_x, touch_y) {
            widget.on_touch(touch_x, touch_y);
            tracker.record_click(&widget.get_label()); // need to add get_label to Touchable
        }
    }

    // check the counts
    tracker.print_all();
    println!("Button clicks: {}", tracker.get_clicks("Submit"));

    // 15A-4 -> multiple widgets updating shared state
    let state = AppState::new();

    let btn_10 = ScoreButton::new(Rc::clone(&state), 10);
    let btn_50 = ScoreButton::new(Rc::clone(&state), 50);
    let display = ScoreDisplay::new(Rc::clone(&state));

    display.show();
    btn_10.click();
    display.show();
    btn_50.click();
    display.show();

    // all three (btn_10, btn_50, display) share the same AppState - buttons mutate it, display reads it.
}
