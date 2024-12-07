import { arraySum } from "../array_tools";
import { getNonEmptyLines, isInRange } from "../utils";
import { Grid2, NumVec, vec2 } from "../vec";


export function firstPart(input: string): number {

    const world = new Grid2(getNonEmptyLines(input).map(line => [...line]));

    const start = findStart(world);
    let d = new NumVec([-1, 0]);
    const [visited, _] = keepWalking(start, d, world)

    return arraySum(visited.data.map(row => arraySum(row)));
}
export function secondPart(input: string): number {
    const world = new Grid2(getNonEmptyLines(input).map(line => [...line]));

    const start = findStart(world);
    let d = vec2(-1, 0);
    const [visited, _] = keepWalking(start, d, world);

    let total = 0;
    for (let y = 0; y < world.height(); y++) {
        for (let x = 0; x < world.width(); x++) {
            const curr = vec2(y, x);
            if (visited.get(curr) && world.get(curr) === ".") {

                const newWorld = new Grid2([...world.data.map(row => [...row])]);
                newWorld.set(curr, "#");

                const [_v, isFinished] = keepWalking(start, d, newWorld);

                if (!isFinished) {
                    total += 1;
                }
            }
        }
    }
    return total;
}


function walk(curr: NumVec, d: NumVec, world: Grid2<string>, eachStepFn: (curr: NumVec, next: NumVec, d: NumVec) => void): [NumVec, NumVec | null] {

    while (true) {
        let next = curr.add(d);
        eachStepFn(curr, next, d);

        if (!world.contains(next)) {
            return [curr, null]
        }

        let nextVal = world.get(next);

        if (nextVal === "#") {
            return [curr, next];
        } else if (nextVal === "." || nextVal === "^") {
            curr = next;
        } else {
            throw Error(`Unexpected ${nextVal}`);
        }
    }
}

function keepWalking(curr: NumVec, d: NumVec, world: Grid2<string>): [Grid2<boolean>, boolean] {
    const visited = Grid2.full(world.height(), world.width(), false);
    const nReaches = Grid2.full(world.height(), world.width(), 0);
    while (true) {
        let [reached, next] = walk(curr, d, world, (atCurr, atNext, atD) => {
            visited.set(atCurr, true);
        });

        if (next === null) {
            return [visited, true];
        } else {
            d = rotate(d);
            if (nReaches.get(reached) > 3) {
                return [visited, false];
            }

            nReaches.set(reached, nReaches.get(reached) + 1);
            curr = reached;
        }
    }
}




function rotate(x: NumVec): NumVec {
    if (x.equals(vec2(-1, 0))) {
        return vec2(0, 1);
    } else if (x.equals(vec2(0, 1))) {
        return vec2(1, 0);
    } else if (x.equals(vec2(1, 0))) {
        return vec2(0, -1);
    } else if (x.equals(vec2(0, -1))) {
        return vec2(-1, 0);
    }
    throw Error(`Unable to rotate ${x}`);
}

function findStart(world: Grid2<string>): NumVec {

    let start: NumVec = new NumVec([0, 0]);
    for (const [y, row] of world.data.entries()) {
        for (const [x, char] of row.entries()) {
            if (char == "^") {
                start = new NumVec([y, x]);
            }
        }
    }
    return start;
}