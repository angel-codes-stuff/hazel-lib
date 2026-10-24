pub struct GameManifest {
    pub id: i32,
    pub title: String,
    pub version: String,
    pub install_path: String,
    pub executable: String,
    pub arguments: Vec<String>,
    pub installed: bool,
}

impl GameManifest {
    pub fn new(
        id: i32,
        title: String,
        version: String,
        install_path: String,
        executable: String,
        arguments: Vec<String>,
        installed: bool,
    ) -> GameManifest {
        GameManifest {
            id,
            title,
            version,
            install_path,
            executable,
            arguments,
            installed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let game_manifest = GameManifest::new(
            123456789,
            "DOOM".to_string(),
            "1.0.0".to_string(),
            "/home/user/.hazel/games/DOOM/".to_string(),
            "DOOM".to_string(),
            vec!["-file iwad".to_string(), "freedoom1.wad".to_string()],
            true,
        );
        assert_eq!(game_manifest.id, 123456789);
        assert_eq!(game_manifest.title, "DOOM".to_string());
        assert_eq!(game_manifest.version, "1.0.0".to_string());
        assert_eq!(
            game_manifest.install_path,
            "/home/user/.hazel/games/DOOM/".to_string()
        );
        assert_eq!(game_manifest.executable, "DOOM".to_string());
        assert_eq!(
            game_manifest.arguments,
            vec!["-file iwad".to_string(), "freedoom1.wad".to_string()]
        );
        assert_eq!(game_manifest.installed, true);
    }
}
