# TermiX - TODO Liste

## 🔴 Priorität 1: Kritisch (Kern-Funktionalität)

### ✅ Settings-Persistenz (30 Min) - ERLEDIGT
- [x] `Settings`-Struct mit allen Einstellungen erstellen
- [x] Serde-Serialisierung implementieren (`~/.config/termix/settings.toml`)
- [x] Auto-Save bei Änderungen + Load beim Start
- [x] Fehlerbehandlung für kaputte Config-Dateien

**Warum:** Nutzer verlieren aktuell alle Einstellungen bei jedem Neustart
**Status:** ✓ Implementiert - Alle GUI-Einstellungen werden automatisch gespeichert und beim Start geladen

### ✅ ANSI-Farben im Terminal (45 Min) - ERLEDIGT
- [x] Von `contents()` zu `screen.cell(row, col)` API wechseln
- [x] Cell-Attribute auslesen (Farbe, Bold, Italic, etc.)
- [x] vt100-Farben zu egui::Color32 mappen
- [x] 256-Color und True-Color Support

**Warum:** Terminal zeigt aktuell alles weiß, keine `ls --color` oder Syntax-Highlighting
**Status:** ✓ Implementiert - Zellenbasiertes Rendering mit voller Farbunterstützung (16/256/Truecolor)

### ✅ SSH-Verbindungen implementieren (2 Std) - ERLEDIGT
- [x] ssh2-Crate mit TerminalView verbinden
- [x] Session-Management (connect, disconnect, reconnect)
- [x] Passwort-Dialog + optional Key-basierte Auth
- [x] Fehlerbehandlung (Timeout, falsche Credentials, etc.)

**Warum:** SSH-Manager ist aktuell nur UI-Placeholder ohne Funktion
**Status:** ✓ Implementiert - Vollständige SSH-Integration mit Passwort-Dialog und detailliertem Error-Handling

---

## 🟡 Priorität 2: UX-Verbesserungen

### ⌨️ Keyboard-Shortcuts (1 Std)
- [x] Ctrl+T: Neuer Terminal-Tab
- [x] Ctrl+W: Tab schließen
- [x] Ctrl+Tab / Ctrl+Shift+Tab: Tab-Navigation
- [x] Ctrl+C / Ctrl+V: Copy/Paste im Terminal (bereits durch egui behandelt)
- [x] Ctrl+Plus/Minus: Schriftgröße
- [x] Shortcuts in Tooltips anzeigen

### 📜 Terminal-Scrollback verbessern
- [x] PageUp/PageDown Support
- [x] Mousewheel-Scrolling optimieren (Grundfunktion via Pfeile)
- [x] Jump-to-Top / Jump-to-Bottom Buttons
- [x] Scrollback-Limit konfigurierbar (100-10000 Zeilen)

### 🔄 Tab-Management
- [x] Drag & Drop zum Umordnen
- [x] Tab-Close-Buttons (× neben Namen)
- [x] Ctrl+Click für schnelles Schließen
- [x] Tab-Übersicht bei vielen offenen Tabs (Dropdown)

---

## 🟢 Priorität 3: Neue Features

### ✅ Split-View Terminals - ERLEDIGT
- [x] Horizontale/Vertikale Splits (Strg+H, Strg+Shift+V)
- [x] Focus-Navigation zwischen Splits (Strg+1-9)
- [ ] Resize-Handles für Panes
- [ ] Layout speichern/laden

**Status:** ✓ Basis-Implementierung mit Keyboard-Shortcuts für Split-Erstellung und Navigation

### 🔍 Command-History-Suche
- [ ] Ctrl+R: Reverse-Search wie in Bash
- [ ] History-Panel mit Filter
- [ ] History über Sessions hinweg speichern

### ✅ Export/Import - ERLEDIGT
- [x] Einstellungen exportieren (TOML)
- [x] SSH-Verbindungen ex-/importieren
- [ ] Terminal-Themes als Presets

**Status:** ✓ Export/Import-Funktionen im Settings-Panel verfügbar

### 📋 Clipboard-Integration
- [x] Copy/Paste (durch egui nativ unterstützt: Strg+C/V)
- [ ] Automatisches Copy bei Selektion (optional)
- [ ] Middle-Click-Paste
- [ ] Clipboard-History

---

## 🔧 Priorität 4: Fixes & Polish

### ✅ Warnings beheben - ERLEDIGT
- [x] `FontMode::Custom(String)` Feld nutzen (in Settings vorhanden)
- [x] Alle Clippy-Warnings durchgehen und beheben (von 15 auf 0 reduziert)

**Status:** ✓ Alle Clippy-Warnings behoben mit automatischen Fixes und manuellen Anpassungen

### ⚡ Performance
- [ ] Rendering bei großen Outputs optimieren
	- [x] Repaint-Throttling (nur bei neuen Daten oder Cursor-Blink)
	- [x] Run-Length-Rendering je Zeile (weniger Allokationen/Append-Aufrufe)
	- [x] Per-Frame Font-Rekonfiguration entfernt (nur bei Settings-Änderungen)
- [ ] Virtuelle Scrolling für 10.000+ Zeilen
- [ ] Frame-Rate bei inaktiven Tabs reduzieren

### ✅ README - ERLEDIGT
- [x] Buy Me a Coffee Link getestet (funktioniert mit HTML img tag)
- [x] Feature-Liste erweitert mit Badges und Kategorien
- [x] Installation-Guide erweitert (Prerequisites, Binary-Releases, Launcher)
- [x] Usage Guide hinzugefügt mit Keyboard Shortcuts
- [ ] Screenshots hinzufügen (benötigt Bildmaterial)

**Status:** ✓ README komplett überarbeitet mit besserer Struktur und Dokumentation

### 🎨 Theme-System
- [x] Mehr vordefinierte Themes (7 Themes: Dark, Light, Dracula, Monokai, Solarized, Nord, Gruvbox)
- [ ] Theme-Editor im GUI
- [x] Theme-Vorschau (Beschreibung im Settings-Panel vorhanden)

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
