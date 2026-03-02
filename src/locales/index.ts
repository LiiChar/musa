import en from './en.json';
import ru from './ru.json';
import { computed, ref } from 'vue';

export type AvailableLocales = 'en' | 'ru';

const messages = { en, ru } as const;

const currentLocale = ref<AvailableLocales>('en');

export function setLocale(locale: AvailableLocales) {
	currentLocale.value = locale;
	document.documentElement.lang = locale;
}

export function getLocale(): AvailableLocales {
	return currentLocale.value;
}

function translate(key: string): string {
	const keys = key.split('.');
	let value: unknown = messages[currentLocale.value];

	for (const k of keys) {
		if (value && typeof value === 'object' && k in value) {
			value = (value as Record<string, unknown>)[k];
		} else {
			// Fallback to English
			value = (messages.en as Record<string, unknown>)[k];
			if (!value) return key;
		}
	}

	return typeof value === 'string' ? value : key;
}

export function useI18n() {
	const t = (key: string) => translate(key);

	return { t, locale: currentLocale };
}

export default { setLocale, getLocale, useI18n };
