import { defineStore } from 'pinia';
import { Music } from '../types/music';
import {
	playMusic as pM,
	stopMusic as sM,
	setMusic as setM,
	setVolume as setV,
	setTime as sT,
	getMusics,
	getTime,
} from '../api/music';
import { load } from '@tauri-apps/plugin-store';

export type Musa = {
	music: null | Music;
	musicList: Music[];
	isPlaying: boolean;
	volume: number;
	repeat: boolean;
	shuffle: boolean;
	index: number;
	time: number;
	playlist: string;
	playlists: string[];
};

export const useMusa = defineStore('musa', {
	state: (): Musa => ({
		music: null,
		musicList: [],
		isPlaying: false,
		time: 0,
		volume: 100,
		repeat: false,
		shuffle: false,
		index: 0,
		playlist: 'all',
		playlists: [],
	}),
	actions: {
		nextMusic() {
			if (this.shuffle) {
				this.index = Math.floor(Math.random() * this.musicList.length);
			} else {
				this.index = (this.index + 1) % this.musicList.length;
			}
			this.setMusic(this.musicList[this.index]);
		},
		prevMusic() {
			this.index =
				(this.index - 1 + this.musicList.length) % this.musicList.length;
			this.setMusic(this.musicList[this.index]);
		},
		setMusics(musics: Music[]) {
			this.musicList = musics;
		},
		addMusics(musics: Music[]) {
			this.musicList = [...this.musicList, ...musics];
		},
		async setVolume(volume: number) {
			await setV(volume / 100);
			this.volume = volume;
		},
		async setMusic(music: Music) {
			await setM(music.path);
			this.setVolume(this.volume);
			this.music = music;
			this.time = 0;
			this.index = this.musicList.indexOf(music);
			this.isPlaying = true;
		},
		async playMusic() {
			this.isPlaying = true;
			await pM();
		},
		async pauseMusic() {
			this.isPlaying = false;
			await sM();
		},
		async playing() {
			if (
				this.music &&
				this.music.duration &&
				this.music.duration >= this.time &&
				this.isPlaying
			) {
				const currentTime = await getTime();

				this.time = Math.floor(currentTime * 1000);
				return true;
			} else {
				if (
					this.music &&
					this.music.duration &&
					this.music.duration <= this.time
				) {
					await this.finish();
				}
				return false;
			}
		},
		async timeSeek(time: number) {
			await sT(time);
			const currentTime = await getTime();
			this.time = Math.floor(currentTime * 1000);
		},
		async finish() {
			if (this.repeat && this.music) {
				await this.setMusic(this.music);
			} else {
				this.nextMusic();
			}
		},
		async removeMusicFromPlaylist(music: Music) {
			const filtredPaths = this.musicList
				.filter((m) => m.path !== music.path)
				.map((m) => m.path);

			const musics = await getMusics(filtredPaths);
			this.setMusics(musics);
			const store = await load('music.json');

			const playlists =
				(await store.get<Record<string, string[]>>('musics')) ?? {};

			const newPlalylistItem: Record<string, string[]> = {};

			newPlalylistItem[this.playlist] = filtredPaths;
			await store.set('musics', {
				...playlists,
				...newPlalylistItem,
			});
			this.setPlaylists(filtredPaths);
		},
		async fetchMusics(paths: string[]) {
			const musics = await getMusics(paths);
			this.addMusics(musics);
			const store = await load('music.json');
			const playlists =
				(await store.get<Record<string, string[]>>('musics')) ?? {};
			const newPlalylistItem: Record<string, string[]> = {};
			newPlalylistItem[this.playlist] = {
				...playlists[this.playlist],
				...this.musicList.map((music) => music.path),
			};
			await store.set('musics', {
				...playlists,
				...newPlalylistItem,
			});
		},
		async setPlaylistMusics() {
			const store = await load('music.json');
			const playlists =
				(await store.get<Record<string, Record<string, string>>>('musics')) ??
				{};
			const musics = await getMusics(Object.values(playlists[this.playlist]));

			this.setMusics(musics);
			const newPlalylistItem: Record<string, string[]> = {};
			newPlalylistItem[this.playlist] = {
				...playlists[this.playlist],
				...this.musicList.map((music) => music.path),
			};
			await store.set('musics', {
				...playlists,
				...newPlalylistItem,
			});
		},
		toggleRepeat() {
			this.repeat = !this.repeat;
		},
		toggleShuffle() {
			this.shuffle = !this.shuffle;
		},
		async getCurrentPlaylist() {
			const store = await load('music.json');
			const currentPlaylist =
				(await store.get<string>('playlist')) ?? this.playlist;
			if (this.playlist !== currentPlaylist) {
				this.setPlaylist(currentPlaylist);
			}
			return currentPlaylist;
		},
		async setPlaylist(playlist: string) {
			const store = await load('music.json');
			await store.set('playlist', playlist);
			this.playlist = playlist;
		},
		async addPlaylist(playlist: string) {
			const store = await load('music.json');
			const playlists =
				(await store.get<Record<string, string[]>>('musics')) ?? {};
			const newPlalylistItem: Record<string, string[]> = {};
			newPlalylistItem[playlist] = [];
			await store.set('musics', {
				...playlists,
				...newPlalylistItem,
			});
			this.setPlaylists(
				Object.keys({
					...playlists,
					...newPlalylistItem,
				})
			);
		},
		setPlaylists(playlists: string[]) {
			this.playlists = playlists;
		},
	},
});
