import { arraySum } from "../array_tools";
import { getNonEmptyLines } from "../utils";
import { BaseN } from "js-combinatorics";


export function firstPart(input: string): number {


    const equations = getNonEmptyLines(input).map(line => parse(line));

    return arraySum(equations.filter(e => isValid(e, [addOp, multiplyOp])).map(e => e[0]));
}
export function secondPart(input: string): number {
    const equations = getNonEmptyLines(input).map(line => parse(line));

    return arraySum(equations.filter(e => isValid(e, [addOp, multiplyOp, concatOp])).map(e => e[0]));

}

type Equation = [number, number[]];
type Op = (a: number, b: number) => number;

const addOp = (a: number, b: number) => a + b;
const multiplyOp = (a: number, b: number) => a * b;
const concatOp = (a: number, b: number) => {
    const aStr = a.toString();
    const bStr = b.toString();
    return parseInt(aStr + bStr);
}

function isValid(equation: Equation, ops: Op[]) {

    const perms = [...new BaseN(ops, equation[1].length - 1)];

    for (const perm of perms) {


        let acc = equation[1][0];
        for (let i = 1; i < equation[1].length; i++) {
            const rhs = equation[1][i];
            acc = perm[i - 1](acc, rhs);
        }

        if (acc === equation[0]) {
            return true;
        }

    }
    return false;
}

function parse(line: string): Equation {
    const [left, right] = line.split(":");
    return [parseInt(left), right.trim().split(" ").map(x => parseInt(x))]
}