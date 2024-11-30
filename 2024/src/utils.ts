
import * as fs from 'fs';
import test from 'node:test';

export function readText(filePath: string): string {
    return fs.readFileSync(filePath, "utf8");
}

export function benchmark(func: (arg1: string) => number, input: string): [number, number] {
    const t0 = performance.now();
    const output = func(input);
    const t1 = performance.now();
    return [output, t1 - t0];
}


export function buildDay(dayNumber: number): void {
    const zeroPadDay = dayNumber.toString().padStart(2, "0");

    const srcContentLines = [
        "export function firstPart(input: string): number {",
        "throw new Error('Unimplemented');",
        "}",
        "export function secondPart(input: string): number {",
        "throw new Error('Unimplemented');",
        "}",
    ];

    const testContentLines = [
        "import {readText} from '../utils';",
        `import {firstPart,secondPart} from './day_${zeroPadDay}';`,
        `describe('day_${zeroPadDay}', () => {`,
        "it('should solve first part', () => {",
        `expect(firstPart(readText('./inputs/${zeroPadDay}'))).toBe(-1)`,
        "});",
        "it('should solve second part', () => {",
        `expect(secondPart(readText('./inputs/${zeroPadDay}'))).toBe(-1)`,
        "});",
        "});"
    ];

    fs.writeFileSync(`./src/days/day_${zeroPadDay}.ts`, srcContentLines.join("\n"));
    fs.writeFileSync(`./src/days/day_${zeroPadDay}.test.ts`, testContentLines.join("\n"));
    fs.writeFileSync(`./inputs/${zeroPadDay}`, "todo");  // todo - pull from remote
}