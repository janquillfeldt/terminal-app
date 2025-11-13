#[cfg(feature = "gui")]
use eframe::{egui, App, Frame, CreationContext};
#[cfg(feature = "gui")]
use std::{
    io::{Read, Write},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};
#[cfg(feature = "gui")]
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem, MasterPty};
#[cfg(feature = "gui")]
use vt100::Parser as VtParser;
#[cfg(feature = "gui")]
use pulldown_cmark::{Parser as MdParser, Event, Tag, HeadingLevel};
use crate::config::{AppSettings, Rgba, load_settings, save_settings};

#[cfg(feature = "gui")]
#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Dark,
    Light,
    Dracula,
    Monokai,
    SolarizedDark,
    Nord,
    GruvboxDark,
}

#[cfg(feature = "gui")]
impl Theme {
    fn name(&self) -> &str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Dracula => "Dracula",
            Theme::Monokai => "Monokai",
            Theme::SolarizedDark => "Solarized Dark",
            Theme::Nord => "Nord",
            Theme::GruvboxDark => "Gruvbox Dark",
        }
    }
    fn from_name(name: &str) -> Self {
        match name {
            "Light" => Theme::Light,
            "Dracula" => Theme::Dracula,
            "Monokai" => Theme::Monokai,
            "Solarized Dark" => Theme::SolarizedDark,
            "Nord" => Theme::Nord,
            "Gruvbox Dark" => Theme::GruvboxDark,
            _ => Theme::Dark,
        }
    }
    
    fn all() -> Vec<Theme> {
        vec![
            Theme::Dark,
            Theme::Light,
            Theme::Dracula,
            Theme::Monokai,
            Theme::SolarizedDark,
            Theme::Nord,
            Theme::GruvboxDark,
        ]
    }
    
    fn apply(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        
        match self {
            Theme::Dark => {
                visuals = egui::Visuals::dark();
            }
            Theme::Light => {
                visuals = egui::Visuals::light();
            }
            Theme::Dracula => {
                visuals.window_fill = egui::Color32::from_rgb(40, 42, 54);
                visuals.panel_fill = egui::Color32::from_rgb(40, 42, 54);
                visuals.extreme_bg_color = egui::Color32::from_rgb(68, 71, 90);
                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(68, 71, 90);
                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(189, 147, 249);
            }
            Theme::Monokai => {
                visuals.window_fill = egui::Color32::from_rgb(39, 40, 34);
                visuals.panel_fill = egui::Color32::from_rgb(39, 40, 34);
                visuals.extreme_bg_color = egui::Color32::from_rgb(73, 72, 62);
                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(73, 72, 62);
                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(249, 38, 114);
            }
            Theme::SolarizedDark => {
                visuals.window_fill = egui::Color32::from_rgb(0, 43, 54);
                visuals.panel_fill = egui::Color32::from_rgb(0, 43, 54);
                visuals.extreme_bg_color = egui::Color32::from_rgb(7, 54, 66);
                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(7, 54, 66);
                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(38, 139, 210);
            }
            Theme::Nord => {
                visuals.window_fill = egui::Color32::from_rgb(46, 52, 64);
                visuals.panel_fill = egui::Color32::from_rgb(46, 52, 64);
                visuals.extreme_bg_color = egui::Color32::from_rgb(59, 66, 82);
                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(59, 66, 82);
                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(136, 192, 208);
            }
            Theme::GruvboxDark => {
                visuals.window_fill = egui::Color32::from_rgb(40, 40, 40);
                visuals.panel_fill = egui::Color32::from_rgb(40, 40, 40);
                visuals.extreme_bg_color = egui::Color32::from_rgb(60, 56, 54);
                visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(60, 56, 54);
                visuals.widgets.active.bg_fill = egui::Color32::from_rgb(251, 73, 52);
            }
        }
        
        ctx.set_visuals(visuals);
    }
}

#[cfg(feature = "gui")]
#[derive(Clone)]
enum FontMode {
    Default,
    MonospaceEverywhere,
    Custom(String), // name or info
}

#[cfg(feature = "gui")]
impl FontMode {
    fn id(&self) -> &'static str {
        match self {
            FontMode::Default => "Default",
            FontMode::MonospaceEverywhere => "MonospaceEverywhere",
            FontMode::Custom(_) => "Custom",
        }
    }
    fn from_id(id: &str, info: Option<String>) -> Self {
        match id {
            "MonospaceEverywhere" => FontMode::MonospaceEverywhere,
            "Custom" => FontMode::Custom(info.unwrap_or_default()),
            _ => FontMode::Default,
        }
    }
}

#[cfg(feature = "gui")]
#[derive(Clone, Copy, PartialEq)]
enum CursorShape {
    Block,          // █
    Underline,      // _
    VerticalBar,    // |
    DoubleUnderscore, // ‗ (thick underline)
    Box,            // ▯ (hollow block)
    Cross,          // ╳
}

#[cfg(feature = "gui")]
impl CursorShape {
    fn id(&self) -> &'static str {
        match self {
            CursorShape::Block => "Block",
            CursorShape::Underline => "Underline",
            CursorShape::VerticalBar => "VerticalBar",
            CursorShape::DoubleUnderscore => "DoubleUnderscore",
            CursorShape::Box => "Box",
            CursorShape::Cross => "Cross",
        }
    }
    fn from_id(id: &str) -> Self {
        match id {
            "Underline" => CursorShape::Underline,
            "VerticalBar" => CursorShape::VerticalBar,
            "DoubleUnderscore" => CursorShape::DoubleUnderscore,
            "Box" => CursorShape::Box,
            "Cross" => CursorShape::Cross,
            _ => CursorShape::Block,
        }
    }
    fn name(&self) -> &str {
        match self {
            CursorShape::Block => "Block █",
            CursorShape::Underline => "Unterstrich _",
            CursorShape::VerticalBar => "Strich |",
            CursorShape::DoubleUnderscore => "Doppelunterstrich ‗",
            CursorShape::Box => "Kasten ▯",
            CursorShape::Cross => "Kreuz ╳",
        }
    }
    
    fn all() -> Vec<CursorShape> {
        vec![
            CursorShape::Block,
            CursorShape::Underline,
            CursorShape::VerticalBar,
            CursorShape::DoubleUnderscore,
            CursorShape::Box,
            CursorShape::Cross,
        ]
    }
    
    fn render(&self, c: char) -> String {
        match self {
            CursorShape::Block => format!("{}", if c == ' ' { '█' } else { c }),
            CursorShape::Underline => "_".to_string(),
            CursorShape::VerticalBar => "|".to_string(),
            CursorShape::DoubleUnderscore => "‗".to_string(),
            CursorShape::Box => "▯".to_string(),
            CursorShape::Cross => "╳".to_string(),
        }
    }
}

#[cfg(feature = "gui")]
pub struct GuiApp {
    selected: usize,
    // Multiple terminals with tabs
    terminals: Vec<TerminalTab>,
    active_terminal_tab: usize,
    // Multiple markdown editors with tabs
    markdown_editors: Vec<MarkdownTab>,
    active_markdown_tab: usize,
    // SSH connections
    ssh_manager: SshManager,
    // Settings state
    current_theme: Theme,
    font_scale: f32,
    // Customization
    terminal_text_color: egui::Color32,
    markdown_text_color: egui::Color32,
    ssh_text_color: egui::Color32,
    cursor_color: egui::Color32,
    cursor_shape: CursorShape,
    cursor_blinking: bool,
    font_mode: FontMode,
    custom_font_info: Option<String>,
    // Sidebar state
    sidebar_collapsed: bool,
    // Rename dialogs
    terminal_rename_dialog: Option<(usize, String)>, // (tab_index, new_name)
    markdown_rename_dialog: Option<(usize, String)>, // (tab_index, new_name)
    // SSH connection request (from SshManager UI)
    pending_ssh_connection: Option<SshConnection>,
    // SSH password prompt
    ssh_password_prompt: Option<(SshConnection, String)>, // (connection, password_input)
}

