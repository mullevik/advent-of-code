import { arraySum } from "../array_tools";

export function firstPart(input: string): number {


    let stones = input.trim().split(" ").map(x => parseInt(x));
    let memo = new Map();

    return arraySum(stones.map(s => simulateSingle(s, 25, memo)));
}
export function secondPart(input: string): number {
    let stones = input.trim().split(" ").map(x => parseInt(x));
    let memo = new Map();

    return arraySum(stones.map(s => simulateSingle(s, 75, memo)));
}


function simulateSingle(stone: number, nBlinksLeft: number, memo: Map<string, number>): number {
    const memoized = memo.get([stone, nBlinksLeft].toString());
    if (memoized !== undefined) {
        return memoized;
    }

    if (nBlinksLeft === 0) {
        return 1;
    }

    let nStones = undefined;
    if (stone === 0) {
        nStones = simulateSingle(1, nBlinksLeft - 1, memo);
    } else if (stone.toString().length % 2 === 0) {
        const sChars = [...stone.toString()];

        const halfIndex = sChars.length / 2;

        const firstHalf = sChars.slice(0, halfIndex);
        const secondHalf = sChars.slice(halfIndex, sChars.length);

        nStones = simulateSingle(parseInt(firstHalf.join("")), nBlinksLeft - 1, memo)
            + simulateSingle(parseInt(secondHalf.join("")), nBlinksLeft - 1, memo);

    } else {
        nStones = simulateSingle(stone * 2024, nBlinksLeft - 1, memo);

    }

    memo.set([stone, nBlinksLeft].toString(), nStones);
    return nStones;

}

function simulate(initialStones: number[], nBlinks: number): number[] {
    let stones = [...initialStones];
    for (let i = 0; i < nBlinks; i++) {

        const newStones = [];

        for (const s of stones) {
            if (s === 0) {
                newStones.push(1);
            } else if (s.toString().length % 2 === 0) {
                const sChars = [...s.toString()];

                const halfIndex = sChars.length / 2;

                const firstHalf = sChars.slice(0, halfIndex);
                const secondHalf = sChars.slice(halfIndex, sChars.length);

                newStones.push(parseInt(firstHalf.join("")));
                newStones.push(parseInt(secondHalf.join("")));

            } else {
                newStones.push(s * 2024);
            }
        }

        console.log(newStones.length, "diff", newStones.length - stones.length);
        console.log(newStones.slice(0, 10));
        stones = newStones;

    }
    return stones;
}