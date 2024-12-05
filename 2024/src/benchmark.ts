import { benchmarkMultiple } from "./utils";
import * as day_01 from "./days/day_01";
import * as day_02 from "./days/day_02";
import * as day_04 from "./days/day_04";


benchmarkMultiple(
    [
        { dayNumber: 1, firstPartFn: day_01.firstPart, secondPartFn: day_01.secondPart },
        { dayNumber: 2, firstPartFn: day_02.firstPart, secondPartFn: day_02.secondPart },
        { dayNumber: 4, firstPartFn: day_04.firstPart, secondPartFn: day_04.secondPart },

    ]
)