#[cfg(feature = "gui")]
struct TerminalTab {
    name: String,
    terminal: TerminalView,
}

#[cfg(feature = "gui")]
struct MarkdownTab {
    name: String,
    editor: MarkdownEditor,
}

#[cfg(feature = "gui")]
impl Default for GuiApp {
    fn default() -> Self {
        let mut terminals = Vec::new();
        if let Ok(term) = TerminalView::new() {
            terminals.push(TerminalTab {
                name: "Terminal 1".to_string(),
                terminal: term,
            });
        }
        
        let mut markdown_editors = Vec::new();
        markdown_editors.push(MarkdownTab {
            name: "Dokument 1".to_string(),
            editor: MarkdownEditor::default(),
        });
        
        Self {
            selected: 0,
            terminals,
            active_terminal_tab: 0,
            markdown_editors,
            active_markdown_tab: 0,
            ssh_manager: SshManager::load_or_default(),
            current_theme: Theme::Dark,
            font_scale: 1.0,
            terminal_text_color: egui::Color32::from_rgb(220, 220, 220),
            markdown_text_color: egui::Color32::from_rgb(220, 220, 220),
            ssh_text_color: egui::Color32::from_rgb(200, 220, 255),
            cursor_color: egui::Color32::from_rgb(0, 255, 0),
            cursor_shape: CursorShape::Block,
            cursor_blinking: false,
            font_mode: FontMode::Default,
            custom_font_info: None,
            sidebar_collapsed: false,
            terminal_rename_dialog: None,
            markdown_rename_dialog: None,
            pending_ssh_connection: None,
            ssh_password_prompt: None,
        }
    }
}

#[cfg(feature = "gui")]
impl GuiApp {
    // TerminalView handles PTY IO
    fn color_to_rgba(c: egui::Color32) -> Rgba {
        let [r, g, b, a] = c.to_srgba_unmultiplied();
        Rgba { r, g, b, a }
    }

    fn color_from_rgba(rgba: Rgba) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(rgba.r, rgba.g, rgba.b, rgba.a)
    }

    fn apply_settings(&mut self, settings: &AppSettings, ctx: &egui::Context) {
        // Theme
        self.current_theme = Theme::from_name(&settings.theme);
        self.current_theme.apply(ctx);
        // Font scale
        self.font_scale = settings.font_scale.clamp(0.5, 3.0);
        ctx.set_pixels_per_point(self.font_scale);
        // Colors
        self.terminal_text_color = Self::color_from_rgba(settings.terminal_text_color);
        self.markdown_text_color = Self::color_from_rgba(settings.markdown_text_color);
        self.ssh_text_color = Self::color_from_rgba(settings.ssh_text_color);
        self.cursor_color = Self::color_from_rgba(settings.cursor_color);
        // Cursor
        self.cursor_shape = CursorShape::from_id(&settings.cursor_shape);
        self.cursor_blinking = settings.cursor_blinking;
        // Font mode
        self.font_mode = FontMode::from_id(&settings.font_mode, settings.custom_font_path.clone());
        match &self.font_mode {
            FontMode::Default => {
                let defs = egui::FontDefinitions::default();
                ctx.set_fonts(defs);
                self.custom_font_info = None;
            }
            FontMode::MonospaceEverywhere => {
                let mut defs = egui::FontDefinitions::default();
                if let Some(mono) = defs.families.get(&egui::FontFamily::Monospace).cloned() {
                    defs.families.insert(egui::FontFamily::Proportional, mono);
                }
                ctx.set_fonts(defs);
                self.custom_font_info = None;
            }
            FontMode::Custom(name_or_path) => {
                // Try to (re)load from path if available
                if let Some(path) = settings.custom_font_path.as_ref() {
                    if let Ok(bytes) = std::fs::read(path) {
                        let mut defs = egui::FontDefinitions::default();
                        defs.font_data.insert("user".into(), egui::FontData::from_owned(bytes));
                        defs.families.insert(egui::FontFamily::Proportional, vec!["user".to_string()]);
                        defs.families.insert(egui::FontFamily::Monospace, vec!["user".to_string()]);
                        ctx.set_fonts(defs);
                        self.custom_font_info = Some(path.clone());
                        self.font_mode = FontMode::Custom("user".into());
                    } else {
                        // Fallback to default fonts if file missing
                        let defs = egui::FontDefinitions::default();
                        ctx.set_fonts(defs);
                        self.custom_font_info = None;
                        self.font_mode = FontMode::Default;
                    }
                } else {
                    // Keep current fonts as-is; ensure flag stays consistent
                    let _ = name_or_path;
                }
            }
        }

        // Sidebar state
        self.sidebar_collapsed = settings.sidebar_collapsed;

        // Propagate to existing terminals
        for t in &mut self.terminals {
            t.terminal.text_color = self.terminal_text_color;
            t.terminal.cursor_color = self.cursor_color;
            t.terminal.cursor_shape = self.cursor_shape;
            t.terminal.cursor_blinking = self.cursor_blinking;
        }
    }

    fn to_settings(&self) -> AppSettings {
        AppSettings {
            theme: self.current_theme.name().to_string(),
            font_scale: self.font_scale,
            terminal_text_color: Self::color_to_rgba(self.terminal_text_color),
            markdown_text_color: Self::color_to_rgba(self.markdown_text_color),
            ssh_text_color: Self::color_to_rgba(self.ssh_text_color),
            cursor_color: Self::color_to_rgba(self.cursor_color),
            cursor_shape: self.cursor_shape.id().into(),
            cursor_blinking: self.cursor_blinking,
            font_mode: self.font_mode.id().into(),
            custom_font_path: self.custom_font_info.clone(),
            sidebar_collapsed: self.sidebar_collapsed,
        }
    }

    fn save_settings(&self) {
        save_settings(&self.to_settings());
    }

    pub fn new(cc: &CreationContext<'_>) -> Self {
        let mut app = GuiApp::default();
        let settings = load_settings();
        app.apply_settings(&settings, &cc.egui_ctx);
        app
    }
}

