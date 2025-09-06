import { createApp } from 'vue';
import App from './App.vue';
import { createPinia } from 'pinia';
import './styles/base.css';
import './styles/ui.css';
import { initApp } from './app';

const pinia = createPinia();

const window_app = createApp(App);

window_app.use(pinia);
window_app.mount('#app');

export const app = initApp();
