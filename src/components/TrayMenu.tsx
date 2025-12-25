import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Monitor, LogOut, Eye, Check } from "lucide-react";

interface Desktop {
  id: number;
  name: string;
  path: string;
  is_active: boolean;
  file_count: number;
}

export function TrayMenu() {
  const [desktops, setDesktops] = useState<Desktop[]>([]);

  useEffect(() => {
    loadDesktops();
    
    // Отключаем контекстное меню браузера
    const preventContextMenu = (e: MouseEvent) => {
      e.preventDefault();
    };
    document.addEventListener('contextmenu', preventContextMenu);

    return () => {
      document.removeEventListener('contextmenu', preventContextMenu);
    };
  }, []);

  const loadDesktops = async () => {
    try {
      const result = await invoke<Desktop[]>("get_desktops");
      setDesktops(result);
    } catch (e) {
      console.error(e);
    }
  };

  const showMainWindow = async () => {
    await invoke("show_main_window");
    await getCurrentWindow().hide();
  };

  const switchDesktop = async (id: number) => {
    try {
      await invoke("switch_workspace", { kimiDesktopId: id });
      await loadDesktops();
    } catch (e) {
      console.error(e);
    }
  };

  const handleExit = async () => {
    await invoke("exit_app");
  };

  return (
    <div id="tray-root" className="w-full h-full p-1.5" style={{ background: 'transparent' }}>
      <div className="bg-white rounded-lg shadow-xl border border-neutral-200/80 overflow-hidden">
        {/* Показать */}
        <button
          onClick={showMainWindow}
          className="w-full px-3 py-2 flex items-center gap-2.5 hover:bg-neutral-100 transition-colors text-left"
        >
          <Eye className="w-3.5 h-3.5 text-neutral-500" />
          <span className="text-xs text-neutral-700">Показать</span>
        </button>

        {/* Разделитель */}
        <div className="h-px bg-neutral-100" />

        {/* Рабочие столы */}
        <div className="py-0.5">
          <div className="px-3 py-1 flex items-center gap-2">
            <Monitor className="w-3 h-3 text-neutral-400" />
            <span className="text-[10px] text-neutral-400 font-medium uppercase tracking-wide">
              Рабочие столы
            </span>
          </div>
          
          {desktops.length === 0 ? (
            <div className="px-3 py-1.5 text-[10px] text-neutral-400">
              Нет рабочих столов
            </div>
          ) : (
            <div className="max-h-32 overflow-y-auto">
              {desktops.map((desktop) => (
                <button
                  key={desktop.id}
                  onClick={() => switchDesktop(desktop.id)}
                  className={`w-full px-3 py-1.5 flex items-center justify-between transition-colors text-left ${
                    desktop.is_active 
                      ? "bg-neutral-900 text-white" 
                      : "hover:bg-neutral-100 text-neutral-700"
                  }`}
                >
                  <div className="flex items-center gap-2 min-w-0">
                    <div className={`w-1 h-1 rounded-full flex-shrink-0 ${
                      desktop.is_active ? "bg-white" : "bg-neutral-300"
                    }`} />
                    <span className="text-xs truncate">{desktop.name}</span>
                  </div>
                  {desktop.is_active && (
                    <Check className="w-3 h-3 flex-shrink-0" />
                  )}
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Разделитель */}
        <div className="h-px bg-neutral-100" />

        {/* Выход */}
        <button
          onClick={handleExit}
          className="w-full px-3 py-2 flex items-center gap-2.5 hover:bg-red-50 transition-colors text-left group"
        >
          <LogOut className="w-3.5 h-3.5 text-neutral-500 group-hover:text-red-500" />
          <span className="text-xs text-neutral-700 group-hover:text-red-600">Выход</span>
        </button>
      </div>
    </div>
  );
}
