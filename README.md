# json_benchmarking

This repository contains all the JSON parsers I could find on crates.io, and benchmarks them.

## Notes
You must be on the nightly branch to run these benchmarks.<br><br>

There are no guarantees that all of these libraries work for all, if any, input files.<br><br>

`json-peek` has a `#![feature]` in its source that doesnt exist anymore, you need to remove it from the lib.rs before it will work.<br>
The compiler will tell you which file that is when it tries to compile json-peek.
