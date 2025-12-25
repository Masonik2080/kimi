import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Keyboard, Power } from "lucide-react";
import { Button } from "@/components/ui/button";

interface HotkeySettings {
  enabled: boolean;
  modifier: string;
}

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  t: {
    settings?: {
      title?: string;
      hotkeys?: string;
      hotkeyEnabled?: string;
      hotkeyModifier?: string;
      modifierAlt?: string;
      modifierCtrlAlt?: string;
      modifierCtrlShift?: string;
      hotkeyHint?: string;
      autostart?: string;
      autostartEnabled?: string;
      autostartHint?: string;
      save?: string;
      close?: string;
    };
  };
}

export function Settings({ isOpen, onClose, t }: SettingsProps) {
  const [hotkeySettings, setHotkeySettings] = useState<HotkeySettings>({
    enabled: true,
    modifier: "alt",
  });
  const [autostartEnabled, setAutostartEnabled] = useState(false);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (isOpen) {
      loadSettings();
    }
  }, [isOpen]);

  const loadSettings = async () => {
    try {
      const hotkeys = await invoke<HotkeySettings>("get_hotkey_settings");
      setHotkeySettings(hotkeys);
      
      const autostart = await invoke<boolean>("get_autostart_enabled");
      setAutostartEnabled(autostart);
    } catch (e) {
      console.error("Failed to load settings:", e);
    }
  };

  const saveSettings = async () => {
    setSaving(true);
    try {
      await invoke("set_hotkey_settings", { settings: hotkeySettings });
      await invoke("set_autostart_enabled", { enabled: autostartEnabled });
      onClose();
    } catch (e) {
      console.error("Failed to save settings:", e);
    }
    setSaving(false);
  };

  const getModifierDisplay = (modifier: string) => {
    switch (modifier) {
      case "ctrl+alt": return "Ctrl + Alt";
      case "ctrl+shift": return "Ctrl + Shift";
      default: return "Alt";
    }
  };

  if (!isOpen) return null;

  const st = t.settings || {};

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-md mx-4">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-neutral-200">
          <h2 className="text-sm font-semibold text-neutral-900">
            {st.title || "Настройки"}
          </h2>
          <button
            onClick={onClose}
            className="p-1 text-neutral-400 hover:text-neutral-600 transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content */}
        <div className="p-4 space-y-5">
          {/* Autostart section */}
          <div className="space-y-3">
            <div className="flex items-center gap-2 text-neutral-700">
              <Power className="w-4 h-4" />
              <span className="text-sm font-medium">{st.autostart || "Автозапуск"}</span>
            </div>

            <label className="flex items-center justify-between py-2 px-3 bg-neutral-50 rounded-md cursor-pointer">
              <span className="text-sm text-neutral-700">
                {st.autostartEnabled || "Запускать при старте Windows"}
              </span>
              <input
                type="checkbox"
                checked={autostartEnabled}
                onChange={(e) => setAutostartEnabled(e.target.checked)}
                className="w-4 h-4 accent-neutral-900"
              />
            </label>
            
            <p className="text-xs text-neutral-500 px-1">
              {st.autostartHint || "Kimi будет автоматически запускаться при входе в систему"}
            </p>
          </div>

          {/* Divider */}
          <div className="h-px bg-neutral-200" />

          {/* Hotkeys section */}
          <div className="space-y-3">
            <div className="flex items-center gap-2 text-neutral-700">
              <Keyboard className="w-4 h-4" />
              <span className="text-sm font-medium">{st.hotkeys || "Горячие клавиши"}</span>
            </div>

            {/* Enable toggle */}
            <label className="flex items-center justify-between py-2 px-3 bg-neutral-50 rounded-md cursor-pointer">
              <span className="text-sm text-neutral-700">
                {st.hotkeyEnabled || "Включить горячие клавиши"}
              </span>
              <input
                type="checkbox"
                checked={hotkeySettings.enabled}
                onChange={(e) => setHotkeySettings({ ...hotkeySettings, enabled: e.target.checked })}
                className="w-4 h-4 accent-neutral-900"
              />
            </label>

            {/* Modifier select */}
            <div className="py-2 px-3 bg-neutral-50 rounded-md">
              <label className="text-sm text-neutral-700 block mb-2">
                {st.hotkeyModifier || "Модификатор"}
              </label>
              <select
                value={hotkeySettings.modifier}
                onChange={(e) => setHotkeySettings({ ...hotkeySettings, modifier: e.target.value })}
                disabled={!hotkeySettings.enabled}
                className="w-full px-2 py-1.5 text-sm border border-neutral-200 rounded-md bg-white disabled:opacity-50"
              >
                <option value="alt">{st.modifierAlt || "Alt"}</option>
                <option value="ctrl+alt">{st.modifierCtrlAlt || "Ctrl + Alt"}</option>
                <option value="ctrl+shift">{st.modifierCtrlShift || "Ctrl + Shift"}</option>
              </select>
            </div>

            {/* Hint */}
            <p className="text-xs text-neutral-500 px-1">
              {st.hotkeyHint || `Используйте ${getModifierDisplay(hotkeySettings.modifier)} + 1-9 для переключения рабочих столов`}
            </p>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-2 px-4 py-3 border-t border-neutral-200">
          <Button
            variant="ghost"
            size="sm"
            onClick={onClose}
            className="h-8 px-3 text-xs"
          >
            {st.close || "Закрыть"}
          </Button>
          <Button
            size="sm"
            onClick={saveSettings}
            disabled={saving}
            className="h-8 px-3 text-xs bg-neutral-900 hover:bg-neutral-800 text-white"
          >
            {st.save || "Сохранить"}
          </Button>
        </div>
      </div>
    </div>
  );
}
