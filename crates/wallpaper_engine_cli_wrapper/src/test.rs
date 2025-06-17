use crate::{Wallpaper, WallpaperEngine, WallpaperKind};

#[cfg(test)]
#[test]
fn test_open_workshop() {
    let mut engine = WallpaperEngine::new().expect("Failed to initialize WallpaperEngine");
    let result = engine.open_workshop_page_for_wallpaper("1836324036");
    assert!(
        result.is_ok(),
        "Failed to open workshop: {:?}",
        result.err()
    );
}

#[test]
fn test_set_wallpaper() {
    let mut engine = WallpaperEngine::new().expect("Failed to initialize WallpaperEngine");
    engine
        .set_wallpaper(
            &Wallpaper {
                id: "1836324036".to_string(),
                kind: WallpaperKind::Workshop,
            },
            0,
        )
        .unwrap();
}
