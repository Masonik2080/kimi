import { useState, useEffect } from 'react';
import { Language, Translations, getTranslations, detectLanguage } from '../lang';

const STORAGE_KEY = 'kimi-language';

export function useLanguage() {
  const [lang, setLang] = useState<Language>(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === 'ru' || saved === 'en') return saved;
    return detectLanguage();
  });

  const [t, setT] = useState<Translations>(getTranslations(lang));

  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, lang);
    setT(getTranslations(lang));
  }, [lang]);

  const toggleLanguage = () => {
    setLang(prev => prev === 'ru' ? 'en' : 'ru');
  };

  return { lang, setLang, toggleLanguage, t };
}
