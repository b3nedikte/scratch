# Functional Programming Exercises

Simple exercises to practice iterator methods. Each focuses on one operation.

---

## Exercise 1: Map - Temperature Converter

Convert a list of Celsius temperatures to Fahrenheit.

**Formula:** `F = C * 9/5 + 32`

```rust
fn main() {
    let celsius = vec![0.0, 20.0, 37.0, 100.0];

    // TODO: Use .iter().map() to convert each to Fahrenheit
    // Expected output: [32.0, 68.0, 98.6, 212.0]

    let fahrenheit: Vec<f64> = todo!();

    println!("{:?}", fahrenheit);
}
```

**Hint:** `.map(|c| ...)` takes each item and transforms it.

---

## Exercise 2: Filter - Find the Adults

Filter a list of ages to keep only those 18 or older.

```rust
fn main() {
    let ages = vec![12, 25, 8, 34, 17, 19, 65, 15];

    // TODO: Use .iter().filter() to keep ages >= 18
    // Expected output: [25, 34, 19, 65]

    let adults: Vec<&i32> = todo!();

    println!("{:?}", adults);
}
```

**Hint:** `.filter(|age| ...)` keeps items where the closure returns `true`.

---

## Exercise 3: Fold - Shopping Cart Total

Calculate the total price of items in a shopping cart.

```rust
fn main() {
    let prices = vec![12.99, 5.50, 3.25, 8.00, 15.75];

    // TODO: Use .iter().fold() to sum all prices
    // Expected output: 45.49

    let total: f64 = todo!();

    println!("Total: ${:.2}", total);
}
```

**Hint:** `.fold(starting_value, |accumulator, item| ...)` combines all items into one.

**Bonus:** Try using `.sum()` instead - it's a shortcut for this common case!

---

## Exercise 4: Find - First Passing Grade

Find the first grade that's 60 or above (passing).

```rust
fn main() {
    let grades = vec![45, 52, 38, 67, 72, 55, 89];

    // TODO: Use .iter().find() to get the first passing grade
    // Expected output: Some(67)

    let first_pass: Option<&i32> = todo!();

    match first_pass {
        Some(grade) => println!("First passing grade: {}", grade),
        None => println!("No passing grades found"),
    }
}
```

**Hint:** `.find(|grade| ...)` returns `Some(&item)` for the first match, or `None`.

---

## Exercise 5: Any/All - Validation Checks

Check if a list of passwords meets certain criteria.

```rust
fn main() {
    let passwords = vec!["abc", "password123", "h!K9#mP2", "12345"];

    // TODO: Use .iter().any() to check if ANY password is longer than 10 chars
    let any_long: bool = todo!();

    // TODO: Use .iter().all() to check if ALL passwords are at least 4 chars
    let all_min_length: bool = todo!();

    // TODO: Use .iter().any() to check if ANY password contains '!'
    let any_has_special: bool = todo!();

    println!("Any longer than 10? {}", any_long);       // Expected: true
    println!("All at least 4 chars? {}", all_min_length); // Expected: false ("abc" is 3)
    println!("Any has '!'? {}", any_has_special);        // Expected: true
}
```

**Hint:**
- `.any(|item| ...)` returns `true` if ANY item matches
- `.all(|item| ...)` returns `true` if ALL items match

---

## Bonus Challenge: Chain Them Together

Combine multiple operations to find the average of all even numbers.

```rust
fn main() {
    let numbers = vec![1, 4, 7, 8, 12, 3, 6, 9, 10];

    // TODO:
    // 1. Filter to keep only even numbers
    // 2. Collect into a Vec
    // 3. Calculate average (sum / count)

    // Expected: evens are [4, 8, 12, 6, 10], average = 40/5 = 8.0

    let evens: Vec<&i32> = todo!();
    let average: f64 = todo!();

    println!("Even numbers: {:?}", evens);
    println!("Average: {}", average);
}
```

---

## Quick Reference

| Method | Returns | Use when you want to... |
|--------|---------|------------------------|
| `map` | Iterator | Transform each item |
| `filter` | Iterator | Keep items matching a condition |
| `fold` | Single value | Combine all items into one |
| `find` | `Option<&T>` | Get first matching item |
| `any` | `bool` | Check if any item matches |
| `all` | `bool` | Check if all items match |
| `collect` | Collection | Convert iterator to Vec, etc. |
| `sum` | Single value | Add up numbers |
| `count` | `usize` | Count items |

---

## Solutions

<details>
<summary>Click to reveal solutions (try first!)</summary>

### Exercise 1
```rust
let fahrenheit: Vec<f64> = celsius.iter().map(|c| c * 9.0/5.0 + 32.0).collect();
```

### Exercise 2
```rust
let adults: Vec<&i32> = ages.iter().filter(|age| **age >= 18).collect();
```

### Exercise 3
```rust
let total: f64 = prices.iter().fold(0.0, |acc, price| acc + price);
// Or simply:
let total: f64 = prices.iter().sum();
```

### Exercise 4
```rust
let first_pass: Option<&i32> = grades.iter().find(|grade| **grade >= 60);
```

### Exercise 5
```rust
let any_long: bool = passwords.iter().any(|p| p.len() > 10);
let all_min_length: bool = passwords.iter().all(|p| p.len() >= 4);
let any_has_special: bool = passwords.iter().any(|p| p.contains('!'));
```

### Bonus
```rust
let evens: Vec<&i32> = numbers.iter().filter(|n| **n % 2 == 0).collect();
let average: f64 = evens.iter().map(|n| **n as f64).sum::<f64>() / evens.len() as f64;
```

</details>
</parameter>
</invoke>