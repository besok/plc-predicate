 
# PLC Predicate
Predicate is a rust framework for building PLCs (Programmable Logic Controllers) 
using the Rust programming language. 
It is designed to be simple, efficient, and easy to use, allowing developers to create PLCs that can be used in a variety of applications.

### Why Rust?
Rust's strengths align very well with the demands of industrial control systems:

 - Memory Safety: Rust's memory safety guarantees eliminate many common bugs that can lead to system crashes or unpredictable behavior, which is crucial in safety-critical PLC applications.
 - Performance: Rust provides performance comparable to C/C++, making it suitable for real-time applications where deterministic execution and low latency are essential.
 - Concurrency: Rust's robust concurrency features make it easier to write safe and efficient code for handling parallel tasks, which are common in PLC applications.
 - Reliability: Rust's strong type system and ownership model contribute to building reliable and maintainable code.


## Compile Rust into PLC
 - ST (Structured Text)
 - LD (Ladder Diagram)
 - FBD (Function Block Diagram)

## Direct compile to PLC Binary
 - S7 (Siemens S7)
 - Rockwell

## Testing and Simulation
 - Unit tests (Enable unit testing of PLC logic before deployment)
 - Simulation of PLCs (Create a runtime that mimics PLC behavior for testing)
 - Model I/O signals and timing for realistic simulation
 - Integration with existing PLC systems