import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X, Settings } from "lucide-react";
import { Language } from "../lang";

const appWindow = getCurrentWindow();

interface TitleBarProps {
  lang: Language;
  onToggleLanguage: () => void;
  onOpenSettings: () => void;
  appName: string;
}

export function TitleBar({ lang, onToggleLanguage, onOpenSettings, appName }: TitleBarProps) {
  const handleMinimize = async () => await appWindow.minimize();
  const handleMaximize = async () => await appWindow.toggleMaximize();
  const handleClose = async () => await appWindow.close();

  return (
    <div className="h-10 flex items-center justify-between xs:justify-between justify-center bg-white border-b border-neutral-200 select-none">
      <div 
        data-tauri-drag-region 
        className="hidden xs:flex flex-1 items-center gap-2 px-3 h-full"
      >
        <div className="w-5 h-5 rounded-md bg-black flex items-center justify-center">
          <span className="text-[10px] font-bold text-white">K</span>
        </div>
        <span className="text-sm font-medium text-neutral-900">{appName}</span>
      </div>

      {/* На маленьких экранах - drag region слева */}
      <div 
        data-tauri-drag-region 
        className="xs:hidden flex-1 h-full"
      />

      <div className="flex items-center gap-1.5 px-2">
        {/* Настройки */}
        <button
          onClick={onOpenSettings}
          title=""
          className="w-7 h-7 flex items-center justify-center rounded-md text-neutral-400 hover:bg-neutral-100 hover:text-neutral-600 transition-all duration-150"
        >
          <Settings className="w-3.5 h-3.5" strokeWidth={2} />
        </button>
        
        {/* Переключатель языка */}
        <button
          onClick={onToggleLanguage}
          title=""
          className="w-7 h-7 flex items-center justify-center rounded-md text-neutral-400 hover:bg-neutral-100 hover:text-neutral-600 transition-all duration-150 text-[10px] font-medium"
        >
          {lang.toUpperCase()}
        </button>
        
        <div className="w-px h-4 bg-neutral-200 mx-0.5" />
        
        <button
          onClick={handleMinimize}
          title=""
          className="w-7 h-7 flex items-center justify-center rounded-md text-neutral-400 hover:bg-neutral-100 hover:text-neutral-600 transition-all duration-150"
        >
          <Minus className="w-3.5 h-3.5" strokeWidth={2} />
        </button>
        <button
          onClick={handleMaximize}
          title=""
          className="w-7 h-7 flex items-center justify-center rounded-md text-neutral-400 hover:bg-neutral-100 hover:text-neutral-600 transition-all duration-150"
        >
          <Square className="w-3 h-3" strokeWidth={2} />
        </button>
        <button
          onClick={handleClose}
          title=""
          className="w-7 h-7 flex items-center justify-center rounded-md text-neutral-400 hover:bg-red-500 hover:text-white transition-all duration-150"
        >
          <X className="w-3.5 h-3.5" strokeWidth={2} />
        </button>
      </div>

      {/* На маленьких экранах - drag region справа для баланса */}
      <div 
        data-tauri-drag-region 
        className="xs:hidden flex-1 h-full"
      />
    </div>
  );
}
