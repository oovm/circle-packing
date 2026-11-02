export interface CircleSpec {
    x: number;
    y: number;
    radius: number;
    hue: number;
}

/** Simple greedy circle packing demo for the homepage canvas. */
export function packCircles(
    width: number,
    height: number,
    count: number,
    minR: number,
    maxR: number,
): CircleSpec[] {
    const placed: CircleSpec[] = [];
    const maxAttempts = count * 80;

    for (let attempt = 0; attempt < maxAttempts && placed.length < count; attempt++) {
        const radius = minR + Math.random() * (maxR - minR);
        const x = radius + Math.random() * (width - 2 * radius);
        const y = radius + Math.random() * (height - 2 * radius);
        const overlaps = placed.some((other) => {
            const dx = other.x - x;
            const dy = other.y - y;
            const dist = Math.hypot(dx, dy);
            return dist < other.radius + radius - 0.5;
        });
        if (!overlaps) {
            placed.push({ x, y, radius, hue: (placed.length * 47) % 360 });
        }
    }
    return placed;
}
