import { readText } from '../utils';
import { firstPart, secondPart } from './day_06';
describe('day_06', () => {
    it('should solve first part on example', () => {
        expect(firstPart(readText('./inputs/06_ex'))).toBe(41)
    });
    it('should solve first part', () => {
        expect(firstPart(readText('./inputs/06'))).toBe(5531)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/06_ex'))).toBe(6)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/06'))).toBe(2165)
    });
});
