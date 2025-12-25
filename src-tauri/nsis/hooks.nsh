; Kimi NSIS Hooks
; Восстанавливает оригинальный рабочий стол при удалении

!macro NSIS_HOOK_PREUNINSTALL
    ; Читаем текущий путь Desktop из реестра
    ReadRegStr $0 HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders" "Desktop"
    
    ; Проверяем начинается ли путь с C:\Kimi
    StrCpy $1 $0 7
    StrCmp $1 "C:\Kimi" 0 skip_restore
    
    ; Получаем реальный путь к Desktop пользователя
    ; $PROFILE = C:\Users\Username
    StrCpy $2 "$PROFILE\Desktop"
    
    ; Восстанавливаем путь
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders" "Desktop" "$2"
    
    ; Уведомляем Explorer об изменении
    System::Call 'Shell32::SHChangeNotify(i 0x8000000, i 0x1000, p 0, p 0)'
    
skip_restore:

    ; Создаём файл с информацией о сохранённых данных
    ; $PROFILE\Desktop - реальный путь к рабочему столу
    ClearErrors
    FileOpen $3 "$PROFILE\Desktop\Kimi - ваши файлы.txt" w
    IfErrors skip_file
    
    FileWrite $3 "Kimi был удалён, но ваши файлы сохранены!$\r$\n"
    FileWrite $3 "$\r$\n"
    FileWrite $3 "Ваши рабочие столы находятся в папке:$\r$\n"
    FileWrite $3 "C:\Kimi\$\r$\n"
    FileWrite $3 "$\r$\n"
    FileWrite $3 "Структура папок:$\r$\n"
    FileWrite $3 "  C:\Kimi\Desktop1\ - Рабочий стол 1$\r$\n"
    FileWrite $3 "  C:\Kimi\Desktop2\ - Рабочий стол 2$\r$\n"
    FileWrite $3 "  ... и т.д.$\r$\n"
    FileWrite $3 "$\r$\n"
    FileWrite $3 "Вы можете:$\r$\n"
    FileWrite $3 "  - Скопировать нужные файлы на обычный рабочий стол$\r$\n"
    FileWrite $3 "  - Удалить папку C:\Kimi\ если файлы больше не нужны$\r$\n"
    FileWrite $3 "  - Переустановить Kimi для продолжения работы$\r$\n"
    FileClose $3
    
skip_file:
!macroend
