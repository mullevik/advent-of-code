
export function mapPairs<T, R>(fn: (a: T, b: T) => R, arr: T[]): R[] {

    const acc = [];
    for (let i = 0; i < arr.length - 1; i++) {

        const first = arr[i];
        const second = arr[i + 1];
        acc.push(fn(first, second));
    }

    return acc;
} 
