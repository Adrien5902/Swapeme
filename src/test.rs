use crate::theme::Theme;

#[test]
fn test_theme() {
    Theme::read_file("resources/test/test.swapeme.json")
        .unwrap()
        .apply()
        .unwrap();
}
