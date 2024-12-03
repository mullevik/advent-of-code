import { readFileSync } from "fs";
import { fetchLeaderBoard, getDayIndexFromDate, getSolvers, getWinners, parseLeaderboardObject } from "./aoc";
import { loadSecretsFromLocal } from "./base";

test("should be able to read aoc leaderboard", async () => {
    const secrets = loadSecretsFromLocal();

    const members = await fetchLeaderBoard(secrets.aocLeaderboardUrl, secrets.aocSessionCookie);
    expect(members.length).toBeGreaterThan(0);
});

test("should be able to parse aoc leaderboard", () => {
    const exampleAocLeaderboard = JSON.parse(readFileSync("src/example_leaderboard.json", "utf8"));

    const parsedLeaderboard = parseLeaderboardObject(exampleAocLeaderboard);
    expect(parsedLeaderboard.length).toBe(2);

    const foo = parsedLeaderboard.find((m) => m.name == "foo");
    expect(foo).toBeDefined();
    expect(foo?.completions.length).toBe(25);
    expect(foo?.completions[0]).toStrictEqual({ firstPart: null, secondPart: null })
    const bar = parsedLeaderboard.find((m) => m.name == "bar");
    expect(bar?.completions.length).toBe(25);
    expect(bar?.completions[0].firstPart).not.toBeNull()
    expect(bar?.completions[0].secondPart).not.toBeNull()
});

test("should find solvers", () => {

    const members = [
        {
            name: "foo",
            n_stars: 2,
            completions: [{ firstPart: new Date("2024-12-01 8:00"), secondPart: new Date("2024-12-01 9:00") }],
        },
        {
            name: "bar",
            n_stars: 2,
            completions: [{ firstPart: new Date("2024-12-02 12:00"), secondPart: new Date("2024-12-02 13:00") }],
        },
    ];

    const [dayIndex, begin, end] = getDayIndexFromDate(new Date("2024-12-01 9:30"));

    expect(getSolvers(members, dayIndex, begin, end)).toStrictEqual(["foo"]);
    expect(getSolvers(members, -1, begin, end)).toStrictEqual([]);
});

test("should find winners", () => {
    const members = [
        {
            name: "foo",
            n_stars: 2,
            completions: [{ firstPart: new Date("2024-12-01 8:00"), secondPart: new Date("2024-12-01 9:00") }],
        },
        {
            name: "bar",
            n_stars: 2,
            completions: [{ firstPart: new Date("2024-12-02 12:00"), secondPart: new Date("2024-12-02 13:00") }],
        },
    ];
    const [dayIndex, begin, end] = getDayIndexFromDate(new Date("2024-12-01 9:30"));

    expect(getWinners(members, begin, end)).toStrictEqual([{ name: "foo", dayIndex: 0, submissionDate: new Date("2024-12-01 9:00"), timeToSolveMs: 10800000 }]);
});