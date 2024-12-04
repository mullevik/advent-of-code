import { mapPairs } from "../array_tools";
import { getNonEmptyLines } from "../utils";

function isSafe(input: string): boolean {

    const nums = input.split(" ").map(x => parseInt(x));

    let diffs = mapPairs((a, b) => b - a, nums);

    const allPositive = diffs.every(x => (x >= 1 && x <= 3));
    const allNegative = diffs.every(x => (x <= -1 && x >= -3));
    return allNegative || allPositive;
}

export function firstPart(input: string): number {

    return getNonEmptyLines(input).map(isSafe).reduce((prev, curr) => prev + (curr ? 1 : 0), 0);

}
export function secondPart(input: string): number {


    return getNonEmptyLines(input).map(line => {

        if (isSafe(line)) { return true };
        const parts = line.split(" ");

        for (let i = 0; i < parts.length; i++) {

            const left = parts.slice(0, i);
            const right = parts.slice(i + 1);

            const newLine = left.concat(right).join(" ");

            if (isSafe(newLine)) {
                return true;
            }
        }

        return false;

    }).reduce((prev, curr) => prev + (curr ? 1 : 0), 0);
}