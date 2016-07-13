// Copyright (C) 2016 Chris Liebert
# benchmark-test

This includes multiple implementations of an algorithm for solving an arbitrary problem that 
involves determining what compounds can be produced from materials X, Y and Z.
There are 8 different products that can be built from materials X, Y and Z:
XXXYYYZ, YYZZZZZ, XXXXXXZZ, XXYYYYYYZ, YZZ, X, Y, Z

The intention is to determine which set of combinations will yield the least number of products.

The solution to this problem is not optimal, it is N^8th. There are much more efficient algorithms that
solve this problem using techniques such as memorization however this version is straight-forward.

There are three versions of the N^8th algorithm included here, they are implemented in Python, Java and Lua.
Running the benchmark script indicates Java and LuaJIT will execute the algorithm close to ~48X faster than Python.
By using the GNU GCJ compiler with optimisations, the runtime can be reduced by half.


**License:**
This program and it's source are availible under the "MIT License" please see LICENSE