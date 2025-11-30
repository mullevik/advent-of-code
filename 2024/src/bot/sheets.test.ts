import { Completion, loadSecretsFromLocal } from "./base";
import { buildTable, getSheet, writeDataToSheet } from "./sheets";

test("should be able to read/write to google sheets", async () => {
    const secrets = loadSecretsFromLocal();
    const sheet = await getSheet(secrets.googleSheetId, "test", secrets.googleServiceAccountClientEmail, secrets.googleServiceAccountPrivateKey);

    await writeDataToSheet(sheet, [["foo", "bar"]]);
    await sheet.loadCells();
    expect(await sheet.getCell(0, 0).value).toBe("foo");
    expect(await sheet.getCell(0, 1).value).toBe("bar");
});

test("should build table from members", () => {
    let barCompletions: Completion[] = [...Array(12).keys()].map(() => { return { firstPart: null, secondPart: null } });
    barCompletions[0].firstPart = new Date();
    barCompletions[0].secondPart = new Date();

    const table = buildTable([
        {
            name: "foo",
            n_stars: 0,
            completions: Array(12).fill({ firstPart: null, secondPart: null }),
        },
        {
            name: "bar",
            n_stars: 2,
            completions: barCompletions,
        }
    ]);
    const expectedWidth = 12 * 2 + 1;
    expect([...table.map((row) => row.length)]).toStrictEqual([expectedWidth, expectedWidth, expectedWidth]);
    expect(table[1][20]).toBe("");
    expect(table[1][1]).not.toBe("");
});

