use agent_preflight::app::interactive::MENU_SELECTIONS;

#[test]
fn menu_options_match_available_commands() {
    assert_eq!(MENU_SELECTIONS.len(), 4, "Expected 4 main menu options");
    assert!(MENU_SELECTIONS[0].contains("Scan"));
    assert!(MENU_SELECTIONS[1].contains("View Report"));
    assert!(MENU_SELECTIONS[2].contains("Approve"));
    assert!(MENU_SELECTIONS[3].contains("Verify"));

    // Every option starts with an arrow marker
    for option in MENU_SELECTIONS {
        assert!(
            option.starts_with('\u{25ba}'),
            "Menu option must start with arrow marker: {option}"
        );
    }
}
