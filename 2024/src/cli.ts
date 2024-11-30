import { buildDay } from "./utils";

if (process.argv.length > 2) {
    const dayNumberString = process.argv[2];
    const dayNumber = parseInt(dayNumberString);
    buildDay(dayNumber);
} else {
    throw Error("Please specify a day number");
}