import { readText } from '../utils';
import { firstPart, secondPart } from './day_09';
test('should solve first part on example', () => {
    expect(firstPart(readText('./inputs/09_ex'))).toBe(1928)
});
test('should solve first part', () => {
    expect(firstPart(readText('./inputs/09'))).toBe(6398608069280)
});
test('should solve second part', () => {
    expect(secondPart(readText('./inputs/09_ex'))).toBe(2858)
});
test('should solve second part on example', () => {
    expect(secondPart(readText('./inputs/09'))).toBe(6427437134372)
});