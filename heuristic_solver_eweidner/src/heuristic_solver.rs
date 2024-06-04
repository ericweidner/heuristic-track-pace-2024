use std::sync::atomic::Ordering;

use crate::{pengraph::{PenaltyGraph, VertexArcBaseHeuristic}, interconnection::{SmartCOOInterconnectionMatrix, InterconnectionMatrix}, TERMINATION_SIGNAL, utils::Direction};


pub enum LocalSearchStrat {
    Interleaved5_2,
    Degree,
    greedy_switch
}

pub(crate) fn solve(interconnection_matrix:&mut SmartCOOInterconnectionMatrix,strategy:LocalSearchStrat)-> Vec<i32>{
    interconnection_matrix.PerformMedianRearrangement();
    let emergency_result = interconnection_matrix.get_loose_order();

    let mut orphans = interconnection_matrix.extractOrphanNodes();

    // let penalty_graph_option = PenaltyGraph::parse(&interconnection_matrix);
    // if(penalty_graph_option.is_none()){
    //     return emergency_result;
    // }

    let mut intermediate_result;
    {


        //let penalty_graph = penalty_graph_option.unwrap();
        let option = solve_base_heuristic_on_penalty_graph( &interconnection_matrix);
        match option {
            Some(_) => intermediate_result = option.unwrap(),
            None => return emergency_result,
        }
        
    }

    let mut did_change = true;
    while did_change {
        for i in 0..intermediate_result.len(){
            if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
                eprintln!("abort");
                break;
            }
            if(intermediate_result[i].len() > 1){
                let  k = &mut intermediate_result[i];
                did_change = local_search( k,interconnection_matrix,LocalSearchStrat::greedy_switch);
                while did_change {
                    if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
                        eprintln!("abort");
                        break;
                    }
                    did_change = local_search( k,interconnection_matrix,LocalSearchStrat::greedy_switch);
                }
            }
        }
       

        if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
            eprintln!("abort");
            break;
        }
    }

    let mut result :Vec<i32> = Vec::new();
    result.append(&mut orphans.into_iter().map(|x| x.0).collect());
    for sublist in &intermediate_result{
        result.append(&mut sublist.clone().into_iter().map(|x| x.0).collect())

    }

    return result;
}

pub(crate) fn local_search(sublist:&mut  Vec<(i32,usize)>,interconnection_matrix:&mut SmartCOOInterconnectionMatrix,strategy:LocalSearchStrat)-> bool{
    match strategy {
        LocalSearchStrat::Interleaved5_2 => return interleave_local_search(sublist,interconnection_matrix,5,2),

        LocalSearchStrat::Degree => todo!(),
        LocalSearchStrat::greedy_switch => return greedy_switch(sublist,interconnection_matrix),
    }
}

pub(crate) fn greedy_switch(sublist:&mut Vec<(i32,usize)>,interconnection_matrix:&SmartCOOInterconnectionMatrix) -> bool{
    let mut switched = false;
    for i in 0.. (sublist.len() -1){
        if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
            break;
        }
        let mut j = i+1;
        let cr = interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1);
        if(cr.1 < cr.0){
            let temp = sublist[i];
            sublist[i] = sublist[j];
            sublist[j] = temp;
            switched = true;
        } 
           
    }

    return switched;
}

// pub(crate) fn greedy_local_step_search(interconnection_matrix:&SmartCOOInterconnectionMatrix,sublist:&mut Vec<(i32,usize)>,index:usize,max_step:u32){
    
//     let (stay,switchright) = interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[index].1, sublist[index+1].1);
//     let (stay,switchleft) = interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[index-1].1, sublist[index].1);
//     let mut direction = Direction::Nowhere;

//     if(sublist.len())

//     for i in 2..max_step{
//         let switch_right = should_switch_between_indices(interconnection_matrix, sublist, index, index+i as usize);
//         let switch_left = should_switch_between_indices(interconnection_matrix, sublist, index-i as usize, index as usize);
        
//         if(switch_right){
//             direction = Direction::Right;

