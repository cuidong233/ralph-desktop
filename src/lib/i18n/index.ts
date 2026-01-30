import { register, init, locale } from 'svelte-i18n';

export const supportedLocales = [
  'en',
  'zh-CN',
  'zh-TW',
  'es',
  'hi',
  'ar',
  'pt',
  'ru',
  'ja',
  'de',
  'fr',
  'bn'
] as const;

export type SupportedLocale = (typeof supportedLocales)[number];

supportedLocales.forEach((lang) => {
  register(lang, () => import(`./locales/${lang}.json`));
});

init({
  fallbackLocale: 'en',
  initialLocale: 'en'
});

function normalizeLocale(input: string): string {
  return input.replace('_', '-');
}

export function resolveLocale(configLanguage?: string): SupportedLocale {
  if (configLanguage && configLanguage !== 'system') {
    return (supportedLocales.includes(configLanguage as SupportedLocale)
      ? configLanguage
      : 'en') as SupportedLocale;
  }

  if (typeof navigator !== 'undefined') {
    const nav = normalizeLocale(navigator.language || '');
    if (supportedLocales.includes(nav as SupportedLocale)) {
      return nav as SupportedLocale;
    }
    const prefix = nav.split('-')[0];
    const match = supportedLocales.find((l) => l.startsWith(prefix));
    if (match) return match;
  }

  return 'en';
}

export function setLocaleFromConfig(configLanguage?: string): SupportedLocale {
  const resolved = resolveLocale(configLanguage);
  locale.set(resolved);
  return resolved;
}
