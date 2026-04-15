<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useMusa } from '../../stores/musa';
import { getUrl } from '../../utils/url';
import Button from '../../components/ui/button.vue';
import Timeline from '../../components/ui/timeline.vue';
import { formattedTime } from '../../utils/time';
import IconPlay from '~icons/lucide/play';
import IconPause from '~icons/lucide/pause';
import IconSkipPrevious from '~icons/lucide/skip-back';
import IconSkipNext from '~icons/lucide/skip-forward';
import IconVinyl from '~icons/lucide/disc-3';

const musaStore = useMusa();

const { music, isPlaying, time } = storeToRefs(musaStore);

const handlePlayMusic = async () => {
	if (!music.value) return;

	if (isPlaying.value) {
		musaStore.pauseMusic();
	} else {
		musaStore.playMusic();
	}
};
</script>

<template>
	<div v-if="music" class="miniplayer_container">
		<div class="miniplayer_background">
			<div :style="{width: `${time / (music.duration ?? 1) * 100}px` }" class="miniplayer_time"></div>
		</div>
		<div class="miniplayer_content">
			<div class="miniplayer_cover">
				<img
					v-if="music.cover"
					class="miniplayer_cover_img"
					:src="getUrl(music.cover)"
					alt="Album cover"
				/>
				<IconVinyl
					v-else
					class="miniplayer_cover_icon rotate-infinity"
				/>
			</div>
			<div class="miniplayer_info">
				<div class="miniplayer_info_title">{{ music.title }}</div>
				<div class="miniplayer_info_artist">{{ music.artist }}</div>
				<div class="miniplayer_info_time">
					{{ formattedTime(Math.abs(time - (music.duration ?? 0)), 'ms', 'm:s') }}
				</div>
			</div>
			<div class="miniplayer_controls">
				<Button
					variant="rounded"
					class="miniplayer_control_btn"
					@click="musaStore.prevMusic"
					title="Previous track"
				>
					<IconSkipPrevious />
				</Button>
				<Button
					variant="rounded"
					class="miniplayer_control_btn miniplayer_play_btn"
					@click="handlePlayMusic"
					:title="isPlaying ? 'Pause' : 'Play'"
				>
					<IconPause v-if="isPlaying" />
					<IconPlay v-else />
				</Button>
				<Button
					variant="rounded"
					class="miniplayer_control_btn"
					@click="musaStore.nextMusic"
					title="Next track"
				>
					<IconSkipNext />
				</Button>
			</div>
		</div>
	</div>
	<div v-else class="miniplayer_empty">
		<IconVinyl height="24" />
		<span>No track loaded</span>
	</div>
</template>

<style scoped>
.miniplayer_container {
	position: relative;
	width: 100%;
	height: 100%;
	overflow: hidden;
	border-radius: 12px;
}

.miniplayer_background {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	opacity: 0.3;
	z-index: 0;
}

.miniplayer_time {
	height: 100%;
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);

}

.miniplayer_content {
	position: relative;
	z-index: 1;
	display: flex;
	align-items: center;
	gap: 12px;
	height: 100%;
	padding: 0 12px;
}

.miniplayer_cover {
	width: 46px;
	height: 46px;
	flex-shrink: 0;
	border-radius: 8px;
	overflow: hidden;
	display: flex;
	align-items: center;
	justify-content: center;
	background-color: var(--secondary);
}

.miniplayer_cover_img {
	width: 100%;
	height: 100%;
	object-fit: cover;
}

.miniplayer_cover_icon {
	width: 28px;
	height: 28px;
	color: var(--foreground);
	opacity: 0.7;
}

.miniplayer_info {
	flex: 1;
	display: flex;
	flex-direction: column;
	gap: 2px;
	min-width: 0;
	overflow: hidden;
}

.miniplayer_info_title {
	font-size: 13px;
	font-weight: 600;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
	color: var(--foreground);
}

.miniplayer_info_artist {
	font-size: 11px;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
	color: var(--foreground);
	opacity: 0.7;
}

.miniplayer_info_time {
	font-size: 10px;
	color: var(--foreground);
	opacity: 0.5;
}

.miniplayer_controls {
	display: flex;
	align-items: center;
	gap: 6px;
}

.miniplayer_control_btn {
	width: 32px;
	height: 32px;
	padding: 6px;
}

.miniplayer_control_btn :deep(.path) {
	fill: var(--foreground);
}

.miniplayer_play_btn {
	width: 36px;
	height: 36px;
	padding: 7px;
}

.miniplayer_empty {
	display: flex;
	align-items: center;
	justify-content: center;
	gap: 8px;
	height: 70px;
	border-radius: 12px;
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);
	color: var(--foreground);
	opacity: 0.5;
	font-size: 12px;
}
</style>
