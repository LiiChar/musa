<script setup lang="ts">
import { Icon } from '@iconify/vue';
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from 'reka-ui';
import Button from './button.vue';
const { volume, onChange } = defineProps<{
	volume: number;
	onChange: (payload: number[] | undefined) => void;
}>();
</script>

<template>
	<div
		:class="{
			controller_container: true,
			min_thumb: volume < 1,
			max_thumb: volume > 50,
		}"
	>
		<Button
			:class="{ controller_icons: true, active: volume == 0 }"
			:style="{ scale: 1.0 + (50 - volume) / 20000 }"
			variant="ghost"
		>
			<Icon @click="onChange([0])" height="24" icon="lucide:volume-1" />
		</Button>
		<SliderRoot
			@update:model-value="onChange"
			:model-value="[volume]"
			class="SliderRoot"
			:max="100"
			:step="1"
		>
			<SliderTrack class="SliderTrack">
				<SliderRange class="SliderRange" />
			</SliderTrack>
			<SliderThumb class="SliderThumb" aria-label="Volume" />
		</SliderRoot>
		<Button
			:class="{ controller_icons: true, active: volume == 100 }"
			:style="{ scale: 1.0 + (volume - 50) / 2000 }"
			variant="ghost"
		>
			<Icon @click="onChange([100])" height="24" icon="lucide:volume-2" />
		</Button>
	</div>
</template>

<style scoped>
.SliderRoot {
	position: relative;
	display: flex;
	align-items: center;
	user-select: none;
	touch-action: none;
	width: 100%;
	height: 20px;
	border-radius: 9999px;
	overflow: hidden;
}

.SliderTrack {
	background-color: #ffffff3f;
	position: relative;
	flex-grow: 1;
	border-radius: 9999px;
	height: 16px;
}

.SliderRange {
	position: absolute;
	background-color: var(--foreground);
	border-radius: 9999px;
	height: 100%;
}

.SliderThumb {
	display: block;
	width: 16px;
	height: 16px;
	background-color: white;
	border-radius: 10px;

	background-color: var(--foreground);
}

.controller_container:not(.min_thumb) .SliderThumb {
	/* width: 20px !important;
	margin-left: -8px !important; */
	/* border-radius: 0px;s */
	border-top-left-radius: 0px;
	border-bottom-left-radius: 0px;
}

.controller_container:not(.min_thumb) .SliderRange {
	/* width: 20px !important;
	margin-left: -8px !important; */
	border-top-right-radius: 0px;
	border-bottom-right-radius: 0px;
}

.controller_container.max_thumb .SliderRange {
	border-top-right-radius: 9999px;
	border-bottom-right-radius: 9999px;
}

.SliderThumb:hover {
}
.SliderThumb:focus {
	outline: none;
}

.controller_icons {
	aspect-ratio: 1 / 1;
	border-radius: 100%;
	transition: all 0.2s ease;
}

.controller_icons.active {
	scale: 1.05 !important;
}

.controller_container {
	display: flex;
	gap: 12px;
	align-items: center;
	position: relative;
}
</style>
