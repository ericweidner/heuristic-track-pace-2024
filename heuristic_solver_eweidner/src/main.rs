

use std::{
    fs::File,
    io::{prelude::*, BufReader, self,Write, LineWriter},
    path::Path, env, thread, cmp::Ordering, sync::{self, atomic::AtomicBool, Mutex}, 
     
};
use std::{collections::HashMap, result, f32::INFINITY};
use interconnection::{COOInterconnectionMatrix, InterconnectionMatrix};
use pengraph::PenaltyGraph;
use problem::GraphInput;
use signal_hook::{consts::{SIGINT, SIGTERM}, iterator::Signals};
mod pengraph;
mod problem;
mod interconnection;
mod utils;
mod heuristic_solver;


use peak_alloc::PeakAlloc;


/**
Tracks memory usage.
 */
#[global_allocator]
pub static PEAK_ALLOC: PeakAlloc = PeakAlloc;

/**
Termination Signal
 */
pub static TERMINATION_SIGNAL: AtomicBool = AtomicBool::new(false);

/**
 Cutoff value for big problem instances.
 */
pub static INSTANCE_SIZE_CUTOFF: usize = 50000;

/**
Tells the solver when to Terminate.
*/
fn should_terminate()->bool{
    return TERMINATION_SIGNAL.load(std::sync::atomic::Ordering::Relaxed);
}


fn main() -> io::Result<()>{   
    //Load problem input from stdin.
    let stdin = std::io::stdin(); 
    let mut lines : Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }

    //Register termination signals.
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?; 
    thread::spawn(move || {
        for sig in signals.forever() {
            TERMINATION_SIGNAL.store(true, sync::atomic::Ordering::Relaxed);
            eprintln!("Received signal {:?}", sig);
        }
    });


    let mut interconnection;
    {
        //Parse input.
        let input = GraphInput::parse_from_lines(&lines, false, false);
        //Create interconnection matrix.
        interconnection =  COOInterconnectionMatrix::parse(&input);
    }
    
    //Call solver.
    let result = heuristic_solver::solve(&mut interconnection);

    //Print Solution to stdout.
    for res in result{
        println!("{}", res);
    }

    Ok(())
}











