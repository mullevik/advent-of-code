```
day       part 1    part 2    
------------------------------
day 04    0.984 ms  5.306 ms  
day 05    143.4 ms  147.9 ms  
day 06    0.014 ms  0.049 ms  
day 07    0.040 ms  0.043 ms  
day 08    0.797 ms  1.515 ms  
day 09    0.974 ms  2.813 ms  
day 13    0.065 ms  0.138 ms  
```

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

Convex optimization problem with bisect search.

> Note that there are some analytic solutions (median and mean)

### 08

Constraint satisfaction problem.

Solved by backpropagation.

### 09

DFS with connected-components.

Using custom grid implementation.


### 13

Very straight-forward imperative simulation.

### 14

Quite difficult task with recursion and memoization.

