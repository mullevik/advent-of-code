
export function mapPairs<T, R>(fn: (a: T, b: T) => R, arr: T[]): R[] {

    let acc = [];
    for (let i = 0; i < arr.length - 1; i++) {

        const first = arr[i];
        const second = arr[i + 1];
        acc.push(fn(first, second));
    }

    return acc;
}


export function mapZip<U, V, R>(fn: (a: U, b: V) => R, leftArray: U[], rightArray: V[]): R[] {

    let acc = [];
    for (let i = 0; i < Math.min(leftArray.length, rightArray.length); i++) {
        acc.push(fn(leftArray[i], rightArray[i]));
    }
    return acc;
}


export function arraySum(arr: number[] | boolean[]): number {

    if (arr.every((item) => typeof item === 'number')) {
        return arr.reduce((prev, curr) => prev + curr, 0);
    } else {
        return arr.reduce((prev, curr) => prev + (curr ? 1 : 0), 0);
    }
}