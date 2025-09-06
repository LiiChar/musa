<script setup lang="ts">
const { variant = 'default', active = false } = defineProps<{
	variant?: 'default' | 'secondary' | 'ghost' | 'rounded';
	active?: boolean;
}>();
</script>

<template>
	<button :class="'btn ' + variant + ' ' + (active ? 'active' : '')">
		<slot></slot>
	</button>
</template>

<style scoped>
.btn {
	border-radius: 4px;
	padding: 8px 16px;
	cursor: pointer;
}
.btn.default {
	padding: 6px;
	display: flex;
	justify-content: center;
	align-items: center;
	border-radius: 8px;
	border-color: transparent;
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);
	transition: all 0.2s ease-in-out;
}

.btn.default.active {
	background-color: var(--border);
}

.btn.default.active:hover {
	background-color: var(--secondary);
}

.btn.default:hover {
	background-color: var(--border);
}

.btn.rounded {
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

.btn.rounded.active {
	background-color: var(--secondary);
}

.btn.ghost {
	background-color: transparent;
	border-color: transparent;
	padding: 2px 4px;
	min-height: min-content;
	display: flex;
	justify-content: center;
	align-items: center;
}

.btn.ghost:hover {
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);
}

.btn.ghost.active {
}

.btn.secondary {
	color: var(--secondary);
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);
	border-color: var(--secondary);
}
</style>
