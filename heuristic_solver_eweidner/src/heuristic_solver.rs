use std::sync::atomic::Ordering;

use crate::{pengraph::{PenaltyGraph, VertexArcBaseHeuristic}, interconnection::SmartCOOInterconnectionMatrix, INSTANCE_SIZE_CUTOFF, PEAK_ALLOC, should_terminate};


pub enum LocalSearchStrat {
    Interleaved5_2,
    Degree,
    GreedySwitch
}



pub(crate) fn solve(interconnection_matrix:&mut SmartCOOInterconnectionMatrix)-> Vec<i32>{
    eprintln!("Extracting orphans...");
    let mut orphans = interconnection_matrix.extract_orphan_nodes();
    eprintln!("Calculating fallback result...");
    let temp_list = interconnection_matrix.mean_heuristic();
    eprintln!("Collapsing fallback result...");
    let emergency_result = interconnection_matrix.collapse_loose_edge_list(temp_list, crate::interconnection::TieBreaker::Median);


    // let penalty_graph_option = PenaltyGraph::parse(&interconnection_matrix);
    // if(penalty_graph_option.is_none()){
    //     return emergency_result;
    // }

    let mut intermediate_result;
    {

        if(interconnection_matrix.loose.len() > INSTANCE_SIZE_CUTOFF){
            eprintln!("Instance too large, executing Plan B...");
            intermediate_result = vec![emergency_result]; 
        }else{
            //let penalty_graph = penalty_graph_option.unwrap();
            let option = solve_base_heuristic_on_penalty_graph( &interconnection_matrix);
            match option {
                Some(_) => intermediate_result = option.unwrap(),
                None => intermediate_result = vec![emergency_result],
            }
        }

        
    }

    eprintln!("Doing greedy search...");

    let mut did_change = true;
    while did_change {
        
        if should_terminate() {
            eprintln!("abort");
            break;
        }
        for i in 0..intermediate_result.len(){
            if should_terminate() {
                eprintln!("abort");
                break;
            }
            if(intermediate_result[i].len() > 1){
                let  k = &mut intermediate_result[i];
                did_change = true;
                while did_change {
                    if should_terminate() {
                        eprintln!("abort");
                        break;
                    }
                    did_change = local_search( k,interconnection_matrix,LocalSearchStrat::GreedySwitch);
                }
            }
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
        LocalSearchStrat::Interleaved5_2 => todo!(), //return interleave_local_search(sublist,interconnection_matrix,5,2),
        LocalSearchStrat::Degree => high_degree_only_greedy_switch(sublist,interconnection_matrix),
        LocalSearchStrat::GreedySwitch => return greedy_switch(sublist,interconnection_matrix),
    }
}

pub(crate) fn greedy_switch(sublist:&mut Vec<(i32,usize)>,interconnection_matrix:&SmartCOOInterconnectionMatrix) -> bool{
    let mut switched = false;
    for i in 0.. (sublist.len() -1){
        if should_terminate() {
            break;
        }
        let j = i+1;
        let cr = interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1);
        if cr.1 < cr.0 {
            let temp = sublist[i];
            sublist[i] = sublist[j];
            sublist[j] = temp;
            switched = true;
        } 
           
    }

    return switched;
}

pub(crate) fn high_degree_only_greedy_switch(sublist:&mut Vec<(i32,usize)>,interconnection_matrix:&SmartCOOInterconnectionMatrix) -> bool{
    let mut switched = false;
    for i in 0.. (sublist.len() -1){
        if should_terminate() {
            break;
        }
        let j = i+1;
        let cr = interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1);
        if cr.1 < cr.0 {
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
    if stay >= switch {
        return true;
    }
    else {
        return false;
    }

}

// pub(crate) fn interleave_local_search(sublist:&mut Vec<(i32,usize)>,interconnection_matrix:&SmartCOOInterconnectionMatrix,width:usize,overlap:usize) -> bool{
//     if sublist.len() < width {
//         return interleave_local_search(sublist, interconnection_matrix, sublist.len(), overlap);
//     }

//     let mut i = 0;
//     let step = width - overlap;
//     let mut did_cange_overall = false;
    
//     while i <= sublist.len() - width {
        
//         if should_terminate() {
//             eprintln!("abort");
//             break;
//         }

//         let (did_change,_count,perm) = interconnection_matrix.permutate_on_sublist(&sublist[i..i+width]);
//         if did_change {
//             for j in 0..perm.len()  {
//                 sublist[j+i] = perm[j];
//             }
//             did_cange_overall = true;
//         }

//         i += step;
//     }

//     return did_cange_overall
// }






pub(crate) fn solve_base_heuristic_on_penalty_graph(interconnection_matrix:&SmartCOOInterconnectionMatrix) -> Option<Vec<Vec<(i32,usize)>>>{
    eprintln!("Generating Penalty graph...");
    
    let penalty_graph_option = PenaltyGraph::parse(&interconnection_matrix);
    if penalty_graph_option.is_none() || PEAK_ALLOC.current_usage_as_gb() > 4.0 {
        eprintln!("Stopping Penalty graph...");
        return None;
    }

    eprintln!("Condensiong Penalty graph...");

    let condensed_option = penalty_graph_option.unwrap().CondenseGraph();
    if condensed_option.is_none() {
        return None;
    }
    let condensed = condensed_option.unwrap();
    //Condense pentalty graph
    if should_terminate() {
        return None;
    }

    //Sort topologically
    eprintln!("Sorting Penalty graph...");



    let topological_sort_option = condensed.SortWithKahnsAlgorithm();
    if topological_sort_option.is_none(){
        return None;
    }
    let topological_sort = topological_sort_option.unwrap();

    if should_terminate() {
        return None;
    }
    let mut intermediate_result : Vec<Vec<(i32,usize)>> = Vec::new();

    let mut speedup = false;
    eprintln!("Calculating heuristics...");

    for vertex_reference in topological_sort {
        let vert = &condensed.vertices[vertex_reference];
        if should_terminate() {
            speedup = true;
        }
        if vert.isCondensed {
            
            //Apply heuristic
            let subproblem =vert.CondensedVertices.as_ref().unwrap().sort_eads_heuristic(VertexArcBaseHeuristic::Baharev);
            
            let mut p_heuristic = Vec::new();
            for vertex in subproblem{
                p_heuristic.push(interconnection_matrix.loose[vertex.InterconRef]);
            }
            
            if speedup {
                intermediate_result.push(p_heuristic)
            }
            else {
                // apply other heuristic
                let subproblem =vert.CondensedVertices.as_ref().unwrap();
                let mut subproblem_references: Vec<(i32,usize)> = Vec::new();
                for vertice in &subproblem.vertices{
                    subproblem_references.push(interconnection_matrix.loose[vertice.InterconRef]);
                }
                
                //let ( m_count,  m_heuristic) = interconnection_matrix.MedianMeanOnSublist(&subproblem_references,false,false);
                let mean_temp=interconnection_matrix.mean_heuristic_from_sublist(&subproblem_references);
                let m_heuristic = interconnection_matrix.collapse_loose_edge_list(mean_temp, crate::interconnection::TieBreaker::Median);
                let m_heur_count = interconnection_matrix.calculate_current_crossing_count_with_slow_fertig_on_sublist(&m_heuristic); 
                let p_heur_count = interconnection_matrix.calculate_current_crossing_count_with_slow_fertig_on_sublist(&p_heuristic);
     
    
                if p_heur_count < m_heur_count {
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