```
day       part 1    part 2    
------------------------------
day 04    0.887 ms  4.871 ms  
day 05    134.8 ms  135.8 ms  
day 06    0.014 ms  0.047 ms  
day 07    0.037 ms  0.040 ms  
day 08    0.864 ms  1.707 ms  
day 09    0.916 ms  3.056 ms  
day 12    9.774 ms  249.3 ms  
day 13    0.067 ms  0.133 ms  
day 14    0.465 ms  2.690 ms  
day 15    3.518 ms  115.9 ms  
day 16    0.092 ms  0.094 ms  
day 17    11.94 ms  11.80 ms  
day 18    36.35 ms  507.9 ms  
day 19    5132. ms  5016. ms
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

### 18

Binary-tree-like structure which needed to be modified using pointers.
This proved to be very challanging in rust.
Solved using `Rc` shared references.
It is likely very very overengineered.

### 19

3D space rotations. Custom Vec3 and Mat3 implementations. 
Not my proudest solution: it is very naive and slow.
