import { loadSecretsFromLocal } from "./base";
import { fetchData, writeToSheet, runBot } from "./run";

test.skip('update google sheet with latest results', async () => {
    const secrets = loadSecretsFromLocal();
    const [slackMessage, tableData] = await fetchData(secrets);
    await writeToSheet(tableData, secrets, false);

    expect(true).toBe(true);
});


test.skip('run bot locally', async () => {
    const secrets = loadSecretsFromLocal();
    await runBot(secrets, false);

    expect(true).toBe(true);
})