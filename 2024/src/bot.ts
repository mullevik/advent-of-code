


import dotenv from 'dotenv';
import { readFileSync } from 'fs';
import { JWT } from 'google-auth-library';
import { GoogleSpreadsheet, GoogleSpreadsheetWorksheet } from 'google-spreadsheet';
import { readText } from './utils';

interface GoogleCloudServiceAccount {
    client_email: string,
    private_key: string,
}


export async function getSheet(auth: JWT, sheetId: string, sheetTitle: string): Promise<GoogleSpreadsheetWorksheet> {
    const doc = new GoogleSpreadsheet(sheetId, auth);
    await doc.loadInfo();
    const sheet = await doc.sheetsByTitle[sheetTitle];
    return sheet;
}

export async function writeDataToSheet(sheet: GoogleSpreadsheetWorksheet, data: string[][]) {
    await sheet.clear();
    await sheet.setHeaderRow(data[0]);
    await sheet.addRows(data.slice(1));
}

function formatDate(d: Date): string {

    const csDateString = new Intl.DateTimeFormat("cs-CZ", { dateStyle: 'short', timeStyle: 'short', timeZone: "Europe/Prague" }).format(d);
    const [firstPart, secondPart] = csDateString.split(" ");
    const [day, month, year] = firstPart.split(".");


    return `20${year}-${month}-${day} ${secondPart}`;
}

export function buildTable(members: Member[]): string[][] {

    let header = ["user"];

    for (const i of Array(25).keys()) {
        header.push(`day ${i + 1} part 1`);
        header.push(`day ${i + 1} part 2`);
    }

    let rows = [header];

    const sortedMembers = [...members].sort((a, b) => (a.name.toLowerCase() < b.name.toLowerCase() ? -1 : 1));
    for (const member of sortedMembers) {
        let row = [member.name];

        for (const i of Array(25).keys()) {
            const first = member.completions[i].firstPart;
            const second = member.completions[i].secondPart;
            row.push(first === null ? "" : formatDate(first));
            row.push(second === null ? "" : formatDate(second));
        }
        rows.push(row);
    }

    return rows;
}


export interface Completion {
    firstPart: Date | null,
    secondPart: Date | null,
}

export interface Member {
    name: string,
    n_stars: number,
    completions: Completion[],
}

interface CompletionDayLevelDTO {
    star_index: number,
    get_star_ts: number
}

interface CompletionsDayLevelDTO {
    [key: string]: CompletionDayLevelDTO
}

interface MemberDTO {
    stars: number,
    name: string,
    local_score: number,
    global_score: number,
    completion_day_level: {
        [key: string]: CompletionsDayLevelDTO
    },
    id: number
}

interface LeaderboardDTO {
    members: {
        [key: string]: MemberDTO
    };
}


export async function fetchLeaderBoard(leaderboardUrl: string, sessionCookie: string): Promise<Member[]> {
    const response = await fetch(leaderboardUrl, {
        method: 'GET',
        headers: {
            'Cookie': `session=${sessionCookie}`,
        },
    });

    if (!response.ok) {
        throw new Error(`Response not OK (${response.status})`)
    }

    const responseObject = await response.json();

    return parseLeaderboardObject(responseObject).filter((m) => m.n_stars > 0);
}

export function parseLeaderboardObject(leaderboard: LeaderboardDTO): Member[] {
    let members = [];
    for (const v of Object.values(leaderboard.members)) {
        members.push(parseMemberObject(v))
    }
    return members;
}


function parseMemberObject(member: MemberDTO): Member {

    let completions: Completion[] = new Array(25).fill({ firstPart: null, secondPart: null });

    for (const [dayName, completion] of Object.entries(member.completion_day_level)) {

        const dayIndex = parseInt(dayName) - 1;
        let firstPart = null;
        let secondPart = null;

        if ("1" in completion) {
            firstPart = new Date(completion["1"].get_star_ts * 1000);
        }

        if ("2" in completion) {
            secondPart = new Date(completion["2"].get_star_ts * 1000);
        }

        completions[dayIndex] = { firstPart: firstPart, secondPart: secondPart };
    }

    return {
        name: member.name,
        completions: completions,
        n_stars: member.stars,
    }
}

export function loadServiceAccountJWT(path: string): GoogleCloudServiceAccount {
    return JSON.parse(readFileSync(path, "utf8"));
}

export function loadAOCSessionCookie(path: string): string {
    return readText("./secret/aoc_session.env").split("=")[1].trim();
}


