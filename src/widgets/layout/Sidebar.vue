<script setup lang="ts">
import { storeToRefs } from 'pinia';
import MusicList from '../../components/music/MusicList.vue';
import { computed, onMounted, ref } from 'vue';
import { useMusa } from '../../stores/musa';
import { getMusics } from '../../api/music';
import { load } from '@tauri-apps/plugin-store';
import { useLayout } from '../../stores/layout';
import { Icon } from '@iconify/vue';
import Input from '../../components/ui/input.vue';
import Button from '../../components/ui/button.vue';
import PlaylistList from '../../components/playlist/PlaylistList.vue';
import SelectPath from '../../components/file/SelectPath.vue';
import PlaylistAction from '../../components/playlist/PlaylistAction.vue';
import SidebarToggle from '../../components/ui/sidebar-toggle.vue';

const musa = useMusa();
const layout = useLayout();

const { musicList, music, playlist, playlists } = storeToRefs(musa);
const { visible } = storeToRefs(layout);

const musicPlaylists = ref<Record<string, string[]>>({});

const searchedString = ref('');
const isSearch = ref(false);

// логика поиска исправлена
const searchedMusic = computed(() => {
	if (!isSearch.value) return musicList.value;
	const musicListTitleSearched = musicList.value.filter((v) =>
		v.title?.toLowerCase().includes(searchedString.value.toLowerCase())
	);
	if (musicListTitleSearched.length > 0) return musicListTitleSearched;
	const musicListArtistSearched = musicList.value.filter((v) =>
		v.artist?.toLowerCase().includes(searchedString.value.toLowerCase())
	);
	if (musicListArtistSearched.length > 0) return musicListArtistSearched;
	const musicListAlbumSearched = musicList.value.filter((v) =>
		v.album?.toLowerCase().includes(searchedString.value.toLowerCase())
	);
	return musicListAlbumSearched;
});

onMounted(async () => {
	const store = await load('music.json');
	const musicPlaylistPaths =
		(await store.get<Record<string, string[]>>('musics')) ?? {};
	musicPlaylists.value = musicPlaylistPaths;
	musa.setPlaylists(Object.keys(musicPlaylistPaths));
	let musicPaths = Object.values(musicPlaylistPaths[playlist.value] ?? {});

	if (musicPaths && musicPaths.length > 0) {
		const musics = await getMusics(musicPaths);
		musa.setMusics(musics);

		if (!musa.music) {
			await musa.setMusic(musics[0]);
			await musa.pauseMusic();
		}
	}
});

const handleSelectPaths = async (paths: string[]) => {
	await musa.fetchMusics(paths);
};

const timeRemain = computed(() => {
	const currentIndex = music.value
		? musicList.value.findIndex((s) => s.title === music.value!.title)
		: 0;
	return musicList.value
		.slice(currentIndex)
		.reduce((total, m) => total + (m.duration ?? 0), 0);
});
</script>

<template>
	<div
		:class="{
			sidebar_container: true,
			'sidebar-hidden': !visible.sidebar,
			'sidebar-visible': visible.sidebar,
		}"
	>
		<div class="sidebar_header">
			<div class="sidebar_info">
				<div class="sidebar_title">Playlist - {{ playlist }}</div>
				<div class="sidebar_description">
					{{ Math.ceil(timeRemain / 1000 / 60) }} minutes remaining
				</div>
			</div>
			<div class="sidebar_header_actions">
				<Button
					variant="rounded"
					:active="visible.sidebar"
					@click="layout.toggleVisibleSidebar"
					class="sidebar_toggle"
				>
					<SidebarToggle :height="17" :is-toggle="visible.sidebar" />
				</Button>
				<Button
					variant="rounded"
					@click="layout.toggleVisiblePlaylist"
					:active="layout.visible.playlist"
					><Icon height="18" icon="mdi-light:menu"
				/></Button>
			</div>
		</div>

		<!-- ОДИН переключатель с анимацией -->
		<div :class="{ switcher: true, switcher1: visible.playlist }">
			<transition name="switch" mode="out-in">
				<PlaylistList
					v-if="visible.playlist"
					key="playlists"
					:playlists="playlists"
					class="playlist_list"
				/>
				<MusicList
					v-else
					:key="'music-' + playlist"
					:musics="searchedMusic"
					class="music_list"
				/>
			</transition>
		</div>
		<transition name="fade-slide">
			<div v-if="isSearch" class="sidebar_search">
				<Input placeholder="Input music name" v-model="searchedString" />
			</div>
		</transition>
		<div
			class="playlist-list_action"
			:style="{ bottom: isSearch ? '46px' : '0px' }"
		>
			<div class="sidebar_action_wrapper">
				<Button
					:active="isSearch"
					variant="rounded"
					@click="isSearch = !isSearch"
				>
					<Icon height="15" icon="material-symbols:search" />
				</Button>
				<SelectPath @select-path="handleSelectPaths" />
				<PlaylistAction />
			</div>
		</div>
	</div>
