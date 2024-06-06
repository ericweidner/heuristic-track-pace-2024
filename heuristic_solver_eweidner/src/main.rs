

use std::{
    fs::File,
    io::{prelude::*, BufReader, self,Write, LineWriter},
    path::Path, env, thread, cmp::Ordering, sync::{self, atomic::AtomicBool, Mutex}, 
     
};
use std::{collections::HashMap, result, f32::INFINITY};
use coarsetime::Instant;
use interconnection::{SmartCOOInterconnectionMatrix, InterconnectionMatrix};
use pengraph::PenaltyGraph;
use problem::GraphInput;
use signal_hook::{consts::{SIGINT, SIGTERM}, iterator::Signals};
mod pengraph;
mod problem;
mod interconnection;
mod utils;
mod heuristic_solver;


use peak_alloc::PeakAlloc;

//Tracks memory usage
#[global_allocator]
pub static PEAK_ALLOC: PeakAlloc = PeakAlloc;

//Termination Signal
pub static TERMINATION_SIGNAL: AtomicBool = AtomicBool::new(false);

//Cutoff value for big problem instances
pub static INSTANCE_SIZE_CUTOFF: usize = 50000;

//Global timer for time-based interruption
pub static mut  START: Option<Instant> = None ;

fn should_terminate()->bool{
    //let res =  unsafe { START.unwrap().elapsed().as_secs() > 300};
    //return res;
    //return unsafe { START.unwrap().elapsed().as_secs() } > 300;
    return TERMINATION_SIGNAL.load(std::sync::atomic::Ordering::Relaxed);
}


fn main() -> io::Result<()>{
    //Load
    eprintln!("Starting");
     //Det local timer
     let start = Instant::now();

    //Set globel start time
    unsafe { START = Some(Instant::now()) } ;
   
    //Load problem input from stdin
    let stdin = std::io::stdin(); 
    let mut lines : Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }

    //Register Termination Signals
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?; 
    thread::spawn(move || {
        for sig in signals.forever() {
            TERMINATION_SIGNAL.store(true, sync::atomic::Ordering::Relaxed);
            eprintln!("Received signal {:?}", sig);
        }
    });

    // let mut interconnection;

    // {
    //     let input = GraphInput::parse_from_lines(&lines, false, false);
    //     interconnection =  SmartCOOInterconnectionMatrix::parse(&input);
    // }

    
        
   
    //Solve


    let mut interconnection;
    
   

    {
        //eprintln!("Parsing from input...");
        //Parse input
        let input = GraphInput::parse_from_lines(&lines, false, false);
        //eprintln!("Parsing Interconnectionmatrix....");
        //Create interconnection matrix
        interconnection =  SmartCOOInterconnectionMatrix::parse(&input);
    }
    
    //Call solver
    let result = heuristic_solver::solve(&mut interconnection);
    
   


   
   
    

    // let file = File::create(outputpath)?;
    // let mut filewriter = LineWriter::new(file);
    

    let mut string = String::new();

    //Print Solution
    for res in result{
        println!("{}", res);
        //string.push_str(&format!("{}\n",res));
    }
    //Print Stats
    let peak_mem = PEAK_ALLOC.peak_usage_as_gb();
    eprintln!("The max amount that was used {}", peak_mem);
    let seconds = start.elapsed().as_secs();
    eprintln!("The Duration was {} Seconds", seconds);
    Ok(())
}











