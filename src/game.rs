#[derive(Debug)]
enum GameState {
    GameActive,
    GameMenu,
    GameWin,
}

#[derive(Debug)]
pub struct Game {
    state: GameState,
    keys: [bool; 1024],
    width: u32,
    height: u32,
}

impl Game {
    fn new(width: u32, height: u32) -> Self {
        Self {
            state: GameState::GameActive,
            width,
            height,
            keys: [false; 1024],
        }
    }

    fn init() {}

    fn process_input() {}

    fn update() {}

    fn render() {}
}
