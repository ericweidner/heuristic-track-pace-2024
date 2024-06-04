

use std::{
    fs::File,
    io::{prelude::*, BufReader, self,Write, LineWriter},
    path::Path, env, thread, cmp::Ordering, sync::{self, atomic::AtomicBool}, 
     
};
use std::{collections::HashMap, result, f32::INFINITY};

use interconnection::{SmartCOOInterconnectionMatrix, InterconnectionMatrix};
use pengraph::PenaltyGraph;
use problem::GraphInput;
use signal_hook::{consts::{SIGINT, SIGTERM}, iterator::Signals};
mod pengraph;
mod problem;
mod interconnection;
mod utils;
mod heuristic_solver;

//use rand::Rng;


pub static TERMINATION_SIGNAL: AtomicBool = AtomicBool::new(false);



fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();



    //Load
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

    let mut interconnection;
    {
        let input = GraphInput::parse_from_lines(&lines, false, false);
        interconnection =  SmartCOOInterconnectionMatrix::parse(&input);
    }

    
        
   
    
    let result = heuristic_solver::Solve(&mut interconnection, heuristic_solver::LocalSearchStrat::Interleaved5_2);



    

    // let file = File::create(outputpath)?;
    // let mut filewriter = LineWriter::new(file);
    

    let mut string = String::new();


    for res in result{
        println!("{}", res);
        //string.push_str(&format!("{}\n",res));
    }

    //filewriter.write_all(string.as_bytes());



    Ok(())
}









