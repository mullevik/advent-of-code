import { Vec, NumVec, Grid2, vec2 } from "./vec";

test("vec", () => {

    const abc = new Vec(["a", "b", "c"]);
    expect(abc.size()).toBe(3);
    expect(abc.get(1)).toBe("b");
    expect(() => { abc.get(15) }).toThrow(Error);

    const foo = vec2(1, 2);
    const bar = vec2(3, 4);

    expect(foo.addScalar(2).data).toStrictEqual([3, 4]);
    expect(foo.multiplyScalar(2).data).toStrictEqual([2, 4]);
    expect(foo.add(bar).data).toStrictEqual([4, 6]);
    expect(foo.multiply(bar).data).toStrictEqual([3, 8]);
    expect(foo.equals(bar)).toBe(false);
    expect(foo.equals(vec2(1, 2))).toBe(true);
});


test("grid", () => {

    const foo = new Grid2([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    const bar = Grid2.full(3, 3, 0);

    expect([foo.height(), foo.width()]).toStrictEqual([3, 3]);
    expect([bar.height(), bar.width()]).toStrictEqual([3, 3]);

    expect(foo.get(vec2(1, 1))).toBe(5);
    expect(foo.data[1][1]).toBe(5);

    expect(foo.contains(vec2(1, 1))).toBe(true);
    expect(foo.contains(vec2(6, 6))).toBe(false);
});