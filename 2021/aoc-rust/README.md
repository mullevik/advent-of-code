day       part 1    part 2                                                                     │
------------------------------                                                                 │
day 04    0.936 ms  5.154 ms                                                                   │
day 05    133.4 ms  136.5 ms                                                                   │
day 06    0.014 ms  0.047 ms                                                                   │
day 07    0.043 ms  0.048 ms


## Daily notes

### 04
Using `hyperfine` command to perf values

Parallelization works but the runtime of aoc input was so fast that it did not make any difference.

I was only able to use `par_iter_mut` with `for_each` stuff...

Once it is written functionally - the paralellization can not be made easier actually.

### 05
Implementing custom `Iterator` to mimic python's yield syntax.

This seems to be very logical in Rust but the syntax is not as clear as in python.

### 06

Dynamic programming with recursion and memoization.

### 07

Straigh-forward iterative optimization.
