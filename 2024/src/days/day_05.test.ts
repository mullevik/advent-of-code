import { readText } from '../utils';
import { firstPart, secondPart } from './day_05';
describe('day_05', () => {
    it('should solve first part on example', () => {
        expect(firstPart(readText('./inputs/05_ex'))).toBe(143)
    });
    it('should solve first part', () => {
        expect(firstPart(readText('./inputs/05'))).toBe(4281)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/05_ex'))).toBe(123)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/05'))).toBe(5466)
    });
});