export function createGoogleSheetsJWT(email: string, key: string): JWT {
    return new JWT({
        email: email,
        key: key,
        scopes: ['https://www.googleapis.com/auth/spreadsheets'],
    })
}

export async function runBot(secrets: Secrets, dryRun: boolean): Promise<Member[]> {

    const members = await fetchLeaderBoard(secrets.aocLeaderboardUrl, secrets.aocSessionCookie);

    const tableData = buildTable(members);

    const sheet = await getSheet(secrets.googleServiceAccountJWT, secrets.googleSheetId, secrets.googleSheetTitle);

    if (dryRun) {
        console.log(`Would write ${tableData.length} rows to google sheet '${sheet.title}'`);
        for (const row of tableData) {
            console.log(row);
        }
    } else {
        await writeDataToSheet(sheet, tableData);
        console.log(`Written ${tableData.length} rows to google sheet '${sheet.title}'`);
    }

    const now = new Date();
    const before24Hours = new Date(now.getTime() - (24 * 60 * 60 * 1000));
    const [yesterDayIndex, yesterdayBegin, yesterdayEnd] = getDayIndexFromDate(before24Hours);
    const solvers = getSolvers(members, yesterDayIndex, yesterdayBegin, yesterdayEnd);
    const winners = getWinners(members, before24Hours, now);
    const slackMessageText = buildSlackMessage(solvers, winners, yesterDayIndex, secrets.googleSheetSharingLink);

    if (dryRun) {
        console.log(`Would send '${slackMessageText}' to ${secrets.slackWebhookUrl}`);
    } else {
        const slackResponse = await sendSlackMessage(secrets.slackWebhookUrl, slackMessageText);
        console.log(`Sent '${slackMessageText}' to ${secrets.slackWebhookUrl} with status ${slackResponse.status}`)
    }

    return members;
}

const GOOGLE_SHEET_SHARING_LINK = "https://docs.google.com/spreadsheets/d/1-Ap8xmA9MSLZgSNXwVgA8hLDGYOcGkEA8_OCrcDxREw/edit?usp=sharing";

function buildSlackMessage(solvers: string[], winners: Winner[], yesterDayIndex: number, sheetsSharingLink: string): string {

    const out = [];

    if (winners.length > 0) {
        for (const w of winners) {

            out.push(`*${w.name}* won day ${w.dayIndex + 1} (${prettyTime(w.timeToSolveMs)} after announcement) :steam_locomotive:`);
        }
    }

    if (solvers.length > 0) {
        const sortedSolvers = [...solvers].sort((a, b) => (a.toLowerCase() < b.toLowerCase() ? -1 : 1));
        out.push(`${sortedSolvers.map((s) => `*${s}*`).join(", ")} solved day ${yesterDayIndex + 1} in the same day it was announced`);
    }

    out.push(`<${sheetsSharingLink}|Completion times> updated`)

    return out.join("\n");
}

async function sendSlackMessage(slackWebhookUrl: string, text: string): Promise<Response> {

    const payload = {
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": text
                }
            }

        ]
    };
    const response = await fetch(slackWebhookUrl, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
    })

    if (!response.ok) {
        throw Error(`Error sending slack message to ${slackWebhookUrl}: STATUS=${response.status}`)
    }
    return response
}



interface Secrets {
    aocSessionCookie: string,
    googleServiceAccountJWT: JWT,
    slackWebhookUrl: string,
    aocLeaderboardUrl: string,
    googleSheetId: string,
    googleSheetTitle: string,
    googleSheetSharingLink: string,
}

function requireEnvVar(envVar: string): string {
    const val = process.env[envVar];
    if (val === undefined || val === null) {
        throw Error(`Undefined required env variable ${envVar}`);
    }
    return val;
}

export function loadSecretsFromLocal(): Secrets {
    dotenv.config({ path: "secrets.env" });
    const jwt = createGoogleSheetsJWT(requireEnvVar("GOOGLE_SERVICE_ACCOUNT_CLIENT_EMAIL"), requireEnvVar("GOOGLE_SERVICE_ACCOUNT_PRIVATE_KEY"));
    return {
        aocSessionCookie: requireEnvVar("AOC_SESSION_COOKIE"),
        googleServiceAccountJWT: jwt,
        slackWebhookUrl: requireEnvVar("SLACK_WEBHOOK_URL"),
        aocLeaderboardUrl: requireEnvVar("AOC_LEADERBOARD_URL"),
        googleSheetId: requireEnvVar("GOOGLE_SHEET_ID"),
        googleSheetTitle: requireEnvVar("GOOGLE_SHEET_TITLE"),
        googleSheetSharingLink: requireEnvVar("GOOGLE_SHEET_SHARING_LINK"),
    }
}

