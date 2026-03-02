<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useMusa } from '../../stores/musa';
import VolumeControll from '../../components/ui/volume-controll.vue';
import { getUrl } from '../../utils/url';
import SidebarToggle from '../../components/ui/sidebar-toggle.vue';
import Button from '../../components/ui/button.vue';
import { onBeforeUnmount, onMounted, onUnmounted, watch, inject } from 'vue';
import { formattedTime } from '../../utils/time';
import Timeline from '../../components/ui/timeline.vue';
import { useLayout } from '../../stores/layout';
import { useSettings, themePresets } from '../../stores/settings';
import { listen } from '@tauri-apps/api/event';
// ts-ignore
import colorthief from 'colorthief';
import IconPlay from '~icons/lucide/play';
import IconPause from '~icons/lucide/pause';
import IconSkipPrevious from '~icons/lucide/skip-back';
import IconSkipNext from '~icons/lucide/skip-forward';
import IconShuffle from '~icons/lucide/shuffle';
import IconRepeat from '~icons/lucide/repeat';
import IconSettings from '~icons/lucide/settings';
import IconVinyl from '~icons/lucide/disc-3';
import IconVolume from '~icons/lucide/volume';
import IconVolume1 from '~icons/lucide/volume-1';
import IconVolume2 from '~icons/lucide/volume-2';
import IconVolumeX from '~icons/lucide/volume-x';

const musaStore = useMusa();
const layoutStore = useLayout();
const settingsStore = useSettings();

const mainComponent = inject<{ navigateTo: (page: 'player' | 'settings') => void } | null>('mainComponent', null);

const { music, volume, isPlaying, time } = storeToRefs(musaStore);
const { visible } = storeToRefs(layoutStore);
const { theme } = storeToRefs(settingsStore);

const handlePlayMusic = async () => {
	if (!music.value) return;

	if (isPlaying.value) {
		musaStore.pauseMusic();
	} else {
		musaStore.playMusic();
	}
};

const navigateToSettings = () => {
	mainComponent?.navigateTo('settings');
};

watch(music, async () => {
	if (music.value?.cover) {
		// Extract colors from album cover
		const color = new colorthief();
		const image = new Image();
		image.crossOrigin = 'Anonymous';
		image.src = getUrl(music.value.cover);

		await new Promise((resolve) => {
			image.onload = resolve;
		});

		const palette: [number, number, number][] = await color.getPalette(image, 5);
		
		if (palette && palette.length >= 2) {
			// Create vibrant gradient from dominant colors
			const [color1, color2] = palette;
			const gradient = `linear-gradient(135deg, 
				rgba(${color1[0]}, ${color1[1]}, ${color1[2]}, 0.95) 0%, 
				rgba(${color2[0]}, ${color2[1]}, ${color2[2]}, 0.85) 100%)`;
			
			document.documentElement.style.setProperty('--app-gradient', gradient);
		}
	} else {
		// Fallback to theme preset when no cover
		document.documentElement.style.setProperty(
			'--app-gradient',
			themePresets[theme.value]
		);
	}
});

let updateTimeInterval: ReturnType<typeof setInterval> | null = null;

const handleSpaceDown = (e: KeyboardEvent) => {
	if (
		e.code === 'Space' &&
		(document.activeElement === document.body ||
			document.activeElement === null)
	) {
		handlePlayMusic();
	}
};



onMounted(async () => {
	listen<string[]>("open-files", (event: any) => {
		console.log(event);
		
		if (event.payload.length) musaStore.fetchMusics(event.payload);
	});

	updateTimeInterval = setInterval(async () => {
		await musaStore.playing();
	}, 500);

	window.addEventListener('keydown', handleSpaceDown);
});

onBeforeUnmount(() => {
	if (updateTimeInterval) clearInterval(updateTimeInterval);
});

onUnmounted(() => {
	window.removeEventListener('keydown', handleSpaceDown);
});

const handleChangeVolume = async (payload: number[] | undefined) => {
	if (!payload) return;

	await musaStore.setVolume(payload[0]);
};

const handleChangeTime = async (payload: number[] | undefined) => {
	if (!payload) return;

	await musaStore.timeSeek((music.value?.duration! / 100) * payload[0]);
};
</script>

