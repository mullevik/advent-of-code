import { fileSync } from 'tmp';
import { writeFileSync } from 'fs';

import { benchmark, getNonEmptyLines, readText } from "./utils";

test("readText should just read text from a file", () => {
    const tmpFile = fileSync({ postfix: ".txt" });
    const expectedText = "example content";
    writeFileSync(tmpFile.name, expectedText);
    expect(readText(tmpFile.name)).toBe(expectedText);
    tmpFile.removeCallback();
});

test("benchmark should return function's output and time", () => {
    const [out, time] = benchmark((x: string) => x.length, "some", 1);
    expect(out).toBe(4);
    expect(time).toBeCloseTo(0, 1);
});

test("nonEmptyLines should return non empty lines", () => {
    expect(getNonEmptyLines("foo\n  \nbar\n")).toStrictEqual(["foo", "bar"]);
});