interface Winner {
    name: string
    dayIndex: number
    submissionDate: Date
    timeToSolveMs: number
}

function compareMembersAtDay(a: Member, b: Member, dayIndex: number): number {
    const a_val = a.completions[dayIndex].secondPart === null ? Infinity : a.completions[dayIndex].secondPart.getTime();
    const b_val = b.completions[dayIndex].secondPart === null ? Infinity : b.completions[dayIndex].secondPart.getTime();
    return a_val - b_val
}

export function getWinners(members: Member[], begin: Date, end: Date): Winner[] {

    function safeSecondPart(m: Member, dayIndex: number): Date | null {
        if (dayIndex < m.completions.length) {
            return m.completions[dayIndex].secondPart;
        }
        return null;
    }

    const winners = [];
    for (const i of Array(25).keys()) {

        const sortedMembers = [...members].filter((m) => safeSecondPart(m, i) !== null).sort((a, b) => compareMembersAtDay(a, b, i));

        if (sortedMembers.length > 0) {
            const submissionDate = sortedMembers[0].completions[i].secondPart as Date;
            winners.push({
                name: sortedMembers[0].name,
                dayIndex: i,
                submissionDate: submissionDate,
                timeToSolveMs: submissionDate.getTime() - AOC_STARTS[i].getTime(),
            })
        }
    }

    return winners.filter((w) => begin.getTime() <= w.submissionDate.getTime() && end.getTime() > w.submissionDate.getTime());
}

function timeOrInf(val: Date | null): number {
    if (val === null) {
        return Infinity;
    }
    return val.getTime();
}

const AOC_STARTS = [
    new Date(1733029200000),
    new Date(1733115600000),
    new Date(1733202000000),
    new Date(1733288400000),
    new Date(1733374800000),
    new Date(1733461200000),
    new Date(1733547600000),
    new Date(1733634000000),
    new Date(1733720400000),
    new Date(1733806800000),
    new Date(1733893200000),
    new Date(1733979600000),
    new Date(1734066000000),
    new Date(1734152400000),
    new Date(1734238800000),
    new Date(1734325200000),
    new Date(1734411600000),
    new Date(1734498000000),
    new Date(1734584400000),
    new Date(1734670800000),
    new Date(1734757200000),
    new Date(1734843600000),
    new Date(1734930000000),
    new Date(1735016400000),
    new Date(1735102800000),
];

export function getDayIndexFromDate(x: Date): [number, Date, Date] {
    const xTime = x.getTime();

    for (var i = 0; i < AOC_STARTS.length; i++) {
        const beginTime = AOC_STARTS[i].getTime();
        const endTime = beginTime + 24 * 60 * 60 * 1000;  // add 24 hours

        if (xTime >= beginTime && xTime < endTime) {
            return [i, new Date(beginTime), new Date(endTime)];
        }
    }

    return [24, AOC_STARTS[24], new Date(AOC_STARTS[24].getTime() + 24 * 60 * 60 * 1000)];
}

export function getSolvers(members: Member[], dayIndex: number, begin: Date, end: Date): string[] {

    if (dayIndex < 0 || dayIndex >= AOC_STARTS.length) {
        return [];
    }

    return members
        .filter(
            (m) => timeOrInf(m.completions[dayIndex].secondPart) >= begin.getTime()
                && timeOrInf(m.completions[dayIndex].secondPart) < end.getTime()
        )
        .map((m) => m.name)
}


export function prettyTime(timeMs: number) {
    let minutes = Math.floor(timeMs / (60 * 1000));
    let hours = Math.floor(timeMs / (60 * 60 * 1000));
    let days = Math.floor(timeMs / (24 * 60 * 60 * 1000));


    let out = [];

    if (days > 0) {
        out.push(`${days} days`)
    }
    if (hours % 24 > 0) {
        out.push(`${hours % 24} hours`)
    }
    if (minutes % 60 > 0) {
        out.push(`${minutes % 60} minutes`)
    }

    if (out.length < 1) {
        return `${timeMs} ms`;
    } else {
        return out.join(" ");
    }
}
