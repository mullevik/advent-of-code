import * as functions from "@google-cloud/functions-framework";
import { loadSecretsFromLocal } from "./bot/base";
import { runBot } from "./bot/run";

interface PubSubData {
    subscription: string;
    message: {
        messageId: string;
        publishTime: string;
        data: string;
        attributes?: { [key: string]: string };
    };
}

functions.cloudEvent<PubSubData>('aocBotEntrypoint', ce => {
    console.log(ce.data?.message.messageId);
    const secrets = loadSecretsFromLocal();
    runBot(secrets, false)
        .then(() => console.log("Execution successful"))
        .catch((e) => console.error(e));
});