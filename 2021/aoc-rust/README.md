```
day       part 1    part 2    
------------------------------
day 04    0.872 ms  4.939 ms  
day 05    133.8 ms  136.6 ms  
day 06    0.014 ms  0.046 ms  
day 07    0.038 ms  0.040 ms  
day 08    0.864 ms  1.679 ms  
day 09    0.906 ms  2.911 ms  
day 12    9.468 ms  241.4 ms  
day 13    0.068 ms  0.129 ms  
day 14    0.451 ms  2.624 ms  
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


### 12

Graph traversal using DFS.

Custom Graph trait implementation.


### 13

Very straight-forward imperative simulation.

### 14

Quite difficult task with recursion and memoization.

