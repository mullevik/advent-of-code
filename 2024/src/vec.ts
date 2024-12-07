import { mapZip } from "./array_tools";
import { isInRange } from "./utils";


export class Vec<T> {
    data: T[]

    constructor(arr: T[]) {
        this.data = [...arr];
    }

    size(): number {
        return this.data.length
    }

    isEmpty(): boolean {
        return this.size() == 0;
    }

    get(index: number) {
        if (index < 0 || index >= this.size()) {
            throw new Error(`Index ${index} out of range for Vec of size ${this.size()}`);
        }
        return this.data[index];
    }

    equals(other: Vec<T>): boolean {
        if (this.size != other.size) {
            return false;
        }
        return mapZip((a, b) => a === b, this.data, other.data).every(x => x);
    }
}

export function vec2(y: number, x: number): NumVec {
    return new NumVec([y, x]);
}

export class NumVec extends Vec<number> {

    addScalar(scalar: number): NumVec {
        return new NumVec(this.data.map(x => x + scalar));
    }

    multiplyScalar(scalar: number): NumVec {
        return new NumVec(this.data.map(x => x * scalar));
    }

    add(other: NumVec): NumVec {
        return new NumVec(mapZip((a, b) => a + b, this.data, other.data));
    }

    multiply(other: NumVec): NumVec {
        return new NumVec(mapZip((a, b) => a * b, this.data, other.data));
    }
}


export class Grid2<T> {
    data: T[][]

    constructor(data: T[][]) {
        const lens = data.map(row => row.length);
        if (lens.length > 0) {
            if (!lens.every(len => len == lens[0])) {
                throw Error("Some rows of grid have different length");
            };
        }

        this.data = [...data.map(row => [...row])];
    }

    static full<T>(height: number, width: number, fillVal: T): Grid2<T> {
        return new Grid2(Array(height).fill(fillVal).map(x => Array(width).fill(fillVal)));
    }

    isEmpty(): boolean {
        return this.data.length == 0 || this.data[0].length == 0;
    }

    width(): number {
        if (this.isEmpty()) {
            return 0;
        } else {
            return this.data[0].length;
        }
    }

    height(): number {
        return this.data.length;
    }

    get(vec: NumVec): T {
        return this.data[vec.get(0)][vec.get(1)];
    }

    set(vec: NumVec, val: T): void {
        this.data[vec.get(0)][vec.get(1)] = val;
    }

    contains(vec: NumVec): boolean {

        return isInRange(vec.get(0), 0, this.height()) && isInRange(vec.get(1), 0, this.width());
    }
}
