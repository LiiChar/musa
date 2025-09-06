export function createRadialGradient(
	colors: string[],
	radiusPercent = 70.71,
	maxOffset = 5
): string {
	const gradients = colors.map((color, i) => {
		const angle = (360 / colors.length) * i;
		const rad = (angle * Math.PI) / 180;

		// координаты центра с рандомным смещением
		const x = 50 + 50 * Math.cos(rad) + randomOffset(maxOffset);
		const y = 50 + 50 * Math.sin(rad) + randomOffset(maxOffset);

		return `radial-gradient(circle at ${x.toFixed(1)}% ${y.toFixed(
			1
		)}%, ${color} 0%, ${color.replace(/[\d.]+\)$/, '0)')} ${radiusPercent}%)`;
	});

	return gradients.join(', ');
}

function randomOffset(maxOffset = 5) {
	return (Math.random() * 2 - 1) * maxOffset; // от -maxOffset до +maxOffset
}

export function softenColor(color: string, factor = 0.5): string {
	const rgba = color
		.replace(/rgba?\(|\)|\s/g, '')
		.split(',')
		.map(Number);

	const r = Math.round(rgba[0] * factor);
	const g = Math.round(rgba[1] * factor);
	const b = Math.round(rgba[2] * factor);
	const a = rgba[3] ?? 1;

	return `rgba(${r}, ${g}, ${b}, ${a})`;
}

function parseRGBA(color: string) {
	const match = color.match(
		/rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.]+))?\)/
	);
	if (!match) return [0, 0, 0, 1];
	const [, r, g, b, a] = match;
	return [Number(r), Number(g), Number(b), a !== undefined ? Number(a) : 1];
}

function softenColorHSL(
	color: string,
	satFactor = 0.5,
	lightFactor = 1.1,
	alpha = 0.5
): string {
	let [r, g, b] = parseRGBA(color);

	r /= 255;
	g /= 255;
	b /= 255;

	const max = Math.max(r, g, b),
		min = Math.min(r, g, b);
	let h = 0,
		s = 0,
		l = (max + min) / 2;

	if (max !== min) {
		const d = max - min;
		s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
		switch (max) {
			case r:
				h = (g - b) / d + (g < b ? 6 : 0);
				break;
			case g:
				h = (b - r) / d + 2;
				break;
			case b:
				h = (r - g) / d + 4;
				break;
		}
		h /= 6;
	}

	s = Math.min(s * satFactor, 1);
	l = Math.min(l * lightFactor, 1);

	const hue2rgb = (p: number, q: number, t: number) => {
		if (t < 0) t += 1;
		if (t > 1) t -= 1;
		if (t < 1 / 6) return p + (q - p) * 6 * t;
		if (t < 1 / 2) return q;
		if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
		return p;
	};

	let r1, g1, b1;
	if (s === 0) r1 = g1 = b1 = l;
	else {
		const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
		const p = 2 * l - q;
		r1 = hue2rgb(p, q, h + 1 / 3);
		g1 = hue2rgb(p, q, h);
		b1 = hue2rgb(p, q, h - 1 / 3);
	}

	return `rgba(${Math.round(r1 * 255)},${Math.round(g1 * 255)},${Math.round(
		b1 * 255
	)},${alpha})`;
}

export function createSoftRadialGradient(
	colors: string[],
	radiusPercent = 70.71
): string {
	console.log(colors);

	const gradients = colors.map((color, i) => {
		const angle = (360 / colors.length) * i;
		const rad = (angle * Math.PI) / 180;

		const x = 50 + 50 * Math.cos(rad);
		const y = 50 + 50 * Math.sin(rad);

		const softened = softenColorHSL(color);

		return `radial-gradient(circle at ${x.toFixed(1)}% ${y.toFixed(
			1
		)}%, ${softened} 0%, ${softened.replace(
			/[\d.]+\)$/,
			'0)'
		)} ${radiusPercent}%)`;
	});

	return gradients.join(', ');
}
