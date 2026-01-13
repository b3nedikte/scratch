use std::cell::RefCell;
use std::rc::Rc;

pub struct AppState {
    score: i32,
    level: u32,
}

pub struct ScoreDisplay {
    state: Rc<RefCell<AppState>>,
}

pub struct ScoreButton {
    state: Rc<RefCell<AppState>>,
    points: i32,
}

impl AppState {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(AppState { score: 0, level: 1 }))
    }
}

impl ScoreButton {
    pub fn new(state: Rc<RefCell<AppState>>, points: i32) -> Self {
        ScoreButton { state, points }
    }

    pub fn click(&self) {
        self.state.borrow_mut().score += self.points;
    }
}

impl ScoreDisplay {
    pub fn new(state: Rc<RefCell<AppState>>) -> Self {
        ScoreDisplay { state }
    }

    pub fn show(&self) {
        let state = self.state.borrow();
        println!("Score: {} | Level: {}", state.score, state.level);
    }
}
