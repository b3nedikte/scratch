enum UiElement {
    Button(String),
    Label(String),
    Slider(f32, f32, f32),
}

fn describe_element(element: &UiElement) {
    match element {
        UiElement::Button(text) => {
            println!("This is a button that says {}", text);
        }
        UiElement::Label(text) => {
            println!("This is a label that says {}", text);
        }
        UiElement::Slider(min, max, current) => {
            println!(
                "This is a slider from {} to {}, current value is {}",
                min, max, current
            );
        }
    }
}

fn handle_click(element: &UiElement) {
    match element {
        UiElement::Button(text) => println!("Clicked button: {}", text),
        // (_) means I don't care about this value
        UiElement::Label(_) => println!("Labels aren't clickable!"),
        UiElement::Slider(_, _, current) => println!("Moved slider to {}", current),
    }
}

fn is_interactive(element: &UiElement) -> bool {
    match element {
        UiElement::Button(_) => true,
        UiElement::Label(_) => false,
        UiElement::Slider(_, _, _) => true,
    }
}

fn reset_slider(element: &mut UiElement) -> Option<f32> {
    match element {
        UiElement::Slider(min, _, current) => {
            *current = *min; // reset to min value
            Some(*current)
        }
        _ => None, // do nothing for other elements
    }
}

fn main() {
    /* Basic example  */
    let ok_button = UiElement::Button(String::from("OK"));
    let username_label = UiElement::Label(String::from("Username"));
    let volume_slider = UiElement::Slider(0.0, 100.0, 30.0);

    /* !! YOU HAVE TO & BORROW
          otherwise the item will be destroyed after the call
    !! */
    describe_element(&ok_button);
    describe_element(&username_label);
    describe_element(&volume_slider);

    /* More advanced example */
    let mut elements = vec![
        UiElement::Button(String::from("Cancel")),
        UiElement::Label(String::from("Password")),
        UiElement::Slider(0.0, 50.0, 10.0),
    ];

    for element in &mut elements {
        describe_element(element);
        handle_click(element);
        println!("This element is clickable: {}", is_interactive(element));
        match reset_slider(element) {
            Some(value) => println!("The slider was reset to: {}", value),
            None => {} // print nothing
        }
    }
}
