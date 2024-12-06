import { readText } from '../utils';
import { firstPart, secondPart } from './day_03';
describe('day_03', () => {
    it('should solve first part on example', () => {
        expect(firstPart(readText('./inputs/03_ex'))).toBe(161)
    });
    it('should solve first part', () => {
        expect(firstPart(readText('./inputs/03'))).toBe(170068701)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/03_ex2'))).toBe(48)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/03'))).toBe(-1)
    });
});