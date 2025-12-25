; Восстанавливаем оригинальный рабочий стол перед удалением Kimi

; Читаем текущий путь Desktop из реестра
ReadRegStr $0 HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders" "Desktop"

; Проверяем начинается ли он с C:\Kimi
StrCpy $1 $0 7
StrCmp $1 "C:\Kimi" 0 skip_restore

    ; Восстанавливаем стандартный путь %USERPROFILE%\Desktop
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders" "Desktop" "%USERPROFILE%\Desktop"
    
    ; Обновляем Explorer
    System::Call 'Shell32::SHChangeNotify(i 0x8000000, i 0, p 0, p 0)'

skip_restore:
