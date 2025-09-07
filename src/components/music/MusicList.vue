<script setup lang="ts">
import { Music } from '../../types/music';
import MusicItem from './MusicItem.vue';

const { musics } = defineProps<{ musics: Music[] }>();
</script>

<template>
	<article class="music_list_container">
		<transition-group name="list" tag="div" class="list-root">
			<MusicItem
				v-for="music in musics"
				:key="music.id"
				:music="music"
				class="music-item"
			/>
		</transition-group>
	</article>
</template>

<style scoped>
.music_list_container {
	display: flex;
	height: 100%;
	flex-direction: column;
}

/* критично для анимации перестановки */
.list-move {
	transition: transform 0.35s ease;
}

/* опционально — красиво вход/выход */
.list-enter-active,
.list-leave-active {
	transition: opacity 0.2s ease, transform 0.2s ease;
}
.list-enter-from,
.list-leave-to {
	opacity: 0;
	transform: scale(0.98);
}
/* чтобы удаляемый элемент не сдвигал остальных во время fade-out */
.list-leave-active {
	position: absolute;
}

.music-item {
	padding: 3px 0 3px 6px;
}
.list-root > :last-child {
	margin-bottom: 44px;
}
</style>
