```
day       part 1    part 2    
------------------------------
day 04    1.390 ms  10.07 ms  
day 05    189.2 ms  161.3 ms  
day 06    0.016 ms  0.052 ms  
day 07    0.048 ms  0.046 ms  
day 08    0.966 ms  1.925 ms  
day 09    0.998 ms  3.314 ms  
day 10    0.064 ms  0.065 ms  
day 12    12.12 ms  286.6 ms  
day 13    0.092 ms  0.154 ms  
day 14    0.658 ms  3.096 ms  
day 15    3.981 ms  131.0 ms  
day 16    0.097 ms  0.102 ms  
day 17    12.93 ms  13.22 ms  
day 18    38.98 ms  519.3 ms  
day 19    545.1 ms  583.9 ms  
day 20    0.507 ms  13.03 ms  
day 21    0.003 ms  9.366 ms  
day 22    6936. ms  7222. ms  
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

### 10

Stack based parsing of matching brackets.


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
Not my proudest solution: it is very naive and very slow.
Faster HashSet and parallelization made it quite fast though.

### 20

2D convolution with out-of-bounds value selection.

### 21

Recursion with memoization.

### 22

Intersections of cuboids in 3D.
Implementation by cutting cuboids so that they do not overlap - stupid and slow but it works.
