import { getNonEmptyLines } from "../utils";


function getProblematicIndices(update: number[], deps: number[][]): [number, number] | null {


    for (let i = 0; i < update.length; i++) {

        const prev = update.slice(0, i);
        const curr = update[i];

        for (const [pIndex, p] of prev.entries()) {

            if (deps.some(d => d[0] == curr && d[1] == p)) {
                return [pIndex, i];
            }
        }

    }
    return null;
}

function reorderUntilValid(update: number[], deps: number[][]): number[] {

    let newUpdate = [...update];

    while (true) {

        // console.log(newUpdate);
        const pIndices = getProblematicIndices(newUpdate, deps);

        if (pIndices === null) {
            return newUpdate;
        }

        newUpdate.splice(pIndices[0], 0, newUpdate[pIndices[1]]); // insert before left
        newUpdate.splice(pIndices[1] + 1, 1);  // remove right
    }
}

function isValid(update: number[], deps: number[][]): boolean {
    return getProblematicIndices(update, deps) === null ? true : false
}

export function firstPart(input: string): number {

    const [part1, part2] = input.split("\n\n");

    const deps = getNonEmptyLines(part1).map(line => line.split("|").map(num => parseInt(num)));

    const updates = getNonEmptyLines(part2).map(line => line.split(",").map(num => parseInt(num)));
    return updates.filter(u => isValid(u, deps)).map(u => u[Math.floor(u.length / 2)]).reduce((prev, curr) => prev + curr, 0);
}
export function secondPart(input: string): number {
    const [part1, part2] = input.split("\n\n");

    const deps = getNonEmptyLines(part1).map(line => line.split("|").map(num => parseInt(num)));

    const updates = getNonEmptyLines(part2).map(line => line.split(",").map(num => parseInt(num)));

    return updates.filter(u => !isValid(u, deps)).map(u => reorderUntilValid(u, deps)).map(u => u[Math.floor(u.length / 2)]).reduce((prev, curr) => prev + curr, 0);
}