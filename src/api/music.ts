import { invoke } from '@tauri-apps/api/core';
import { Music } from '../types/music';

export type MusicFile = {
	path: string;
	tags: {
		EncoderSettings: string | null;
		TrackArtist: string | null;
		TrackTitle: string | null;
		album: null;
		artist: string | null;
		duration_ms: 278018;
		genre: string | null;
		title: string | null;
		cover: string | null;
	};
};

export const getMusics = async (
	paths: string[] = ['C:/Users/Public/Music']
) => {
	const musicFiles = await invoke<MusicFile[]>('get_musics', { paths });

	const musics = musicFiles.map<Music>((musicFile, i) => {
		const title =
			musicFile.tags.title ||
			musicFile.tags.TrackTitle ||
			musicFile.path.split('/').pop() ||
			`Music ${i + 1}`;
		return {
			id: i,
			path: musicFile.path,
			title: title,
			genre: musicFile.tags.genre,
			album: musicFile.tags.album,
			artist: musicFile.tags.artist || musicFile.tags.TrackArtist,
			duration: musicFile.tags.duration_ms,
			cover: musicFile.tags.cover,
			url: null,
		};
	});
	return musics;
};

export const playMusic = async () => await invoke('play_music');

export const setMusic = async (path: string) =>
	await invoke('set_music', { path });

export const stopMusic = async () => await invoke('stop_music');

export const setVolume = async (volume: number) =>
	await invoke('set_volume', { volume });

export const setTime = async (ms: number) => {
	return await invoke('seek_music', { sec: ms * 0.001 });
};
export const getWave = async (path: string, points: number) => {
	return await invoke<number[]>('get_wave', { path, points });
};

export const getTime = async () => await invoke<number>('get_time');

// export const getWave = async (path: string, points: number) => {
// 	return normalizeAndReduce(
// 		await invoke<number[]>('get_wave', { path, points })
// 	);
// };

// function normalizeAndReduce(samples: number[]): number[] {
// 	// 2. Берём абсолютные значения
// 	const absSamples = samples.map(Math.abs);

// 	// 3. Находим максимум
// 	const maxVal = Math.max(...absSamples);
// 	if (maxVal === 0) return new Array(samples.length).fill(0);

// 	// 4. Приводим к диапазону 0..100
// 	return absSamples.map(v => (v / maxVal) * 100);
// }
