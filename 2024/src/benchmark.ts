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
import * as day_10 from "./days/day_10";
import * as day_11 from "./days/day_11";


function day(num: number, module: any): RunnableDay {
    return {
        dayNumber: num,
        firstPartFn: module.firstPart,
        secondPartFn: module.secondPart,
    };
}

benchmarkMultiple(
    [
        day(1, day_01),
        day(2, day_02),
        day(3, day_03),
        day(4, day_04),
        day(5, day_05),
        day(6, day_06),
        day(7, day_07),
        day(8, day_08),
        day(9, day_09),
        day(10, day_10),
        day(11, day_11),
    ]
)