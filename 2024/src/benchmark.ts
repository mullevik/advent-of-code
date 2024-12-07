import { benchmarkMultiple } from "./utils";
import * as day_01 from "./days/day_01";
import * as day_02 from "./days/day_02";
import * as day_03 from "./days/day_03";
import * as day_04 from "./days/day_04";
import * as day_05 from "./days/day_05";
import * as day_06 from "./days/day_06";
import * as day_07 from "./days/day_07";


benchmarkMultiple(
    [
        { dayNumber: 1, firstPartFn: day_01.firstPart, secondPartFn: day_01.secondPart },
        { dayNumber: 2, firstPartFn: day_02.firstPart, secondPartFn: day_02.secondPart },
        { dayNumber: 3, firstPartFn: day_03.firstPart, secondPartFn: day_03.secondPart },
        { dayNumber: 4, firstPartFn: day_04.firstPart, secondPartFn: day_04.secondPart },
        { dayNumber: 5, firstPartFn: day_05.firstPart, secondPartFn: day_05.secondPart },
        { dayNumber: 6, firstPartFn: day_06.firstPart, secondPartFn: day_06.secondPart },
        { dayNumber: 7, firstPartFn: day_07.firstPart, secondPartFn: day_07.secondPart },
    ]
)