</template>

<style scoped>
.playlist-list_action {
	display: flex;
	justify-content: center;
	align-items: center;
	padding: 6px 0;
	position: absolute;
	bottom: 0px;
	left: 0px;
	width: 100%;
	transition: all 0.2s ease-in-out;
}
.playlist-list_wrapper {
	position: relative;
}
.sidebar_action_wrapper {
	display: flex;
	align-items: center;
	gap: 4px;
}
.sidebar_title {
	font-weight: bold;
	font-size: 18px;
	margin-bottom: 4px;
	white-space: nowrap;
}

.sidebar_search {
	display: flex;
	justify-content: center;
	padding: 10px 24px;
}

.sidebar_header {
	padding: 6px 6px 10px 6px;
	border-bottom: 1px solid #ffffff48;
	display: flex;
	justify-content: space-between;
	align-items: center;
}

.sidebar_container {
	position: relative;
	display: flex;
	flex-direction: column;
	max-height: 100%;
	min-height: 100%;
	height: 100%;
	background-color: #0000000f;
	width: 100%;
	transition: width 0.35s cubic-bezier(0.36, 0.07, 0.19, 0.97),
		opacity 0.25s ease;
	overflow-y: auto;
	overflow-x: hidden;
}

.playlist_list {
	top: 0;
	left: 0;
	width: 100%;
	height: 100%; /* если нужно перекрывать всю высоту */
	z-index: 10;
	overflow-y: hidden auto;
}

.drop-down-enter-active,
.drop-down-leave-active {
	transition: all 0.35s ease;
}
.drop-down-enter-from,
.drop-down-leave-to {
	transform: translateX(100%);
	opacity: 0;
}
.drop-down-enter-to,
.drop-down-leave-from {
	transform: translateX(0);
	opacity: 1;
}

.drop-down1-enter-active,
.drop-down1-leave-active {
	transition: all 0.35s ease;
}
.drop-down1-enter-from,
.drop-down1-leave-to {
	transform: translateX(-100%);
	opacity: 0;
}
.drop-down1-enter-to,
.drop-down1-leave-from {
	transform: translateX(0);
	opacity: 1;
}

/* скрываем скроллбар */
.sidebar_container::-webkit-scrollbar {
	width: 0;
	height: 0;
}
.sidebar_container {
	-ms-overflow-style: none; /* IE и Edge */
	scrollbar-width: none; /* Firefox */
}

.sidebar_description {
	white-space: nowrap;
}

.sidebar-hidden {
	width: 0 !important;
}

.sidebar-visible {
}

/* анимация поля поиска */
.fade-slide-enter-active,
.fade-slide-leave-active {
	transition: all 0.15s ease;
}
.fade-slide-enter-from,
.fade-slide-leave-to {
	transform: translateY(12px);
	opacity: 0;
}
.fade-slide-enter-to,
.fade-slide-leave-from {
	transform: translateY(0);
	opacity: 1;
}

.music_list {
	height: 100%;
}

/* область, в которой переключаем списки */
.switcher {
	position: relative;
	flex: 1 1 auto;
	min-height: 0;
	/* скрываем прокрутку у контейнера, скролл будет у внутренних списков при необходимости */
	overflow: hidden;
}

/* оба экрана перекрывают друг друга во время анимации */
.switch-enter-active,
.switch-leave-active {
	transition: transform 0.35s ease, opacity 0.25s ease;
	position: absolute;
	inset: 0; /* top:0; right:0; bottom:0; left:0; */
}

/* когда открываем Плейлисты — падают сверху */
.switch-enter-from {
	transform: translateX(-100%);
	opacity: 0;
}
.switch-enter-to {
	transform: translateX(0);
	opacity: 1;
}

/* когда закрываем текущий экран — слегка уезжает вниз и исчезает */
.switch-leave-from {
	transform: translateX(0);
	opacity: 1;
}
.switch-leave-to {
	transform: translateX(20%);
	opacity: 0;
}

.switch-leave-from.switch1-leave-from {
	transform: translateX(0);
	opacity: 1;
}
.switch-leave-to.switch1-leave-to {
	transform: translateX(-20%);
	opacity: 0;
}

.music_list,
.playlist_list {
	height: 100%;
	overflow-y: auto;
	-ms-overflow-style: none;
	scrollbar-width: none;
}
.music_list::-webkit-scrollbar,
.playlist_list::-webkit-scrollbar {
	width: 0;
	height: 0;
}

.sidebar_toggle {
	opacity: 0;
	transition: none !important;
}

.sidebar_header_actions {
	display: flex;
	align-items: center;
	gap: 6px;
}

@media (max-width: 600px) {
	.sidebar_toggle {
		opacity: 1;
	}
}
</style>