#[cfg(feature = "gui")]
impl App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Apply selected theme
        self.current_theme.apply(ctx);
        ctx.set_pixels_per_point(self.font_scale);

        // Apply global font mode
        match &self.font_mode {
            FontMode::Default => {
                // Reset to default fonts once per frame (cheap)
                let defs = egui::FontDefinitions::default();
                ctx.set_fonts(defs);
            }
            FontMode::MonospaceEverywhere => {
                let mut defs = egui::FontDefinitions::default();
                if let Some(mono) = defs.families.get(&egui::FontFamily::Monospace).cloned() {
                    defs.families.insert(egui::FontFamily::Proportional, mono);
                }
                ctx.set_fonts(defs);
            }
            FontMode::Custom(_) => {
                if let Some(info) = &self.custom_font_info {
                    // Keep previously loaded custom font active; nothing to do here.
                    // If user cleared custom font, font_mode will be changed back to Default.
                    let _ = info; // silence warning
                }
            }
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("TermiX");
                ui.label(egui::RichText::new(" • GUI").color(egui::Color32::LIGHT_BLUE));
                ui.separator();
                ui.label("Modus:");
                ui.label(match self.selected {0=>"Terminal",1=>"SSH",2=>"Markdown",3=>"Einstellungen",4=>"Über",5=>"Beenden", _=>"?"});
            });
        });

        let sidebar_width = if self.sidebar_collapsed { 60.0 } else { 220.0 };
        
        egui::SidePanel::left("menu")
            .resizable(false)
            .exact_width(sidebar_width)  // Use exact_width instead of default_width
            .show(ctx, |ui| {
            // Toggle button at top
            ui.horizontal(|ui| {
                let toggle_icon = if self.sidebar_collapsed { "☰" } else { "◀" };
                if ui.button(egui::RichText::new(toggle_icon).size(20.0)).clicked() {
                    self.sidebar_collapsed = !self.sidebar_collapsed;
                    self.save_settings();
                }
                if !self.sidebar_collapsed {
                    ui.heading("Hauptmenü");
                }
            });
            ui.separator();
            
            // Menu items with icons (top items)
            let top_items = [
                ("💻", "Terminal", egui::Color32::from_rgb(0, 200, 120), 0),
                ("🔌", "SSH", egui::Color32::from_rgb(100, 150, 255), 1),
                ("📝", "Markdown", egui::Color32::from_rgb(255, 150, 50), 2),
                ("⚙", "Einstellungen", egui::Color32::YELLOW, 3),
            ];
            
            for (icon, label, color, idx) in top_items.iter() {
                let button_text = if self.sidebar_collapsed {
                    egui::RichText::new(*icon).size(24.0).color(*color)
                } else {
                    egui::RichText::new(format!("{} {}", icon, label)).color(*color)
                };
                
                let button = egui::Button::new(button_text);
                let button_size = if self.sidebar_collapsed {
                    [50.0, 40.0]
                } else {
                    [200.0, 35.0]
                };
                
                if ui.add_sized(button_size, button).clicked() {
                    self.selected = *idx;
                }
                
                if self.selected == *idx && !self.sidebar_collapsed {
                    ui.colored_label(egui::Color32::from_gray(140), "◀ aktiv");
                }
                
                ui.separator();
            }
            
            // Bottom items (fixed at bottom)
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                // Collapse hint
                if !self.sidebar_collapsed {
                    ui.add_space(5.0);
                    ui.colored_label(egui::Color32::GRAY, "◀ Klicke zum Einklappen");
                    ui.add_space(10.0);
                }
                
                // Beenden button
                let beenden_text = if self.sidebar_collapsed {
                    egui::RichText::new("🚪").size(24.0).color(egui::Color32::LIGHT_RED)
                } else {
                    egui::RichText::new("🚪 Beenden").color(egui::Color32::LIGHT_RED)
                };
                let beenden_button = egui::Button::new(beenden_text);
                let beenden_size = if self.sidebar_collapsed {
                    [50.0, 40.0]
                } else {
                    [200.0, 35.0]
                };
                
                ui.separator();
                if ui.add_sized(beenden_size, beenden_button).clicked() {
                    self.selected = 5;
                }
                if self.selected == 5 && !self.sidebar_collapsed {
                    ui.colored_label(egui::Color32::from_gray(140), "◀ aktiv");
                }
                
                ui.separator();
                
                // Über button
                let ueber_text = if self.sidebar_collapsed {
                    egui::RichText::new("ℹ").size(24.0).color(egui::Color32::LIGHT_BLUE)
                } else {
                    egui::RichText::new("ℹ Über").color(egui::Color32::LIGHT_BLUE)
                };
                let ueber_button = egui::Button::new(ueber_text);
                let ueber_size = if self.sidebar_collapsed {
                    [50.0, 40.0]
                } else {
                    [200.0, 35.0]
                };
                
                if ui.add_sized(ueber_size, ueber_button).clicked() {
                    self.selected = 4;
                }
                if self.selected == 4 && !self.sidebar_collapsed {
                    ui.colored_label(egui::Color32::from_gray(140), "◀ aktiv");
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
            match self.selected {
                0 => {
                    ui.heading("Terminal");
                    
                    // Tab bar for terminals
                    ui.horizontal(|ui| {
                        let mut to_close = None;
                        let mut to_rename = None;
                        for (idx, tab) in self.terminals.iter().enumerate() {
                            let selected = idx == self.active_terminal_tab;
                            if ui.selectable_label(selected, &tab.name).clicked() {
                                self.active_terminal_tab = idx;
                            }
                            if ui.small_button("✏").on_hover_text("Umbenennen").clicked() {
                                to_rename = Some(idx);
                            }
                            if ui.small_button("✕").clicked() && self.terminals.len() > 1 {
                                to_close = Some(idx);
                            }
                            ui.separator();
                        }
                        if ui.button("➕ Neues Terminal").clicked() {
                            if let Ok(mut term) = TerminalView::new() {
                                term.text_color = self.terminal_text_color;
                                term.cursor_color = self.cursor_color;
                                term.cursor_shape = self.cursor_shape;
                                term.cursor_blinking = self.cursor_blinking;
                                self.terminals.push(TerminalTab {
                                    name: format!("Terminal {}", self.terminals.len() + 1),
                                    terminal: term,
                                });
                                self.active_terminal_tab = self.terminals.len() - 1;
                            }
                        }
                        
                        if let Some(idx) = to_rename {
                            self.terminal_rename_dialog = Some((idx, self.terminals[idx].name.clone()));
                        }
                        
                        if let Some(idx) = to_close {
                            self.terminals.remove(idx);
                            if self.active_terminal_tab >= self.terminals.len() {
                                self.active_terminal_tab = self.terminals.len().saturating_sub(1);
                            }
                        }
                    });
                    
                    ui.separator();
                    
                    // Active terminal
                    if let Some(tab) = self.terminals.get_mut(self.active_terminal_tab) {
                        // Ensure terminal respects current settings if changed elsewhere
                        tab.terminal.text_color = self.terminal_text_color;
                        tab.terminal.cursor_color = self.cursor_color;
                        tab.terminal.cursor_shape = self.cursor_shape;
                        tab.terminal.cursor_blinking = self.cursor_blinking;
                        tab.terminal.ui(ui);
                    } else {
                        ui.colored_label(egui::Color32::RED, "Kein Terminal verfügbar.");
                    }
                }
                1 => {
                    ui.heading("SSH Verbindungen");
                    // Apply SSH text color only within this panel
                    let old = ui.visuals_mut().override_text_color;
                    ui.visuals_mut().override_text_color = Some(self.ssh_text_color);
                    self.ssh_manager.ui(ui, &mut self.pending_ssh_connection);
                    ui.visuals_mut().override_text_color = old;
                }
                2 => {
                    ui.heading("Markdown Editor");
                    
                    // Tab bar for markdown editors
                    ui.horizontal(|ui| {
                        let mut to_close = None;
                        let mut to_rename = None;
                        for (idx, tab) in self.markdown_editors.iter().enumerate() {
                            let selected = idx == self.active_markdown_tab;
                            if ui.selectable_label(selected, &tab.name).clicked() {
                                self.active_markdown_tab = idx;
                            }
                            if ui.small_button("✏").on_hover_text("Umbenennen").clicked() {
                                to_rename = Some(idx);
                            }
                            if ui.small_button("✕").clicked() && self.markdown_editors.len() > 1 {
                                to_close = Some(idx);
                            }
                            ui.separator();
                        }
                        if ui.button("➕ Neues Dokument").clicked() {
                            self.markdown_editors.push(MarkdownTab {
                                name: format!("Dokument {}", self.markdown_editors.len() + 1),
                                editor: MarkdownEditor::default(),
                            });
                            self.active_markdown_tab = self.markdown_editors.len() - 1;
                        }
                        
                        if let Some(idx) = to_rename {
                            self.markdown_rename_dialog = Some((idx, self.markdown_editors[idx].name.clone()));
                        }
                        
                        if let Some(idx) = to_close {
                            self.markdown_editors.remove(idx);
                            if self.active_markdown_tab >= self.markdown_editors.len() {
                                self.active_markdown_tab = self.markdown_editors.len().saturating_sub(1);
                            }
                        }
                    });
                    
                    ui.separator();
                    
                    // Active markdown editor
                    if let Some(tab) = self.markdown_editors.get_mut(self.active_markdown_tab) {
                        let old = ui.visuals_mut().override_text_color;
                        ui.visuals_mut().override_text_color = Some(self.markdown_text_color);
                        tab.editor.ui(ui);
                        ui.visuals_mut().override_text_color = old;
                    }
                }
                3 => {
                    ui.heading("Einstellungen");
                    ui.add_space(10.0);
                    
                    // Theme selection
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("🎨 Theme:").strong());
                        egui::ComboBox::from_id_source("theme_selector")
                            .selected_text(self.current_theme.name())
                            .show_ui(ui, |ui| {
                                for theme in Theme::all() {
                                    let selected = self.current_theme == theme;
                                    if ui.selectable_label(selected, theme.name()).clicked() {
                                        self.current_theme = theme;
                                        self.save_settings();
                                    }
                                }
                            });
                    });
                    
                    ui.add_space(10.0);
                    
                    // Theme preview
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Theme Vorschau:").strong());
                        ui.separator();
                        match self.current_theme {
                            Theme::Dark => ui.label("Standard dunkles Theme mit neutralen Farben"),
                            Theme::Light => ui.label("Helles Theme für bessere Lesbarkeit bei Tageslicht"),
                            Theme::Dracula => ui.label("Beliebtes dunkles Theme mit lila Akzenten"),
                            Theme::Monokai => ui.label("Klassisches Entwickler-Theme mit warmen Farben"),
                            Theme::SolarizedDark => ui.label("Wissenschaftlich optimiertes dunkles Theme"),
                            Theme::Nord => ui.label("Arktisch inspiriertes Theme mit kalten Farbtönen"),
                            Theme::GruvboxDark => ui.label("Retro-inspiriertes warmes dunkles Theme"),
                        };
                    });
                    
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Per-view text colors
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Farben pro Bereich:").strong());
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label("Terminal Textfarbe:");
                            let mut c = self.terminal_text_color;
                            if ui.color_edit_button_srgba(&mut c).changed() {
                                self.terminal_text_color = c;
                                // Apply to all terminal tabs
                                for t in &mut self.terminals {
                                    t.terminal.text_color = c;
                                }
                                self.save_settings();
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Markdown Textfarbe:");
                            let mut c = self.markdown_text_color;
                            if ui.color_edit_button_srgba(&mut c).changed() {
                                self.markdown_text_color = c;
                                self.save_settings();
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("SSH Textfarbe:");
                            let mut c = self.ssh_text_color;
                            if ui.color_edit_button_srgba(&mut c).changed() {
                                self.ssh_text_color = c;
                                self.save_settings();
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Cursor Farbe:");
                            let mut c = self.cursor_color;
                            if ui.color_edit_button_srgba(&mut c).changed() {
                                self.cursor_color = c;
                                // Apply to all terminal tabs
                                for t in &mut self.terminals {
                                    t.terminal.cursor_color = c;
                                }
                                self.save_settings();
                            }
                        });
                    });

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Cursor settings
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Cursor Einstellungen:").strong());
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label("Cursor Form:");
                            egui::ComboBox::from_id_source("cursor_shape")
                                .selected_text(self.cursor_shape.name())
                                .show_ui(ui, |ui| {
                                    for shape in CursorShape::all() {
                                        let selected = self.cursor_shape == shape;
                                        if ui.selectable_label(selected, shape.name()).clicked() {
                                            self.cursor_shape = shape;
                                            // Apply to all terminal tabs
                                            for t in &mut self.terminals {
                                                t.terminal.cursor_shape = shape;
                                            }
                                            self.save_settings();
                                        }
                                    }
                                });
                        });
                        ui.horizontal(|ui| {
                            let mut blink = self.cursor_blinking;
                            if ui.checkbox(&mut blink, "Cursor blinken").changed() {
                                self.cursor_blinking = blink;
                                // Apply to all terminal tabs
                                for t in &mut self.terminals {
                                    t.terminal.cursor_blinking = blink;
                                }
                                self.save_settings();
                            }
                        });
                    });

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Global font selection
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Schriftart (global):").strong());
                        ui.add_space(6.0);
                        egui::ComboBox::from_id_source("font_mode")
                            .selected_text(match self.font_mode { FontMode::Default => "Standard", FontMode::MonospaceEverywhere => "Monospace überall", FontMode::Custom(_) => "Benutzerdefiniert" })
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(matches!(self.font_mode, FontMode::Default), "Standard").clicked() {
                                    self.font_mode = FontMode::Default;
                                    self.custom_font_info = None;
                                    self.save_settings();
                                }
                                if ui.selectable_label(matches!(self.font_mode, FontMode::MonospaceEverywhere), "Monospace überall").clicked() {
                                    self.font_mode = FontMode::MonospaceEverywhere;
                                    self.custom_font_info = None;
                                    self.save_settings();
                                }
                                if ui.selectable_label(matches!(self.font_mode, FontMode::Custom(_)), "Benutzerdefiniert").clicked() {
                                    self.font_mode = FontMode::Custom(String::new());
                                    self.save_settings();
                                }
                            });

                        if matches!(self.font_mode, FontMode::Custom(_)) {
                            ui.horizontal(|ui| {
                                if ui.button("📁 Schrift laden (.ttf/.otf)").clicked() {
                                    if let Some(path) = rfd::FileDialog::new().add_filter("Font", &["ttf", "otf"]).pick_file() {
                                        if let Ok(bytes) = std::fs::read(&path) {
                                            let mut defs = egui::FontDefinitions::default();
                                            defs.font_data.insert("user".into(), egui::FontData::from_owned(bytes));
                                            // Use custom font for both families by name
                                            defs.families.insert(egui::FontFamily::Proportional, vec!["user".to_string()]);
                                            defs.families.insert(egui::FontFamily::Monospace, vec!["user".to_string()]);
                                            ui.ctx().set_fonts(defs);
                                            self.custom_font_info = Some(path.display().to_string());
                                            self.font_mode = FontMode::Custom("user".into());
                                            self.save_settings();
                                        }
                                    }
                                }
                                if let Some(info) = &self.custom_font_info {
                                    ui.label(format!("Aktiv: {}", info));
                                    if ui.button("Zurücksetzen").clicked() {
                                        self.custom_font_info = None;
                                        self.font_mode = FontMode::Default;
                                        self.save_settings();
                                    }
                                }
                            });
                        }
                    });
                    
                    // Font scale
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("📏 Schriftgröße:").strong());
                        let mut tmp = self.font_scale;
                        if ui.add(egui::Slider::new(&mut tmp, 0.75..=2.0).text("Skalierung")).changed() {
                            self.font_scale = tmp;
                            self.save_settings();
                        }
                    });
                    
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    // System info
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("System-Information:").strong());
                        ui.separator();
                        ui.label(format!("🖥️ Plattform: {}", std::env::consts::OS));
                        ui.label(format!("🦀 Rust Version: {}", rustc_version_runtime::version()));
                        ui.label(format!("📦 TermiX Version: 0.1.0"));
                    });
                }
                4 => {
                    ui.heading("Über TermiX");
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("TermiX - Modern Terminal Application").size(18.0).strong());
                    ui.label("Version 0.1.0");
                    ui.add_space(10.0);
                    ui.separator();
                    ui.heading("Features:");
                    ui.label("✓ Multi-Tab Terminal & Markdown Editor");
                    ui.label("✓ Smart Command Suggestions (50+ Befehle)");
                    ui.label("✓ SSH Connection Manager");
                    ui.label("✓ Cross-Platform (Linux & Windows)");
                    ui.label("✓ Dual Interface (TUI & GUI)");
                    ui.add_space(10.0);
                    ui.separator();
                    ui.heading("Technologie:");
                    ui.label("• TUI: Crossterm");
                    ui.label("• GUI: egui/eframe");
                    ui.label("• Terminal: portable-pty + vt100");
                    ui.label("• SSH: ssh2");
                    ui.label("• Markdown: pulldown-cmark");
                    ui.add_space(10.0);
                    ui.separator();
                    ui.heading("Support:");
                    ui.label("Wenn du TermiX nützlich findest, unterstütze die Entwicklung:");
                    ui.add_space(5.0);
                    
                    // Buy Me a Coffee styled button
                    let button_text = egui::RichText::new("☕ Buy me a coffee")
                        .color(egui::Color32::BLACK)
                        .size(16.0);
                    
                    let button = egui::Button::new(button_text)
                        .fill(egui::Color32::from_rgb(255, 221, 0)) // #FFDD00
                        .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
                        .rounding(egui::Rounding::same(8.0));
                    
                    if ui.add_sized([200.0, 40.0], button).on_hover_text("Öffnet buymeacoffee.com/janquillfeldt").clicked() {
                        let _ = open::that("https://buymeacoffee.com/janquillfeldt");
                    }
                }
                5 => {
                    ui.heading("Beenden");
                    ui.label("Schließt das Programm.");
                    if ui.button("Fenster schließen").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                _ => {}
            }
        });

        // Terminal rename dialog
        let mut close_rename_dialog = false;
        if let Some((idx, ref mut new_name)) = self.terminal_rename_dialog {
            egui::Window::new("Terminal umbenennen")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Neuer Name:");
                        ui.text_edit_singleline(new_name);
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("✓ Speichern").clicked() {
                            if idx < self.terminals.len() {
                                self.terminals[idx].name = new_name.clone();
                            }
                            close_rename_dialog = true;
                        }
                        if ui.button("✗ Abbrechen").clicked() {
                            close_rename_dialog = true;
                        }
                    });
                });
        }
        if close_rename_dialog {
            self.terminal_rename_dialog = None;
        }

        // Markdown rename dialog
        let mut close_markdown_rename_dialog = false;
        if let Some((idx, ref mut new_name)) = self.markdown_rename_dialog {
            egui::Window::new("Markdown-Dokument umbenennen")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Neuer Name:");
                        ui.text_edit_singleline(new_name);
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("✓ Speichern").clicked() {
                            if idx < self.markdown_editors.len() {
                                self.markdown_editors[idx].name = new_name.clone();
                            }
                            close_markdown_rename_dialog = true;
                        }
                        if ui.button("✗ Abbrechen").clicked() {
                            close_markdown_rename_dialog = true;
                        }
                    });
                });
        }
        if close_markdown_rename_dialog {
            self.markdown_rename_dialog = None;
        }

        // Handle pending SSH connection request
        if let Some(ssh_conn) = self.pending_ssh_connection.take() {
            // If no password stored, show prompt dialog
            if ssh_conn.password.is_empty() {
                self.ssh_password_prompt = Some((ssh_conn, String::new()));
            } else {
                // Try to create SSH terminal
                match TerminalView::new_ssh(&ssh_conn) {
                    Ok(mut term) => {
                        term.text_color = self.terminal_text_color;
                        term.cursor_color = self.cursor_color;
                        term.cursor_shape = self.cursor_shape;
                        term.cursor_blinking = self.cursor_blinking;
                        self.terminals.push(TerminalTab {
                            name: format!("SSH: {}", ssh_conn.name),
                            terminal: term,
                        });
                        self.active_terminal_tab = self.terminals.len() - 1;
                        self.selected = 0; // Switch to Terminal view
                        self.ssh_manager.status_message = format!("✓ Verbunden mit {}", ssh_conn.name);
                    }
                    Err(e) => {
                        // Show error in SSH manager status
                        self.ssh_manager.status_message = format!("❌ Fehler: {}", e);
                    }
                }
            }
        }

        // SSH password prompt dialog
        let mut close_password_prompt = false;
        let mut attempt_connection = None;
        if let Some((ref conn, ref mut password)) = self.ssh_password_prompt {
            egui::Window::new("SSH Passwort erforderlich")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("Verbindung zu: {}@{}:{}", conn.username, conn.host, conn.port));
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Passwort:");
                        ui.add(egui::TextEdit::singleline(password).password(true));
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("✓ Verbinden").clicked() {
                            let mut conn_with_pwd = conn.clone();
                            conn_with_pwd.password = password.clone();
                            attempt_connection = Some(conn_with_pwd);
                            close_password_prompt = true;
                        }
                        if ui.button("✗ Abbrechen").clicked() {
                            close_password_prompt = true;
                        }
                    });
                });
        }
        if close_password_prompt {
            self.ssh_password_prompt = None;
        }
        if let Some(conn) = attempt_connection {
            // Try SSH connection with provided password
            match TerminalView::new_ssh(&conn) {
                Ok(mut term) => {
                    term.text_color = self.terminal_text_color;
                    term.cursor_color = self.cursor_color;
                    term.cursor_shape = self.cursor_shape;
                    term.cursor_blinking = self.cursor_blinking;
                    self.terminals.push(TerminalTab {
                        name: format!("SSH: {}", conn.name),
                        terminal: term,
                    });
                    self.active_terminal_tab = self.terminals.len() - 1;
                    self.selected = 0;
                    self.ssh_manager.status_message = format!("✓ Verbunden mit {}", conn.name);
                }
                Err(e) => {
                    self.ssh_manager.status_message = format!("❌ Fehler: {}", e);
                }
            }
        }
    }
}

