import * as fs from 'fs';
import dotenv from 'dotenv';

export function readText(filePath: string): string {
    return fs.readFileSync(filePath, "utf8");
}

export function getNonEmptyLines(input: string): string[] {
    return input.split("\n").filter(x => x.trim().length > 0);
}

export function isInRange(x: number, a: number, b: number) {
    return x >= a && x < b;
}


export function buildDay(day: number): void {
    dotenv.config({ path: "secrets.env" });
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
        "test('should solve first part on example', () => {",
        `expect(firstPart(readText('./inputs/${zeroPadDay}_ex'))).toBe(-1);`,
        "});",
        "test('should solve first part', () => {",
        `expect(firstPart(readText('./inputs/${zeroPadDay}'))).toBe(-1);`,
        "});",
        "test('should solve second part on example', () => {",
        `expect(secondPart(readText('./inputs/${zeroPadDay}_ex'))).toBe(-1);`,
        "});",
        "test('should solve second part', () => {",
        `expect(secondPart(readText('./inputs/${zeroPadDay}'))).toBe(-1);`,
        "});",
    ];

    const daySrcPath = `./src/days/day_${zeroPadDay}.ts`;
    const dayTestPath = `./src/days/day_${zeroPadDay}.test.ts`;
    const dayInputPath = `./inputs/${zeroPadDay}`;
    const dayInputPathExample = `./inputs/${zeroPadDay}_ex`;


    const sessionCookie = process.env.AOC_SESSION_COOKIE;

    if (sessionCookie === undefined) {
        throw Error("Undefined AOC_SESSION_COOKIE env var");
    }

    fetchAocDayInput(2024, day, sessionCookie).then((textInput) => {
        fs.writeFileSync(daySrcPath, srcContentLines.join("\n"));
        console.log(`${srcContentLines.length} lines written to '${daySrcPath}'`);
        fs.writeFileSync(dayTestPath, testContentLines.join("\n"));
        console.log(`${testContentLines.length} lines written to '${dayTestPath}'`);
        fs.writeFileSync(dayInputPath, textInput);
        console.log(`${textInput.length} chars written to '${dayInputPath}'`);
        fs.writeFileSync(dayInputPathExample, "");
        console.log(`0 chars written to '${dayInputPathExample}'`);
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


export function benchmark(func: (arg1: string) => number, input: string): [number, number] {
    const t0 = performance.now();
    const firstOutput = func(input);
    const t1 = performance.now();
    const firstDuration = t1 - t0;

    const nRepeats = Math.max(Math.floor(250 / firstDuration), 1);
    let durations = [firstDuration];

    for (let i = 0; i < nRepeats - 1; i++) {
        const t0 = performance.now();
        func(input);
        const t1 = performance.now();
        durations.push(t1 - t0);
    }
    return [firstOutput, durations.reduce((prev, curr) => prev + curr, 0) / nRepeats];
}

export interface RunnableDay {
    dayNumber: number,
    firstPartFn: (input: string) => number,
    secondPartFn: (input: string) => number,
}

export function benchmarkMultiple(days: RunnableDay[]) {

    console.log("```");
    console.log(`day ${"p1 (ms)".padStart(10, " ")} ${"p2 (ms)".padStart(10, " ")}`);
    console.log("");
    for (const day of days) {
        const zeroPadDay = day.dayNumber.toString().padStart(2, "0");
        const input = fs.readFileSync(`./inputs/${zeroPadDay}`).toString();

        const [_1, durationFirstPart] = benchmark(day.firstPartFn, input);
        console.error(`${zeroPadDay} returned ${_1} in avg duration of ${durationFirstPart.toFixed(2)} ms`);
        const [_2, durationSecondPart] = benchmark(day.secondPartFn, input);
        console.error(`${zeroPadDay} returned ${_2} in avg duration of ${durationSecondPart.toFixed(2)} ms`);
        console.log(` ${zeroPadDay} ${durationFirstPart.toFixed(2).padStart(10, " ")} ${durationSecondPart.toFixed(2).padStart(10, " ")}`);
    }
    console.log("```");
}

