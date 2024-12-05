import { getNonEmptyLines, isInRange } from "../utils";


export function findPattern(origin: number[], d: number[], lines: string[], pattern: RegExp): number[] {
    let text = "";
    let target = [...origin];

    const h = lines.length;
    const w = lines[0].length;

    while (isInRange(target[0], 0, h) && isInRange(target[1], 0, w)) {
        text += lines[target[0]][target[1]];
        target[0] += d[0];
        target[1] += d[1];
    }
    return [...text.matchAll(pattern)].map(x => x.index);
}

export function paintMiddle(origin: number[], d: number[], lines: string[], middles: number[][]) {

    const indices = findPattern(origin, d, lines, /MAS/g);

    for (const i of indices) {
        let [tY, tX] = origin;
        tY += d[0] * (i + 1);
        tX += d[1] * (i + 1);
        middles[tY][tX] += 1;
    }
}

export function firstPart(input: string): number {

    const lines = getNonEmptyLines(input);
    let total = 0;
    const h = lines.length;
    const w = lines[0].length;
    const pattern = /XMAS/g;

    for (let y = 0; y < h; y++) {
        total += findPattern([y, 0], [0, 1], lines, pattern).length;
        total += findPattern([y, w - 1], [0, -1], lines, pattern).length;
    }

    for (let x = 0; x < w; x++) {
        total += findPattern([0, x], [1, 0], lines, pattern).length;
        total += findPattern([h - 1, x], [-1, 0], lines, pattern).length;
    }

    // down right diag
    for (let x = 0; x < w; x++) {
        total += findPattern([0, x], [1, 1], lines, pattern).length;
    }
    for (let y = 1; y < h; y++) {
        total += findPattern([y, 0], [1, 1], lines, pattern).length;
    }
    // up right diag
    for (let x = 0; x < w; x++) {
        total += findPattern([h - 1, x], [-1, 1], lines, pattern).length;
    }
    for (let y = h - 2; y >= 0; y--) {
        total += findPattern([y, 0], [-1, 1], lines, pattern).length;
    }

    // down left diag
    for (let x = 0; x < w; x++) {
        total += findPattern([0, x], [1, -1], lines, pattern).length;
    }
    for (let y = 1; y < h; y++) {
        total += findPattern([y, w - 1], [1, -1], lines, pattern).length;
    }

    // up left diag
    for (let x = 0; x < w; x++) {
        total += findPattern([h - 1, x], [-1, -1], lines, pattern).length;
    }
    for (let y = h - 2; y >= 0; y--) {
        total += findPattern([y, w - 1], [-1, -1], lines, pattern).length;
    }

    return total;
}
export function secondPart(input: string): number {
    const lines = getNonEmptyLines(input);
    const h = lines.length;
    const w = lines[0].length;
    let middles = Array(h).fill(0).map(() => Array(w).fill(0));

    // down right diag
    for (let x = 0; x < w; x++) {
        paintMiddle([0, x], [1, 1], lines, middles);
    }
    for (let y = 1; y < h; y++) {
        paintMiddle([y, 0], [1, 1], lines, middles);
    }
    // up right diag
    for (let x = 0; x < w; x++) {
        paintMiddle([h - 1, x], [-1, 1], lines, middles);
    }
    for (let y = h - 2; y >= 0; y--) {
        paintMiddle([y, 0], [-1, 1], lines, middles);
    }

    // down left diag
    for (let x = 0; x < w; x++) {
        paintMiddle([0, x], [1, -1], lines, middles);

    }
    for (let y = 1; y < h; y++) {
        paintMiddle([y, w - 1], [1, -1], lines, middles);
    }

    // up left diag

    for (let x = 0; x < w; x++) {

        paintMiddle([h - 1, x], [-1, -1], lines, middles);
    }
    for (let y = h - 2; y >= 0; y--) {
        paintMiddle([y, w - 1], [-1, -1], lines, middles);
    }

    return middles.reduce((prev, curr) => prev + curr.reduce((p, c) => p + (c == 2 ? 1 : 0), 0), 0);
}