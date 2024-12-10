import { getNonEmptyLines } from "../utils";
import { Grid2, NumVec, vec2 } from "../vec";

export function firstPart(input: string): number {

    const world = new Grid2(getNonEmptyLines(input).map(line => [...line].map(c => parseInt(c))));

    let total = 0;
    for (let y = 0; y < world.height(); y++) {
        for (let x = 0; x < world.width(); x++) {
            const curr = vec2(y, x);
            if (world.get(curr) === 0) {
                total += findPeaks(curr, world).size;
            }
        }
    }

    return total;
}
export function secondPart(input: string): number {
    const world = new Grid2(getNonEmptyLines(input).map(line => [...line].map(c => parseInt(c))));

    let total = 0;
    for (let y = 0; y < world.height(); y++) {
        for (let x = 0; x < world.width(); x++) {
            const curr = vec2(y, x);
            if (world.get(curr) === 0) {
                total += countPathsToPeaks(curr, world);
            }
        }
    }

    return total;
}


function findPeaks(start: NumVec, world: Grid2<number>): Set<string> {

    let stack = [start];

    let visited = new Set([start.toString()]);

    let peaks: Set<string> = new Set();

    while (stack.length > 0) {

        let curr = stack.pop() as NumVec;
        let currVal = world.get(curr);
        if (currVal === 9) {
            peaks.add(curr.toString());
        }

        for (let adj of world.fourNeighborhood(curr)) {
            if (!visited.has(adj.toString())) {
                const adjVal = world.get(adj);
                if (adjVal === currVal + 1) {
                    visited.add(adj.toString());
                    stack.push(adj);
                }
            }
        }
    }
    return peaks;
}

function countPathsToPeaks(start: NumVec, world: Grid2<number>): number {

    let stack = [start];

    // let visited = new Set([start.toString()]);
    let total = 0;

    while (stack.length > 0) {

        let curr = stack.pop() as NumVec;
        let currVal = world.get(curr);
        if (currVal === 9) {
            total += 1;
        }

        for (let adj of world.fourNeighborhood(curr)) {
            const adjVal = world.get(adj);
            if (adjVal === currVal + 1) {
                stack.push(adj);
            }
        }
    }
    return total;
}