import { fetchLeaderBoard, getDayIndexFromDate, getSolvers, getWinners } from "./aoc";
import { Member, Secrets } from "./base";
import { buildTable, getSheet, writeDataToSheet } from "./sheets";
import { buildSlackMessage, sendSlackMessage } from "./slack";

export async function fetchData(secrets: Secrets): Promise<[string, string[][]]> {

    const members = await fetchLeaderBoard(secrets.aocLeaderboardUrl, secrets.aocSessionCookie);
    const tableData = buildTable(members);

    const now = new Date();
    const before24Hours = new Date(now.getTime() - (24 * 60 * 60 * 1000));
    const [yesterDayIndex, yesterdayBegin, yesterdayEnd] = getDayIndexFromDate(before24Hours);
    const solvers = getSolvers(members, yesterDayIndex, yesterdayBegin, yesterdayEnd);
    const winners = getWinners(members, before24Hours, now);
    const slackMessageText = buildSlackMessage(solvers, winners, yesterDayIndex, secrets.googleSheetSharingLink);

    return [slackMessageText, tableData];
}

export async function writeToSheet(tableData: string[][], secrets: Secrets, dryRun: boolean) {
    const sheet = await getSheet(secrets.googleSheetId, secrets.googleSheetTitle, secrets.googleServiceAccountClientEmail, secrets.googleServiceAccountPrivateKey);
    if (dryRun) {
        console.log(`Would write ${tableData.length} rows to google sheet '${sheet.title}'`);
        for (const row of tableData) {
            console.log(row);
        }
    } else {
        await writeDataToSheet(sheet, tableData);
        console.log(`Written ${tableData.length} rows to google sheet '${sheet.title}'`);
    }
}

export async function writeToSlack(slackMessageText: string, secrets: Secrets, dryRun: boolean) {

    if (dryRun) {
        console.log(`Would send '${slackMessageText}' to ${secrets.slackWebhookUrl}`);
    } else {
        const slackResponse = await sendSlackMessage(secrets.slackWebhookUrl, slackMessageText);
        console.log(`Sent '${slackMessageText}' to ${secrets.slackWebhookUrl} with status ${slackResponse.status}`)
    }
}

export async function runBot(secrets: Secrets, dryRun: boolean) {

    const [slackMessageText, tableData] = await fetchData(secrets);

    await writeToSheet(tableData, secrets, dryRun);
    await writeToSlack(slackMessageText, secrets, dryRun);
}
