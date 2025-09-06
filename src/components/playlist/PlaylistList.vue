<script setup lang="ts">
import { Icon } from '@iconify/vue';
import PlaylistItem from './PlaylistItem.vue';
import { useMusa } from '../../stores/musa';
import InputButton from '../ui/input-button.vue';

const { playlists } = defineProps<{
	playlists: string[];
}>();

const musa = useMusa();

const handleAddPlaylist = (playlist: string) => {
	musa.addPlaylist(playlist);
};
</script>

<template>
	<article class="playlist_list_container">
		<div v-for="playlist in playlists" :key="playlist">
			<PlaylistItem :playlist="playlist" />
		</div>
		<InputButton
			class="playlist-add"
			button-class="playlist-add_btn"
			@submit="handleAddPlaylist"
		>
			<Icon icon="ic:round-plus" height="24" />
			<div>Add playlist</div>
		</InputButton>
	</article>
</template>

<style scoped>
.playlist_list_container {
	display: flex;
	flex-direction: column;
	position: relative;
	height: 100%;
}

.playlist-item:last-child {
	margin-bottom: 44px;
}

.playlist-add {
	display: flex;
	border-radius: 0 !important;
	align-items: center;
	gap: 8px;
	padding: 0 6px;
	align-items: center;
	text-overflow: ellipsis;
	white-space: nowrap;
	position: relative;
	transition: background 0.2s ease;
	position: absolute;
	bottom: 0;
	left: -6px;
	width: 100%;
}

.playlist-add_btn {
	border-radius: 0 !important;
}
</style>
