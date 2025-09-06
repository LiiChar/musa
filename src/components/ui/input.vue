<!-- InputButton.vue -->
<script setup lang="ts">
const {
	modelValue = '',
	variant = 'default',
	active = false,
	type = 'text',
	placeholder = '',
	disabled = false,
} = defineProps<{
	modelValue?: string | number;
	variant?: 'default' | 'secondary' | 'ghost';
	active?: boolean;
	type?: string;
	placeholder?: string;
	disabled?: boolean;
}>();

const emit = defineEmits<{
	(e: 'update:modelValue', v: string): void;
	(e: 'enter', v: string): void;
}>();

const onInput = (e: Event) =>
	emit('update:modelValue', (e.target as HTMLInputElement).value);
const onKeydown = (e: KeyboardEvent) => {
	if (e.key === 'Enter') emit('enter', String(modelValue ?? ''));
};
</script>

<template>
	<input
		:class="['input', 'btn', variant, { active, disabled }]"
		:type="type"
		:value="modelValue ?? ''"
		:placeholder="placeholder"
		:disabled="disabled"
		@input="onInput"
		@keydown="onKeydown"
	/>
</template>

<style scoped>
.input {
	width: 100%;
	border-radius: 8px;
	padding: 8px 12px;
	line-height: 1.2;
	background: transparent;
	outline: none;
	color: var(--foreground);
	transition: all 0.2s ease-in-out;
}

.input::placeholder {
	color: var(--foreground);
}

/* убираем нативный стиль */
.input::-ms-reveal,
.input::-ms-clear {
	display: none;
}
.input::placeholder {
	opacity: 0.7;
}
.input.default:not(:hover):not(.disabled):not(:disabled)::placeholder {
	color: var(--foreground);
}

.input:not(:hover):not(.disabled):not(:disabled) {
	color: var(--foreground);
}

/* состояния */
.input:focus {
	box-shadow: 0 0 0 2px var(--border);
}
.input.disabled,
.input:disabled {
	opacity: 0.6;
	cursor: not-allowed;
}

.input.default {
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);
	border-color: transparent;
	box-shadow: 0 0 0 2px var(--secondary-glass);
}

.input.default:focus {
	box-shadow: 0 0 0 2px var(--secondary-glass);
}

.input.default:focus:not(.disabled):not(:disabled) {
	background-color: var(--border);
}

.input.default:not(:hover):not(.disabled):not(:disabled) {
	color: var(--foreground);
}
.input.default.active {
	background-color: var(--border);
}
.input.default.active:focus {
	background-color: var(--secondary);
}

/* ghost */
.input.ghost {
	background-color: transparent;
	border-color: transparent;
	padding: 8px 12px; /* для инпута оставим нормальный внутренний отступ */
}
.input.ghost:hover:not(.disabled):not(:disabled) {
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);
}

/* secondary */
.input.secondary {
	color: var(--secondary);
	background-color: var(--secondary-glass);
	backdrop-filter: blur(10px);
	border-color: var(--secondary);
}
</style>
