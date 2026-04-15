<script setup lang="ts">
import { Music } from '../../types/music';
import { useMusa } from '../../stores/musa';
import { getUrl } from '../../utils/url';
import { storeToRefs } from 'pinia';
import MusicAction from './MusicAction.vue';
import IconVinyl from '~icons/lucide/disc-3';
import IconPlay from '~icons/lucide/play-circle';

const { music } = defineProps<{
	music: Music;
}>();

const musa = useMusa();

const chooseMusic = () => {
	musa.setMusic(music);
};

const { music: m, isPlaying } = storeToRefs(musa);
</script>

<template>
	<div
		@click="chooseMusic"
		:class="'music_item ' + (m && m.id === music.id ? 'active' : '')"
	>
		<img
			v-if="music.cover"
			class="music_item_logo"
			:src="getUrl(music.cover)"
			alt="Album cover"
		/>
		<IconVinyl
			v-else
			:class="{'music_item_logo ': true, 'rotate-infinity': m && m.id === music.id, 'animation-paused': m && m.id === music.id && !isPlaying}"
		/>
		<div class="music_item_info">
			<h3 class="music_info_title">{{ music.title }}</h3>
			<div class="music_info_artist" v-if="music.artist">
				{{ music.artist }}
			</div>
		</div>
		<div class="played" v-if="m && m.id === music.id">
			<IconPlay class="played_icon" />
		</div>
		<div class="played music_action_wrapper">
			<MusicAction class="music_action"/>
		</div>
	</div>
</template>

<style scoped>
.music_item {
	display: flex;
	align-items: center;
	position: relative;
	gap: 8px;
	width: 100%;
	box-sizing: border-box;
	overflow: hidden;
	padding: 6px;
	border-radius: 6px;
	transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.music_item:hover {
	background: rgba(255, 255, 255, 0.1);
	transform: translateX(4px);
}

.music_item.active {
		background: rgba(255, 255, 255, 0.1);
	transform: translateX(4px);
}

.music_item:has(.dropdawn-trigger[data-state="open"]) {
	transform: translateX(4px);
}

.music_item:has(.dropdawn-trigger[data-state="open"]) .played:last-child {
		z-index: 1 !important;
	opacity: 1 !important;
		backdrop-filter: blur(10px);
	-webkit-backdrop-filter: blur(10px);
}

.music_item:active {
	transform: scale(0.98);
}

.music_item_logo {
	height: 36px;
	width: 36px;
	aspect-ratio: 1 / 1;
	border-radius: 4px;
	object-fit: cover;
	min-width: 36px;
	transition: all 0.2s ease;
}

.music_item:hover .music_item_logo {
	transform: scale(1.05);
	border-radius: 8px;
}

.music_item_info {
	display: flex;
	gap: 3px;
	flex-direction: column;
	width: calc(100% - 50px);
	overflow: hidden;
}

.music_info_title,
.music_info_artist {
	text-overflow: ellipsis;
	white-space: nowrap;
	overflow: hidden;
	transition: color 0.2s ease;
}

.music_item:hover .music_info_title {
	color: var(--foreground);
}

.music_info_artist {
	text-overflow: ellipsis;
	white-space: nowrap;
	font-size: 14px;
	color: var(--secondary-foreground);
}

.music_info_title {
	text-overflow: ellipsis;
	font-size: 14px;
	margin: 0;
	white-space: nowrap;
}

.music_item .played {
	position: absolute;
}

.music_item.active .played {
	position: absolute;
	top: 50%;
	transform: translateY(-50%);
	right: 8px;
	width: 16px;
	height: 16px;

	backdrop-filter: blur(10px);
	-webkit-backdrop-filter: blur(10px);
	border-radius: 100%;
	padding: 8px;
	transition: all 0.2s ease;
}

.music_item.active {
	background: rgba(255, 255, 255, 0.15);
}

.music_item.active:hover {
	background: rgba(255, 255, 255, 0.2);
}

.played {
}

.music_item.active .played:last-child {
	z-index: -1;
	opacity: 0;
	transition: opacity 0.2s ease;
}

.music_item.active:hover .played:last-child {
	z-index: 1;
	opacity: 1;
}

.played .music_action {
	z-index: -1;
	opacity: 0;
}

.music_action_wrapper {
		position: absolute;
	top: 50%;
	transform: translateY(-50%);
	right: 8px;
	width: 16px;
	height: 16px;

	border-radius: 100%;
	padding: 8px;
	transition: all 0.2s ease;
	padding: 0 !important;
	width: 32px !important;
	height: 32px !important;
}

.music_item:has(button[data-state="open"]) .played:last-child {
	z-index: -1 !important;
	opacity: 0 !important;
	& > .played_icon {
		display: none !important;
	}

	& > .music_action {
		z-index: 1;
		opacity: 1;
	}
}

.music_item:has(button[data-state="open"]) .played:first-child {
	z-index: 1 !important;
	opacity: 1 !important;
}

.music_item:hover .played {
	position: absolute;
	top: 50%;
	transform: translateY(-50%);
	right: 8px;
	width: 16px;
	height: 16px;

	backdrop-filter: blur(10px);
	-webkit-backdrop-filter: blur(10px);
	border-radius: 100%;
	padding: 8px;

	& > .played_icon {
		display: none !important;
	}

	& > .music_action {
		z-index: 1;
		opacity: 1;
	}
}
</style>
