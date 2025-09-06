<!-- InputButton.vue -->
<script setup lang="ts">
import { ref } from 'vue';
import Button from './button.vue';
import Input from './input.vue';
import { Icon } from '@iconify/vue';

const {
	modelValue = '',
	active = false,
	type = 'text',
	placeholder = 'Input playlist name',
	disabled = false,
	variantButton = 'default',
	variantInput = '',
	minLenght = 3,
	onSubmit,
} = defineProps<{
	modelValue?: string | number;
	variantInput?: 'default' | 'secondary' | 'ghost';
	variantButton?: 'default' | 'secondary' | 'ghost' | 'rounded';
	active?: boolean;
	type?: string;
	placeholder?: string;
	disabled?: boolean;
	onSubmit?: (text: string) => void;
	buttonClass?: string;
	inputClass?: string;
	minLenght?: number;
}>();

const isInput = ref(false);

const emit = defineEmits<{
	(e: 'update:modelValue', v: string): void;
	(e: 'enter', v: string): void;
}>();

const input = ref('');

const onInput = (e: Event) => {
	input.value = (e.target as HTMLInputElement).value;
	emit('update:modelValue', (e.target as HTMLInputElement).value);
};
const onKeydown = (e: KeyboardEvent) => {
	if (e.key === 'Enter') emit('enter', String(modelValue ?? ''));
};

const hadleIsInput = () => {
	isInput.value = true;
};

const submitinput = () => {
	if (minLenght <= input.value.length) {
		isInput.value = false;
		onSubmit && onSubmit(input.value);
	}
};
</script>

<template>
	<div>
		<div class="input-block" v-if="isInput == true">
			<Input
				:class="[
					'input',
					'btn',
					'input-block_input ',
					variantInput,
					inputClass,
					{ active, disabled },
				]"
				:type="type"
				:value="modelValue ?? ''"
				:placeholder="placeholder"
				:disabled="disabled"
				@input="onInput"
				@keydown="onKeydown"
			/>
			<Button @click="submitinput()" class="input-block_btn"
				><Icon icon="weui:arrow-filled" height="24"
			/></Button>
		</div>
		<Button
			@click="hadleIsInput"
			:class="[variantButton, 'button-block', buttonClass]"
			v-if="isInput == false"
		>
			<slot></slot>
		</Button>
	</div>
</template>

<style scoped>
.input-block {
	display: flex;
	justify-content: center;
	align-items: center;
	width: 100%;
}

.input-block_input {
	width: 100%;
	height: 100%;
	margin: 0;
	border-radius: 0 !important;
}

.input-block_input:hover {
	background-color: var(--secondary-glass) !important;
	backdrop-filter: blur(10px);
}

.input-block_btn {
	aspect-ratio: 1 / 1;
	border-left: 1px solid var(--foreground) !important;
	border-radius: 0 !important;
}

.button-block {
	width: 100%;
	border-radius: 0 !important;
}
</style>
