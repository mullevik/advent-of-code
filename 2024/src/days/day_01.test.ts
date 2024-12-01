import {readText} from '../utils';
import {firstPart,secondPart} from './day_01';
describe('day_01', () => {
it('should solve first part', () => {
expect(firstPart(readText('./inputs/01'))).toBe(-1)
});
it('should solve second part', () => {
expect(secondPart(readText('./inputs/01'))).toBe(-1)
});
});