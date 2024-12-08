import { getNonEmptyLines } from "../utils";
import { Grid2, NumVec, vec2 } from "../vec";

export function firstPart(input: string): number {

    const world = new Grid2(getNonEmptyLines(input).map(row => [...row]));

    const signalMap = mapSignals(world);

    const uniqueLocations = new Set();

    for (const [signal, locations] of signalMap.entries()) {

        for (let i = 0; i < locations.length; i++) {
            for (let j = 0; j < i; j++) {
                const loc1 = locations[i];
                const loc2 = locations[j];

                const diff = loc1.minus(loc2);
                const pot1 = loc1.add(diff);
                const pot2 = loc2.minus(diff);

                if (world.contains(pot1)) {
                    uniqueLocations.add(pot1.toString());
                }
                if (world.contains(pot2)) {
                    uniqueLocations.add(pot2.toString());
                }
            }
        }
    }
    return uniqueLocations.size;
}
export function secondPart(input: string): number {
    const world = new Grid2(getNonEmptyLines(input).map(row => [...row]));

    const signalMap = mapSignals(world);

    const uniqueLocations = new Set();

    for (const [signal, locations] of signalMap.entries()) {

        for (let i = 0; i < locations.length; i++) {
            for (let j = 0; j < i; j++) {
                let loc1 = locations[i];
                let loc2 = locations[j];
                const diff = loc1.minus(loc2);

                while (world.contains(loc1)) {
                    uniqueLocations.add(loc1.toString())
                    loc1 = loc1.add(diff);
                }
                while (world.contains(loc2)) {
                    uniqueLocations.add(loc2.toString())
                    loc2 = loc2.minus(diff);
                }
            }
        }
    }
    return uniqueLocations.size;
}


function mapSignals(world: Grid2<string>): Map<string, NumVec[]> {

    const signalMap = new Map();

    for (const [y, row] of world.data.entries()) {
        for (const [x, signal] of row.entries()) {

            if (signal === ".") {
                continue;
            } else if (signalMap.has(signal)) {
                const locations = signalMap.get(signal);
                locations.push(vec2(y, x));
            } else {
                signalMap.set(signal, [vec2(y, x)]);
            }
        }
    }

    return signalMap;
}