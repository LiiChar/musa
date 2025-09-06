<script setup lang="ts">
import { Music } from '../../types/music';
import { useMusa } from '../../stores/musa';
import { getUrl } from '../../utils/url';
import { storeToRefs } from 'pinia';
import { Icon } from '@iconify/vue';

const { music } = defineProps<{
	music: Music;
}>();

const musa = useMusa();

const chooseMusic = () => {
	musa.setMusic(music);
};

const { music: m } = storeToRefs(musa);
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
			alt="Логотип музыки"
		/>
		<Icon
			v-else="music.cover"
			icon="iconamoon:music-1-fill"
			class="music_item_logo"
			alt="Логотип музыки"
		/>
		<div class="music_item_info">
			<h3 class="music_info_title">{{ music.title }}</h3>
			<div class="music_info_artist" v-if="music.artist">
				{{ music.artist }}
			</div>
		</div>
		<div class="played" v-if="m && m.id === music.id">
			<Icon height="16" icon="solar:play-stream-linear" />
		</div>
	</div>
</template>

<style scoped>
.music_item {
	display: flex;
	align-items: center;
	position: relative;
	gap: 8px;
	width: 100%; /* ← уже есть */
	box-sizing: border-box; /* ← важно */
	overflow: hidden; /* ← это оставляем */
}

.music_item:hover {
	background: #00000019;
}

.music_item_logo {
	height: 45px;
	width: 45px;
	aspect-ratio: 1 / 1;
	border-radius: 4px;
	object-fit: cover;
	min-width: 45px;
}

/* img.music_item_logo {
	height: 45px;
	width: 45px;
} */

.music_item_info {
	display: flex;
	gap: 3px;
	flex-direction: column;
	width: calc(100% - 50px); /* минус логотип + отступы */
	overflow: hidden; /* ← теперь текст не выбьется */
}

.music_info_title,
.music_info_artist {
	text-overflow: ellipsis;
	white-space: nowrap;
	overflow: hidden; /* ← ключ! */
}

.music_info_artist {
	text-overflow: ellipsis;
	white-space: nowrap;
	font-size: 14px;
}

.music_info_title {
	text-overflow: ellipsis;
	font-size: 14px;
	margin: 0;
	white-space: nowrap;
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
}
</style>
