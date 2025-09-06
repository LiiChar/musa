<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
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

const { onSelectPath } = defineProps<{
	onSelectPath: (paths: string[]) => void;
}>();

const handleFileDialog = async () => {
	const file = await open({
		multiple: true,
		directory: false,
	});

	if (file && onSelectPath) onSelectPath(file);
};

const handleDirDialog = async () => {
	const file = await open({
		multiple: true,
		directory: true,
	});

	if (file && onSelectPath) onSelectPath(file);
};
</script>

<template>
	<div class="dialog_selector_wrapper">
		<DropdownMenuRoot as-child class="dropdawn-root">
			<DropdownMenuTrigger class="dropdawn-trigger"
				><Icon height="15" icon="line-md:plus"
			/></DropdownMenuTrigger>
			<DropdownMenuPortal>
				<DropdownMenuContent class="dropdawn-content">
					<DropdownMenuItem
						><Button
							class="btn_dialog_selector"
							variant="ghost"
							@click="handleFileDialog"
						>
							<Icon height="24" icon="mdi-light:file" /> Нажмите чтобы выбрать
							файл</Button
						></DropdownMenuItem
					>
					<DropdownMenuItem
						><Button
							class="btn_dialog_selector"
							variant="ghost"
							@click="handleDirDialog"
						>
							<Icon height="24" icon="mdi-light:folder" /> Нажмите чтобы выбрать
							папку</Button
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
	padding: 6px;
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
	display: flex;
	align-items: center;
	gap: 6px;
}
</style>
