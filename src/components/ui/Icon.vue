<script setup lang="ts">
import { iconPaths, type IconName } from '../../utils/icons';

defineProps<{
	icon: IconName;
	width?: number | string;
	height?: number | string;
	color?: string;
}>();
</script>

<template>
	<svg 
		:width="width || 24" 
		:height="height || 24" 
		viewBox="0 0 24 24" 
		fill="none" 
		xmlns="http://www.w3.org/2000/svg"
		:style="{ color }"
	>
		<g v-html="iconPaths[icon]"/>
	</svg>
</template>
