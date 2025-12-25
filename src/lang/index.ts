import ru from './ru.json';
import en from './en.json';

export type Language = 'ru' | 'en';

const translations = { ru, en };

export function getTranslations(lang: Language) {
  return translations[lang];
}

export function detectLanguage(): Language {
  const browserLang = navigator.language.slice(0, 2);
  return browserLang === 'ru' ? 'ru' : 'en';
}

export type Translations = typeof ru;
