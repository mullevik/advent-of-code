import dotenv from 'dotenv';

export interface Secrets {
    aocSessionCookie: string,
    googleServiceAccountClientEmail: string,
    googleServiceAccountPrivateKey: string,
    slackWebhookUrl: string,
    aocLeaderboardUrl: string,
    googleSheetId: string,
    googleSheetTitle: string,
    googleSheetSharingLink: string,
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

export interface Winner {
    name: string
    dayIndex: number
    submissionDate: Date
    timeToSolveMs: number
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
    return {
        aocSessionCookie: requireEnvVar("AOC_SESSION_COOKIE"),
        googleServiceAccountClientEmail: requireEnvVar("GOOGLE_SERVICE_ACCOUNT_CLIENT_EMAIL"),
        googleServiceAccountPrivateKey: requireEnvVar("GOOGLE_SERVICE_ACCOUNT_PRIVATE_KEY"),
        slackWebhookUrl: requireEnvVar("SLACK_WEBHOOK_URL"),
        aocLeaderboardUrl: requireEnvVar("AOC_LEADERBOARD_URL"),
        googleSheetId: requireEnvVar("GOOGLE_SHEET_ID"),
        googleSheetTitle: requireEnvVar("GOOGLE_SHEET_TITLE"),
        googleSheetSharingLink: requireEnvVar("GOOGLE_SHEET_SHARING_LINK"),
    }
}