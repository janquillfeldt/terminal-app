# TermiX - TODO Liste

## 🔴 Priorität 1: Kritisch (Kern-Funktionalität)

### ⚙️ Settings-Persistenz (30 Min)
- [ ] `Settings`-Struct mit allen Einstellungen erstellen
- [ ] Serde-Serialisierung implementieren (`~/.config/termix/settings.toml`)
- [ ] Auto-Save bei Änderungen + Load beim Start
- [ ] Fehlerbehandlung für kaputte Config-Dateien

**Warum:** Nutzer verlieren aktuell alle Einstellungen bei jedem Neustart

### 🎨 ANSI-Farben im Terminal (45 Min)
- [ ] Von `contents()` zu `screen.cell(row, col)` API wechseln
- [ ] Cell-Attribute auslesen (Farbe, Bold, Italic, etc.)
- [ ] vt100-Farben zu egui::Color32 mappen
- [ ] 256-Color und True-Color Support

**Warum:** Terminal zeigt aktuell alles weiß, keine `ls --color` oder Syntax-Highlighting

### 🔌 SSH-Verbindungen implementieren (2 Std)
- [ ] ssh2-Crate mit TerminalView verbinden
- [ ] Session-Management (connect, disconnect, reconnect)
- [ ] Passwort-Dialog + optional Key-basierte Auth
- [ ] Fehlerbehandlung (Timeout, falsche Credentials, etc.)

**Warum:** SSH-Manager ist aktuell nur UI-Placeholder ohne Funktion

---

## 🟡 Priorität 2: UX-Verbesserungen

### ⌨️ Keyboard-Shortcuts (1 Std)
- [ ] Ctrl+T: Neuer Terminal-Tab
- [ ] Ctrl+W: Tab schließen
- [ ] Ctrl+Tab / Ctrl+Shift+Tab: Tab-Navigation
- [ ] Ctrl+C / Ctrl+V: Copy/Paste im Terminal
- [ ] Ctrl+Plus/Minus: Schriftgröße
- [ ] Shortcuts in Tooltips anzeigen

### 📜 Terminal-Scrollback verbessern
- [ ] PageUp/PageDown Support
- [ ] Mousewheel-Scrolling optimieren
- [ ] Jump-to-Top / Jump-to-Bottom Buttons
- [ ] Scrollback-Limit konfigurierbar (aktuell fix 2000 Zeilen)

### 🔄 Tab-Management
- [ ] Drag & Drop zum Umordnen
- [ ] Tab-Close-Buttons (× neben Namen)
- [ ] Ctrl+Click für schnelles Schließen
- [ ] Tab-Übersicht bei vielen offenen Tabs (Dropdown)

---

## 🟢 Priorität 3: Neue Features

### ➗ Split-View Terminals
- [ ] Horizontale/Vertikale Splits
- [ ] Focus-Navigation zwischen Splits
- [ ] Resize-Handles für Panes
- [ ] Layout speichern/laden

### 🔍 Command-History-Suche
- [ ] Ctrl+R: Reverse-Search wie in Bash
- [ ] History-Panel mit Filter
- [ ] History über Sessions hinweg speichern

### 💾 Export/Import
- [ ] Einstellungen exportieren (JSON/TOML)
- [ ] SSH-Verbindungen ex-/importieren
- [ ] Terminal-Themes als Presets

### 📋 Clipboard-Integration
- [ ] Automatisches Copy bei Selektion (optional)
- [ ] Middle-Click-Paste
- [ ] Clipboard-History

---

## 🔧 Priorität 4: Fixes & Polish

### ⚠️ Warnings beheben
- [ ] `FontMode::Custom(String)` Feld nutzen oder entfernen
- [ ] Alle Clippy-Warnings durchgehen

### ⚡ Performance
- [ ] Rendering bei großen Outputs optimieren
- [ ] Virtuelle Scrolling für 10.000+ Zeilen
- [ ] Frame-Rate bei inaktiven Tabs reduzieren

### 📖 README
- [ ] Buy Me a Coffee Link testen (Markdown-Format funktioniert?)
- [ ] Screenshots hinzufügen
- [ ] Installation-Guide erweitern (Binary-Releases?)

### 🎨 Theme-System
- [ ] Mehr vordefinierte Themes (Solarized, Dracula, Gruvbox)
- [ ] Theme-Editor im GUI
- [ ] Theme-Vorschau bevor angewendet

---

## 🎁 Bonus-Features (Nice-to-Have)

### 🔔 System-Tray
- [ ] Icon in System-Tray
- [ ] Minimize to Tray
- [ ] Quick-Actions im Tray-Menü

### 🧩 Plugin-System
- [ ] API für externe Plugins
- [ ] Beispiel-Plugins (z.B. Git-Integration)

### 🌐 Sprach-Support
- [ ] i18n-Framework integrieren
- [ ] Deutsch/Englisch umschaltbar

---

## 🏆 Top 3 Quick Wins

1. **Settings-Persistenz** (30 Min) → Verhindert Frust beim Neustart
2. **ANSI-Farben** (45 Min) → Sofort sichtbare Verbesserung
3. **Keyboard-Shortcuts** (1 Std) → Macht tägliche Nutzung viel schneller

---

## 📝 Notizen

- Alle Features sind mit geschätztem Aufwand versehen
- Kritische Features haben direkte Auswirkung auf Nutzbarkeit
- UX-Features verbessern Komfort ohne neue Funktionalität
- Bonus-Features sind "nice-to-have" für später

**Letztes Update:** 13.11.2025
