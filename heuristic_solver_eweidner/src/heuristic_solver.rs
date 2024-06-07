

use crate::{pengraph::{PenaltyGraph, VertexArcBaseHeuristic}, interconnection::COOInterconnectionMatrix, INSTANCE_SIZE_CUTOFF, PEAK_ALLOC, should_terminate};


/**
 * Local search strats
 */
pub enum LocalSearchStrat {
    Interleaved5_2,
    Degree,
    GreedySwitch,
    GreedySwitchKstep,
}



pub(crate) fn solve(interconnection_matrix:&mut COOInterconnectionMatrix)-> Vec<i32>{

    //Extracting orphans (vertices with degree = 0)
    let mut orphans = interconnection_matrix.extract_orphan_nodes();

    //Calculate fallback result
    let temp_list = interconnection_matrix.mean_heuristic();
    let emergency_result = interconnection_matrix.collapse_loose_edge_list(temp_list, crate::interconnection::TieBreaker::Median);


    let mut intermediate_result;
    {

        //If instance is so big that it might break the memory limit default to emergency result
        if(interconnection_matrix.loose.len() > INSTANCE_SIZE_CUTOFF){
            intermediate_result = vec![emergency_result]; 
        }else{
            //Solve penalty graph base-heuristic. If it gets interrupted (SIGTERM or memory), default to emergency result)
            let option = solve_base_heuristic_on_penalty_graph( &interconnection_matrix);
            match option {
                Some(_) => intermediate_result = option.unwrap(),
                None => intermediate_result = vec![emergency_result],
            }
        }
    }

    //Perform initial local search 
    local_search(&mut intermediate_result, interconnection_matrix, LocalSearchStrat::GreedySwitch,0);
    let mut continue_search = true;

    //perform a set of increasing i-step-greedy searches followed by a greedyswitch search on the intermediate result.
    //Stops only if for all parameters no benefitial changes where perfomed.
    while continue_search {
        continue_search = false;
        for i in 2..30{
            if should_terminate() {
                break;
            }
            continue_search = continue_search || local_search(&mut intermediate_result, interconnection_matrix, LocalSearchStrat::GreedySwitchKstep,i);
        }
        if should_terminate() {
            break;
        }
        continue_search = continue_search || local_search(&mut intermediate_result, interconnection_matrix, LocalSearchStrat::GreedySwitch,0);
    }
    
    //Generates result
    let mut result :Vec<i32> = Vec::new();
    //reattach orphans
    result.append(&mut orphans.into_iter().map(|x| x.0).collect());
    for sublist in &intermediate_result{
        result.append(&mut sublist.clone().into_iter().map(|x| x.0).collect())

    }

    return result;
}


/**
 * Perfoms a local search on each block of the intermediate result. Returns true if vertices have been switched.
 */
pub(crate) fn local_search(intermediate_result:&mut  Vec<Vec<(i32,usize)>>,interconnection_matrix:&mut COOInterconnectionMatrix,strategy:LocalSearchStrat,param : usize)->bool{
    let mut did_change = true;
    let mut counter = 0;
    while did_change {
        did_change = false;
        counter +=1;
        if should_terminate() {
            break;
        }

        for i in 0..intermediate_result.len(){
            if should_terminate() {
                break;
            }

            if(intermediate_result[i].len() > param){
                let  sublist = &mut intermediate_result[i];

                //Decide which local search strategy to use
                match strategy {
                    LocalSearchStrat::Interleaved5_2 => todo!(), //return interleave_local_search(sublist,interconnection_matrix,5,2),
                    LocalSearchStrat::Degree => todo!(),//did_change = high_degree_only_greedy_switch(sublist,interconnection_matrix),
                    LocalSearchStrat::GreedySwitch => did_change =  greedy_switch(sublist,interconnection_matrix),
                    LocalSearchStrat::GreedySwitchKstep => did_change = greedy_switch_k_step(sublist,interconnection_matrix,param),
                }
            }
        }      
    }
    return counter > 1;
    
    
}



/**
 * Performs one pass of the greedy switch heuristic over the sublist
 */
