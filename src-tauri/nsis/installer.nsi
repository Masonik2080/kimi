; Кастомный NSIS скрипт для Kimi
; Добавляет восстановление рабочего стола при удалении

!include "MUI2.nsh"
!include "FileFunc.nsh"

; Секция удаления - восстанавливаем рабочий стол
Section "Uninstall"
    ; Читаем текущий путь Desktop
    ReadRegStr $0 HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders" "Desktop"
    
    ; Проверяем начинается ли с C:\Kimi
    StrCpy $1 $0 7
    StrCmp $1 "C:\Kimi" 0 +3
        ; Восстанавливаем стандартный путь
        WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders" "Desktop" "%USERPROFILE%\Desktop"
        ; Уведомляем систему
        System::Call 'Shell32::SHChangeNotify(i 0x8000000, i 0, p 0, p 0)'
SectionEnd
