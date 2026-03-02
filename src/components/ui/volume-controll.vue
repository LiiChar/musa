<script setup lang="ts">
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from 'reka-ui';
import Button from './button.vue';
import IconVolumeX from '~icons/lucide/volume-x';
import IconVolume2 from '~icons/lucide/volume-2';

const { volume, onChange } = defineProps<{
	volume: number;
	onChange: (payload: number[] | undefined) => void;
}>();

const handleMute = () => {
	if (volume > 0) {
		onChange([0]);
	} else {
		onChange([50]);
	}
};
</script>

<template>
	<div
		:class="{
			controller_container: true,
			min_thumb: volume < 1,
			max_thumb: volume > 50,
		}"
	>
		<!-- Volume level indicator -->
		<div class="volume_indicator">
			<div class="volume_bars">
				<div 
					v-for="i in 4" 
					:key="i"
					:class="{
						volume_bar: true,
						active: volume > (i - 1) * 25
					}"
				/>
			</div>
		</div>

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

		<!-- Mute button -->
		<Button
			class="mute_button"
			@click="handleMute"
			variant="ghost"
			title="Mute/Unmute"
		>
			<IconVolumeX v-if="volume === 0" />
			<IconVolume2 v-else />
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

.controller_container {
	display: flex;
	gap: 12px;
	align-items: center;
	position: relative;
}

.volume_indicator {
	display: flex;
	align-items: center;
	gap: 6px;
}

.volume_bars {
	display: flex;
	align-items: flex-end;
	gap: 2px;
	height: 16px;
}

.volume_bar {
	width: 3px;
	height: 4px;
	background: var(--secondary-foreground);
	border-radius: 1px;
	transition: all 0.15s ease;
}

.volume_bar:nth-child(1).active {
	height: 4px;
	background: var(--foreground);
}

.volume_bar:nth-child(2).active {
	height: 8px;
	background: var(--foreground);
}

.volume_bar:nth-child(3).active {
	height: 12px;
	background: var(--foreground);
}

.volume_bar:nth-child(4).active {
	height: 16px;
	background: var(--foreground);
}

.volume_icon {
	width: 18px;
	height: 18px;
	color: var(--foreground);
	display: none;
}

.mute_button {
	width: 28px;
	height: 28px;
	display: flex;
	align-items: center;
	justify-content: center;
	border-radius: 50%;
	transition: all 0.2s ease;
}

.mute_button:hover {
	background: rgba(255, 255, 255, 0.1);
}

.mute_button svg {
	width: 18px;
	height: 18px;
}
</style>
