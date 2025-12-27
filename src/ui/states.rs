enum State {
    Game,
    SettingsMenu(SettingsState),
    MainMenu,
}

enum SettingsState {
    General,
    Controls,
    Graphics,
    Audio,
}
