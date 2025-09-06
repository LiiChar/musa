import { defineStore } from 'pinia';

export type Layout = {
	visible: {
		sidebar: boolean;
		playlist: boolean;
	};
};

export const useLayout = defineStore('layout', {
	state: (): Layout => ({
		visible: {
			sidebar: true,
			playlist: false,
		},
	}),
	actions: {
		toggleVisibleSidebar(visible?: boolean) {
			if ((visible || this.visible.sidebar) && this.visible.playlist) {
				this.visible.playlist = false;
			}
			if (typeof visible == 'boolean') {
				this.visible.sidebar = visible;
			} else {
				this.visible.sidebar = !this.visible.sidebar;
			}
		},
		toggleVisiblePlaylist(visible?: boolean) {
			if (!this.visible.sidebar && !this.visible.playlist) {
				this.toggleVisibleSidebar(true);
			}
			if (typeof visible == 'boolean') {
				this.visible.playlist = visible;
			} else {
				this.visible.playlist = !this.visible.playlist;
			}
		},
	},
});
