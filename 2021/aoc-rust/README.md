```
day       part 1    part 2    
------------------------------
day 04    1.066 ms  5.677 ms  
day 05    140.4 ms  140.0 ms  
day 06    0.015 ms  0.050 ms  
day 07    0.041 ms  0.043 ms  
day 08    0.916 ms  1.838 ms  
day 09    0.930 ms  3.137 ms  
day 12    10.11 ms  257.9 ms  
day 13    0.076 ms  0.140 ms  
day 14    0.483 ms  2.807 ms  
day 15    3.602 ms  117.9 ms  
day 16    0.104 ms  0.107 ms  
day 17    17.80 ms  15.00 ms  
day 18    38.45 ms  536.5 ms  
day 19    622.5 ms  657.0 ms  
day 20    0.490 ms  13.01 ms  
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
Not my proudest solution: it is very naive and very slow.
Faster HashSet and parallelization made it quite fast though.

### 20

2D convolution with out-of-bounds value selection.
