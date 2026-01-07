#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // exercise 1
    let celsius = vec![0, 12, -4, 78, -44];

    for item in &celsius {
        let fahr = item * 9 / 5 + 32;
        println!("From {item} to {fahr}");
    }

    celsius
        .iter()
        .for_each(|item| println!("Fahrenheit: {:?}", item * 9 / 5 + 32));

    let fahr: Vec<i32> = celsius.iter().map(|item| item * 9 / 5 + 32).collect();
    println!("To {:?}", fahr);

    // exercise 2
    let ages = vec![12, 8, 18, 34, 59, 4];

    let adults: Vec<&i32> = ages.iter().filter(|item| *item - 18 >= 0).collect();
    println!("Adults: {:?}", adults);

    // (You need **item because: iter() → &i32 and closure arg → &&i32)

    // Math operators are "helpful" with deref. *item - 18 >= 0 WORKS
    // Comparisons are "strict". **item >= 18 REQUIRED

    let adults: Vec<&i32> = ages.iter().filter(|item| **item >= 18).collect();
    println!("Adults: {:?}", adults);

    // exercise 3
    let prices = vec![12.99, 5.83, 46.17, 56.00];

    let total: f64 = prices.iter().sum::<f64>();
    let total2: f64 = prices.iter().fold(0.0, |acc, &item| acc + item);

    println!("Total: {:?} {:?}", total, total2);

    // exercise 4
    let grades = vec![45, 52, 38, 59, 89, 12];

    let first_pass = grades.iter().find(|grade| **grade >= 60);

    match first_pass {
        Some(grade) => println!("First passing grade: {}", grade),
        None => println!("No passing grades found"),
    }

    // exercise 5
    let passwords = vec!["abc", "password123", "h!K9#mP2", "12345"];

    let any_long = passwords.iter().any(|&item| item.len() > 10);
    let all_min_length = passwords.iter().all(|item| item.len() > 4);
    let any_has_special = passwords.iter().any(|item| item.contains('!'));

    println!("Any longer than 10? {}", any_long);
    println!("All at least 4 chars? {}", all_min_length);
    println!("Any has '!'? {}", any_has_special);
}
/*

  | Method  | Closure receives | To get value             |
  |---------|------------------|--------------------------|
  | map     | &T               | *item or &item pattern   |
  | fold    | &T               | *item or &item pattern   |
  | filter  | &&T              | **item or &&item pattern |
  | find    | &&T              | **item or &&item pattern |
  | any/all | &T               | *item or &item pattern   |

*/
