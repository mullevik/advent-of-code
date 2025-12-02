import { Completion, Member, Winner } from "./base";

const AOC_STARTS = [
    new Date(1764565200000),// 01
    new Date(1764651600000),// 02
    new Date(1764738000000),// 03
    new Date(1764824400000),// 04
    new Date(1764910800000),// 05
    new Date(1764997200000),// 06
    new Date(1765083600000),// 07
    new Date(1765170000000),// 08
    new Date(1765256400000),// 09
    new Date(1765342800000),// 10
    new Date(1765429200000),// 11
    new Date(1765515600000),// 12
];

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

const N_DAYS = 12;


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

    let completions: Completion[] = new Array(N_DAYS).fill({ firstPart: null, secondPart: null });

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
    for (const i of Array(N_DAYS).keys()) {

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

export function getDayIndexFromDate(x: Date): [number, Date, Date] {
    const xTime = x.getTime();

    for (var i = 0; i < AOC_STARTS.length; i++) {
        const beginTime = AOC_STARTS[i].getTime();
        const endTime = beginTime + 24 * 60 * 60 * 1000;  // add 24 hours

        if (xTime >= beginTime && xTime < endTime) {
            return [i, new Date(beginTime), new Date(endTime)];
        }
    }

    return [N_DAYS - 1, AOC_STARTS[N_DAYS - 1], new Date(AOC_STARTS[N_DAYS - 1].getTime() + 24 * 60 * 60 * 1000)];
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

