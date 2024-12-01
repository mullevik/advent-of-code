


import { GoogleSpreadsheet, GoogleSpreadsheetWorksheet } from 'google-spreadsheet';
import { JWT } from 'google-auth-library';
import { readFileSync } from 'fs';
import { readText } from './utils';

const LEADERBOARD_URL = "https://adventofcode.com/2021/leaderboard/private/view/664050.json"
const SHEET_ID = "1-Ap8xmA9MSLZgSNXwVgA8hLDGYOcGkEA8_OCrcDxREw";
const SHEET_TITLE = "2024";

interface GoogleCloudServiceAccount {
    client_email: string,
    private_key: string,
}


export async function getSheet(auth: JWT, sheetTitle: string): Promise<GoogleSpreadsheetWorksheet> {
    const doc = new GoogleSpreadsheet(SHEET_ID, auth);
    await doc.loadInfo();
    const sheet = await doc.sheetsByTitle[sheetTitle];
    return sheet;
}

export async function writeDataToSheet(sheet: GoogleSpreadsheetWorksheet, data: string[][]) {
    await sheet.clear();
    await sheet.setHeaderRow(data[0]);
    await sheet.addRows(data.slice(1));
    console.log(`Written ${data.length} rows to ${sheet.title}`);
}

function formatDate(d: Date): string {
    const zeroPadMonth = (d.getMonth() + 1).toString().padStart(2, "0");
    const zeroPadDay = (d.getDate()).toString().padStart(2, "0");
    const zeroPadHours = (d.getHours()).toString().padStart(2, "0");
    const zeroPadMinutes = (d.getMinutes()).toString().padStart(2, "0");
    return `${d.getFullYear()}-${zeroPadMonth}-${zeroPadDay} ${zeroPadHours}:${zeroPadMinutes}`
}

export function buildTable(members: Member[]): string[][] {

    let header = ["user"];

    for (const i of Array(25).keys()) {
        header.push(`day ${i + 1} part 1`);
        header.push(`day ${i + 1} part 2`);
    }

    let rows = [header];

    const sortedMembers = [...members].sort((a, b) => (a.name < b.name ? -1 : 1));
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


export async function fetchLeaderBoard(sessionCookie: string): Promise<Member[]> {
    const response = await fetch(LEADERBOARD_URL, {
        method: 'GET',
        headers: {
            'Cookie': `session=${sessionCookie}`,
        },
    });

    if (!response.ok) {
        throw new Error(`Response not OK (${response.status})`)
    }

    const responseObject = await response.json();

    return parseLeaderboardObject(responseObject);
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
            firstPart = new Date(completion["1"].get_star_ts);
        }

        if ("2" in completion) {
            secondPart = new Date(completion["2"].get_star_ts);
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

export async function runBot(auth: Auth, dryRun: boolean): Promise<Member[]> {

    const members = await fetchLeaderBoard(auth.aocSessionCookie);

    const tableData = buildTable(members);

    const sheet = await getSheet(auth.googleServiceAccountJWT, SHEET_TITLE);

    if (dryRun) {
        console.log(`Would write ${tableData.length} rows to google sheet '${sheet.title}'`)
    } else {
        await writeDataToSheet(sheet, tableData);
    }

    return members;
}

interface Auth {
    aocSessionCookie: string,
    googleServiceAccountJWT: JWT,
}

export function loadSecretsFromLocal(): Auth {
    const account = loadServiceAccountJWT("./secret/aoc-bot-443408-9cb9c6dd7dc7.json");
    const jwt = createGoogleSheetsJWT(account.client_email, account.private_key);
    return {
        aocSessionCookie: loadAOCSessionCookie("./secret/aoc_session.env"),
        googleServiceAccountJWT: jwt,
    }
}