import { defineStore } from 'pinia';
import { AvailableLocales, setLocale } from '../locales';
import { load } from '@tauri-apps/plugin-store';

export type ThemePreset =
	| 'warm'
	| 'cool'
	| 'forest'
	| 'ocean'
	| 'sunset'
	| 'midnight'
	| 'rose'
	| 'golden';

export const themePresets: Record<ThemePreset, string> = {
	warm: 'linear-gradient(135deg, rgba(230, 180, 180, 0.9), rgba(244, 200, 180, 0.8))',
	cool: 'linear-gradient(135deg, rgba(180, 200, 230, 0.9), rgba(190, 180, 244, 0.8))',
	forest: 'linear-gradient(135deg, rgba(180, 220, 180, 0.9), rgba(200, 230, 180, 0.8))',
	ocean: 'linear-gradient(135deg, rgba(180, 220, 230, 0.9), rgba(180, 240, 230, 0.8))',
	sunset: 'linear-gradient(135deg, rgba(255, 180, 180, 0.9), rgba(255, 210, 180, 0.8))',
	midnight: 'linear-gradient(135deg, rgba(180, 180, 220, 0.9), rgba(210, 180, 230, 0.8))',
	rose: 'linear-gradient(135deg, rgba(240, 180, 210, 0.9), rgba(230, 180, 200, 0.8))',
	golden: 'linear-gradient(135deg, rgba(255, 230, 180, 0.9), rgba(255, 210, 160, 0.8))',
};

export type Settings = {
	theme: ThemePreset;
	language: AvailableLocales;
	volume: number;
	normalizeAudio: boolean;
	crossfade: number;
};

export const useSettings = defineStore('settings', {
	state: (): Settings => ({
		theme: 'warm',
		language: 'en',
		volume: 100,
		normalizeAudio: false,
		crossfade: 0,
	}),
	actions: {
		setTheme(theme: ThemePreset) {
			this.theme = theme;
			document.documentElement.style.setProperty(
				'--app-gradient',
				themePresets[theme]
			);
			this.saveSettings();
		},
		setLanguage(language: AvailableLocales) {
			this.language = language;
			setLocale(language);
			this.saveSettings();
		},
		async loadSettings() {
			try {
				const store = await load('settings.json');
				const savedSettings =
					(await store.get<Partial<Settings>>('settings')) ?? {};

				if (savedSettings.theme) {
					this.setTheme(savedSettings.theme as ThemePreset);
				}
				if (savedSettings.language) {
					this.setLanguage(savedSettings.language as AvailableLocales);
				}
			} catch (error) {
				console.error('Failed to load settings:', error);
			}
		},
		async saveSettings() {
			try {
				const store = await load('settings.json');
				await store.set('settings', {
					theme: this.theme,
					language: this.language,
				});
			} catch (error) {
				console.error('Failed to save settings:', error);
			}
		},
	},
});
