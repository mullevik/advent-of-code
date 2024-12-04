import { readText } from '../utils';
import { firstPart, secondPart } from './day_02';
describe('day_02', () => {
    it('should solve first part on example', () => {
        expect(firstPart(readText('./inputs/02_ex'))).toBe(2);
    });
    it('should solve first part', () => {
        expect(firstPart(readText('./inputs/02'))).toBe(598)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/02_ex'))).toBe(4)
    });
    it('should solve second part', () => {
        expect(secondPart(readText('./inputs/02'))).toBe(634)
    });
});