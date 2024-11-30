
import * as fs from 'fs';

export function readText(filePath: string): string {
    return fs.readFileSync(filePath, "utf8");
}

export function benchmark(func: (arg1: string) => number, input: string): [number, number] {
    const t0 = performance.now();
    const output = func(input);
    const t1 = performance.now();
    return [output, t1 - t0];
}


export function buildDay(day: number): void {
    const zeroPadDay = day.toString().padStart(2, "0");

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

    const daySrcPath = `./src/days/day_${zeroPadDay}.ts`;
    const dayTestPath = `./src/days/day_${zeroPadDay}.test.ts`;
    const dayInputPath = `./inputs/${zeroPadDay}`;


    const sessionCookie = process.env.AOC_SESSION_COOKIE;

    if (sessionCookie === undefined) {
        throw Error("Undefined AOC_SESSION_COOKIE env var");
    }

    fetchAocDayInput(2024, day, sessionCookie).then((textInput) => {
        fs.writeFileSync(daySrcPath, srcContentLines.join("\n"));
        console.log(`${srcContentLines.length} lines written to '${daySrcPath}'`);
        fs.writeFileSync(`./src/days/day_${zeroPadDay}.test.ts`, testContentLines.join("\n"));
        console.log(`${testContentLines.length} lines written to '${dayTestPath}'`);
        fs.writeFileSync(`./inputs/${zeroPadDay}`, textInput);
        console.log(`${textInput.length} chars written to '${dayInputPath}'`);
    }).catch(e => {
        console.error(e);
    });
}


function fetchAocDayInput(year: number, day: number, sessionCookie: string): Promise<string> {
    const url = `https://adventofcode.com/${year}/day/${day}/input`;

    console.log(`Fetching '${url}' ...`)
    return fetch(url, {
        method: 'GET',
        headers: {
            'Cookie': `session=${sessionCookie}`,
        },
    })
        .then(response => {
            if (response.ok) {
                return response.text();
            }
            throw new Error(`Response not OK (${response.status})`);
        })
}

