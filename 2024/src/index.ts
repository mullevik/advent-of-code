import * as functions from "@google-cloud/functions-framework";
import { loadSecretsFromLocal, runBot } from "./bot";

functions.http('aocBotEntrypoint', (req, res) => {
    const auth = loadSecretsFromLocal();
    runBot(auth, false)
        .then((m) => res.send('Bot completed successfully'))
        .catch((e) => console.error(e));
});