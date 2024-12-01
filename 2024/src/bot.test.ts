import { buildTable, Completion, createGoogleSheetsJWT, fetchLeaderBoard, getSheet, loadSecretsFromLocal, loadServiceAccountJWT as loadServiceAccount, parseLeaderboardObject, runBot, writeDataToSheet } from "./bot";
import { readText } from "./utils";


describe("aoc bot", () => {
    it("should be able to read/write to google sheets", async () => {
        const auth = loadSecretsFromLocal()
        const sheet = await getSheet(auth.googleServiceAccountJWT, "test");

        await writeDataToSheet(sheet, [["foo", "bar"]]);
        await sheet.loadCells();
        expect(await sheet.getCell(0, 0).value).toBe("foo");
        expect(await sheet.getCell(0, 1).value).toBe("bar");
    });

    it("should be able to read aoc leaderboard", async () => {
        const auth = loadSecretsFromLocal();

        const members = await fetchLeaderBoard(auth.aocSessionCookie);
        expect(members.length).toBeGreaterThan(0);
    });

    it("should be able to parse aoc leaderboard", () => {
        const exampleAocLeaderboard = JSON.parse(readText("src/example_leaderboard.json"));

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

    it("should build table from members", () => {
        let barCompletions: Completion[] = [...Array(25).keys()].map(() => { return { firstPart: null, secondPart: null } });
        barCompletions[0].firstPart = new Date();
        barCompletions[0].secondPart = new Date();

        const table = buildTable([
            {
                name: "foo",
                n_stars: 0,
                completions: Array(25).fill({ firstPart: null, secondPart: null }),
            },
            {
                name: "bar",
                n_stars: 2,
                completions: barCompletions,
            }
        ]);
        const expectedWidth = 25 * 2 + 1;
        expect([...table.map((row) => row.length)]).toStrictEqual([expectedWidth, expectedWidth, expectedWidth]);
        expect(table[1][20]).toBe("");
        expect(table[1][1]).not.toBe("");
    });

    it("should not fail on dry run", async () => {
        const auth = loadSecretsFromLocal();
        const members = await runBot(auth, true);
        expect(members.length).toBeGreaterThan(0);
    });
});