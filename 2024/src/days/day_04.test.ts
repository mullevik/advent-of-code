import { readText } from '../utils';
import { findPattern, firstPart, paintMiddle, secondPart } from './day_04';


test("countXmas", () => {
    const pattern = /XMAS/g;
    expect(findPattern([0, 0], [0, 1], ["  XMAS  XMAS  "], pattern).length).toBe(2);
    expect(findPattern([0, 13], [0, -1], ["  XMAS  SAMX  "], pattern).length).toBe(1);
    expect(findPattern([0, 0], [1, 0], ["X ", "M ", "A ", "S "], pattern).length).toBe(1);
    expect(findPattern([3, 1], [-1, 0], ["XS", "MA", "AM", "SX"], pattern).length).toBe(1);
    expect(findPattern([0, 0], [1, 1], ["XMAS", "XMAS", "XMAS", "XMAS"], pattern).length).toBe(1);
    expect(findPattern([3, 0], [-1, 1], ["XMAS", "XMAS", "XMAS", "XMAS"], pattern).length).toBe(1);
    expect(findPattern([0, 3], [1, -1], ["SAMX", "SAMX", "SAMX", "SAMX"], pattern).length).toBe(1);
    expect(findPattern([3, 3], [-1, -1], ["SAMX", "SAMX", "SAMX", "SAMX"], pattern).length).toBe(1);
});

test("paintMiddle", () => {
    const middles = [[0, 0, 0, 0, 0]];

    paintMiddle([0, 0], [0, 1], [" MAS "], middles);
    expect(middles).toStrictEqual([[0, 0, 1, 0, 0]]);
    paintMiddle([0, 4], [0, -1], [" SAM "], middles);
    expect(middles).toStrictEqual([[0, 0, 2, 0, 0]]);
})

describe('day_04', () => {
    it('should solve first part on example', () => {
        expect(firstPart(readText('./inputs/04_ex'))).toBe(18)
    });
    it('should solve first part', () => {
        expect(firstPart(readText('./inputs/04'))).toBe(2591)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/04_ex'))).toBe(9)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/04'))).toBe(-1)
    });
});