import { createApp } from 'vue';
import App from './App.vue';
import { createPinia } from 'pinia';
import './styles/base.css';
import './styles/ui.css';
import { initApp } from './app';
import { useSettings } from './stores/settings';

const pinia = createPinia();

const window_app = createApp(App);

window_app.use(pinia);
window_app.mount('#app');

// Load settings after app is mounted
const settingsStore = useSettings(pinia);
settingsStore.loadSettings();

export const app = initApp();
