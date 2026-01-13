# Rust Mastery Program - Book + Touch GUI Development

where rusty contemplations stands for
> reflections that are careful, constrained, safety-aware, and structurally rigorous.

**Goal:** Master Rust fundamentals through The Rust Book while building toward a touchscreen GUI application

**Background:** Strong C experience, Python experience, some C++ experience

**Book:** https://doc.rust-lang.org/book/

**Version:** Rust 1.85.0 / 2024 Edition

---

## 📊 Overall Progress

**Book Progress:** 16/21 chapters
**Practical Exercises:** 10/10 completed
**Major Projects:** 2/3 completed

---

## Phase 1: Core Fundamentals (Weeks 1-2)

### Chapter 1: Getting Started
- [x] Read chapter
- [x] Install Rust toolchain
- [x] Run `cargo new hello_world`
- [x] Verify setup with simple program

**Time estimate:** 1-2 hours

---

### Chapter 2: Guessing Game (Project 1/3)
- [x] Read chapter
- [x] Build the guessing game
- [x] Experiment with modifications

**Concepts:** Basic I/O, random numbers, match statements, error handling intro

**Time estimate:** 2-3 hours

---

### Chapter 3: Common Programming Concepts
- [x] Read chapter
- [x] Practice: variables and mutability
- [x] Practice: data types
- [x] Practice: functions
- [x] Practice: control flow

**C comparison notes:**
- `let` vs `let mut` - explicit mutability (unlike C where everything is mutable by default)
- Type inference (but can be explicit like C)
- Expressions vs statements (like C)
- No implicit type conversions (safer than C)

**Time estimate:** 3-4 hours

---

### Chapter 4: Understanding Ownership ⭐ CRITICAL
- [x] Read chapter (read twice if needed!)
- [x] Complete all inline examples
- [x] Do Exercise 1 below

**This is the biggest mental shift from C/C++. In C, you manage memory manually with malloc/free and pass pointers freely. Rust's ownership system prevents:**
- Use-after-free (dangling pointers)
- Double-free
- Memory leaks (mostly)
- Data races

**All at compile time!**

#### Exercise 1: Ownership Basics ⭐
**Goal:** Internalize Rust's ownership rules

