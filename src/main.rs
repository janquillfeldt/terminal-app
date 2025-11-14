#[cfg(feature = "gui")]
mod gui;
mod config;
mod ui;

// Initialize i18n support
rust_i18n::i18n!("locales");

use ui::UI; // Immer verfügbar, auch wenn im GUI-Build nur als Fallback genutzt

#[cfg(feature = "gui")]
fn main() {
    // '--tui' erzwingt Terminal-Modus selbst wenn GUI-Feature aktiv ist
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--tui") {
        run_tui();
        return;
    }

    if let Err(e) = gui::run_gui() {
        eprintln!("GUI konnte nicht gestartet werden: {}\nFalle zurück auf TUI...", e);
        run_tui();
    }
}

#[cfg(not(feature = "gui"))]
fn main() { run_tui(); }

fn run_tui() {
    let mut app = UI::new();
    if let Err(e) = app.run() {
        eprintln!("Fehler beim Starten der Anwendung: {}", e);
        std::process::exit(1);
    }
}