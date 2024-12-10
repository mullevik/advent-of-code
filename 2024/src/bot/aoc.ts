import { Completion, Member, Winner } from "./base";

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

