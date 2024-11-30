import { fileSync } from 'tmp';
import { writeFileSync } from 'fs';

import { benchmark, readText } from "./utils";

describe("readText", () => {
    it("should read text from a file", () => {
        const tmpFile = fileSync({ postfix: ".txt" });
        const expectedText = "example content";
        writeFileSync(tmpFile.name, expectedText);
        expect(readText(tmpFile.name)).toBe(expectedText);
        tmpFile.removeCallback();
    })
})


describe("benchmark", () => {
    it("should return the function's output", () => {
        const [out, time] = benchmark((x: string) => x.length, "some");
        expect(out).toBe(4);
        expect(time).toBeCloseTo(0, 1);
    })
})