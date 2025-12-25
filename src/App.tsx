import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@/components/ui/button";
import { TitleBar } from "@/components/TitleBar";
import { Settings } from "@/components/Settings";
import { Plus, Trash2, RefreshCw, RotateCcw, HelpCircle, ChevronDown, ChevronUp } from "lucide-react";
import { useLanguage } from "./hooks/useLanguage";

interface Desktop {
  id: number;
  name: string;
  path: string;
  is_active: boolean;
  file_count: number;
}

function App() {
  const [desktops, setDesktops] = useState<Desktop[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [faqOpen, setFaqOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const { lang, toggleLanguage, t } = useLanguage();

  const loadDesktops = async () => {
    try {
      const result = await invoke<Desktop[]>("get_desktops");
      setDesktops(result);
    } catch (e) {
      setError(String(e));
    }
  };

  useEffect(() => {
    loadDesktops();
    
    // Слушаем горячие клавиши
    const unlisten = listen<number>("hotkey-switch-desktop", async (event) => {
      const desktopIndex = event.payload;
      // Получаем актуальный список рабочих столов
      try {
        const currentDesktops = await invoke<Desktop[]>("get_desktops");
        if (desktopIndex >= 1 && desktopIndex <= currentDesktops.length) {
          const targetDesktop = currentDesktops[desktopIndex - 1];
          if (targetDesktop && !targetDesktop.is_active) {
            await invoke("switch_workspace", { kimiDesktopId: targetDesktop.id });
            await loadDesktops();
          }
        }
      } catch (e) {
        console.error("Hotkey switch error:", e);
      }
    });
    
    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const createDesktop = async () => {
    setLoading(true);
    setError(null);
    try {
      await invoke("create_desktop");
      await loadDesktops();
    } catch (e) {
      setError(String(e));
    }
    setLoading(false);
  };

  const switchDesktop = async (id: number) => {
    setDesktops(prev => prev.map(d => ({ ...d, is_active: d.id === id })));
    setError(null);
    try {
      // switch_workspace переключает и файлы, и виртуальный рабочий стол Windows
      await invoke("switch_workspace", { kimiDesktopId: id });
    } catch (e) {
      setError(String(e));
      await loadDesktops();
    }
  };

  const deleteDesktop = async (id: number) => {
    if (desktops.length <= 1) {
      setError(t.errors.cannotDeleteLast);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      await invoke("delete_desktop", { id });
      await loadDesktops();
    } catch (e) {
      setError(String(e));
    }
    setLoading(false);
  };

  const restoreOriginal = async () => {
    setLoading(true);
    setError(null);
    try {
      await invoke("restore_original_desktop");
      await loadDesktops();
    } catch (e) {
      setError(String(e));
    }
    setLoading(false);
  };

  const hasActiveDesktop = desktops.some(d => d.is_active);

  return (
    <div className="h-screen flex flex-col bg-white">
      <TitleBar 
        lang={lang} 
        onToggleLanguage={toggleLanguage} 
        onOpenSettings={() => setSettingsOpen(true)}
        appName={t.app.name} 
      />
      
      <Settings 
        isOpen={settingsOpen} 
        onClose={() => setSettingsOpen(false)} 
        t={t} 
      />
      
      <div className="flex-1 overflow-auto">
        <div className="max-w-2xl mx-auto px-3 sm:px-6 py-4 sm:py-8">
          {/* Header */}
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 mb-4 sm:mb-8">
            <div>
              <h1 className="text-base sm:text-lg font-semibold text-neutral-900">{t.header.title}</h1>
              <p className="text-neutral-500 text-xs sm:text-sm">{t.header.subtitle}</p>
            </div>
            <div className="flex items-center gap-1 sm:gap-1.5">
              {/* Refresh button */}
              <Button 
                variant="ghost" 
                size="sm"
                onClick={loadDesktops} 
                disabled={loading}
                className="h-8 px-2.5 text-xs text-neutral-500 hover:text-neutral-900 hover:bg-neutral-100 gap-1.5"
              >
                <RefreshCw className={`w-3.5 h-3.5 ${loading ? 'animate-spin' : ''}`} />
                <span className="hidden sm:inline">{t.actions.refresh}</span>
              </Button>
              
              {/* Reset button */}
              {hasActiveDesktop && (
                <Button 
                  variant="ghost"
                  size="sm"
                  onClick={restoreOriginal} 
                  disabled={loading}
                  className="h-8 px-2.5 text-xs text-neutral-500 hover:text-neutral-900 hover:bg-neutral-100 gap-1.5"
                >
                  <RotateCcw className="w-3.5 h-3.5" />
                  <span className="hidden sm:inline">{t.actions.reset}</span>
                </Button>
              )}
              
              {/* Divider */}
              <div className="w-px h-5 bg-neutral-200 mx-1 hidden sm:block" />
              
              {/* Create button */}
              <Button 
                onClick={createDesktop} 
                disabled={loading || desktops.length >= 20}
                size="sm"
                className="h-8 px-3 text-xs bg-neutral-900 hover:bg-neutral-800 text-white gap-1.5"
              >
                <Plus className="w-3.5 h-3.5" />
                <span>{t.actions.create}</span>
              </Button>
            </div>
          </div>

          {/* Error */}
          {error && (
            <div className="mb-4 sm:mb-6 px-2.5 sm:px-3 py-2 bg-neutral-100 border border-neutral-200 rounded-md text-neutral-600 text-xs sm:text-sm">
              {error}
            </div>
          )}

          {/* Desktop list */}
          <div className="space-y-0.5 sm:space-y-1">
            {desktops.map((desktop) => (
              <div
                key={desktop.id}
                onClick={() => switchDesktop(desktop.id)}
                className={`group flex items-center justify-between px-2.5 sm:px-3 py-2 sm:py-2.5 rounded-md cursor-pointer transition-colors ${
                  desktop.is_active
                    ? "bg-neutral-900 text-white"
                    : "hover:bg-neutral-50 text-neutral-900"
                }`}
              >
                <div className="flex items-center gap-2 sm:gap-3 min-w-0">
                  <div className={`w-1.5 h-1.5 sm:w-2 sm:h-2 rounded-full flex-shrink-0 ${desktop.is_active ? 'bg-white' : 'bg-neutral-300'}`} />
                  <span className="text-xs sm:text-sm font-medium truncate">{desktop.name}</span>
                  <span className={`text-[10px] sm:text-xs flex-shrink-0 ${desktop.is_active ? 'text-neutral-400' : 'text-neutral-400'}`}>
                    {desktop.file_count} {t.desktop.files}
                  </span>
                </div>
                
                {!desktop.is_active && (
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      deleteDesktop(desktop.id);
                    }}
                    className="opacity-0 group-hover:opacity-100 p-1 text-neutral-400 hover:text-neutral-900 transition-opacity flex-shrink-0"
                  >
                    <Trash2 className="w-3 h-3 sm:w-3.5 sm:h-3.5" />
                  </button>
                )}
              </div>
            ))}
          </div>

          {/* Empty state */}
          {desktops.length === 0 && (
            <div className="text-center py-10 sm:py-16">
              <p className="text-neutral-400 text-xs sm:text-sm mb-3 sm:mb-4">{t.empty.title}</p>
              <Button 
                onClick={createDesktop} 
                className="h-7 sm:h-8 text-xs sm:text-sm bg-neutral-900 hover:bg-neutral-800 text-white"
              >
                <Plus className="w-3 h-3 sm:w-3.5 sm:h-3.5 mr-1" />
                {t.empty.createFirst}
              </Button>
            </div>
          )}
        </div>
      </div>

      {/* Footer with FAQ */}
      <div className="border-t border-neutral-100">
        {/* FAQ Toggle */}
        <button
          onClick={() => setFaqOpen(!faqOpen)}
          className="w-full px-3 sm:px-6 py-2 sm:py-3 flex items-center justify-center gap-2 text-neutral-500 hover:text-neutral-700 transition-colors"
        >
          <HelpCircle className="w-3.5 h-3.5" />
          <span className="text-[10px] sm:text-xs">{t.faq?.title || "How does it work?"}</span>
          {faqOpen ? <ChevronDown className="w-3 h-3" /> : <ChevronUp className="w-3 h-3" />}
        </button>
        
        {/* FAQ Content */}
        <div 
          className={`overflow-hidden transition-all ease-in-out ${faqOpen ? 'max-h-96 opacity-100' : 'max-h-0 opacity-0'}`}
          style={{ transitionDuration: '400ms' }}
        >
          <div className="px-3 sm:px-6 pb-4 space-y-3 max-h-64 overflow-y-auto">
            {(t.faq?.items || []).map((item: { q: string; a: string }, i: number) => (
              <div 
                key={i} 
                className="text-xs"
                style={{ 
                  transitionDelay: faqOpen ? `${i * 50}ms` : '0ms',
                  opacity: faqOpen ? 1 : 0,
                  transform: faqOpen ? 'translateY(0)' : 'translateY(-8px)',
                  transition: 'opacity 300ms ease-out, transform 300ms ease-out'
                }}
              >
                <p className="font-medium text-neutral-700">{item.q}</p>
                <p className="text-neutral-500 mt-0.5">{item.a}</p>
              </div>
            ))}
          </div>
        </div>
        
        {/* Info line */}
        <div className="px-3 sm:px-6 py-1.5 text-center border-t border-neutral-50">
          <p className="text-[10px] text-neutral-400">{t.footer.info}</p>
        </div>
      </div>
    </div>
  );
}

export default App;
