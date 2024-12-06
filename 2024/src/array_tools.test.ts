import { arraySum } from "./array_tools"

test("arraySum", () => {
    expect(arraySum([1, 2, 3])).toBe(6);
    expect(arraySum([])).toBe(0);
    expect(arraySum([true, false, true])).toBe(2);
})