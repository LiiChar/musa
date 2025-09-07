<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useMusa } from '../../stores/musa';
import { Icon } from '@iconify/vue';
import VolumeControll from '../../components/ui/volume-controll.vue';
import { getUrl } from '../../utils/url';
import { DEFAULT_IMAGE } from '../../const/image';
import SidebarToggle from '../../components/ui/sidebar-toggle.vue';
import Button from '../../components/ui/button.vue';
import { onBeforeUnmount, onMounted, onUnmounted, watch } from 'vue';
import { formattedTime } from '../../utils/time';
import Timeline from '../../components/ui/timeline.vue';
import { useLayout } from '../../stores/layout';
// ts-ignore
import colorthief from 'colorthief';
import { createSoftRadialGradient } from '../../utils/color';

const musaStore = useMusa();
const layoutStore = useLayout();

const { music, volume, isPlaying, time } = storeToRefs(musaStore);
const { visible } = storeToRefs(layoutStore);

const handlePlayMusic = async () => {
	if (!music.value) return;

	if (isPlaying.value) {
		musaStore.pauseMusic();
	} else {
		musaStore.playMusic();
	}
};

watch(music, async () => {
	if (music.value?.cover) {
		const color = new colorthief();
		const image = new Image();
		image.src = getUrl(music.value.cover);

		const gradient: [number, number, number][] = await color.getPalette(
			image,
			4
		);

		document.documentElement.style.setProperty(
			'--app-gradient',
			createSoftRadialGradient(gradient.map((g) => `rgba(${g.join(',')},1)`))
		);
	} else {
		const g = `linear-gradient(
		80deg,
		rgba(174,110,110,0.8),
		rgba(244,139,83,0.7)
	)`;

		document.documentElement.style.setProperty('--app-gradient', g);
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
					alt="Логотип музыки"
				/>
				<Icon
					icon="iconamoon:music-1-fill"
					v-else="music.cover"
					class="music_logo"
					:src="getUrl(DEFAULT_IMAGE)"
					alt="Логотип музыки"
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
					>
						<Icon height="16" icon="fluent:previous-16-filled" />
					</Button>
					<Button
						variant="rounded"
						@click="handlePlayMusic"
						class="music_control_item music_control_play"
					>
						<Icon
							v-if="isPlaying"
							height="22"
							icon="line-md:play-filled-to-pause-transition"
						/>
						<Icon
							v-else="isPlaying"
							height="22"
							icon="line-md:pause-to-play-filled-transition"
						/>
					</Button>
					<Button
						variant="rounded"
						class="music_control_item music_control_next"
					>
						<Icon
							height="16"
							icon="fluent:next-16-filled"
							@click="musaStore.nextMusic"
						/>
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
					>
						<SidebarToggle :height="16" :is-toggle="visible.sidebar" />
					</Button>
					<Button
						variant="rounded"
						@click="musaStore.toggleShuffle"
						:active="musaStore.shuffle"
						><Icon height="16" icon="solar:shuffle-linear"
					/></Button>
				</div>
				<div class="music_settings_container">
					<Button
						variant="rounded"
						@click="musaStore.toggleRepeat"
						:active="musaStore.repeat"
						><Icon height="16" icon="mdi-light:repeat"
					/></Button>
					<Button
						variant="rounded"
						@click="layoutStore.toggleVisiblePlaylist"
						:active="layoutStore.visible.playlist"
						><Icon height="18" icon="mdi-light:menu"
					/></Button>
				</div>
			</div>
		</div>
	</div>
	<div v-else="music" class="not-found_wrapper">
		<Button
			variant="rounded"
			:active="visible.sidebar"
			@click="layoutStore.toggleVisibleSidebar"
		>
			<SidebarToggle :height="17" :is-toggle="visible.sidebar" />
		</Button>
		<Icon height="68" icon="svg-spinners:bars-scale-middle" />
		<Button
			variant="rounded"
			@click="layoutStore.toggleVisiblePlaylist"
			:active="layoutStore.visible.playlist"
			><Icon height="17" icon="mdi-light:menu"
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
	width: 100%;
	display: flex;
	align-items: center;
	justify-content: center;
}

.music_logo {
	width: 60%;
	height: 100%;
	margin: 0 auto;
	object-fit: cover;
	aspect-ratio: 1 / 1;
	border-radius: 24px;
	box-shadow: rgba(100, 100, 111, 0.4) 0px 7px 29px 0px;
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
