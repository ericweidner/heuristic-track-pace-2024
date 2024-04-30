
mod problem;
mod utils;
mod interconnection;
use std::{
    fs::File,
    io::{prelude::*, BufReader, self,Write, LineWriter},
    path::Path, env, 
};

use crate::interconnection::{SmartCOOInterconnectionMatrix, InterconnectionMatrix};

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    // let mut inputpath = "89.gr"; //&args[0];
    // let mut outputpath = "test3.out";//&args[1];

    let mut inputpath = &args[1];
    let mut outputpath = &args[2];

    println!("input is {}",inputpath);
    println!("output is {}",outputpath);


    let input = problem::GraphInput::parse(&inputpath, false, true);
    let interconnection : interconnection::SmartCOOInterconnectionMatrix = SmartCOOInterconnectionMatrix::parse(&input);

   
    let medi = interconnection.MedianHeuristic();
    let resultsmart = interconnection.collapse(&medi);

    let file = File::create(outputpath)?;
    let mut filewriter = LineWriter::new(file);
    

    let mut string = String::new();


    for res in resultsmart{
        string.push_str(&format!("{}\n",res));
    }

    filewriter.write_all(string.as_bytes());



    Ok(())
}
