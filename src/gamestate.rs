#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    InHanger,
    InGame,
    Paused,
    // GameOver,
}
