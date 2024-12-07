import { readText } from '../utils';
import { firstPart, secondPart } from './day_07';
import { BaseN } from 'js-combinatorics';

test("permutations", () => {

    expect(new BaseN("ab", 2).length).toBe(4n);
    expect(new BaseN("ab", 4).length).toBe(16n);
});

describe('day_07', () => {
    it('should solve first part on example', () => {
        expect(firstPart(readText('./inputs/07_ex'))).toBe(3749)
    });
    it('should solve first part', () => {
        expect(firstPart(readText('./inputs/07'))).toBe(6231007345478)
    });
    it('should solve second part on example', () => {
        expect(secondPart(readText('./inputs/07_ex'))).toBe(11387)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/07'))).toBe(333027885676693)
    });
});