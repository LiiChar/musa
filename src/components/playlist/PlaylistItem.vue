<script setup lang="ts">
import { useMusa } from '../../stores/musa';
import { storeToRefs } from 'pinia';
import { Icon } from '@iconify/vue';

const { playlist } = defineProps<{
	playlist: string;
}>();

const musa = useMusa();

const chooseMusic = async () => {
	musa.setPlaylist(playlist);
	await musa.setPlaylistMusics();
};

const { playlist: p } = storeToRefs(musa);
</script>

<template>
	<div
		@click="chooseMusic"
		:class="'playlist_item ' + (p === playlist ? 'active' : '')"
	>
		<!-- <img
			v-if="playlist.cover"
			class="playlist_item_logo"
			:src="getUrl(playlist.cover)"
			alt="Логотип музыки"
		/> -->
		<Icon
			icon="mingcute:playlist-fill"
			class="playlist_item_logo"
			height="24"
			alt="Логотип музыки"
		/>
		<div class="playlist_item_info">
			<h3 class="playlist_info_title">{{ playlist }}</h3>
		</div>
		<!-- <div class="played" v-if="m && m.id === playlist.id">
			<Icon height="16" icon="solar:play-stream-linear" />
		</div> -->
	</div>
</template>

<style scoped>
.playlist_item {
	display: flex;
	align-items: center;
	gap: 8px;
	width: 100%;
	text-overflow: ellipsis;
	white-space: nowrap;
	position: relative;
	transition: background 0.2s ease;
}

.playlist_item:hover {
	background: #00000019;
}

.playlist_item_logo {
	min-width: 50px;
	height: 50px;
	width: 50px;
	aspect-ratio: 1 / 1;
	border-radius: 4px;
	object-fit: cover;
}

.playlist_item_info {
	display: flex;
	gap: 3px;
	flex-direction: column;
	width: 100%;
	text-overflow: ellipsis;
	white-space: nowrap;
}

.playlist_info_artist {
	text-overflow: ellipsis;
	white-space: nowrap;
	font-size: 14px;
}

.playlist_info_title {
	text-overflow: ellipsis;
	font-size: 14px;
	margin: 0;
	white-space: nowrap;
}

.playlist_item.active .played {
	position: absolute;
	top: 50%;
	transform: translateY(-50%);
	right: 15px;
	width: 16px;
	height: 16px;

	backdrop-filter: blur(10px);
	-webkit-backdrop-filter: blur(10px);
	border-radius: 100%;
	padding: 8px;
}
</style>
