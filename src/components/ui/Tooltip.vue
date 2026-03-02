<script setup lang="ts">
import { ref } from 'vue';

defineProps<{
	content: string;
	position?: 'top' | 'bottom' | 'left' | 'right';
}>();

const visible = ref(false);
let hideTimeout: ReturnType<typeof setTimeout> | null = null;

const showTooltip = () => {
	if (hideTimeout) clearTimeout(hideTimeout);
	visible.value = true;
};

const hideTooltip = () => {
	hideTimeout = setTimeout(() => {
		visible.value = false;
	}, 150);
};
</script>

<template>
	<div 
		class="tooltip-wrapper"
		@mouseenter="showTooltip"
		@mouseleave="hideTooltip"
	>
		<slot />
		<transition name="tooltip-fade">
			<div 
				v-if="visible" 
				:class="['tooltip', `tooltip-${position || 'top'}`]"
			>
				{{ content }}
			</div>
		</transition>
	</div>
</template>

<style scoped>
.tooltip-wrapper {
	position: relative;
	display: inline-block;
}

.tooltip {
	position: absolute;
	z-index: 1000;
	padding: 6px 10px;
	background: rgba(30, 30, 30, 0.95);
	backdrop-filter: blur(8px);
	border: 1px solid rgba(255, 255, 255, 0.08);
	border-radius: 6px;
	font-size: 12px;
	color: var(--foreground);
	white-space: nowrap;
	pointer-events: none;
	box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
	font-weight: 500;
	letter-spacing: 0.2px;
}

.tooltip-top {
	bottom: calc(100% + 8px);
	left: 50%;
	transform: translateX(-50%);
}

.tooltip-bottom {
	top: calc(100% + 8px);
	left: 50%;
	transform: translateX(-50%);
}

.tooltip-left {
	right: calc(100% + 8px);
	top: 50%;
	transform: translateY(-50%);
}

.tooltip-right {
	left: calc(100% + 8px);
	top: 50%;
	transform: translateY(-50%);
}

.tooltip::after {
	content: '';
	position: absolute;
	width: 6px;
	height: 6px;
	background: rgba(30, 30, 30, 0.95);
	border-right: 1px solid rgba(255, 255, 255, 0.08);
	border-bottom: 1px solid rgba(255, 255, 255, 0.08);
	transform: rotate(45deg);
}

.tooltip-top::after {
	bottom: -4px;
	left: 50%;
	margin-left: -3px;
}

.tooltip-bottom::after {
	top: -4px;
	left: 50%;
	margin-left: -3px;
	transform: rotate(-135deg);
}

.tooltip-left::after {
	right: -4px;
	top: 50%;
	margin-top: -3px;
	transform: rotate(-45deg);
}

.tooltip-right::after {
	left: -4px;
	top: 50%;
	margin-top: -3px;
	transform: rotate(135deg);
}

.tooltip-fade-enter-active,
.tooltip-fade-leave-active {
	transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
}

.tooltip-fade-enter-from,
.tooltip-fade-leave-to {
	opacity: 0;
	transform: translateX(-50%) translateY(4px);
}

.tooltip-fade-enter-to,
.tooltip-fade-leave-from {
	opacity: 1;
	transform: translateX(-50%) translateY(0);
}
</style>
