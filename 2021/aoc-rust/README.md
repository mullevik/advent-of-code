```
day       part 1    part 2    
------------------------------
day 04    0.922 ms  5.079 ms  
day 05    139.7 ms  140.7 ms  
day 06    0.014 ms  0.048 ms  
day 07    0.038 ms  0.041 ms  
day 08    0.882 ms  1.673 ms  
day 09    0.864 ms  2.925 ms  
day 12    9.309 ms  239.6 ms  
day 13    0.063 ms  0.125 ms  
day 14    0.441 ms  2.567 ms  
day 15    3.316 ms  109.8 ms  
day 16    0.087 ms  0.089 ms  
day 17    11.23 ms  11.06 ms  
day 19    4673. ms  5253. ms  
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

### 19

3D space rotations. Custom Vec3 and Mat3 implementations. 
Not my proudest solution: it is very naive and slow.
