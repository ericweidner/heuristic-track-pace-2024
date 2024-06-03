use std::sync::atomic::Ordering;

use crate::{pengraph::{PenaltyGraph, VertexArcBaseHeuristic}, interconnection::{SmartCOOInterconnectionMatrix, InterconnectionMatrix}, TERMINATION_SIGNAL};


pub enum LocalSearchStrat {
    Interleaved5_2,
    Degree,
    greedy_switch
}

pub(crate) fn Solve(interconnection_matrix:&mut SmartCOOInterconnectionMatrix,strategy:LocalSearchStrat)-> Vec<i32>{
    interconnection_matrix.PerformMeanRearrangement();
    let emergency_result = interconnection_matrix.get_loose_order();

    let mut orphans = interconnection_matrix.extractOrphanNodes();

    let penalty_graph_option = PenaltyGraph::parse(&interconnection_matrix);

    if(penalty_graph_option.is_none()){
        return emergency_result;
    }

    let penalty_graph = PenaltyGraph::parse(&interconnection_matrix).unwrap();

    let mut intermediate_result = SolveBaseHeuristicOnPenaltyGraph(&penalty_graph, &interconnection_matrix);
    
    let mut did_change = true;
    while did_change {
        for i in 0..intermediate_result.len(){
            if(intermediate_result[i].len() > 1){
                let  k = &mut intermediate_result[i];
                did_change = local_search( k,interconnection_matrix,LocalSearchStrat::greedy_switch);
                while did_change {
                    did_change = local_search( k,interconnection_matrix,LocalSearchStrat::greedy_switch);
                    if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
                        eprintln!("abort");
                        break;
                    }
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






pub(crate) fn SolveBaseHeuristicOnPenaltyGraph(graph:&PenaltyGraph,interconnection_matrix:&SmartCOOInterconnectionMatrix) -> Vec<Vec<(i32,usize)>>{
    //Condense pentalty graph
    let condensed = graph.CondenseGraph();
    //Sort topologically
    let topological_sort = condensed.SortWithKahnsAlgorithm();

    let mut intermediate_result : Vec<Vec<(i32,usize)>> = Vec::new();

    for vertex_reference in topological_sort {
        let vert = &condensed.vertices[vertex_reference];

        if(vert.isCondensed){
            
            //Apply heuristic
            let mut subproblem =vert.CondensedVertices.as_ref().unwrap().sort_eads_heuristic(VertexArcBaseHeuristic::Baharev);
            
            let mut p_heuristic = Vec::new();
            for vertex in subproblem{
                p_heuristic.push(interconnection_matrix.loose[vertex.InterconRef]);
            }
            
            // apply other heuristic
            let subproblem =vert.CondensedVertices.as_ref().unwrap();
            let mut subproblem_references: Vec<(i32,usize)> = Vec::new();
            for vertice in &subproblem.vertices{
                subproblem_references.push(interconnection_matrix.loose[vertice.InterconRef]);
            }
            let mut m_heuristic = interconnection_matrix.MedianMeanOnSublist(&subproblem_references,false,false);
              
            let pheur = interconnection_matrix.CalculateCurrentCrossingCountWithSlowFertigOnSublist(&p_heuristic);
            let mheur = interconnection_matrix.CalculateCurrentCrossingCountWithSlowFertigOnSublist(&m_heuristic);

            if(pheur < mheur){
                intermediate_result.push(p_heuristic);
            }else {
                intermediate_result.push(m_heuristic);
            }
        }
        else{
            let mut temp = vec![interconnection_matrix.loose[vert.InterconRef]];
            intermediate_result.push(temp);
        }
    }

    return intermediate_result;
}