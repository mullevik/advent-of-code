import { buildTable, fetchLeaderBoard, getSheet, loadSecretsFromLocal, runBot, SHEET_TITLE, writeDataToSheet } from "./bot";


test.skip('it is not snowing', async () => {
    const auth = loadSecretsFromLocal();
    const members = await fetchLeaderBoard(auth.aocSessionCookie);

    const tableData = buildTable(members);

    const sheet = await getSheet(auth.googleServiceAccountJWT, SHEET_TITLE);

    await writeDataToSheet(sheet, tableData);

    expect(true).toBe(true);
});