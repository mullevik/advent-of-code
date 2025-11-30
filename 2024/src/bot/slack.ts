import { Winner } from "./base";

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


export function buildSlackMessage(solvers: string[], winners: Winner[], yesterDayIndex: number, sheetsSharingLink: string): string {

    const out = [];

    if (winners.length > 0) {
        for (const w of winners) {

            out.push(`*${w.name}* deserves a :gift: for day ${w.dayIndex + 1} (solved ${prettyTime(w.timeToSolveMs)} after announcement)`);
        }
    }

    if (solvers.length > 0) {
        const sortedSolvers = [...solvers].sort((a, b) => (a.toLowerCase() < b.toLowerCase() ? -1 : 1));
        out.push(`${sortedSolvers.map((s) => `*${s}*`).join(", ")} solved day ${yesterDayIndex + 1} in the same day it was announced`);
    }

    out.push(`<${sheetsSharingLink}|Completion times> updated`)

    return out.join("\n");
}

export async function sendSlackMessage(slackWebhookUrl: string, text: string): Promise<Response> {

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
