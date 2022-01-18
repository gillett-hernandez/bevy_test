use crate::config::Config;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    InGame,
    Paused,
    // GameOver,
}

pub struct Game {
    pub config: Config,
}