#[cfg(feature = "gui")]
pub fn run_gui() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([640.0, 420.0])
            .with_title("TermiX"),
        ..Default::default()
    };
    eframe::run_native(
        "TermiX",
        options,
        Box::new(|cc| Box::new(GuiApp::new(cc))),
    )
}

// ===================== Embedded PTY Terminal =====================
#[cfg(feature = "gui")]
struct TerminalView {
    rx: Receiver<Vec<u8>>,     // from reader thread
    writer: Sender<Vec<u8>>,   // to writer thread
    parser: VtParser,
    cols: u16,
    rows: u16,
    master: Box<dyn MasterPty + Send>,
    // Command suggestion
    input_buffer: String,
    suggestions: Vec<String>,
    selected_suggestion: usize,
    show_suggestions: bool,
    // Appearance
    text_color: egui::Color32,
    cursor_color: egui::Color32,
    cursor_shape: CursorShape,
    cursor_blinking: bool,
    cursor_visible: bool, // for blink state
    last_blink_time: f64,
}

// Common shell commands for suggestions
#[cfg(feature = "gui")]
const COMMON_COMMANDS: &[&str] = &[
    "ls", "ls -la", "ls -lh", "cd", "pwd", "cat", "echo", "mkdir", "rmdir", "rm", "rm -rf",
    "cp", "mv", "touch", "grep", "find", "chmod", "chown", "ps", "ps aux", "top", "htop",
    "kill", "killall", "df", "df -h", "du", "du -sh", "free", "free -h", "uname", "uname -a",
    "whoami", "which", "whereis", "man", "history", "clear", "exit", "sudo", "apt update",
    "apt install", "apt upgrade", "systemctl status", "systemctl start", "systemctl stop",
    "journalctl", "wget", "curl", "ssh", "scp", "tar", "tar -xzf", "tar -czf", "zip", "unzip",
    "git status", "git add", "git commit", "git push", "git pull", "git log", "git clone",
    "docker ps", "docker run", "docker build", "docker-compose up", "npm install", "npm start",
    "cargo build", "cargo run", "python", "python3", "node", "vim", "nano", "less", "more",
];