**Tasks:**
1. Create a `String` and pass it to a function
2. Try to use it afterward (won't compile - ownership moved!)
3. Fix using borrowing (`&`)
4. Try mutable borrowing (`&mut`)
5. Experiment with multiple borrows
6. Try to break the rules and read the compiler errors

**Key concepts:**
- Ownership transfer (move semantics)
- Borrowing (references)
- Mutable vs immutable borrows
- Borrow checker rules

**C/C++ comparison:**
```c
// C code - prone to errors
char* str = malloc(100);
some_function(str);
// str might be freed inside some_function - who knows?
// Using str here = undefined behavior!
free(str); // Might be double-free!
```

```rust
// Rust equivalent - compile-time safety
let s = String::from("hello");
some_function(s); // Ownership moved
// Using s here = compile error!
// No need to free - automatic when owner goes out of scope
```

**Time estimate:** 1-2 days (spend extra time here!)

**Notes:**


---

### Chapter 5: Structs
- [x] Read chapter
- [x] Do Exercise 2 below

#### Exercise 2: Structs and Methods
**Goal:** Master Rust's struct system (similar to C structs + C++ methods)

**Tasks:**
Create a `MenuItem` struct with fields:
- `name: String`
- `enabled: bool`
- `position: i32`

Implement methods in an `impl` block:
- `new()` - constructor
- `display()` - print the item
- `toggle_enabled()` - flip the enabled state
- `is_enabled()` - getter

**Key concepts:**
- `struct` definition
- `impl` blocks (methods separate from struct definition)
- `self` (move), `&self` (borrow), `&mut self` (mutable borrow)

**C comparison:**
```c
// C - struct and functions separate
typedef struct {
    char* name;
    bool enabled;
    int position;
} MenuItem;

void menu_item_display(MenuItem* item) { /* ... */ }
void menu_item_toggle(MenuItem* item) { /* ... */ }
```

```rust
// Rust - methods in impl block
struct MenuItem {
    name: String,
    enabled: bool,
    position: i32,
}

impl MenuItem {
    fn display(&self) { /* ... */ }
    fn toggle_enabled(&mut self) { /* ... */ }
}
```

**Time estimate:** 1 day

**Notes:**


---

### Chapter 6: Enums and Pattern Matching
- [x] Read chapter
- [x] Do Exercise 3 below

#### Exercise 3: Enums and Pattern Matching
**Goal:** Master Rust's powerful enums (way beyond C enums!)

**Tasks:**
1. Create an enum `UiElement` with variants:
   - `Button(String)` - holds button text
   - `Label(String)` - holds label text
   - `Slider { min: f32, max: f32, current: f32 }` - slider data

2. Write a function that pattern matches on the enum:
```rust
fn describe_element(elem: &UiElement) {
    match elem {
        UiElement::Button(text) => println!("Button: {}", text),
        UiElement::Label(text) => println!("Label: {}", text),
        UiElement::Slider { min, max, current } => {
            println!("Slider: {} (range {}-{})", current, min, max)
        }
    }
}
```

3. Create a `Vec<UiElement>` with mixed types and iterate through it

**Key concepts:**
- `enum` with data (like C union + enum combined, but type-safe!)
- `match` exhaustiveness (compiler ensures all cases handled)
- Pattern destructuring
- `Option<T>` (Rust's solution to null pointers)
- `Result<T, E>` (for error handling)

**C comparison:**
```c
// C - enums can't hold data, need separate union
enum ElementType { BUTTON, LABEL, SLIDER };

struct Element {
    enum ElementType type;
    union {
        char* text;  // for button/label
        struct { float min, max, current; } slider;
    } data;
};

// Error-prone: nothing stops you from accessing wrong union member!
```

```rust
// Rust - enum holds data, pattern matching ensures correctness
enum UiElement {
    Button(String),
    Label(String),
    Slider { min: f32, max: f32, current: f32 },
}
// Compiler prevents accessing wrong variant!
```

**Time estimate:** 1-2 days

**Notes:**


---

## Phase 2: Collections & Error Handling (Week 3)

### Chapter 7: Packages, Crates, and Modules
- [x] Read chapter
- [x] Do Exercise 8 (Module Organization)

**Concepts:** Code organization, privacy, paths

**Time estimate:** 2-3 hours

---

### Chapter 8: Common Collections
- [x] Read chapter
- [x] Do Exercise 4 below

#### Exercise 4: Collections - Vec and HashMap
**Goal:** Work with dynamic arrays and hash tables

**Tasks:**
Build a menu system using:
1. `Vec<MenuItem>` to store menu items
2. `HashMap<String, usize>` to map names to indices

Implement methods:
- `add_item(&mut self, item: MenuItem)` - add to vec and map
- `remove_item(&mut self, name: &str) -> Option<MenuItem>` - remove by name
- `find_item(&self, name: &str) -> Option<&MenuItem>` - find by name
- `list_all(&self)` - iterate and display all

**Key concepts:**
- `Vec<T>` - growable array
- `HashMap<K, V>` - hash table
- Iterators (`iter()`, `iter_mut()`, `into_iter()`)
- `Option<T>` for operations that might fail

**C/C++ comparison:**
- `Vec<T>` ≈ C++ `std::vector<T>` (but ownership-aware)
- `HashMap<K, V>` ≈ C++ `std::unordered_map<K, V>`
- In C: you'd use arrays + realloc, or roll your own hash table
- Rust handles memory automatically through ownership

**Time estimate:** 1-2 days

**Notes:**


---

### Chapter 9: Error Handling
- [x] Read chapter
- [x] Do Exercise 5 below

#### Exercise 5: Error Handling
**Goal:** Master Rust's `Result` type (no exceptions, no error codes!)

**Tasks:**
Extend Exercise 4 menu system:

1. Define custom error type:
```rust
#[derive(Debug)]
enum MenuError {
    NotFound(String),
    AlreadyExists(String),
    Empty,
}
```

2. Update methods to return `Result<T, MenuError>`:
   - `find_item(&self, name: &str) -> Result<&MenuItem, MenuError>`
   - `remove_item(&mut self, name: &str) -> Result<MenuItem, MenuError>`
   - `add_item(&mut self, item: MenuItem) -> Result<(), MenuError>`

3. Practice error propagation with `?` operator:
```rust
fn get_and_display(&self, name: &str) -> Result<(), MenuError> {
    let item = self.find_item(name)?;  // Propagates error if not found
    item.display();
    Ok(())
}
```

4. Handle errors with `match`, `unwrap_or()`, `unwrap_or_else()`

**Key concepts:**
- `Result<T, E>` - explicit error handling
- `?` operator - error propagation
- `Option<T>` vs `Result<T, E>`
- No exceptions! (unlike C++)
- Pattern matching on results

**C comparison:**
```c
// C - error codes, easy to ignore
int result = find_item(menu, "test");
if (result < 0) {
    // Error handling - but caller can ignore!
}
```

```rust
// Rust - must handle Result, compiler enforces!
match menu.find_item("test") {
    Ok(item) => { /* use item */ },
    Err(e) => { /* must handle error */ },
}
```

**Time estimate:** 1-2 days

**Notes:**


---

## Phase 3: Advanced Concepts (Week 4)

### Chapter 10: Generic Types, Traits, and Lifetimes ⭐
- [x] Read chapter (read twice if needed!)
- [x] Do Exercise 6 below
- [x] Do Exercise 7 below

#### Exercise 6: Traits (Rust's Interfaces)
**Goal:** Understand polymorphism without inheritance

**Tasks:**

1. Create a new file `src/widgets.rs` and declare it in `main.rs` with `mod widgets;`

2. Define a `Touchable` trait with:
   - `on_touch(&mut self, x: f32, y: f32) -> bool` - handle touch, return true if handled
   - `get_bounds(&self) -> (f32, f32, f32, f32)` - return (x, y, width, height)
   - `contains_point(&self, x: f32, y: f32) -> bool` - default impl using get_bounds()

```rust
pub trait Touchable {
    fn on_touch(&mut self, x: f32, y: f32) -> bool;
    fn get_bounds(&self) -> (f32, f32, f32, f32);

    // Default implementation
    fn contains_point(&self, x: f32, y: f32) -> bool {
        let (bx, by, width, height) = self.get_bounds();
        x >= bx && x <= bx + width && y >= by && y <= by + height
    }
}
```

3. Create three structs that implement `Touchable`:
   - `Button { x: f32, y: f32, width: f32, height: f32, label: String, pressed: bool }`
   - `Checkbox { x: f32, y: f32, size: f32, checked: bool }`
   - `Slider { x: f32, y: f32, width: f32, height: f32, value: f32 }` (value 0.0-1.0)

4. Implement `Touchable` for each:
   - Button: `on_touch` toggles `pressed`, prints "Button '{label}' pressed!"
   - Checkbox: `on_touch` toggles `checked`, prints "Checkbox toggled: {checked}"
   - Slider: `on_touch` calculates value from x position, prints "Slider value: {value}"

5. In `main()`:
   - Create a `Vec<Box<dyn Touchable>>` containing one of each widget
   - Simulate touches by iterating and calling `on_touch(50.0, 50.0)` on each
   - Use `contains_point()` to check if touch is within bounds before handling

**Key concepts:**
- `trait` definition (like C++ interface)
- `impl Trait for Type`
- Default implementations
- Trait objects (`dyn Trait`)
- Trait bounds
- No inheritance hierarchy!

**C++ comparison:**
```cpp
// C++ - inheritance-based polymorphism
class Touchable {
public:
    virtual bool onTouch(float x, float y) = 0;
    virtual Bounds getBounds() = 0;
};

class Button : public Touchable { /* ... */ };
```

```rust
// Rust - composition over inheritance
trait Touchable {
    fn on_touch(&mut self, x: f32, y: f32) -> bool;
    fn get_bounds(&self) -> (f32, f32, f32, f32);
}

struct Button { /* ... */ }
impl Touchable for Button { /* ... */ }
// No inheritance! More flexible composition
```

**Time estimate:** 1-2 days

**Notes:**


---

#### Exercise 7: Generic Programming
**Goal:** Write reusable code with generics (like C++ templates)

**Tasks:**
Create a generic `ScrollList<T>` that can hold any type:

```rust
struct ScrollList<T> {
    items: Vec<T>,
    scroll_offset: usize,
    visible_count: usize,
}

impl<T> ScrollList<T> {
    fn new(visible_count: usize) -> Self { /* ... */ }
    fn add(&mut self, item: T) { /* ... */ }
    fn scroll_up(&mut self) { /* ... */ }
    fn scroll_down(&mut self) { /* ... */ }
    fn get_visible(&self) -> &[T] { /* ... */ }
}

// Add trait bounds for types that need Display
impl<T: std::fmt::Display> ScrollList<T> {
    fn display_visible(&self) { /* ... */ }
}
```

Make it work with:
- `ScrollList<MenuItem>`
- `ScrollList<String>`
- `ScrollList<i32>`

**Key concepts:**
- Generic type parameters
- Trait bounds (`T: Display`)
- `where` clauses for complex bounds
- Monomorphization (like C++ templates)

**C++ comparison:**
- Similar to C++ templates
- But: explicit trait bounds instead of duck typing
- Better error messages!
- Clear requirements up front

**Time estimate:** 1-2 days

**Notes:**


---

#### Exercise 8: Modules and Visibility
**Goal:** Organize code (replaces C header files)

**Tasks:**
Split your code into modules:

```
src/
├── main.rs              // Entry point, uses menu module
├── menu/
│   ├── mod.rs           // Module declaration, re-exports
│   ├── item.rs          // MenuItem struct and methods
│   ├── system.rs        // MenuSystem struct and methods
│   └── error.rs         // MenuError enum
```

Practice:
- `mod` declarations
- `use` statements
- `pub` for public APIs
- `pub(crate)` for internal APIs
- `super` and `self` paths

**Key concepts:**
- Module system (no .h files!)
- Privacy by default
- `pub` keyword
- Crate organization

**C comparison:**
```c
// C - separate header files
// menu.h
typedef struct MenuItem MenuItem;
MenuItem* menu_create();

// menu.c
#include "menu.h"
struct MenuItem { /* private */ };
```

```rust
// Rust - no header files!
// menu.rs
pub struct MenuItem { /* can be public or private */ }
impl MenuItem { /* ... */ }

// main.rs
mod menu;
use menu::MenuItem;
```

**Time estimate:** 1 day

**Notes:**


---

### Chapter 11: Testing
- [x] Read chapter
- [x] Add tests to previous exercises
- [x] Practice TDD workflow below

**Tasks:**
- Write unit tests with `#[test]`
- Write integration tests
- Run with `cargo test`

**Time estimate:** 2-3 hours

---

#### Test Driven Development (TDD) Practice
**Goal:** Learn the Red-Green-Refactor cycle

**The TDD Cycle:**
1. **Red** — Write a failing test first
2. **Green** — Write minimal code to make it pass
3. **Refactor** — Clean up while keeping tests green

**Practice Task:** Add a `count()` method to `ScrollList<T>` using TDD

```rust
// Step 1: RED - Write the test first (it won't compile yet)
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
        list.add(10);
        list.add(20);
        assert_eq!(list.count(), 2);
    }
}
```

```rust
// Step 2: GREEN - Write minimal code to pass
impl<T> ScrollList<T> {
    pub fn count(&self) -> usize {
        self.items.len()
    }
}
```

```rust
// Step 3: REFACTOR - Clean up if needed (this is already clean)
```

**More TDD Tasks:**
1. Add `is_empty(&self) -> bool` to ScrollList using TDD
2. Add `clear(&mut self)` to ScrollList using TDD
3. Add `get(&self, index: usize) -> Option<&T>` using TDD
4. Write tests for existing MenuSystem methods (find_item, remove_item, add_item)

**Key Testing Macros:**
| Macro | Purpose |
|-------|---------|
| `assert!(expr)` | Assert expression is true |
| `assert_eq!(a, b)` | Assert a equals b |
| `assert_ne!(a, b)` | Assert a not equals b |
| `#[should_panic]` | Test expects a panic |

**Running Tests:**
```bash
cargo test              # run all tests
cargo test count        # run tests with "count" in name
cargo test -- --nocapture  # show println! output
```

**Time estimate:** 2-3 hours

---

### Chapter 12: I/O Project - Building a grep Clone (Project 2/3)
- [x] Read chapter
- [x] Build the grep program
- [x] Refactor as suggested
- [x] Add tests

**Concepts:** File I/O, command-line args, error handling, testing

**Time estimate:** 1-2 days

---

## Phase 4: Advanced Features (Week 5)

### Chapter 13: Functional Language Features
- [x] Read chapter
- [x] Practice: closures
- [x] Practice: iterators
- [x] Do Exercise 13A below

**Time estimate:** 3-4 hours

---

#### Exercise 13A: Closures and Iterators
**Goal:** Refactor existing code using functional patterns

**Tasks:**

1. Add iterator methods to `ScrollList<T>`:
```rust
impl<T> ScrollList<T> {
    // Filter items with a closure
    pub fn filter<F>(&self, predicate: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool
    {
        self.items.iter().filter(|item| predicate(item)).collect()
    }

    // Map items with a closure
    pub fn map<F, U>(&self, f: F) -> Vec<U>
    where
        F: Fn(&T) -> U
    {
        self.items.iter().map(f).collect()
    }

    // Find first item matching predicate
    pub fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool
    {
        self.items.iter().find(|item| predicate(item))
    }
}
```

2. Use closures to filter MenuItems:
```rust
let enabled_items = menu_list.filter(|item| item.enabled);
let names: Vec<String> = menu_list.map(|item| item.name.clone());
```

3. Refactor `display_visible` to use `for_each`:
```rust
pub fn display_visible(&self) {
    self.get_visible()
        .iter()
        .for_each(|item| println!("{}", item));
}
```

4. Add a `sort_by` method using closures:
```rust
pub fn sort_by<F>(&mut self, compare: F)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering
{
    self.items.sort_by(compare);
}
```

**Key concepts:**
- `Fn`, `FnMut`, `FnOnce` traits
- Iterator adapters: `map`, `filter`, `find`, `fold`
- Collecting results
- Closure capture

**Time estimate:** 2-3 hours

---

### Chapter 14: More About Cargo
- [x] Read chapter
- [x] Set up workspace
- [x] Practice: profiles, dependencies

**Time estimate:** 1-2 hours

---

### Chapter 15: Smart Pointers
- [x] Read chapter
- [x] Practice: `Box<T>`
- [x] Practice: `Rc<T>` and `RefCell<T>`
- [x] Do Exercise 15A below

**Concepts:** Heap allocation, reference counting, interior mutability

**Time estimate:** 3-4 hours

---

#### Exercise 15A: Smart Pointers in Practice
**Goal:** Understand when and why to use smart pointers

**Tasks:**

1. **Box<T> — Recursive data structure:**
   Create a tree menu structure:
```rust
enum MenuNode {
    Item(MenuItem),
    Submenu {
        name: String,
        children: Vec<Box<MenuNode>>,  // Box needed for recursive type
    },
}

impl MenuNode {
    fn display(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        match self {
            MenuNode::Item(item) => println!("{}{}", indent, item.name),
            MenuNode::Submenu { name, children } => {
                println!("{}[{}]", indent, name);
                for child in children {
                    child.display(depth + 1);
                }
            }
        }
    }
}
```

2. **Rc<T> — Shared ownership:**
   Create widgets that share a theme:
```rust
use std::rc::Rc;

struct Theme {
    primary_color: String,
    font_size: u32,
}

struct ThemedButton {
    label: String,
    theme: Rc<Theme>,  // shared, not owned
}

struct ThemedSlider {
    value: f32,
    theme: Rc<Theme>,  // same theme, shared
}

// Usage:
let theme = Rc::new(Theme {
    primary_color: String::from("#007bff"),
    font_size: 14,
});

let btn = ThemedButton {
    label: String::from("OK"),
    theme: Rc::clone(&theme),
};
let slider = ThemedSlider {
    value: 0.5,
    theme: Rc::clone(&theme),
};
// Both share the same Theme without copying
```

3. **RefCell<T> — Interior mutability:**
   Create a click counter that can be modified through shared references:
```rust
use std::cell::RefCell;
use std::rc::Rc;

struct ClickTracker {
    counts: RefCell<HashMap<String, u32>>,
}

impl ClickTracker {
    fn new() -> Self {
        ClickTracker {
            counts: RefCell::new(HashMap::new()),
        }
    }

    fn record_click(&self, widget_name: &str) {
        // Can mutate even with &self!
        let mut counts = self.counts.borrow_mut();
        *counts.entry(widget_name.to_string()).or_insert(0) += 1;
    }

    fn get_clicks(&self, widget_name: &str) -> u32 {
        *self.counts.borrow().get(widget_name).unwrap_or(&0)
    }
}
```

4. **Rc<RefCell<T>> — Shared mutable state:**
   Multiple widgets updating shared state:
```rust
use std::rc::Rc;
use std::cell::RefCell;

struct AppState {
    score: i32,
    level: u32,
}

struct ScoreDisplay {
    state: Rc<RefCell<AppState>>,
}

struct ScoreButton {
    state: Rc<RefCell<AppState>>,
    points: i32,
}

impl ScoreButton {
    fn click(&self) {
        self.state.borrow_mut().score += self.points;
    }
}
```

**When to use what:**
| Type | Use when |
|------|----------|
| `Box<T>` | Recursive types, large data on heap, trait objects |
| `Rc<T>` | Multiple owners, single-threaded, read-only sharing |
| `RefCell<T>` | Interior mutability with runtime borrow checking |
| `Rc<RefCell<T>>` | Multiple owners + mutation (single-threaded) |

**Time estimate:** 3-4 hours

---

### Chapter 16: Fearless Concurrency
- [x] Read chapter
- [x] Practice: threads
- [x] Practice: channels
- [x] Practice: `Arc<Mutex<T>>`
- [ ] Do Exercise 16A below

**Time estimate:** 3-4 hours

---

#### Exercise 16A: Deadlocks and Mitigation
**Goal:** Understand deadlocks and learn prevention strategies

**File:** `deadlock_exercises.md`

**Tasks:**
1. **Create a deadlock** - Two threads locking resources in opposite order
2. **Fix with lock ordering** - Always acquire locks in same order (A→B)
3. **Use try_lock()** - Non-blocking attempt with backoff and retry
4. **Minimize lock scope** - Clone data, release lock, then do slow work
5. **Use channels** - Replace shared state with message passing

**Key concepts:**
- `Arc<Mutex<T>>` for shared state across threads
- `try_lock()` vs `lock()`
- `mpsc::channel()` for message passing
- Lock contention and performance

**When to use what:**
| Pattern | Use when |
|---------|----------|
| `Mutex<T>` | Single-threaded or simple cases |
| `Arc<Mutex<T>>` | Multi-threaded shared state |
| `RwLock<T>` | Many readers, few writers |
| Channels | Avoid shared state entirely |

**Time estimate:** 2-3 hours

---

### Chapter 17: Async and Await
- [ ] Read chapter
- [ ] Practice: async functions
- [ ] Practice: futures

**Time estimate:** 2-3 hours

---

### Chapter 18: Patterns and Matching
- [ ] Read chapter
- [ ] Practice: advanced patterns

**Time estimate:** 2-3 hours

---

### Chapter 19: Advanced Features
- [ ] Read chapter
- [ ] Unsafe Rust
- [ ] Advanced traits
- [ ] Advanced types
- [ ] Advanced functions
- [ ] Macros

**Time estimate:** 3-4 hours

---

### Chapter 21: Final Project - Multithreaded Web Server (Project 3/3)
- [ ] Read chapter
- [ ] Build the web server
- [ ] Add thread pool
- [ ] Test thoroughly

**Time estimate:** 2-3 days

---

## Phase 5: GUI Development (Weeks 6-7)

### Exercise 9: Basic 2D Graphics with a Library
**Goal:** Actually draw something on screen!

**Recommended library: `egui`** (immediate mode, beginner-friendly)

**Setup:**
```toml
# Cargo.toml
[dependencies]
eframe = "0.24"
egui = "0.24"
```

**Tasks:**
1. Create a window with `eframe`
2. Draw rectangles for menu items using `egui::Rect`
3. Respond to mouse clicks (simulating touch)
4. Draw a scrollable list using your `ScrollList<T>` from Exercise 7
5. Add visual feedback on click (highlight, color change)

**Alternative libraries:**
- `iced` - More structured, reactive (like React/Flutter)
- `slint` - Declarative UI (like QML)
- `druid` - Data-driven

**Key concepts:**
- External crates
- GUI event loops
- Rendering basics
- Input handling

**Time estimate:** 2-3 days

**Notes:**


---

### Exercise 10: Build Your Touch GUI 🎯 FINAL PROJECT
**Goal:** Combine everything into a complete application

**Requirements:**
Build a touch-based menu navigation app with:

1. **Multiple Menu Screens:**
   - Use your `UiElement` enum from Exercise 3
   - Main menu, settings menu, about screen
   - Each screen has different widgets

2. **Scrollable Lists:**
   - Use your generic `ScrollList<T>` from Exercise 7
   - Smooth scrolling (bonus: animation)

3. **Navigation:**
   - Touch/click to navigate between menus
   - Navigation stack for "back" button
   - Breadcrumb trail

4. **Visual Feedback:**
   - Highlight on touch
   - Button press animation
   - Smooth transitions

5. **State Management:**
   - Settings that persist
   - Save/load state to file (use Chapter 12 I/O skills)

**Architecture suggestions:**
```rust
// High-level structure
struct App {
    current_screen: Screen,
    navigation_stack: Vec<Screen>,
    settings: Settings,
}

enum Screen {
    MainMenu(MenuScreen),
    Settings(SettingsScreen),
    About(AboutScreen),
}

trait Screen {
    fn update(&mut self, ctx: &egui::Context);
    fn render(&self, ui: &mut egui::Ui);
}
```

**Bonus Challenges:**
- [ ] Smooth scrolling animation (use timer/easing)
- [ ] Navigation stack with back button
- [ ] Save/load menu state to JSON file
- [ ] Custom themes/styling (colors, fonts)
- [ ] Touch gesture support (swipe, pinch)
- [ ] Accessibility features (keyboard nav)

**Time estimate:** 1-2 weeks

**Notes:**


---

## Phase 6: Embedded Graphics (Optional - STM32 Touchscreen)

### Overview: C + Rust Bindings Approach
**Goal:** Build a touchscreen UI on STM32 using LVGL (C) with Rust bindings

**Why this approach?**
- LVGL is battle-tested for embedded displays
- Rich widget library out of the box
- Rust handles application logic safely
- C handles low-level graphics efficiently

---

### Step 1: Understand the Architecture
```
┌─────────────────────────────────────────┐
│           Your Rust Application         │
│  (Menu logic, state, ScrollList<T>)     │
├─────────────────────────────────────────┤
│         Rust FFI Bindings (unsafe)      │
│            lv_binding_rust              │
├─────────────────────────────────────────┤
│              LVGL (C Library)           │
│  (Widgets, rendering, touch handling)   │
├─────────────────────────────────────────┤
│         Display Driver (C/HAL)          │
│    (SPI/Parallel to LCD, touch IC)      │
├─────────────────────────────────────────┤
│              STM32 Hardware             │
└─────────────────────────────────────────┘
```

---

### Step 2: Prerequisites
- [ ] STM32 development board with display (e.g., STM32F429-Discovery, STM32H7)
- [ ] Working C toolchain for STM32 (STM32CubeIDE or arm-none-eabi-gcc)
- [ ] Rust embedded toolchain: `rustup target add thumbv7em-none-eabihf`
- [ ] Basic LVGL working in pure C first

---

### Step 3: Learn Rust FFI Basics
**Goal:** Understand how Rust calls C code

**Key concepts:**
```rust
// Declaring external C functions
extern "C" {
    fn lv_init();
    fn lv_label_create(parent: *mut lv_obj_t) -> *mut lv_obj_t;
}

// Calling C from Rust (unsafe!)
unsafe {
    lv_init();
    let label = lv_label_create(std::ptr::null_mut());
}
```

**Tasks:**
- [ ] Read Rustonomicon chapter on FFI
- [ ] Practice calling simple C functions from Rust
- [ ] Understand `unsafe`, raw pointers, `extern "C"`

**Resources:**
- Rustonomicon FFI: https://doc.rust-lang.org/nomicon/ffi.html
- bindgen (auto-generate bindings): https://rust-lang.github.io/rust-bindgen/

---

### Step 4: LVGL Basics in C
**Goal:** Get comfortable with LVGL before adding Rust

**Tasks:**
- [ ] Set up LVGL with your display driver
- [ ] Create a simple button that prints on click
- [ ] Create a label and update its text
- [ ] Handle touch input

**Example (C):**
```c
#include "lvgl.h"

void create_ui(void) {
    lv_obj_t *btn = lv_btn_create(lv_scr_act());
    lv_obj_set_size(btn, 120, 50);
    lv_obj_center(btn);

    lv_obj_t *label = lv_label_create(btn);
    lv_label_set_text(label, "Click Me");
    lv_obj_center(label);

    lv_obj_add_event_cb(btn, btn_event_cb, LV_EVENT_CLICKED, NULL);
}

void btn_event_cb(lv_event_t *e) {
    printf("Button clicked!\n");
}
```

**Resources:**
- LVGL docs: https://docs.lvgl.io/
- LVGL examples: https://github.com/lvgl/lvgl/tree/master/examples

---

### Step 5: Rust + LVGL Integration
**Goal:** Control LVGL from Rust

**Option A: Use existing bindings**
```toml
# Cargo.toml
[dependencies]
lvgl = "0.6"  # check for latest
```

**Option B: Generate bindings with bindgen**
```bash
cargo install bindgen-cli
bindgen lvgl.h -o bindings.rs
```

**Simple Rust + LVGL example:**
```rust
use lvgl;

fn main() {
    lvgl::init();

    let mut screen = lvgl::ActiveScreen::new();
    let mut button = lvgl::Button::new(&mut screen);
    button.set_size(120, 50);
    button.center();

    let mut label = lvgl::Label::new(&mut button);
    label.set_text("Rust + LVGL!");

    loop {
        lvgl::tick_inc(5);
        lvgl::task_handler();
    }
}
```

---

### Step 6: Build Your Embedded Touch Menu
**Goal:** Port your desktop menu concepts to embedded

**Tasks:**
- [ ] Implement ScrollList using LVGL's lv_list or lv_roller
- [ ] Implement Touchable trait wrapping LVGL widgets
- [ ] Create menu navigation with LVGL screens
- [ ] Handle touch events through Rust callbacks

**Architecture:**
```rust
// Your safe Rust layer
pub struct EmbeddedMenu {
    items: ScrollList<MenuItem>,
    lvgl_list: lvgl::List,  // wraps unsafe LVGL
}

impl EmbeddedMenu {
    pub fn add_item(&mut self, item: MenuItem) {
        self.items.add(item.clone());
        // Update LVGL widget (unsafe internally)
        self.lvgl_list.add_button(&item.name);
    }
}
```

---

### Hardware Resources
**Recommended dev boards:**
| Board | Display | Touch | Notes |
|-------|---------|-------|-------|
| STM32F429-Discovery | 2.4" 240x320 | Yes | Good starter |
| STM32H747-Discovery | 4" 480x800 | Yes | More powerful |
| STM32F469-Discovery | 4" 800x480 | Yes | Good resolution |

**Time estimate:** 2-4 weeks (after completing desktop GUI)

**Notes:**


---

## 📅 Recommended Schedule

### Weeks 1-2: Core Fundamentals
- Chapters 1-6
- Exercises 1-3
- Focus: Ownership, structs, enums

### Week 3: Collections & Errors
- Chapters 7-9
- Exercises 4-5
- Focus: Data structures, error handling

### Week 4: Advanced Concepts
- Chapters 10-12
- Exercises 6-8
- Focus: Traits, generics, organization

### Week 5: Advanced Features
- Chapters 13-21
- Focus: Functional features, concurrency, advanced topics

### Weeks 6-7: GUI Development
- Exercises 9-10
- Focus: Practical application

**Total Time:** 6-8 weeks at steady pace (2-3 hours daily)

---

## 🎯 Key Milestones

- [x] **Milestone 1:** Understand ownership (can explain to someone else)
- [ ] **Milestone 2:** Can write and use traits
- [ ] **Milestone 3:** Completed all 3 book projects
- [ ] **Milestone 4:** First GUI window displays
- [ ] **Milestone 5:** Complete touch GUI app working

---

## 📚 Resources

### Official
- **The Rust Book:** https://doc.rust-lang.org/book/
- **Rust by Example:** https://doc.rust-lang.org/rust-by-example/
- **Rustlings:** https://github.com/rust-lang/rustlings (interactive exercises)
- **Rust Standard Library:** https://doc.rust-lang.org/std/

### For C/C++ Programmers
- **Rust for C++ Programmers:** https://github.com/nrc/r4cppp
- **Rust vs C++ Cheat Sheet:** https://programming-idioms.org/cheatsheet/Rust/C++

### GUI-Specific
- **egui documentation:** https://docs.rs/egui/
- **egui examples:** https://github.com/emilk/egui/tree/master/examples
- **Are we GUI yet?:** https://areweguiyet.com/

### Community
- **r/rust:** https://reddit.com/r/rust
- **Rust Users Forum:** https://users.rust-lang.org/
- **Rust Discord:** https://discord.gg/rust-lang

---

## ❓ Questions & Challenges Log

Track specific questions or challenges here:

1. 
2. 
3. 

---

## 💡 Key Takeaways for C Programmers

### What's Different:
- **No manual memory management** - ownership system handles it
- **No null pointers** - use `Option<T>` instead
- **No undefined behavior** (mostly) - compiler catches issues
- **No header files** - module system
- **Explicit error handling** - `Result<T, E>`, no errno
- **Immutable by default** - opt into mutability with `mut`

### What's Similar:
- **Systems programming focus** - low-level control
- **Zero-cost abstractions** - performance like C
- **Static typing** - compile-time checks
- **Procedural core** - not forced into OOP

### Mental Model Shifts:
1. **Ownership:** Every value has one owner, think about who owns data
2. **Borrowing:** References are temporary loans, not just pointers
3. **Lifetimes:** Explicit tracking of how long references are valid
4. **Match exhaustiveness:** Compiler ensures all cases handled
5. **Immutability:** Default is immutable, makes code easier to reason about

---

## 🦀 Progress Notes

### Overall Reflections:


### Biggest Challenges:


### Most Useful Concepts:


### Things I'd Do Differently:


---

**Remember:** 
- Rust's learning curve is steep initially (especially ownership)
- The compiler is your teacher - read error messages carefully!
- Don't fight the borrow checker - learn to work with it
- After ~2-3 weeks, it "clicks" and becomes much easier
- The safety guarantees are worth the initial investment

**Good luck on your Rust journey! 🦀**
