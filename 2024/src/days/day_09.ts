type Drive = (number | null)[];

interface Segment {
    index: number,
    length: number,
}

export function firstPart(input: string): number {

    let [drive, segments] = buildDrive(input);

    let pStart = 0;
    let pEnd = drive.length - 1;


    while (pEnd > pStart) {

        const x = drive[pEnd];

        if (x !== null) {

            while (pStart < drive.length) {
                const f = drive[pStart];
                if (f === null) {
                    break;
                } else {
                    pStart += 1;
                }
            }
            if (pStart < pEnd) {
                drive[pStart] = x;
            }
        }
        pEnd -= 1;
    }

    let total = 0;
    for (let i = 0; i < pEnd + 2; i++) {
        total += (drive[i] as number) * i;
    }
    return total;
}
export function secondPart(input: string): number {
    let [drive, segments] = buildDrive(input);


    let e = segments.length - 1;
    while (e >= 0) {
        const endSeg = segments[e];

        if (drive[endSeg.index] === null) {
            e -= 1;
            continue;
        }

        for (let i = 0; i < e; i++) {
            const currSeg = segments[i];

            if (currSeg.length < endSeg.length || drive[currSeg.index] !== null) {
                continue;
            } else {

                // add new stuff
                drive.splice(currSeg.index, endSeg.length, ...drive.slice(endSeg.index, endSeg.index + endSeg.length));

                // remove old stuff
                drive.splice(endSeg.index, endSeg.length, ...Array(endSeg.length).fill(null));

                // update segments
                if (endSeg.length !== currSeg.length) {
                    segments.splice(i, 1, { index: currSeg.index, length: endSeg.length }, { index: currSeg.index + endSeg.length, length: currSeg.length - endSeg.length });
                    e += 1;
                }

                break;
            }

        }
        e -= 1;

    }
    let total = 0;
    for (const [i, x] of drive.entries()) {
        if (x !== null) {
            total += x * i;
        }
    }
    return total;
}

function buildDrive(input: string): [Drive, Segment[]] {

    let drive = [];
    let fileIndex = 0;
    let isFile = true;

    let segments = [];
    for (const s of input.trim()) {
        const l = parseInt(s);
        segments.push({ index: drive.length, length: l });

        for (let i = 0; i < l; i++) {
            if (isFile) {
                drive.push(fileIndex)
            } else {
                drive.push(null);
            }
        }
        if (isFile) {
            fileIndex += 1;
            isFile = false;
        } else {
            isFile = true;
        }

    }

    return [drive, segments];
}