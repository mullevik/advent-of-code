export function firstPart(input: string): number {

    return [...input.matchAll(/mul\((\d+),(\d+)\)/g)].map(m => parseInt(m[1]) * parseInt(m[2])).reduce((prev, curr) => prev + curr, 0);
}

export function secondPart(input: string): number {

    let isEnabled = true;
    let total = 0;
    for (const m of input.matchAll(/(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))/g)) {

        if (m[0].startsWith("mul") && isEnabled) {
            total += parseInt(m[2]) * parseInt(m[3]);

        } else if (m[0].startsWith("do(")) {
            isEnabled = true;
        } else if (m[0].startsWith("don't(")) {
            isEnabled = false;
        }

    }
    return total;
}