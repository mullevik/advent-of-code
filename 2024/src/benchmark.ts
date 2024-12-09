import { benchmarkMultiple, RunnableDay } from "./utils";
import * as day_01 from "./days/day_01";
import * as day_02 from "./days/day_02";
import * as day_03 from "./days/day_03";
import * as day_04 from "./days/day_04";
import * as day_05 from "./days/day_05";
import * as day_06 from "./days/day_06";
import * as day_07 from "./days/day_07";
import * as day_08 from "./days/day_08";
import * as day_09 from "./days/day_09";


function day(num: number, module: any, firstRepeats: number = 5, secondRepeats: number = 5): RunnableDay {
    return {
        dayNumber: num,
        firstPartFn: module.firstPart,
        secondPartFn: module.secondPart,
        firstPartRepeats: firstRepeats,
        secondPartRepeats: secondRepeats
    };
}

benchmarkMultiple(
    [
        day(1, day_01),
        day(2, day_02),
        day(3, day_03),
        day(4, day_04),
        day(5, day_05),
        day(6, day_06, 5, 1),
        day(7, day_07, 5, 1),
        day(8, day_08),
        day(9, day_09),
    ]
)