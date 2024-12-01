import { readText } from '../utils';
import { firstPart, secondPart } from './day_01';
describe('day_01', () => {
    it('should solve example first part', () => {
        expect(firstPart(readText('./inputs/01_ex'))).toBe(11)
    });

    it('should solve first part', () => {
        expect(firstPart(readText('./inputs/01'))).toBe(2066446)
    });

    it('should solve example second part', () => {
        expect(secondPart(readText('./inputs/01_ex'))).toBe(31);
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/01'))).toBe(24931009)
    });
});