<template>
	<div ref="player-wrapper" v-if="music" class="player_container">
		<div class="music_player">
			<div class="music_logo_wrapper">
				<img
					v-if="music.cover"
					class="music_logo"
					:src="getUrl(music.cover)"
					alt="Album cover"
				/>
				<IconVinyl
					v-else
					class="music_logo"
					:style="{
						border: '1px solid var(--secondary-glass)',
						background: '#ffffff21',
					}"
				/>
			</div>
			<div class="music_info">
				<h1 class="music_info_title">{{ music?.title }}</h1>
				<div class="music_info_artist">{{ music?.artist }}</div>
				<div class="music_info_album">{{ music?.album }}</div>
			</div>
		</div>
		<div class="music_main">
			<div class="music_timeline">
				<div class="music_timeline_line">
					<Timeline
						:path="music.path"
						:time="time"
						:duration="music.duration!"
						v-on:change="handleChangeTime"
					/>
				</div>
				<div class="music_timeline_time">
					<div>{{ formattedTime(time, 'ms', 'm:s') }}</div>
					<div>{{ formattedTime(music.duration!, 'ms', 'm:s') }}</div>
				</div>
			</div>
			<div class="music_controls">
				<div class="music_controls_container">
					<Button
						variant="rounded"
						class="music_control_item music_control_prev"
						@click="musaStore.prevMusic"
						title="Previous track"
					>
						<IconSkipPrevious />
					</Button>
					<Button
						variant="rounded"
						@click="handlePlayMusic"
						class="music_control_item music_control_play"
						:title="isPlaying ? 'Pause' : 'Play'"
					>
						<IconPause v-if="isPlaying" />
						<IconPlay v-else />
					</Button>
					<Button
						variant="rounded"
						class="music_control_item music_control_next"
						@click="musaStore.nextMusic"
						title="Next track"
					>
						<IconSkipNext />
					</Button>
				</div>
			</div>
			<div class="music_volume">
				<VolumeControll v-on:change="handleChangeVolume" :volume="volume" />
			</div>
			<div class="music_settings">
				<div class="music_settings_container">
					<Button
						variant="rounded"
						:active="visible.sidebar"
						@click="layoutStore.toggleVisibleSidebar"
						title="Toggle sidebar"
					>
						<SidebarToggle :height="16" :is-toggle="visible.sidebar" />
					</Button>
					<Button
						variant="rounded"
						@click="musaStore.toggleShuffle"
						:active="musaStore.shuffle"
						title="Shuffle"
						><IconShuffle
					/></Button>
				</div>
				<div class="music_settings_container">
					<Button
						variant="rounded"
						@click="musaStore.toggleRepeat"
						:active="musaStore.repeat"
						title="Repeat"
						><IconRepeat
					/></Button>
					<Button
						variant="rounded"
						@click="navigateToSettings"
						title="Settings"
					>
						<IconSettings />
					</Button>
				</div>
			</div>
		</div>
	</div>
	<div v-else="music" class="not-found_wrapper">
		<Button
			variant="rounded"
			:active="visible.sidebar"
			@click="layoutStore.toggleVisibleSidebar"
			title="Toggle sidebar"
		>
			<SidebarToggle :height="17" :is-toggle="visible.sidebar" />
		</Button>
		<IconVinyl height="68" />
		<Button
			variant="rounded"
			@click="layoutStore.toggleVisiblePlaylist"
			:active="layoutStore.visible.playlist"
			title="Playlist"
			><IconVinyl height="20"
		/></Button>
	</div>
</template>

<style scoped>
.not-found_wrapper {
	display: flex;
	gap: 14px;
	align-items: center;
}
.player_container {
	width: 100%;
	max-width: 300px;
	height: 100%;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
}

.music_timeline_line {
	display: flex;
	justify-content: center;
}
.music_play {
	cursor: pointer;
	border-radius: 50%;
	border: 1px solid black;
	height: 50px;
	width: 50px;
}
.music_player {
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	gap: 14px;
	width: 100%;
}

.music_controls {
	gap: 18px;
	display: flex;
	align-items: center;
	justify-content: center;
}

.music_logo_wrapper {
	width: calc(100% - 60px);
	display: flex;
	align-items: center;
	justify-content: center;
	aspect-ratio: 1 / 1;

}

.music_logo {
	width: 100%;
	height: 100%;
	aspect-ratio: 1 / 1;
	object-fit: cover;
	border-radius: 24px;
	box-shadow: rgba(100, 100, 111, 0.2) 0px 7px 29px 0px;
}

.music_timeline {
	margin: 0 0 18px;
	width: 100%;
	gap: 14px;
}

.music_timeline_time {
	display: flex;
	justify-content: space-between;
}

.music_controls_container {
	justify-content: center;
	display: flex;
	gap: 14px;
	align-items: center;
}

.music_control_item {
	cursor: pointer;
	padding: 10px;
	display: flex;
	justify-content: center;
	align-items: center;
	border-radius: 100%;
	background-color: var(--secondary);
}

.music_control_item .path {
	fill: var(--background);
}

.music_control_item.music_control_play {
	padding: 8px;
}

.music_control_item:not(.music_control_play) {
	padding: 8px;
}

.music_info {
	display: flex;
	flex-direction: column;
	gap: 4px;
	justify-content: center;
	align-items: center;
	margin-bottom: 8px;
}

.music_info > div {
	text-align: center;
}

.music_info_title {
	margin: 0px;
	font-size: 16px;
	text-align: center;
}

.music_info_album {
	font-size: 14px;
}

.music_settings_container {
	display: flex;
	gap: 10px;
	align-items: center;
}

.music_settings {
	display: flex;
	align-items: center;
	justify-content: space-between;
	width: 100%;
}

.music_volume {
	width: 100%;
	margin: 12px 0;
}

.music_main {
	width: 100%;
	display: flex;
	justify-content: center;
	align-items: center;
	flex-direction: column;
}

@media (max-height: 500px) {
	.music_player {
		display: none;
	}
}
</style>
