import { titlebarInit } from './titlebar';
import { trayInit } from './tray';

export const initApp = async () => {
	titlebarInit();
	await trayInit();
};
