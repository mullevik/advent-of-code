```
day       part 1    part 2    
------------------------------
day 04    0.980 ms  5.227 ms  
day 05    148.2 ms  147.7 ms  
day 06    0.016 ms  0.052 ms  
day 07    0.042 ms  0.046 ms  
day 08    0.959 ms  1.792 ms  
day 09    0.994 ms  3.267 ms  
day 12    10.33 ms  267.6 ms  
day 13    0.073 ms  0.148 ms  
day 14    0.510 ms  2.965 ms  
day 15    3.781 ms  125.9 ms  
day 16    0.103 ms  0.107 ms  
day 17    16.11 ms  15.58 ms  
day 18    40.15 ms  551.9 ms  
day 19    558.3 ms  574.9 ms  
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
