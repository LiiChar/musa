<script setup lang="ts">
import Button from '../ui/button.vue';
import { Icon } from '@iconify/vue';
import {
	DropdownMenuArrow,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuPortal,
	DropdownMenuRoot,
	DropdownMenuTrigger,
} from 'reka-ui';
import { useMusa } from '../../stores/musa';
import { load } from '@tauri-apps/plugin-store';
import { storeToRefs } from 'pinia';

const musa = useMusa();

const { playlist } = storeToRefs(musa);

// const handleRename = () => {};

const handleClear = async () => {
	const store = await load('music.json');
	const playlists = (await store.get<Record<string, string[]>>('musics')) ?? {};
	playlists[playlist.value] = [];
	store.set('musics', playlists);
	await musa.setPlaylistMusics();
};

const handleDelete = async () => {
	const store = await load('music.json');
	const playlists = (await store.get<Record<string, string[]>>('musics')) ?? {};
	delete playlists[playlist.value];
	store.set('musics', playlists);
	musa.setPlaylist(Object.keys(playlists)[0]);
	musa.setPlaylists(Object.keys(playlists));
	await musa.setPlaylistMusics();
};
</script>

<template>
	<div class="dialog_selector_wrapper">
		<DropdownMenuRoot as-child class="dropdawn-root">
			<DropdownMenuTrigger class="dropdawn-trigger"
				><Icon height="22" icon="qlementine-icons:menu-dots-16"
			/></DropdownMenuTrigger>
			<DropdownMenuPortal>
				<DropdownMenuContent class="dropdawn-content">
					<!-- <DropdownMenuItem
						><Button class="btn_dialog_selector" variant="ghost">
							<Icon height="24" icon="ic:round-drive-file-rename-outline" />
							Переименовать</Button
						></DropdownMenuItem
					> -->
					<DropdownMenuItem
						><Button
							@click="handleClear"
							class="btn_dialog_selector"
							variant="ghost"
						>
							<Icon height="24" icon="mdi:eraser" /> Очистить</Button
						></DropdownMenuItem
					>
					<DropdownMenuItem
						><Button
							@click="handleDelete"
							class="btn_dialog_selector"
							variant="ghost"
						>
							<Icon height="24" icon="material-symbols:delete" />
							Удалить</Button
						></DropdownMenuItem
					>
					<DropdownMenuArrow class="dropdawn-arrow" />
				</DropdownMenuContent>
			</DropdownMenuPortal>
		</DropdownMenuRoot>
	</div>
</template>

<style>
.dropdawn-trigger {
	padding: 4px;
	height: 30px;
	width: 30px;
	display: flex;
	justify-content: center;
	align-items: center;
	border-radius: 100%;
	border-color: transparent;
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);
	transition: all 0.2s ease-in-out;
}
.dropdawn-arrow path {
	fill: var(--secondary-glass) !important;
}
.dropdawn-content {
	background-color: var(--secondary-glass) !important;

	border-radius: 6px;
	backdrop-filter: blur(10px) !important;
}
.dialog_selector_wrapper {
	display: flex;
	gap: 6px;
	align-items: center;
}
.btn_dialog_selector {
	gap: 6px;
	width: 100%;
	justify-content: start !important;
}
</style>
