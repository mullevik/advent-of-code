import { readText } from '../utils';
import { firstPart, secondPart } from './day_10';
test('should solve first part on example', () => {
    expect(firstPart(readText('./inputs/10_ex'))).toBe(36)
});
test('should solve first part', () => {
    expect(firstPart(readText('./inputs/10'))).toBe(667)
});
test('should solve second part', () => {
    expect(secondPart(readText('./inputs/10_ex'))).toBe(81)
});
test('should solve second part on example', () => {
    expect(secondPart(readText('./inputs/10'))).toBe(-1)
});