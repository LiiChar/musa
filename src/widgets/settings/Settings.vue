<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useSettings, themePresets, type ThemePreset } from '../../stores/settings';
import { useI18n, type AvailableLocales } from '../../locales';
import Button from '../../components/ui/button.vue';
import IconCheck from '~icons/lucide/check';
import IconBack from '~icons/lucide/arrow-left';
import IconPalette from '~icons/lucide/palette';
import IconLanguage from '~icons/lucide/languages';
import IconVolume from '~icons/lucide/volume-2';
import IconSliders from '~icons/lucide/sliders-horizontal';

interface Props {
	onBack?: () => void;
}

const props = defineProps<Props>();

const settings = useSettings();
const { t } = useI18n();

const { theme, language, volume, normalizeAudio, crossfade } = storeToRefs(settings);

const handleThemeChange = (preset: ThemePreset) => {
	settings.setTheme(preset);
	settings.saveSettings();
};

const handleLanguageChange = (locale: AvailableLocales) => {
	settings.setLanguage(locale);
	settings.saveSettings();
};

const handleBack = () => {
	props.onBack?.();
};

const handleVolumeChange = (event: Event) => {
	const target = event.target as HTMLInputElement;
	settings.volume = Number(target.value);
	settings.saveSettings();
};

const handleCrossfadeChange = (event: Event) => {
	const target = event.target as HTMLInputElement;
	settings.crossfade = Number(target.value);
	settings.saveSettings();
};

const toggleNormalizeAudio = () => {
	settings.normalizeAudio = !settings.normalizeAudio;
	settings.saveSettings();
};
</script>

<template>
	<div class="settings_wrapper">
		<div class="settings_container">
			<div class="settings_header">
				<Button 
					variant="rounded" 
					@click="handleBack"
					class="back_button"
					title="Back to player"
				>
					<IconBack />
				</Button>
				<h1 class="settings_title">{{ t('settings.title') }}</h1>
			</div>

			<div class="settings_section">
				<h2 class="settings_section_title">
					<IconPalette style="vertical-align: middle; margin-right: 8px;" />
					{{ t('settings.appearance') }}
				</h2>
				
				<div class="settings_theme">
					<div class="settings_theme_label">{{ t('settings.theme') }}</div>
					<div class="theme_presets">
						<button
							v-for="(gradient, preset) in themePresets"
							:key="preset"
							@click="handleThemeChange(preset as ThemePreset)"
							:class="{
								theme_preset: true,
								active: theme === preset,
							}"
							:style="{ background: gradient }"
							:title="t(`settings.presets.${preset}`)"
						>
							<IconCheck
								v-if="theme === preset"
								class="preset_check"
							/>
							<span class="preset_name">{{ t(`settings.presets.${preset}`) }}</span>
						</button>
					</div>
				</div>
			</div>

			<div class="settings_section">
				<h2 class="settings_section_title">
					<IconLanguage style="vertical-align: middle; margin-right: 8px;" />
					{{ t('settings.language') }}
				</h2>

				<div class="settings_language">
					<Button
						:active="language === 'en'"
						@click="handleLanguageChange('en')"
						class="language_button"
					>
						🇬🇧 English
					</Button>
					<Button
						:active="language === 'ru'"
						@click="handleLanguageChange('ru')"
						class="language_button"
					>
						🇷🇺 Русский
					</Button>
				</div>
			</div>

			<div class="settings_section">
				<h2 class="settings_section_title">
					<IconVolume style="vertical-align: middle; margin-right: 8px;" />
					{{ t('settings.audio') }}
				</h2>

				<div class="settings_audio">
					<div class="setting_row">
						<label class="setting_label">{{ t('settings.volume') }}: {{ volume }}%</label>
						<input
							type="range"
							min="0"
							max="100"
							:value="volume"
							@input="handleVolumeChange"
							class="setting_slider"
						/>
					</div>

					<div class="setting_row">
						<label class="setting_label">{{ t('settings.crossfade') }}: {{ crossfade }}s</label>
						<input
							type="range"
							min="0"
							max="10"
							:value="crossfade"
							@input="handleCrossfadeChange"
							class="setting_slider"
						/>
					</div>

					<div class="setting_row">
						<label class="setting_label">{{ t('settings.normalizeAudio') }}</label>
						<button
							@click="toggleNormalizeAudio"
							:class="{ toggle_button: true, active: normalizeAudio }"
						>
							<IconCheck v-if="normalizeAudio" class="toggle_check" />
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<style scoped>
.settings_wrapper {
	width: 100%;
	height: 100%;
	padding-top: 64px;
	padding-bottom: 32px;
	padding-left: 24px;
	padding-right: 24px;
	overflow: auto;
}

.settings_wrapper::-webkit-scrollbar {
	width: 8px;
}

.settings_wrapper::-webkit-scrollbar-track {
	background: transparent;
}

