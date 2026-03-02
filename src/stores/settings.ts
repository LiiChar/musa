import { defineStore } from 'pinia';
import { AvailableLocales, setLocale } from '../locales';
import { load, Store } from '@tauri-apps/plugin-store';

export type ThemePreset =
	| 'forest'
	| 'ocean'
	| 'sunset'
	| 'midnight'

export const themePresets: Record<ThemePreset, string> = {
	midnight: `
		radial-gradient(1200px at 80% 20%, rgba(80, 100, 255, 0.25), transparent),
		radial-gradient(1000px at 20% 80%, rgba(60, 80, 200, 0.2), transparent),
		linear-gradient(135deg, #0b0d11, #14171f)
	`,

	ocean: `
		radial-gradient(1200px at 90% 10%, rgba(0, 180, 200, 0.25), transparent),
		radial-gradient(1000px at 10% 90%, rgba(0, 120, 150, 0.2), transparent),
		linear-gradient(135deg, #0c1114, #121923)
	`,

	sunset: `
		radial-gradient(1200px at 85% 0%, rgba(255, 120, 80, 0.25), transparent),
		radial-gradient(1000px at 0% 100%, rgba(180, 60, 120, 0.2), transparent),
		linear-gradient(135deg, #140f16, #1b1624)
	`,

	forest: `
		radial-gradient(1200px at 80% 20%, rgba(80, 200, 120, 0.25), transparent),
		radial-gradient(1000px at 0% 100%, rgba(40, 120, 80, 0.2), transparent),
		linear-gradient(135deg, #0d1512, #16211c)
	`,
};

let tauriStore: Store | null = null;

export type Settings = {
	theme: ThemePreset;
	language: AvailableLocales;
	volume: number;
	normalizeAudio: boolean;
	crossfade: number;
};
export const useSettings = defineStore('settings', {
	state: (): Settings => ({
		theme: 'sunset',
		language: 'en',
		volume: 100,
		normalizeAudio: false,
		crossfade: 0,
	}),

	actions: {
		async initStore() {
			if (!tauriStore) {
				tauriStore = await load('settings.json');
			}
		},

		async setTheme(theme: ThemePreset) {
			this.theme = theme;

			document.documentElement.style.setProperty(
				'--app-gradient',
				themePresets[theme],
			);

			await this.saveSettings();
		},

		async setLanguage(language: AvailableLocales) {
			this.language = language;
			setLocale(language);
			await this.saveSettings();
		},

		async loadSettings() {
			try {
				await this.initStore();

				const savedSettings =
					(await tauriStore!.get<Partial<Settings>>('settings')) ?? {};

				Object.assign(this, savedSettings);

				// применяем визуальные настройки
				document.documentElement.style.setProperty(
					'--app-gradient',
					themePresets[this.theme],
				);

				setLocale(this.language);
			} catch (error) {
				console.error('Failed to load settings:', error);
			}
		},

		async saveSettings() {
			try {
				await this.initStore();

				await tauriStore!.set('settings', {
					theme: this.theme,
					language: this.language,
					volume: this.volume,
					normalizeAudio: this.normalizeAudio,
					crossfade: this.crossfade,
				});

				await tauriStore!.save(); // ВОТ ЭТО КЛЮЧЕВОЕ
			} catch (error) {
				console.error('Failed to save settings:', error);
			}
		},
	},
});