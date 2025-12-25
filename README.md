<p align="center">
  <img src="src-tauri/icons/icon.png" width="128" alt="Kimi Logo">
</p>

<h1 align="center">Kimi</h1>

<p align="center">
  Desktop manager for Windows
</p>

<p align="center">
  <a href="README.md"><b>English</b></a> • <a href="README.ru.md">Русский</a>
</p>

<p align="center">
  <a href="../../releases/latest"><img src="https://img.shields.io/github/v/release/Masonik2080/kimi?style=for-the-badge" alt="Release"></a>
  <a href="../../releases/latest"><img src="https://img.shields.io/github/downloads/Masonik2080/kimi/total?style=for-the-badge" alt="Downloads"></a>
  <img src="https://img.shields.io/badge/platform-Windows-blue?style=for-the-badge" alt="Platform">
</p>

---

<p align="center">
  <img src="screenshots/main_en.png" width="700" alt="Main window">
</p>

## Features

- Multiple desktops with separate files and icons
- Icon position saving for each desktop
- Hotkeys for quick switching
- Autostart with Windows
- System tray support

<p align="center">
  <img src="screenshots/settings_en.png" width="500" alt="Settings">
</p>

## How it works

<p align="center">
  <img src="screenshots/how_it_work_en.png" width="700" alt="How it works">
</p>

Kimi combines two approaches:

| Approach | Description |
|----------|-------------|
| Virtual Desktops | Uses Windows Virtual Desktops (Win+Tab) for desktop switching and window management |
| Separate Folders | Each desktop has its own folder with files and saved icon positions |

## Installation

<a href="../../releases/latest">
  <img src="https://img.shields.io/badge/Download-Latest%20Release-green?style=for-the-badge&logo=windows" alt="Download">
</a>

## Requirements

- Windows 10/11
- Administrator rights

## Build from source

```bash
git clone https://github.com/Masonik2080/kimi.git
cd kimi
npm install
npm run tauri build
```

## License

MIT
