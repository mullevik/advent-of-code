import { GoogleSpreadsheet, GoogleSpreadsheetWorksheet } from "google-spreadsheet";
import { Member } from "./base";
import { JWT } from "google-auth-library";

export function createGoogleSheetsJWT(email: string, key: string): JWT {
    return new JWT({
        email: email,
        key: key,
        scopes: ['https://www.googleapis.com/auth/spreadsheets'],
    })
}

export async function getSheet(sheetId: string, sheetTitle: string, clientEmail: string, privateKey: string): Promise<GoogleSpreadsheetWorksheet> {
    const jwt = createGoogleSheetsJWT(clientEmail, privateKey);
    const doc = new GoogleSpreadsheet(sheetId, jwt);
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
    const [hours, minutes] = secondPart.split(":");


    return `${year}-${month}-${day} ${hours.padStart(2, "0")}:${minutes.padStart(2, "0")}`;
}

const N_DAYS = 12;

export function buildTable(members: Member[]): string[][] {

    let header = ["user"];

    for (const i of Array(N_DAYS).keys()) {
        header.push(`day ${i + 1} part 1`);
        header.push(`day ${i + 1} part 2`);
    }

    let rows = [header];

    const sortedMembers = [...members].sort((a, b) => (a.name.toLowerCase() < b.name.toLowerCase() ? -1 : 1));
    for (const member of sortedMembers) {
        let row = [member.name];

        for (const i of Array(N_DAYS).keys()) {
            const first = member.completions[i].firstPart;
            const second = member.completions[i].secondPart;
            row.push(first === null ? "" : formatDate(first));
            row.push(second === null ? "" : formatDate(second));
        }
        rows.push(row);
    }

    return rows;
}