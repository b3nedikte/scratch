fn main() {
    let mut hello = String::from("Hello");
    hello = use_string(&mut hello);

    println!("same string again: {hello}");
}

fn use_string(x: &String) -> String {
    let goodbye = x.to_owned() + " and bye";
    println!("Got more strings: {x} and {goodbye}");
    goodbye
}
