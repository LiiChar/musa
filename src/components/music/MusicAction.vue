<script setup lang="ts">
import Button from '../ui/button.vue';
import IconMoreVertical from '~icons/lucide/more-vertical';
import IconDelete from '~icons/lucide/trash-2';
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
			<div @click="(e) => e.stopPropagation()">
			<DropdownMenuTrigger class="dropdawn-trigger">
					<IconMoreVertical/>
				</DropdownMenuTrigger>
			</div>
			<DropdownMenuPortal >
				<DropdownMenuContent class="dropdawn-content">
					<DropdownMenuItem>
						<Button
							@click="handleDelete"
							class="btn_dialog_selector"
							variant="ghost"
						>
							<IconDelete />
							<span>Удалить</span>
						</Button>
					</DropdownMenuItem>
					<!-- <DropdownMenuSeparator class="dropdawn-separator" /> -->
					<!-- <DropdownMenuItem>
						<Button
							class="btn_dialog_selector"
							variant="ghost">
							<IconMoreVertical class="rotated-icon" />
							<span>Действия</span>
						</Button>
					</DropdownMenuItem> -->
					<DropdownMenuArrow class="dropdawn-arrow" />
				</DropdownMenuContent>
			</DropdownMenuPortal>
		</DropdownMenuRoot>
	</div>
</template>

<style>
.music_dialog_selector_wrapper {
	position: relative;
	display: flex;
	align-items: center;
	justify-content: center;
}

.music_dialog_selector_wrapper .dropdawn-root {
	position: relative;
}

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
  width: 32px !important;
  height: 32px !important;
}

.dropdawn-trigger svg {
  width: 18px;
  height: 18px;
}

.music_dialog_selector_wrapper .dropdawn-arrow path {
	fill: var(--secondary-glass) !important;
}

.music_dialog_selector_wrapper .dropdawn-content {
	background-color: var(--secondary-glass) !important;
	border: 1px solid var(--border-glass);
	border-radius: 8px;
	backdrop-filter: blur(12px) !important;
	padding: 4px;
	min-width: 140px;
	box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	z-index: 1000;
}

.music_dialog_selector_wrapper .btn_dialog_selector {
	gap: 8px;
	width: 100%;
	justify-content: flex-start !important;
	padding: 6px 8px;
	font-size: 13px;
	border-radius: 4px;
	transition: all 0.15s ease;
	background: transparent !important;
	border: none !important;
	color: var(--foreground);
}

.music_dialog_selector_wrapper .btn_dialog_selector:hover {
	background: rgba(255, 255, 255, 0.1) !important;
}

.music_dialog_selector_wrapper .btn_dialog_selector svg {
	width: 16px;
	height: 16px;
	flex-shrink: 0;
}

.dropdawn-separator {
	height: 1px;
	background: var(--border-glass);
	margin: 4px 0;
}

.rotated-icon {
	transform: rotate(90deg);
}
</style>