.settings_wrapper::-webkit-scrollbar-thumb {
	background: var(--border-glass);
	border-radius: 4px;
}

.settings_wrapper::-webkit-scrollbar-thumb:hover {
	background: var(--secondary);
}

.settings_container {
	display: flex;
	flex-direction: column;
	
	gap: 24px;
	width: 100%;
	margin: 0 auto;
}

.settings_header {
	display: flex;
	align-items: center;
	gap: 16px;
	margin-bottom: 8px;
}

.back_button {
	width: 36px;
	height: 36px;
	display: flex;
	align-items: center;
	justify-content: center;
	border-radius: 50%;
	transition: all 0.2s ease;
	flex-shrink: 0;
}

.back_button:hover {
	background: rgba(255, 255, 255, 0.1);
}

.back_button svg {
	width: 20px;
	height: 20px;
}

.settings_title {
	font-size: 26px;
	margin: 0;
	font-weight: 600;
	letter-spacing: -0.5px;
}

.settings_section {
	display: flex;
	flex-direction: column;
	gap: 16px;
	padding: 20px;
	background: rgba(255, 255, 255, 0.05);
	border-radius: 12px;
	border: 1px solid rgba(255, 255, 255, 0.08);
	backdrop-filter: blur(10px);
}

.settings_section_title {
	font-size: 16px;
	margin: 0;
	font-weight: 600;
	color: var(--foreground);
	display: flex;
	align-items: center;
}

.settings_section_title svg {
	width: 18px;
	height: 18px;
}

.settings_theme_label {
	font-size: 13px;
	color: var(--foreground);
	margin-bottom: 8px;
	text-transform: uppercase;
	letter-spacing: 0.5px;
}

.theme_presets {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
	gap: 12px;
}

.theme_preset {
	position: relative;
	height: 70px;
	border-radius: 10px;
	border: 2px solid transparent;
	cursor: pointer;
	transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	overflow: hidden;
}

.theme_preset:hover {
	transform: translateY(-2px);
	border-color: rgba(255, 255, 255, 0.3);
	box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
}

.theme_preset.active {
	border-color: var(--foreground);
	box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.1), 0 8px 24px rgba(0, 0, 0, 0.3);
}

.preset_check {
	position: absolute;
	top: 6px;
	right: 6px;
	width: 18px;
	height: 18px;
	color: var(--foreground);
	filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.5));
}

.preset_name {
	font-size: 11px;
	color: var(--foreground);
	text-shadow: 0 2px 8px rgba(0, 0, 0, 0.8);
	font-weight: 600;
	letter-spacing: 0.3px;
}

.settings_language {
	display: flex;
	gap: 10px;
	flex-wrap: wrap;
}

.language_button {
	min-width: 110px;
	font-size: 14px;
}

.settings_audio {
	display: flex;
	flex-direction: column;
	gap: 20px;
}

.setting_row {
	display: flex;
	justify-content: space-between;
	align-items: center;
	gap: 16px;
}

.setting_label {
	font-size: 14px;
	color: var(--secondary-foreground);
	white-space: nowrap;
	font-weight: 500;
}

.setting_slider {
	flex: 1;
	height: 6px;
	border-radius: 3px;
	background: rgba(255, 255, 255, 0.1);
	appearance: none;
	cursor: pointer;
	transition: all 0.2s ease;
}

.setting_slider:hover {
	height: 8px;
}

.setting_slider::-webkit-slider-thumb {
	appearance: none;
	width: 16px;
	height: 16px;
	border-radius: 50%;
	background: var(--foreground);
	cursor: pointer;
	transition: all 0.2s ease;
	box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.setting_slider::-webkit-slider-thumb:hover {
	transform: scale(1.15);
}

.setting_slider::-moz-range-thumb {
	width: 16px;
	height: 16px;
	border-radius: 50%;
	background: var(--foreground);
	cursor: pointer;
	border: none;
	transition: all 0.2s ease;
}

.toggle_button {
	width: 48px;
	height: 26px;
	border-radius: 13px;
	background: rgba(255, 255, 255, 0.1);
	border: 1px solid rgba(255, 255, 255, 0.15);
	cursor: pointer;
	transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
	display: flex;
	align-items: center;
	padding: 0 4px;
	flex-shrink: 0;
	position: relative;
}

.toggle_button::before {
	content: '';
	position: absolute;
	left: 3px;
	width: 18px;
	height: 18px;
	border-radius: 50%;
	background: var(--secondary-foreground);
	transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.toggle_button.active {
	background: var(--foreground);
	border-color: var(--foreground);
}

.toggle_button.active::before {
	left: 25px;
	background: var(--background);
}

.toggle_button.active .toggle_check {
	color: var(--background);
}

.toggle_check {
	width: 14px;
	height: 14px;
	margin-left: auto;
	transition: all 0.2s ease;
	z-index: 1;
	opacity: 0;
}

.toggle_button.active .toggle_check {
	opacity: 1;
}
</style>
