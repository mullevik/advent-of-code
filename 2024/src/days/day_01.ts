
export function firstPart(input: string): number {

    const numberPairs: number[][] = input
        .split("\n")
        .filter((line) => line.trim().length > 0)
        .map((line) => line.split(/[\s]+/).map((v) => parseInt(v)));
    let firstCol = numberPairs.map((x) => x[0]);
    let secondCol = numberPairs.map((x) => x[1]);

    firstCol.sort();
    secondCol.sort();

    return firstCol.map((firstVal, i) => Math.abs(firstVal - secondCol[i])).reduce((sum, curr) => sum + curr, 0);
}

export function secondPart(input: string): number {
    const numberPairs: number[][] = input
        .split("\n")
        .filter((line) => line.trim().length > 0)
        .map((line) => line.split(/[\s]+/).map((v) => parseInt(v)));

    let firstCol = numberPairs.map((x) => x[0]);
    let secondCol = numberPairs.map((x) => x[1]);

    return firstCol.map((firstVal) => secondCol.filter((secondVal) => firstVal == secondVal).length * firstVal).reduce((sum, curr) => sum + curr, 0);
}