import { readText } from '../utils';
import { firstPart, secondPart } from './day_08';
test('should solve first part on example', () => {
    expect(firstPart(readText('./inputs/08_ex'))).toBe(14)
});
test('should solve first part', () => {
    expect(firstPart(readText('./inputs/08'))).toBe(394)
});
test('should solve second part', () => {
    expect(secondPart(readText('./inputs/08_ex'))).toBe(34)
});
test('should solve second part on example', () => {
    expect(secondPart(readText('./inputs/08'))).toBe(1277)
});