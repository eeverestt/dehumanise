enum State {
    Game,
    SettingsMenu(SettingsState),
    MainMenu,
    CreditsMenu,
}

enum SettingsState {
    General,
    Controls,
    Video,
    Audio,
    Saves,
}
