export function smoothRandom(prev: number, step = 0.1): number {
	const delta = (Math.random() * 2 - 1) * step; // случайный шаг [-step, step]
	let next = prev + delta;
	if (next > 1) next = 1;
	if (next < 0) next = 0;
	return next;
}