pub(crate) fn greedy_switch(sublist:&mut Vec<(i32,usize)>,interconnection_matrix:&COOInterconnectionMatrix) -> bool{
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


/**
 * Perfoms one pass of k-step greedy switch local search
 * This searches for benefitial exchanges between vertices which are k steps appart.
 */
pub(crate) fn greedy_switch_k_step(sublist:&mut Vec<(i32,usize)>,interconnection_matrix:&COOInterconnectionMatrix,k:usize) -> bool{
    let mut switched = false;
    for i in 0.. (sublist.len() -k){
        let j = i+k;
        if should_terminate() {
            break;
        }
        let switch = should_switch_between_indices(interconnection_matrix,sublist, i,j);
        if switch {
            let temp = sublist[i];
            sublist[i] = sublist[j];
            sublist[j] = temp;
            switched = true;
        } 
           
    }

    return switched;
}

/**
 * Checks if it is beneficial to switch between index i and j in a sublist.

 */
fn should_switch_between_indices(interconnection_matrix:&COOInterconnectionMatrix,sublist:&mut Vec<(i32,usize)>,i:usize,j:usize) -> bool{
    let (mut stay,mut switch) = interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1);
    for k in  i+1..j{
        stay += interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[k].1).0;
        stay += interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[k].1, sublist[j].1).0;
        switch += interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[j].1, sublist[k].1).0;
        switch += interconnection_matrix.calc_local_cross_count_touple_between_edgelists(sublist[k].1, sublist[i].1).0;
    }
    if stay > switch {
        return true;
    }
    else {
        return false;
    }

}


/**
 * Performs the following Steps:
 * 1. Generate a penalty graph from the given interconnection Matrix
 * 2. Condenses the graph to make it acyclic.
 * 3. Sorts the condensed graph topologically.
 * 4. Sorts the condensed vertices according to a heuristic.
 */
pub(crate) fn solve_base_heuristic_on_penalty_graph(interconnection_matrix:&COOInterconnectionMatrix) -> Option<Vec<Vec<(i32,usize)>>>{

    let penalty_graph_option = PenaltyGraph::parse(&interconnection_matrix);

    //Break if parsing has failed due to timing or current memory usage would not allow to proceed without going over the maximum allowed allocated Memory.
    if penalty_graph_option.is_none() || PEAK_ALLOC.current_usage_as_gb() > 4.0 {
        return None;
    }


    let condensed_option = penalty_graph_option.unwrap().CondenseGraph();
    
    if condensed_option.is_none() {
        return None;
    }

    let condensed = condensed_option.unwrap();

    if should_terminate() {
        return None;
    }

    //Sort topologically
    let topological_sort_option = condensed.sort_with_kahns_algorithm();
    if topological_sort_option.is_none(){
        return None;
    }
    let topological_sort = topological_sort_option.unwrap();

    if should_terminate() {
        return None;
    }
    let mut intermediate_result : Vec<Vec<(i32,usize)>> = Vec::new();


    let mut speedup = false;

    for vertex_reference in topological_sort {
        let vert = &condensed.vertices[vertex_reference];
        
        //if termination is imminent speedup the process by just calculating one of the heuristics.
        if should_terminate() {
            speedup = true;
        }

        if vert.isCondensed {
            
            //Apply first heuristic.
            let subproblem =vert.CondensedVertices.as_ref().unwrap().sort_penalty_graph_with_heuristic(VertexArcBaseHeuristic::Baharev);
            
            let mut p_heuristic = Vec::new();
            for vertex in subproblem{
                p_heuristic.push(interconnection_matrix.loose[vertex.InterconRef]);
            }
            
            if speedup {
                intermediate_result.push(p_heuristic)
            }
            else {
                //Apply second heuristic.
                let subproblem =vert.CondensedVertices.as_ref().unwrap();
                let mut subproblem_references: Vec<(i32,usize)> = Vec::new();
                for vertice in &subproblem.vertices{
                    subproblem_references.push(interconnection_matrix.loose[vertice.InterconRef]);
                }
                
                //Decide which heuristic to use.
                let mean_temp=interconnection_matrix.mean_heuristic_from_sublist(&subproblem_references);
                let m_heuristic = interconnection_matrix.collapse_loose_edge_list(mean_temp, crate::interconnection::TieBreaker::Median);
                let m_heur_count = interconnection_matrix.calculate_current_crossing_count_on_sublist(&m_heuristic); 
                let p_heur_count = interconnection_matrix.calculate_current_crossing_count_on_sublist(&p_heuristic);
     
    
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