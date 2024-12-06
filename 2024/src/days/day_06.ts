import { arraySum } from "../array_tools";
import { getNonEmptyLines, isInRange } from "../utils";


export function firstPart(input: string): number {

    const world = getNonEmptyLines(input).map(line => [...line]);
    const w = world[0].length;
    const h = world.length;

    const start = findStart(world);
    let d: [number, number] = [-1, 0];
    const [visited, _] = keepWalking(start, d, world)

    return arraySum(visited.map(row => arraySum(row)));
}
export function secondPart(input: string): number {
    const world = getNonEmptyLines(input).map(line => [...line]);
    const w = world[0].length;
    const h = world.length;

    const start = findStart(world);
    let d: [number, number] = [-1, 0];
    const [visited, _] = keepWalking(start, d, world);

    let total = 0;
    for (let y = 0; y < h; y++) {
        for (let x = 0; x < w; x++) {
            if (visited[y][x] && world[y][x] === ".") {

                const newWorld = [...world.map(row => [...row])];
                newWorld[y][x] = "#";

                const [_v, isFinished] = keepWalking(start, d, newWorld);

                if (!isFinished) {
                    total += 1;
                }
            }
        }
    }
    return total;
}


function walk(from: [number, number], d: [number, number], world: string[][], eachStepFn: (curr: [number, number], next: [number, number], d: [number, number]) => void): [[number, number], [number, number] | null] {
    const w = world[0].length;
    const h = world.length;

    let curr: [number, number] = [...from];
    while (true) {
        let next: [number, number] = [curr[0] + d[0], curr[1] + d[1]];
        eachStepFn(curr, next, d);

        if (!(isInRange(next[0], 0, h) && isInRange(next[1], 0, w))) {
            return [curr, null]
        }

        let nextVal = world[next[0]][next[1]];

        if (nextVal === "#") {
            return [curr, next];
        } else if (nextVal === "." || nextVal === "^") {
            curr = next;
        } else {
            throw Error(`Unexpected ${nextVal}`);
        }
    }
}

function keepWalking(curr: [number, number], d: [number, number], world: string[][]): [boolean[][], boolean] {
    const h = world.length;
    const w = world[0].length;
    const visited = Array(h).fill(0).map(row => Array(w).fill(false));
    const nReaches = Array(h).fill(0).map(row => Array(w).fill(0));
    while (true) {
        let [reached, next] = walk(curr, d, world, (atCurr, atNext, atD) => {
            visited[atCurr[0]][atCurr[1]] = true;
        });

        if (next === null) {
            return [visited, true];
        } else {
            d = rotate(d);

            if (nReaches[reached[0]][reached[1]] > 3) {
                return [visited, false];
            }

            nReaches[reached[0]][reached[1]] += 1;
            curr = reached;
        }
    }
}




function rotate(x: [number, number]): [number, number] {
    if (x[0] == -1 && x[1] == 0) {
        return [0, 1]
    } else if (x[0] == 0 && x[1] == 1) {
        return [1, 0]
    } else if (x[0] == 1 && x[1] == 0) {
        return [0, -1]
    } else if (x[0] == 0 && x[1] == -1) {
        return [-1, 0]
    }
    throw Error(`Unable to rotate ${x}`);
}

function findStart(world: string[][]): [number, number] {
    const w = world[0].length;
    const h = world.length;

    let start: [number, number] = [0, 0];
    for (const [y, row] of world.entries()) {
        for (const [x, char] of row.entries()) {
            if (char == "^") {
                start = [y, x];
            }
        }
    }
    return start;
}