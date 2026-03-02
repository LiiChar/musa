<script setup lang="ts">
import Player from '../music/Player.vue';
import Settings from '../settings/Settings.vue';
import { ref } from 'vue';

const currentPage = ref<'player' | 'settings'>('player');

const navigateTo = (page: 'player' | 'settings') => {
	currentPage.value = page;
};

const navigateBack = () => {
	currentPage.value = 'player';
};

defineExpose({ navigateTo, currentPage, navigateBack });
</script>

<template>
	<div class="main_player_wrapper">
		<Player v-if="currentPage === 'player'" />
		<Settings v-else :on-back="navigateBack" />
	</div>
</template>

<style scoped>
.main_player_wrapper {
	width: 60%;
	min-width: 330px;
}
</style>
