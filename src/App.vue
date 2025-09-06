<script setup lang="ts">
import Sidebar from './widgets/layout/Sidebar.vue';
import Main from './widgets/layout/Main.vue';
import { SplitterGroup, SplitterPanel, SplitterResizeHandle } from 'reka-ui';
import { storeToRefs } from 'pinia';
import { useLayout } from './stores/layout';
import { ref, watch } from 'vue';

const layout = useLayout();

const { visible } = storeToRefs(layout);
const isCollapsed = ref(true);

watch(isCollapsed, (v) => {
	layout.visible.sidebar = v;
});

watch(visible, (v) => {
	if (!visible.value) {
		isCollapsed.value = v.sidebar;
	}
});
</script>

<template>
	<main class="app_container">
		<SplitterGroup direction="horizontal">
			<SplitterPanel
				class="sidebar_panel"
				:max-size="!visible.sidebar ? 0.1 : 100"
				:default-size="30"
				:min-size="0.1"
				v-on:resize="
					(size) => {
						size < 0.15 ? (isCollapsed = false) : (isCollapsed = true);
					}
				"
			>
				<Sidebar class="sidebar_container" />
			</SplitterPanel>
			<SplitterResizeHandle />
			<SplitterPanel
				class="main_panel"
				:min-size="0"
				:default-size="70"
				:max-size="100"
				><Main class="main_container"
			/></SplitterPanel>
		</SplitterGroup>
	</main>
</template>

<style scoped>
.sidebar_container {
}

.main_container {
	width: 100%;
	height: 100%;
	display: flex;
	justify-content: center;
	align-items: center;
}

.sidebar_panel {
	transition: all 0.2s ease-in-out;
}

@media (max-width: 600px) {
	.sidebar_panel {
		position: fixed;
		z-index: 10;
		left: 0;
		background: var(--app-gradient);
		top: 0;
		backdrop-filter: blur(0px);
		height: 100%;
		width: 300px;
		max-width: 100%;

		&:has(> .sidebar-hidden) {
			width: 0px !important;
		}
	}
}
</style>
