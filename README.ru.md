<p align="center">
  <img src="src-tauri/icons/icon.png" width="128" alt="Kimi Logo">
</p>

<h1 align="center">Kimi</h1>

<p align="center">
  Менеджер рабочих столов для Windows
</p>

<p align="center">
  <a href="README.md">English</a> • <a href="README.ru.md"><b>Русский</b></a>
</p>

<p align="center">
  <a href="../../releases/latest"><img src="https://img.shields.io/github/v/release/Masonik2080/kimi?style=for-the-badge" alt="Релиз"></a>
  <a href="../../releases/latest"><img src="https://img.shields.io/github/downloads/Masonik2080/kimi/total?style=for-the-badge" alt="Загрузки"></a>
  <img src="https://img.shields.io/badge/platform-Windows-blue?style=for-the-badge" alt="Платформа">
</p>

---

<p align="center">
  <img src="screenshots/main_ru.png" width="700" alt="Главное окно">
</p>

## Возможности

- Несколько рабочих столов с отдельными файлами и иконками
- Сохранение позиций иконок для каждого стола
- Горячие клавиши для быстрого переключения
- Автозапуск с Windows
- Работа в трее

<p align="center">
  <img src="screenshots/settings_ru.png" width="500" alt="Настройки">
</p>

## Как работает

<p align="center">
  <img src="screenshots/how_it_work_ru.png" width="700" alt="Как работает">
</p>

Kimi комбинирует два подхода:

| Подход | Описание |
|--------|----------|
| Виртуальные столы | Использует виртуальные рабочие столы Windows (Win+Tab) для переключения и управления окнами |
| Отдельные папки | Каждый стол имеет свою папку с файлами и сохранёнными позициями иконок |

## Установка

<a href="../../releases/latest">
  <img src="https://img.shields.io/badge/Скачать-Последний%20релиз-green?style=for-the-badge&logo=windows" alt="Скачать">
</a>

## Требования

- Windows 10/11
- Права администратора

## Сборка из исходников

```bash
git clone https://github.com/Masonik2080/kimi.git
cd kimi
npm install
npm run tauri build
```

## Лицензия

MIT
