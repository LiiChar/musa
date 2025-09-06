<script setup lang="ts">
import { Icon } from '@iconify/vue';

const { isToggle, height } = defineProps<{
	isToggle: Boolean;
	height?: number;
}>();
</script>

<template>
	<Icon
		v-if="isToggle"
		:height="height ?? 24"
		icon="tabler:layout-sidebar-left-collapse-filled"
	/>
	<Icon
		v-else="isToggle"
		:height="height ?? 24"
		icon="tabler:layout-sidebar-left-collapse"
	/>
</template>