#[cfg(feature = "gui")]
impl TerminalView {
    fn new() -> anyhow::Result<Self> {
        let (to_writer_tx, to_writer_rx) = mpsc::channel::<Vec<u8>>();
        let (from_reader_tx, from_reader_rx) = mpsc::channel::<Vec<u8>>();

        // Open PTY
        let pty_system = NativePtySystem::default();
        let initial_cols = 80u16;
        let initial_rows = 25u16;
        let pair = pty_system.openpty(PtySize {
            rows: initial_rows,
            cols: initial_cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // Spawn shell
        #[cfg(target_os = "windows")]
        let mut cmd = CommandBuilder::new("cmd.exe");
        #[cfg(not(target_os = "windows"))]
        let cmd = CommandBuilder::new(std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into()));

        let _child = pair.slave.spawn_command(cmd)?;

        let mut writer = pair.master.take_writer()?;
        let mut reader = pair.master.try_clone_reader()?;
        let master: Box<dyn MasterPty + Send> = pair.master;

        // Writer thread: send bytes to PTY
        let writer_thread = thread::spawn(move || {
            while let Ok(buf) = to_writer_rx.recv() {
                let _ = writer.write_all(&buf);
                let _ = writer.flush();
            }
        });
        // Detach thread
        let _ = writer_thread.thread().id();

        // Reader thread: read bytes from PTY and forward
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => {
                        let _ = from_reader_tx.send(b"\n[PTY closed]\n".to_vec());
                        break;
                    }
                    Ok(n) => {
                        let _ = from_reader_tx.send(buf[..n].to_vec());
                    }
                    Err(_) => {
                        // reduce busy loop on error
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        });

        Ok(Self {
            rx: from_reader_rx,
            writer: to_writer_tx,
            parser: VtParser::new(initial_rows, initial_cols, 2000),
            cols: initial_cols,
            rows: initial_rows,
            master,
            input_buffer: String::new(),
            suggestions: Vec::new(),
            selected_suggestion: 0,
            show_suggestions: false,
            text_color: egui::Color32::from_rgb(220, 220, 220),
            cursor_color: egui::Color32::from_rgb(0, 255, 0),
            cursor_shape: CursorShape::Block,
            cursor_blinking: false,
            cursor_visible: true,
            last_blink_time: 0.0,
        })
    }

