import { buildTable, fetchLeaderBoard, getSheet, loadSecretsFromLocal as loadSecretsFromEnv, runBot, writeDataToSheet } from "./bot";


test.skip('update google sheet with latest results', async () => {
    const secrets = loadSecretsFromEnv();
    const members = await fetchLeaderBoard(secrets.aocLeaderboardUrl, secrets.aocSessionCookie);

    const tableData = buildTable(members);

    const sheet = await getSheet(secrets.googleServiceAccountJWT, secrets.googleSheetId, secrets.googleSheetTitle);

    await writeDataToSheet(sheet, tableData);

    expect(true).toBe(true);
});


test.skip('run bot locally', async () => {
    const secrets = loadSecretsFromEnv();
    const members = await runBot(secrets, false);
    expect(members.length).toBeGreaterThan(0);
})