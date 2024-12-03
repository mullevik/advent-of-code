import { loadSecretsFromLocal } from "./base";
import { runBot } from "./run";


test("should not fail on dry run", async () => {
    const secrets = loadSecretsFromLocal();
    await runBot(secrets, true);
    expect(true).toBe(true);
});