    fn new_ssh(conn: &SshConnection) -> anyhow::Result<Self> {
        use ssh2::Session;
        use std::net::TcpStream;

        let (to_writer_tx, to_writer_rx) = mpsc::channel::<Vec<u8>>();
        let (from_reader_tx, from_reader_rx) = mpsc::channel::<Vec<u8>>();

        // Connect to SSH server
        let tcp = TcpStream::connect(format!("{}:{}", conn.host, conn.port))
            .map_err(|e| anyhow::anyhow!("Verbindung zu {}:{} fehlgeschlagen: {}", conn.host, conn.port, e))?;
        
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()
            .map_err(|e| anyhow::anyhow!("SSH-Handshake fehlgeschlagen: {}", e))?;

        // Authenticate
        if conn.password.is_empty() {
            anyhow::bail!("Kein Passwort angegeben. Bitte Passwort in den Verbindungseinstellungen speichern.");
        }
        
        sess.userauth_password(&conn.username, &conn.password)
            .map_err(|e| anyhow::anyhow!("Authentifizierung fehlgeschlagen: {}", e))?;
        
        if !sess.authenticated() {
            anyhow::bail!("Authentifizierung fehlgeschlagen: Falsches Passwort oder Benutzer nicht berechtigt");
        }

        // Open channel and request PTY
        let mut channel = sess.channel_session()
            .map_err(|e| anyhow::anyhow!("Kanal-Erstellung fehlgeschlagen: {}", e))?;
        channel.request_pty("xterm", None, Some((80, 24, 0, 0)))
            .map_err(|e| anyhow::anyhow!("PTY-Anfrage fehlgeschlagen: {}", e))?;
        channel.shell()
            .map_err(|e| anyhow::anyhow!("Shell-Start fehlgeschlagen: {}", e))?;

        let initial_cols = 80u16;
        let initial_rows = 24u16;

        // Split channel for read/write
        let read_channel = channel.stream(0);
        let mut write_channel = channel.stream(0);

        // Writer thread
        thread::spawn(move || {
            while let Ok(buf) = to_writer_rx.recv() {
                let _ = write_channel.write_all(&buf);
                let _ = write_channel.flush();
            }
        });

        // Reader thread
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            let mut read_stream = read_channel;
            loop {
                match read_stream.read(&mut buf) {
                    Ok(0) => {
                        let _ = from_reader_tx.send(b"\n[SSH connection closed]\n".to_vec());
                        break;
                    }
                    Ok(n) => {
                        let _ = from_reader_tx.send(buf[..n].to_vec());
                    }
                    Err(_) => {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        });

        // Create a dummy PTY master that does nothing (SSH channel handles I/O via threads)
        let pty_system = NativePtySystem::default();
        let pair = pty_system.openpty(PtySize {
            rows: initial_rows,
            cols: initial_cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        let master: Box<dyn MasterPty + Send> = pair.master;

        Ok(Self {
            rx: from_reader_rx,
            writer: to_writer_tx,
            parser: VtParser::new(initial_rows, initial_cols, 2000),
            cols: initial_cols,
            rows: initial_rows,
            master, // Dummy; resize will be ignored for SSH
            input_buffer: String::new(),
            suggestions: Vec::new(),
            selected_suggestion: 0,
            show_suggestions: false,
            text_color: egui::Color32::from_rgb(220, 220, 220),
            cursor_color: egui::Color32::from_rgb(0, 255, 0),
            cursor_shape: CursorShape::Block,
            cursor_blinking: false,
            cursor_visible: true,
            last_blink_time: 0.0,
        })
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        if cols == self.cols && rows == self.rows { return; }
        self.cols = cols.max(10);
        self.rows = rows.max(5);
        self.parser.set_size(self.rows, self.cols);
        let _ = self.master.resize(PtySize { rows: self.rows, cols: self.cols, pixel_width: 0, pixel_height: 0 });
    }

    fn send_str(&mut self, s: &str) {
        let _ = self.writer.send(s.as_bytes().to_vec());
    }

    fn send_bytes(&mut self, b: &[u8]) {
        let _ = self.writer.send(b.to_vec());
    }

    fn update_suggestions(&mut self) {
        if self.input_buffer.is_empty() {
            self.show_suggestions = false;
            self.suggestions.clear();
            return;
        }
        
        // Find matching commands
        self.suggestions = COMMON_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(&self.input_buffer))
            .take(5)  // Show max 5 suggestions
            .map(|s| s.to_string())
            .collect();
        
        self.show_suggestions = !self.suggestions.is_empty();
        self.selected_suggestion = 0;
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        // Drain incoming bytes and update VT parser
        for chunk in self.rx.try_iter() { 
            self.parser.process(&chunk); 
        }

        // Handle cursor blinking
        if self.cursor_blinking {
            let current_time = ui.input(|i| i.time);
            if current_time - self.last_blink_time > 0.5 {
                self.cursor_visible = !self.cursor_visible;
                self.last_blink_time = current_time;
            }
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));
        } else {
            self.cursor_visible = true;
        }

        // Request repaint to show updates
        ui.ctx().request_repaint();

        // Create a visually distinct terminal frame
        let frame = egui::Frame::default()
            .fill(egui::Color32::from_rgb(20, 20, 30))  // Dark blue-black background
            .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 200, 120)))  // Green border
            .inner_margin(egui::Margin::same(10.0))
            .rounding(egui::Rounding::same(5.0));
        
        frame.show(ui, |ui| {
            // Estimate character cell size and compute rows/cols
            let char_w = ui.fonts(|f| f.glyph_width(&egui::TextStyle::Monospace.resolve(ui.style()), 'W'));
            let char_h = ui.text_style_height(&egui::TextStyle::Monospace);
            let avail = ui.available_size();
            if char_w > 0.0 && char_h > 0.0 {
                let cols = (avail.x / char_w).floor().max(10.0) as u16;
                let rows = (avail.y / char_h).floor().max(5.0) as u16;
                self.resize(cols, rows);
            }

            // Render VT screen with ANSI colors using cell-based iteration
            let _scroll_output = egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                use egui::text::LayoutJob;
                let screen = self.parser.screen();
                // Cursor position from vt100 (1-based columns -> we use saturating_sub(1))
                let (cursor_row, cursor_col) = screen.cursor_position();
                let display_col: u16 = cursor_col.saturating_sub(1);

                let rows = self.rows as usize;
                let cols = self.cols as usize;
                let font_id = egui::TextStyle::Monospace.resolve(ui.style());

                for row in 0..rows {
                    let mut job = LayoutJob::default();
                    for col in 0..cols {
                        // Fetch cell; fallback to space if out of bounds/missing
                        let mut ch: char = ' ';
                        let mut fg = self.text_color;
                        let mut bg: Option<egui::Color32> = None;
                        let mut inverse = false;
                        // vt100 cell API (try common method names; compile will guide adjustments)
                        if let Some(cell) = screen.cell(row as u16, col as u16) {
                            // character
                            let s = cell.contents();
                            ch = s.chars().next().unwrap_or(' ');
                            // colors (always provided as a Color; Default means fallback)
                            let c_fg = cell.fgcolor();
                            fg = vt_color_to_egui(c_fg, self.text_color);
                            let c_bg = cell.bgcolor();
                            let bg_c = vt_color_to_egui(c_bg, egui::Color32::TRANSPARENT);
                            if bg_c != egui::Color32::TRANSPARENT { bg = Some(bg_c); }
                            // attributes
                            if cell.inverse() { inverse = true; }
                        }

                        // Integrate cursor rendering by replacing the cell at cursor position
                        let is_cursor_cell = self.cursor_visible && (row as u16 == cursor_row) && (col as u16 == display_col);
                        let (text, txt_fg, txt_bg) = if is_cursor_cell {
                            // Draw cursor with chosen shape/color, ignoring underlying bg
                            let rendered = self.cursor_shape.render(ch);
                            (rendered, self.cursor_color, None)
                        } else {
                            let mut eff_fg = fg;
                            let mut eff_bg = bg;
                            if inverse {
                                std::mem::swap(&mut eff_fg, &mut eff_bg.get_or_insert(self.text_color));
                            }
                            (ch.to_string(), eff_fg, eff_bg)
                        };

                        let mut format = egui::TextFormat { font_id: font_id.clone(), color: txt_fg, ..Default::default() };
                        // Background color if present
                        #[allow(deprecated)]
                        {
                            if let Some(bgc) = txt_bg {
                                format.background = bgc;
                            }
                        }
                        job.append(&text, 0.0, format);
                    }
                    ui.label(job);
                }
            });

        // Always handle keyboard input when Terminal is the active panel
        ui.input(|i| {
            for ev in &i.events {
                match ev {
                    egui::Event::Text(t) => { 
                        if !t.is_empty() {
                            // Track input for suggestions
                            for ch in t.chars() {
                                if ch.is_alphanumeric() || ch == '-' || ch == '_' || ch == '/' || ch == '.' {
                                    self.input_buffer.push(ch);
                                    self.update_suggestions();
                                }
                            }
                            self.send_str(t); 
                        } 
                    }
                    egui::Event::Key { key, pressed: true, modifiers, .. } => {
                        match key {
                            egui::Key::Enter => {
                                self.input_buffer.clear();
                                self.show_suggestions = false;
                                self.send_bytes(b"\r");
                            }
                            egui::Key::Backspace => {
                                self.input_buffer.pop();
                                self.update_suggestions();
                                self.send_bytes(&[0x7f]);
                            }
                            egui::Key::Tab => {
                                // Auto-complete with selected suggestion
                                if self.show_suggestions && !self.suggestions.is_empty() {
                                    let suggestion = self.suggestions[self.selected_suggestion].clone();
                                    let to_complete = suggestion[self.input_buffer.len()..].to_string();
                                    self.send_str(&to_complete);
                                    self.input_buffer = suggestion;
                                    self.show_suggestions = false;
                                } else {
                                    self.send_bytes(b"\t");
                                }
                            }
                            egui::Key::ArrowUp => {
                                if self.show_suggestions && !self.suggestions.is_empty() {
                                    self.selected_suggestion = self.selected_suggestion.saturating_sub(1);
                                } else {
                                    self.send_bytes(b"\x1b[A");
                                }
                            }
                            egui::Key::ArrowDown => {
                                if self.show_suggestions && !self.suggestions.is_empty() {
                                    self.selected_suggestion = (self.selected_suggestion + 1).min(self.suggestions.len() - 1);
                                } else {
                                    self.send_bytes(b"\x1b[B");
                                }
                            }
                            egui::Key::ArrowRight => self.send_bytes(b"\x1b[C"),
                            egui::Key::ArrowLeft => self.send_bytes(b"\x1b[D"),
                            egui::Key::Escape => {
                                self.show_suggestions = false;
                            }
                            egui::Key::C if modifiers.ctrl => {
                                self.input_buffer.clear();
                                self.show_suggestions = false;
                                self.send_bytes(&[0x03]);
                            }
                            egui::Key::D if modifiers.ctrl => self.send_bytes(&[0x04]),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        });
        
        ui.separator();
        
        // Show command suggestions
        if self.show_suggestions && !self.suggestions.is_empty() {
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::YELLOW, "💡 Vorschläge:");
                for (idx, suggestion) in self.suggestions.iter().enumerate() {
                    let selected = idx == self.selected_suggestion;
                    let color = if selected { egui::Color32::GREEN } else { egui::Color32::LIGHT_GRAY };
                    ui.colored_label(color, suggestion);
                    if idx < self.suggestions.len() - 1 {
                        ui.label("|");
                    }
                }
                ui.label("(Tab = Vervollständigen, ↑↓ = Auswählen, Esc = Schließen)");
            });
        } else {
            ui.colored_label(egui::Color32::GREEN, "⌨️ Terminal aktiv - Befehle werden direkt verarbeitet (Tab für Vorschläge)");
        }
        }); // Close frame
    }
}

