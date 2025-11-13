use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    // Theme and scaling
    pub theme: String,          // e.g. "Dark", "Light", ...
    pub font_scale: f32,        // 0.75 ..= 2.0

    // Colors
    pub terminal_text_color: Rgba,
    pub markdown_text_color: Rgba,
    pub ssh_text_color: Rgba,
    pub cursor_color: Rgba,

    // Cursor
    pub cursor_shape: String,   // e.g. "Block", "Underline", ...
    pub cursor_blinking: bool,

    // Fonts
    pub font_mode: String,      // "Default" | "MonospaceEverywhere" | "Custom"
    pub custom_font_path: Option<String>,

    // UI state
    pub sidebar_collapsed: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "Dark".into(),
            font_scale: 1.0,
            terminal_text_color: Rgba { r: 220, g: 220, b: 220, a: 255 },
            markdown_text_color: Rgba { r: 220, g: 220, b: 220, a: 255 },
            ssh_text_color: Rgba { r: 200, g: 220, b: 255, a: 255 },
            cursor_color: Rgba { r: 0, g: 255, b: 0, a: 255 },
            cursor_shape: "Block".into(),
            cursor_blinking: false,
            font_mode: "Default".into(),
            custom_font_path: None,
            sidebar_collapsed: false,
        }
    }
}

pub fn settings_path() -> PathBuf {
    // Prefer XDG config dir on Unix, APPDATA on Windows, ~/Library/Application Support on macOS
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            let p = Path::new(&appdata).join("termix").join("settings.toml");
            return p;
        }
        // Fallback to current dir
        return PathBuf::from("settings.toml");
    }

    #[cfg(target_os = "macos")]
    {
        let base = dirs_home().unwrap_or_else(|| PathBuf::from("."));
        return base
            .join("Library")
            .join("Application Support")
            .join("termix")
            .join("settings.toml");
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            return Path::new(&xdg).join("termix").join("settings.toml");
        }
        if let Some(home) = dirs_home() {
            return home.join(".config").join("termix").join("settings.toml");
        }
        // Fallback
        return PathBuf::from("settings.toml");
    }
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

pub fn load_settings() -> AppSettings {
    let path = settings_path();
    if let Ok(content) = fs::read_to_string(&path) {
        match toml::from_str::<AppSettings>(&content) {
            Ok(s) => s,
            Err(_) => AppSettings::default(),
        }
    } else {
        AppSettings::default()
    }
}

pub fn save_settings(settings: &AppSettings) {
    let path = settings_path();
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    if let Ok(content) = toml::to_string_pretty(settings) {
        let _ = fs::write(path, content);
    }
}
