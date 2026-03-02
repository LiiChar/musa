<script setup lang="ts">
import Button from '../ui/button.vue';
import IconMoreVertical from '~icons/lucide/more-vertical';
import IconDelete from '~icons/lucide/trash-2';
import IconClear from '~icons/lucide/eraser';
import {
	DropdownMenuArrow,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuPortal,
	DropdownMenuRoot,
	DropdownMenuTrigger,
	DropdownMenuSeparator,
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
				><IconMoreVertical
			/></DropdownMenuTrigger>
			<DropdownMenuPortal>
				<DropdownMenuContent class="dropdawn-content">
					<DropdownMenuItem
						><Button
							@click="handleClear"
							class="btn_dialog_selector"
							variant="ghost"
						>
							<IconClear />
							<span>Очистить</span></Button
						></DropdownMenuItem
					>
					<DropdownMenuSeparator class="dropdawn-separator" />
					<DropdownMenuItem
						><Button
							@click="handleDelete"
							class="btn_dialog_selector"
							variant="ghost"
						>
							<IconDelete />
							<span>Удалить</span></Button
						></DropdownMenuItem
					>
					<DropdownMenuArrow class="dropdawn-arrow" />
				</DropdownMenuContent>
			</DropdownMenuPortal>
		</DropdownMenuRoot>
	</div>
</template>

<style>
.dropdawn-root .dropdawn-trigger {
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
.dropdawn-root .dropdawn-trigger:hover {
	background-color: rgba(255, 255, 255, 0.15);
}
.dropdawn-arrow path {
	fill: var(--secondary-glass) !important;
}
.dropdawn-content {
	background-color: var(--secondary-glass) !important;
	border: 1px solid var(--border-glass);
	border-radius: 8px;
	backdrop-filter: blur(12px) !important;
	padding: 4px;
	min-width: 140px;
	box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}
.dialog_selector_wrapper {
	display: flex;
	gap: 6px;
	align-items: center;
}
.btn_dialog_selector {
	gap: 8px;
	width: 100%;
	justify-content: flex-start !important;
	padding: 6px 8px;
	font-size: 13px;
	border-radius: 4px;
	transition: all 0.15s ease;
}
.btn_dialog_selector:hover {
	background: rgba(255, 255, 255, 0.1);
}
.btn_dialog_selector svg {
	width: 16px;
	height: 16px;
	flex-shrink: 0;
}
.dropdawn-separator {
	height: 1px;
	background: var(--border-glass);
	margin: 4px 0;
}
</style>
