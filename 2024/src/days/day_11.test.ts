import { readText } from '../utils';
import { firstPart, secondPart } from './day_11';
test('should solve first part on example', () => {
    expect(firstPart(readText('./inputs/11_ex'))).toBe(55312)
});
test('should solve first part', () => {
    expect(firstPart(readText('./inputs/11'))).toBe(231278)
});

test('should solve second part', () => {
    expect(secondPart(readText('./inputs/11_ex'))).toBe(65601038650482)
});
test('should solve second part on example', () => {
    expect(secondPart(readText('./inputs/11'))).toBe(274229228071551)
});