// Map vt100 colors to egui::Color32
#[cfg(feature = "gui")]
fn vt_color_to_egui(c: vt100::Color, default: egui::Color32) -> egui::Color32 {
    use vt100::Color;
    match c {
        Color::Default => default,
        Color::Rgb(r, g, b) => egui::Color32::from_rgb(r, g, b),
        // Fallback: 256-color indexed palette
        other => {
            if let Some(idx) = color_to_index(other) {
                ansi256_to_rgb(idx).unwrap_or(default)
            } else {
                default
            }
        }
    }
}

#[cfg(feature = "gui")]
fn color_to_index(c: vt100::Color) -> Option<u8> {
    match c {
        vt100::Color::Idx(i) => Some(i),
        _ => None,
    }
}

#[cfg(feature = "gui")]
fn ansi256_to_rgb(i: u8) -> Option<egui::Color32> {
    let idx = i as u16;
    if idx <= 15 {
        let (r, g, b) = match idx {
            0 => (0, 0, 0),
            1 => (128, 0, 0),
            2 => (0, 128, 0),
            3 => (128, 128, 0),
            4 => (0, 0, 128),
            5 => (128, 0, 128),
            6 => (0, 128, 128),
            7 => (192, 192, 192),
            8 => (128, 128, 128),
            9 => (255, 0, 0),
            10 => (0, 255, 0),
            11 => (255, 255, 0),
            12 => (0, 0, 255),
            13 => (255, 0, 255),
            14 => (0, 255, 255),
            15 => (255, 255, 255),
            _ => (255, 255, 255),
        };
        return Some(egui::Color32::from_rgb(r, g, b));
    }
    if (16..=231).contains(&idx) {
        let n = idx - 16;
        let r = n / 36;
        let g = (n % 36) / 6;
        let b = n % 6;
        let conv = |v: u16| -> u8 { [0, 95, 135, 175, 215, 255][v as usize] };
        return Some(egui::Color32::from_rgb(conv(r), conv(g), conv(b)));
    }
    if (232..=255).contains(&idx) {
        let l = (8 + (idx - 232) * 10) as u8;
        return Some(egui::Color32::from_rgb(l, l, l));
    }
    None
}

// ===================== Markdown Editor =====================
#[cfg(feature = "gui")]
#[derive(Default)]
struct MarkdownEditor {
    content: String,
    file_path: Option<std::path::PathBuf>,
}

