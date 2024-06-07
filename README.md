# Solver heuristic-track-pace-2024

## Authors:
* Dr. Carolin Rehs*
* Eric Weidner B.Sc.

\*Accademic supervision
## How it Works
The program solves a given One-Sided Crossing Minimization (OSCM) instance approximately with the help of heuristics. It is based on the construction of the penalty graph as used, for example, in an exact method by Sugiyama et al. (10.1109/TSMC.1981.4308636). Sugiyama et al. are using an instance of the Feedback Arc Set (FAS) Problem on the penalty graph to derive an exact solution for the OSCM-Problem.
But instead of computing an exact solution for the resulting strongly connected components, the solver applies various heuristic strategies (both for OSCM and FAS) to compute an initial solution. In the end, several local search methods are used to improve the given result.
## How To Build:
* 1 Install Cargo and Rust.  
(https://doc.rust-lang.org/cargo/getting-started/installation.html)
* 2 In the root directory (the directory with the Cargo.toml file), call:  
`cargo build --release`

    If you want to cross-compile the Code for use on Optil.io:
    * 2a. Make sure the Rust target `x86_64-unknown-linux-musl` is installed.  
    (https://rust-lang.github.io/rustup/cross-compilation.html)
    * 2b. Call `cargo build --profile submit --target=x86_64-unknown-linux-musl`.
    
If everything was correct the compiler will return:

> Finished release [optimized] target(s) in 1.23s

(Depending on the operating system, this output might differ.)

The executable is then located under `/target/release/`.  
(or `/target/x86_64-unknown-linux-musl/`)

## How To Run
The solver uses stdin and stout so to run the solver via terminal call: 

`$solver_path < $InputPath  > $OutputPath` 

Example:  

`./heuristic_solver_eweidner < ./10.gr  > ./10.sol` 

## Dependencies
The Code uses the following dependencies.
*  **peak_alloc** - For keeping track of allocated Memory to handle the Memory requirements of the PACE-2024 Challenge.  
(sources: https://docs.rs/peak_alloc/latest/src/peak_alloc/lib.rs.html)
* **signal-hook** - For handling SIGTERM and SIGINT signals.  
(sources: https://github.com/vorner/signal-hook)

Both dependencies are linked in the Cargo.toml file and automatically included by Cargo when building the Project.


