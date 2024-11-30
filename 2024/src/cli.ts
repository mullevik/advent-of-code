import { buildDay } from "./utils";

if (process.argv.length > 2) {
    console.log(process.argv);
    const dayNumberString = process.argv[2];

    const dayNumber = parseInt(dayNumberString);

    buildDay(dayNumber);

} else {
    console.log('Please specify a day number');
}