//             break
//         }
//         else if switch_left {
//             direction = Direction::Left;
//             let temp = sublist[index-i];
//             sublist[index-i as usize] = sublist[index];
//             sublist[index] = temp;
//         }
//     }

//     if(direction == Direction::Nowhere){
//         return;
//     }
//     else {
//         for i in 2..max_step{
//             if(direction == Direction::Left){
//                 if(should_switch_between_indices(interconnection_matrix, sublist, index, index+i as usize){

//                 }
//             }
//             else {
//                 if(should_switch_between_indices(interconnection_matrix, sublist, index-i, index as usize)){

//                 };
                
//             }
            
           
//         }
//     }

    
    
// }

fn should_switch_between_indices(interconnection_matrix:&SmartCOOInterconnectionMatrix,sublist:&mut Vec<(i32,usize)>,i:usize,j:usize) -> bool{
    let (mut stay,mut switch) = interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1);
    for k in  i+1..j{
        stay += interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[k].1).0;
        switch += interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[j].1, sublist[k].1).0;
    }
    if(stay >= switch){
        return true;
    }
    else {
        return false;
    }

}

pub(crate) fn interleave_local_search(sublist:&mut Vec<(i32,usize)>,interconnection_matrix:&SmartCOOInterconnectionMatrix,width:usize,overlap:usize) -> bool{
    if sublist.len() < width {
        return interleave_local_search(sublist, interconnection_matrix, sublist.len(), overlap);
    }

    let mut i = 0;
    let step = width - overlap;
    let mut did_cange_overall = false;
    
    while i <= sublist.len() - width {
        
        if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
            eprintln!("abort");
            break;
        }

        let (did_change,count,perm) = interconnection_matrix.PermutateOnSublist(&sublist[i..i+width]);
        if(did_change){
            for j in 0..perm.len()  {
                sublist[j+i] = perm[j];
            }
            did_cange_overall = true;
        }

        i += step;
    }

    return did_cange_overall
}






pub(crate) fn solve_base_heuristic_on_penalty_graph(interconnection_matrix:&SmartCOOInterconnectionMatrix) -> Option<Vec<Vec<(i32,usize)>>>{
    
    let penalty_graph_option = PenaltyGraph::parse(&interconnection_matrix);
    if(penalty_graph_option.is_none()){
        return None;
    }
    let condensed = penalty_graph_option.unwrap().CondenseGraph();

    //Condense pentalty graph
    if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
        return None;
    }

    //Sort topologically
    let topological_sort = condensed.SortWithKahnsAlgorithm();

    if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
        return None;
    }
    let mut intermediate_result : Vec<Vec<(i32,usize)>> = Vec::new();

    let mut speedup = false;
    for vertex_reference in topological_sort {
        let vert = &condensed.vertices[vertex_reference];
        if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
            speedup = true;
        }
        if(vert.isCondensed){
            
            //Apply heuristic
            let mut subproblem =vert.CondensedVertices.as_ref().unwrap().sort_eads_heuristic(VertexArcBaseHeuristic::Baharev);
            
            let mut p_heuristic = Vec::new();
            for vertex in subproblem{
                p_heuristic.push(interconnection_matrix.loose[vertex.InterconRef]);
            }
            
            if(speedup){
                intermediate_result.push(p_heuristic)
            }
            else {
                // apply other heuristic
                let subproblem =vert.CondensedVertices.as_ref().unwrap();
                let mut subproblem_references: Vec<(i32,usize)> = Vec::new();
                for vertice in &subproblem.vertices{
                    subproblem_references.push(interconnection_matrix.loose[vertice.InterconRef]);
                }
                let ( m_count,  m_heuristic) = interconnection_matrix.MedianMeanOnSublist(&subproblem_references,false,false);
                  
                let pheur = interconnection_matrix.CalculateCurrentCrossingCountWithSlowFertigOnSublist(&p_heuristic);
     
    
                if(pheur < m_count){
                    intermediate_result.push(p_heuristic);
                }else {
                    intermediate_result.push(m_heuristic);
                }
                
            }
            
        }
        else{
            let temp = vec![interconnection_matrix.loose[vert.InterconRef]];
            intermediate_result.push(temp);
        }
    }

    return Some(intermediate_result);
}