#[cfg(feature = "gui")]
impl MarkdownEditor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("📂 Öffnen").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("Markdown", &["md", "markdown"]).pick_file() {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        self.content = content;
                        self.file_path = Some(path);
                    }
                }
            }
            if ui.button("💾 Speichern").clicked() {
                if let Some(path) = &self.file_path {
                    let _ = std::fs::write(path, &self.content);
                } else if let Some(path) = rfd::FileDialog::new().add_filter("Markdown", &["md"]).save_file() {
                    let _ = std::fs::write(&path, &self.content);
                    self.file_path = Some(path);
                }
            }
            if ui.button("📄 Neu").clicked() {
                self.content.clear();
                self.file_path = None;
            }
            ui.separator();
            if let Some(path) = &self.file_path {
                ui.label(format!("📝 {}", path.display()));
            } else {
                ui.label("📝 Ungespeichert");
            }
        });
        ui.separator();

        // Split editor and preview
        egui::TopBottomPanel::bottom("preview").resizable(true).default_height(ui.available_height() * 0.5).show_inside(ui, |ui| {
            ui.heading("Vorschau");
            egui::ScrollArea::vertical().show(ui, |ui| {
                render_markdown(ui, &self.content);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Editor");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.content).font(egui::TextStyle::Monospace).desired_width(f32::INFINITY).desired_rows(30));
            });
        });
    }
}

#[cfg(feature = "gui")]
fn render_markdown(ui: &mut egui::Ui, text: &str) {
    let parser = MdParser::new(text);
    let mut in_heading = None;
    let mut in_code = false;
    let mut _list_depth = 0;

    for event in parser {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => { in_heading = Some(level); }
            Event::End(Tag::Heading(_, _, _)) => { in_heading = None; ui.end_row(); }
            Event::Start(Tag::Paragraph) => {}
            Event::End(Tag::Paragraph) => { ui.end_row(); }
            Event::Start(Tag::CodeBlock(_)) => { in_code = true; }
            Event::End(Tag::CodeBlock(_)) => { in_code = false; ui.end_row(); }
            Event::Start(Tag::List(_)) => { _list_depth += 1; }
            Event::End(Tag::List(_)) => { _list_depth -= 1; }
            Event::Start(Tag::Item) => { ui.label("  •"); }
            Event::End(Tag::Item) => { ui.end_row(); }
            Event::Text(t) => {
                let label = if let Some(level) = in_heading {
                    match level {
                        HeadingLevel::H1 => egui::RichText::new(t.as_ref()).heading().strong().size(24.0),
                        HeadingLevel::H2 => egui::RichText::new(t.as_ref()).heading().size(20.0),
                        HeadingLevel::H3 => egui::RichText::new(t.as_ref()).strong().size(18.0),
                        _ => egui::RichText::new(t.as_ref()).strong().size(16.0),
                    }
                } else if in_code {
                    egui::RichText::new(t.as_ref()).monospace().background_color(egui::Color32::from_gray(40))
                } else {
                    egui::RichText::new(t.as_ref())
                };
                ui.label(label);
            }
            Event::Code(c) => {
                ui.label(egui::RichText::new(c.as_ref()).monospace().background_color(egui::Color32::from_gray(50)));
            }
            Event::SoftBreak | Event::HardBreak => { ui.end_row(); }
            _ => {}
        }
    }
}

// ===================== SSH Manager =====================
#[cfg(feature = "gui")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "gui")]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SshConnection {
    name: String,
    host: String,
    port: u16,
    username: String,
    #[serde(skip)]
    password: String, // Not saved to disk for security
}

#[cfg(feature = "gui")]
impl Default for SshConnection {
    fn default() -> Self {
        Self {
            name: String::new(),
            host: String::new(),
            port: 22,
            username: String::new(),
            password: String::new(),
        }
    }
}

#[cfg(feature = "gui")]
#[derive(Default, Serialize, Deserialize)]
struct SshManager {
    connections: Vec<SshConnection>,
    #[serde(skip)]
    new_connection: SshConnection,
    #[serde(skip)]
    show_add_dialog: bool,
    #[serde(skip)]
    status_message: String,
    #[serde(skip)]
    rename_dialog: Option<(usize, String)>, // (connection_index, new_name)
}

#[cfg(feature = "gui")]
impl SshManager {
    fn load_or_default() -> Self {
        let config_path = std::path::PathBuf::from("ssh_connections.toml");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(manager) = toml::from_str::<SshManager>(&content) {
                return manager;
            }
        }
        Self::default()
    }

    fn save(&self) {
        let config_path = std::path::PathBuf::from("ssh_connections.toml");
        if let Ok(content) = toml::to_string_pretty(self) {
            let _ = std::fs::write(config_path, content);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, pending_connection: &mut Option<SshConnection>) {
        // Add connection button
        ui.horizontal(|ui| {
            if ui.button("➕ Neue SSH Verbindung").clicked() {
                self.show_add_dialog = true;
                self.new_connection = SshConnection::default();
            }
            if ui.button("💾 Speichern").clicked() {
                self.save();
                self.status_message = "Verbindungen gespeichert!".into();
            }
        });

        if !self.status_message.is_empty() {
            ui.colored_label(egui::Color32::GREEN, &self.status_message);
        }

        ui.separator();

        // Add connection dialog
        if self.show_add_dialog {
            egui::Window::new("Neue SSH Verbindung")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.new_connection.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Host:");
                        ui.text_edit_singleline(&mut self.new_connection.host);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Port:");
                        ui.add(egui::DragValue::new(&mut self.new_connection.port).clamp_range(1..=65535));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Benutzer:");
                        ui.text_edit_singleline(&mut self.new_connection.username);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Passwort:");
                        ui.add(egui::TextEdit::singleline(&mut self.new_connection.password).password(true));
                    });

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("✓ Hinzufügen").clicked() {
                            self.connections.push(self.new_connection.clone());
                            self.show_add_dialog = false;
                            self.status_message = format!("Verbindung '{}' hinzugefügt", self.new_connection.name);
                        }
                        if ui.button("✗ Abbrechen").clicked() {
                            self.show_add_dialog = false;
                        }
                    });
                });
        }

        // List connections
        ui.heading("Gespeicherte Verbindungen:");
        ui.separator();

        let mut to_remove = None;
        let mut to_connect = None;
        let mut to_rename = None;

        for (idx, conn) in self.connections.iter().enumerate() {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(100, 200, 255), &conn.name);
                    ui.separator();
                    ui.label(format!("{}@{}:{}", conn.username, conn.host, conn.port));
                });

                ui.horizontal(|ui| {
                    if ui.button("🔌 Verbinden").clicked() {
                        to_connect = Some(idx);
                    }
                    if ui.button("✏ Umbenennen").clicked() {
                        to_rename = Some(idx);
                    }
                    if ui.button("🗑 Löschen").clicked() {
                        to_remove = Some(idx);
                    }
                });
            });
            ui.separator();
        }

        if let Some(idx) = to_rename {
            self.rename_dialog = Some((idx, self.connections[idx].name.clone()));
        }

        if let Some(idx) = to_remove {
            let removed = self.connections.remove(idx);
            self.status_message = format!("Verbindung '{}' gelöscht", removed.name);
        }

        if let Some(idx) = to_connect {
            let conn = self.connections[idx].clone();
            self.status_message = format!("Verbinde zu {}@{}:{}...", conn.username, conn.host, conn.port);
            *pending_connection = Some(conn);
        }

        if self.connections.is_empty() {
            ui.colored_label(egui::Color32::GRAY, "Keine SSH Verbindungen gespeichert. Klicke auf '➕ Neue SSH Verbindung' um eine hinzuzufügen.");
        }

        // Rename dialog
        let mut close_rename_dialog = false;
        let mut new_status_message = None;
        if let Some((idx, ref mut new_name)) = self.rename_dialog {
            egui::Window::new("SSH Verbindung umbenennen")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Neuer Name:");
                        ui.text_edit_singleline(new_name);
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("✓ Speichern").clicked() {
                            if idx < self.connections.len() {
                                self.connections[idx].name = new_name.clone();
                                new_status_message = Some(format!("Verbindung umbenannt zu '{}'", new_name));
                            }
                            close_rename_dialog = true;
                        }
                        if ui.button("✗ Abbrechen").clicked() {
                            close_rename_dialog = true;
                        }
                    });
                });
        }
        if close_rename_dialog {
            self.rename_dialog = None;
        }
        if let Some(msg) = new_status_message {
            self.status_message = msg;
        }
    }
}
