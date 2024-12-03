import { prettyTime } from "./slack";

test("should have pretty time", () => {
    expect(prettyTime(300)).toBe("300 ms");
    expect(prettyTime(32 * 1000)).toBe("32000 ms");
    expect(prettyTime(32 * 60 * 1000)).toBe("32 minutes");
    expect(prettyTime(32 * 60 * 60 * 1000)).toBe("1 days 8 hours");
    expect(prettyTime((32 * 24 * 60 * 60 * 1000) + (70 * 60 * 1000))).toBe("32 days 1 hours 10 minutes");
});