use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    Result,
};
use std::io::{stdout, Write};

pub struct UI {
    selected_menu: usize,
    running: bool,
}

impl UI {
    pub fn new() -> Self {
        UI {
            selected_menu: 0,
            running: true,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

        while self.running {
            self.render()?;
            self.handle_input()?;
        }

        execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn render(&self) -> Result<()> {
        let mut stdout = stdout();
        
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
        )?;

        // Header
        self.print_header(&mut stdout)?;
        
        // Menu
        self.print_menu(&mut stdout)?;
        
        // Footer
        self.print_footer(&mut stdout)?;

        stdout.flush()?;
        Ok(())
    }

    fn print_header(&self, stdout: &mut impl Write) -> Result<()> {
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            SetBackgroundColor(Color::Blue),
            SetForegroundColor(Color::White),
        )?;
        
        let header = "═══════════════════════════════════════════════════════════════════════════════";
    writeln!(stdout, "{}", header)?;
    writeln!(stdout, "                                 🖥️  TermiX                                  ")?;
        writeln!(stdout, "{}", header)?;
        execute!(stdout, ResetColor)?;
        writeln!(stdout)?;
        
        Ok(())
    }

    fn print_menu(&self, stdout: &mut impl Write) -> Result<()> {
        let menu_items = vec![
            "1. Terminal öffnen",
            "2. Einstellungen",
            "3. Über diese Anwendung",
            "4. Beenden",
        ];

        execute!(
            stdout,
            cursor::MoveTo(5, 5),
            SetForegroundColor(Color::Cyan),
        )?;
        writeln!(stdout, "╔═══════════════════════════════════════╗")?;
        writeln!(stdout, "     ║          HAUPTMENÜ                    ║")?;
        writeln!(stdout, "     ╠═══════════════════════════════════════╣")?;
        execute!(stdout, ResetColor)?;

        for (index, item) in menu_items.iter().enumerate() {
            if index == self.selected_menu {
                execute!(
                    stdout,
                    cursor::MoveTo(5, 8 + index as u16),
                    SetBackgroundColor(Color::Green),
                    SetForegroundColor(Color::Black),
                )?;
                writeln!(stdout, "     ║  ▶ {}                  ║", item)?;
                execute!(stdout, ResetColor)?;
            } else {
                execute!(
                    stdout,
                    cursor::MoveTo(5, 8 + index as u16),
                    SetForegroundColor(Color::White),
                )?;
                writeln!(stdout, "     ║    {}                  ║", item)?;
            }
        }

        execute!(
            stdout,
            cursor::MoveTo(5, 12),
            SetForegroundColor(Color::Cyan),
        )?;
        writeln!(stdout, "     ╚═══════════════════════════════════════╝")?;
        execute!(stdout, ResetColor)?;

        Ok(())
    }

    fn print_footer(&self, stdout: &mut impl Write) -> Result<()> {
        execute!(
            stdout,
            cursor::MoveTo(0, 16),
            SetForegroundColor(Color::DarkGrey),
        )?;
        writeln!(stdout)?;
        writeln!(stdout, "  ┌─────────────────────────────────────────────────────────────────────────┐")?;
        writeln!(stdout, "  │ ↑/↓: Navigation  │  Enter: Auswählen  │  ESC/Q: Beenden              │")?;
        writeln!(stdout, "  └─────────────────────────────────────────────────────────────────────────┘")?;
        execute!(stdout, ResetColor)?;

        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {
                        if self.selected_menu > 0 {
                            self.selected_menu -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected_menu < 3 {
                            self.selected_menu += 1;
                        }
                    }
                    KeyCode::Enter => {
                        self.execute_menu_action()?;
                    }
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.running = false;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn execute_menu_action(&mut self) -> Result<()> {
        let mut stdout = stdout();
        
        match self.selected_menu {
            0 => {
                // Terminal öffnen
                execute!(
                    stdout,
                    Clear(ClearType::All),
                    cursor::MoveTo(0, 0),
                    SetForegroundColor(Color::Green),
                )?;
                writeln!(stdout, "\n  Terminal-Modus wird gestartet...")?;
                writeln!(stdout, "  (Diese Funktion wird noch implementiert)")?;
                execute!(stdout, ResetColor)?;
                writeln!(stdout, "\n  Drücke eine beliebige Taste zum Fortfahren...")?;
                event::read()?;
            }
            1 => {
                // Einstellungen
                execute!(
                    stdout,
                    Clear(ClearType::All),
                    cursor::MoveTo(0, 0),
                    SetForegroundColor(Color::Yellow),
                )?;
                writeln!(stdout, "\n  ⚙️  EINSTELLUNGEN")?;
                writeln!(stdout, "  ═══════════════════════════════════")?;
                execute!(stdout, ResetColor)?;
                writeln!(stdout, "\n  - Farbschema: Standard")?;
                writeln!(stdout, "  - Schriftgröße: Mittel")?;
                writeln!(stdout, "  - Plattform: {}", std::env::consts::OS)?;
                writeln!(stdout, "\n  Drücke eine beliebige Taste zum Fortfahren...")?;
                event::read()?;
            }
            2 => {
                // Über
                execute!(
                    stdout,
                    Clear(ClearType::All),
                    cursor::MoveTo(0, 0),
                    SetForegroundColor(Color::Cyan),
                )?;
                writeln!(stdout, "\n  ℹ️  ÜBER DIESE ANWENDUNG")?;
                writeln!(stdout, "  ═══════════════════════════════════")?;
                execute!(stdout, ResetColor)?;
                writeln!(stdout, "\n  TermiX v0.1.0")?;
                writeln!(stdout, "  Läuft auf: Linux & Windows")?;
                writeln!(stdout, "  Entwickelt mit Rust & Crossterm")?;
                writeln!(stdout, "\n  Drücke eine beliebige Taste zum Fortfahren...")?;
                event::read()?;
            }
            3 => {
                // Beenden
                self.running = false;
            }
            _ => {}
        }
        Ok(())
    }
}