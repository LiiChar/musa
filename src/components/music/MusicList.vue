<script setup lang="ts">
import { Music } from '../../types/music';
import MusicItem from './MusicItem.vue';

const { musics } = defineProps<{
	musics: Music[];
}>();
</script>

<template>
	<article class="music_list_container">
		<div v-for="music in musics" :key="music.id">
			<MusicItem :music="music" class="music-item" />
		</div>
	</article>
</template>

<style scoped>
.music_list_container {
	display: flex;
	height: 100%;
	flex-direction: column;
}

.music-item {
	padding: 3px 0 3px 6px;
}

.music_list_container > div:last-child {
	margin-bottom: 44px;
}
</style>
