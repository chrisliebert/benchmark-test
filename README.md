// Copyright (C) 2017 Chris Liebert
# benchmark-test

This includes multiple implementations of an algorithm for solving an arbitrary problem that 
involves determining what compounds can be produced from materials X, Y and Z.
There are 8 different products that can be built from materials X, Y and Z:
XXXYYYZ, YYZZZZZ, XXXXXXZZ, XXYYYYYYZ, YZZ, X, Y, and Z.

The intention is to determine which set of combinations will yield the least number of products.

The solution to this problem is not optimal, it is N^8th. There are much more efficient algorithms that
solve this problem using techniques such as memorization however this version is straight-forward.

There are four versions of the N^8th algorithm included here, they are implemented in Python, Java, C# and Lua.

Running the benchmark script demonstrates that C# on Mono will outperform Java and LuaJIT which have comparable performance, executing the algorithm close to ~48X faster than the Python3 interpreter.
The Lua interpreter performance doesn't come close to that of LuaJIT, however it is significantly faster than Python 3.
The Python execution can be sped up by using the Pypy JIT, although it is still the slowest JIT tested.
Java performance was more than doubled when using GNU GCJ compiler with optimisations.

Note: This benchmark not indicate which platform is better or faster since they address different concerns.

**License:**
This program and it's source are availible under the "MIT License" please see LICENSE
