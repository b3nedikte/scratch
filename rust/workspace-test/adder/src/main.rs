use rand;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {num} plus {num} is {}!",
        add_one::add(num, num)
    );

    println!(
        "hELLO WOrLD! {num} plus {num} plus 2 is {}!",
        add_two::add_two(num, num)
    );
}
