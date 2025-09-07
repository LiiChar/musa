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
import { storeToRefs } from 'pinia';

const musa = useMusa();

const { music } = storeToRefs(musa);


const handleDelete = async () => {
  if (music.value) await musa.removeMusicFromPlaylist(music.value);
};
</script>

<template>
	<div class="music_dialog_selector_wrapper">
		<DropdownMenuRoot as-child class="dropdawn-root">
			<DropdownMenuTrigger class="dropdawn-trigger"
				><Icon height="22" icon="mdi:dots-vertical"
			/></DropdownMenuTrigger>
			<DropdownMenuPortal >
				<DropdownMenuContent :as-child="true" class="dropdawn-content">
					<!-- <DropdownMenuItem
						><Button class="btn_dialog_selector" variant="ghost">
							<Icon height="24" icon="ic:round-drive-file-rename-outline" />
							Переименовать</Button
						></DropdownMenuItem
					> -->
					<!-- <DropdownMenuItem
						><Button
							@click="handleClear"
							class="btn_dialog_selector"
							variant="ghost"
						>
							<Icon height="24" icon="mdi:eraser" /> Очистить</Button
						></DropdownMenuItem
					> -->
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
.music_dialog_selector_wrapper .dropdawn-trigger {
  padding: 0 !important;
  background: transparent !important;
  margin: 0 !important;
  width: 100% !important;
  height: 100% !important;
	display: flex;
	justify-content: center;
	align-items: center;
	transition: all 0.2s ease-in-out;
  backdrop-filter: none;
}

.music_action .dropdawn-trigger {
  padding: 4px !important;
}

.dropdawn-trigger svg {
  height: 100%;
  width: 100%;
} 

.music_dialog_selector_wrapper .dropdawn-arrow path {
	fill: var(--secondary-glass) !important;
}
.music_dialog_selector_wrapper .dropdawn-content {
	background-color: var(--foregrounds) !important;

	border-radius: 6px;
	backdrop-filter: blur(10px) !important;
}
.music_dialog_selector_wrapper .dialog_selector_wrapper {
	display: flex;
	gap: 6px;
	align-items: center;
}
.music_dialog_selector_wrapper .btn_dialog_selector {
	gap: 6px;
	width: 100%;
	justify-content: start !important;
}
</style>
