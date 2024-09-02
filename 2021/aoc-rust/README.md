```
day       part 1    part 2    
------------------------------
day 04    0.910 ms  5.182 ms  
day 05    140.5 ms  141.0 ms  
day 06    0.015 ms  0.048 ms  
day 07    0.039 ms  0.042 ms  
day 08    0.890 ms  1.846 ms  
day 09    0.953 ms  3.127 ms  
day 12    10.28 ms  268.7 ms  
day 13    0.067 ms  0.133 ms  
day 14    0.475 ms  2.731 ms  
day 15    3.135 ms  106.6 ms  
day 16    0.096 ms  0.098 ms  
day 17    11.95 ms  11.67 ms  
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


### 15

Very straight-forward shortest path problem.

### 16

Custom regular-language parser - tedious but simple.

### 17

Shooting probe at target. There probably exists an analytical solution... 
oh well, I solved it iteratively and with a baked-in constant. Shame on me.
