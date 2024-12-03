import * as functions from "@google-cloud/functions-framework";
import { loadSecretsFromLocal } from "./bot/base";
import { runBot } from "./bot/run";

functions.http('aocBotEntrypoint', (req, res) => {
    const secrets = loadSecretsFromLocal();
    runBot(secrets, false)
        .then(() => res.send('Bot completed successfully'))
        .catch((e) => console.error(e));
});