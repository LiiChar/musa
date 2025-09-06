<script setup lang="ts">
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from 'reka-ui';
import { ref, onMounted, onBeforeUnmount, computed, watchEffect } from 'vue';
import { getWave } from '../../api/music';

const { time, duration, onChange, path } = defineProps<{
	path: string;
	time: number;
	duration: number;
	onChange: (payload: number[] | undefined) => void;
}>();

// ширина палки и зазор
const barWidth = 3;
const gap = 2;

const container = ref<HTMLDivElement | null>(null);
const containerWidth = ref(300);
const waves = ref<number[]>([]);

// ресайзер для отслеживания ширины
let observer: ResizeObserver;

// watch(containerWidth, async w => {
// 	waves.value = await getWave(path, Math.floor(w / (barWidth + gap)));
// });

watchEffect(async () => {
	waves.value = await getWave(
		path,
		Math.floor(containerWidth.value / (barWidth + gap))
	);
});

onMounted(async () => {
	if (container.value) {
		observer = new ResizeObserver(entries => {
			for (const entry of entries) {
				containerWidth.value = entry.contentRect.width;
			}
		});
		observer.observe(container.value);
	}
});

onBeforeUnmount(() => {
	if (observer && container.value) {
		observer.unobserve(container.value);
	}
});

const progress = computed(() => {
	if (duration <= 0) return 0;
	return Math.min(100, Math.max(0, (time / duration) * 100));
});
</script>

<template>
	<div class="controller_container" ref="container">
		<!-- SVG с динамическим количеством палок -->
		<svg
			class="bitrate_svg"
			:width="containerWidth"
			height="50"
			:viewBox="`0 0 ${containerWidth} 50`"
			xmlns="http://www.w3.org/2000/svg"
		>
			<!-- Базовые линии -->
			<g opacity="0.3">
				<rect
					v-for="(bit, idx) in waves"
					:key="'base-' + idx"
					:x="idx * (barWidth + gap)"
					:y="25 - (bit * 50) / 2"
					:width="barWidth"
					:height="bit * 50"
					rx="2"
					:fill="'var(--foreground)'"
				/>
			</g>

			<!-- Прогресс (клип по ширине) -->
			<clipPath id="progress-clip">
				<rect :width="(progress / 100) * containerWidth" height="50" />
			</clipPath>

			<g clip-path="url(#progress-clip)">
				<rect
					v-for="(bit, idx) in waves"
					:key="'prog-' + idx"
					:x="idx * (barWidth + gap)"
					:y="25 - (bit * 50) / 2"
					:width="barWidth"
					:height="bit * 50"
					rx="2"
					:fill="'var(--foreground)'"
				/>
			</g>
		</svg>

		<!-- Слайдер -->
		<SliderRoot
			@update:model-value="onChange"
			:model-value="[(time / duration) * 100]"
			class="SliderRoot"
			:max="100"
			:step="1"
		>
			<SliderTrack class="SliderTrack">
				<SliderRange class="SliderRange" />
			</SliderTrack>
			<SliderThumb class="SliderThumb" aria-label="Volume" />
		</SliderRoot>
	</div>
</template>

<style scoped>
.controller_container {
	width: 100%;
	height: 50px;
	position: relative;
}

.bitrate_svg {
	display: block;
	position: absolute;
	top: 0;
	left: 0;
}

.SliderRoot {
	position: absolute;
	left: 0;
	top: 0;
	width: 100%;
	height: 50px;
	opacity: 0;
	display: flex;
	align-items: center;
	user-select: none;
	touch-action: none;
}

.SliderTrack {
	background-color: var(--secondary);
	position: relative;
	flex-grow: 1;
	border-radius: 9999px;
	height: 3px;
}

.SliderRange {
	position: absolute;
	background-color: var(--foreground);
	border-radius: 9999px;
	height: 100%;
}

.SliderThumb {
	width: 20px;
	height: 20px;
	border-radius: 50%;
	background-color: var(--foreground);
}
</style>
