



//use rand::Rng;

//use crate::{problem::GraphInput, main::Direction, solver::bf_cross_counter::CrossingCountFormularSolver};
use crate::{problem::GraphInput, should_terminate};



pub(crate) trait InterconnectionMatrix {
    fn parse(graph:&GraphInput) -> Self where Self: Sized;
    fn get_fixed_order(&self) -> &Vec<i32>;
    fn get_loose_order(&self) -> Vec<i32>;
    fn switch_loose_by_vertexlabel(&mut self,a:i32,b:i32);
    fn switch_loose_by_position(&mut self,a:usize,b:usize);
    // fn get_value_at(&self,fixedIndex:usize,looseIndex:usize)-> bool;
    // fn get_edge_set(&self) -> Vec<(i32,i32)>;
    fn to_string(&self) -> String;
    fn print(&self);
}

pub enum TieBreaker{
    Native,
    Median
}


fn default_edgelabel_to_interconnection_coordinates(fixed:i32,loose:i32,fixedSize:i32)->(usize,usize){
    let fixedPos = fixed - 1;
    let loosePos   = (loose - fixedSize) - 1;
    return (fixedPos as usize,loosePos as usize);
}
// pub(crate) struct SimpleInterconnectionMatrix{
//     pub fixed: Vec<i32>,
//     pub loose:Vec<i32>,
//     pub matrix : Vec<Vec<bool>>
// }

pub(crate) struct SmartCOOInterconnectionMatrix{
    pub fixed: Vec<i32>,
    pub loose:Vec<(i32,usize)>, //(edgeName,indexOfFexedSet)
    
    //Matrix in COO Format daved as edgelist for the loose Vertices sorted by coordinate 
    pub adjcency_list: Vec<Vec<usize>>,

    pub average_loose_vertex_degree : usize

}

impl SmartCOOInterconnectionMatrix{

    // pub fn f_extractOrphanNodes(&mut self) -> Vec<i32>{
    //     let temp = self.extractOrphanNodes();
    //     let mut result = Vec::new();
    //     for node in temp  {
    //         result.push(node.0);
    //     }
    //     return  result;
    // }

    pub fn extract_orphan_nodes(&mut self) -> Vec<(i32,usize)>{
        let mut newNodes : Vec<(i32,usize)> = Vec::new();
        let mut extractedOrphanNodes : Vec<(i32,usize)> = Vec::new();
        for node in &self.loose  {
            if self.adjcency_list[node.1].len() > 0 {
                newNodes.push(*node);
            }
            else {
                extractedOrphanNodes.push(node.clone());
            }
        }
        self.loose = newNodes;
        return  extractedOrphanNodes;
    }

    

    // pub fn CalculateMedianPosition(&self) -> Vec<Vec<(i32,usize)>>{
    //     let resultsize = self.fixed.len();
    //     let mut positionVector: Vec<Vec<(i32,usize)>> = vec![Vec::new(); resultsize + 1 as usize];
    //     for looseVertex in &self.loose{
    //         let arraylen = self.adjcency_list[looseVertex.1].len();
    //         if(arraylen > 0){
    //             positionVector[self.calculate_median(looseVertex)].push((looseVertex.0,looseVertex.1))
    //         }
    //         else {
    //             positionVector[resultsize].push((looseVertex.0,looseVertex.1))
    //         }
    //     }
    //     return positionVector;
    // }


    // pub fn switch_with_neighbour_if_beneficial(& mut self, pos:usize,direction : Direction)-> bool{
    //     let (mut pos_u, mut pos_v) = (0,1);
    //     if direction == Direction::Left {
    //         (pos_u,pos_v) = (pos - 1, pos);
    //     }else {
    //         (pos_u,pos_v) = (pos , pos + 1);
    //     }

    //     let (stay,switch) = self.calc_local_cross_count_touple_between_positions(pos_u , pos_v);

    //     if stay > switch{
    //         self.switch_loose_by_position(pos_u, pos_v);
    //         return true;
    //     }
    //     return false;
    // }

    // pub fn calc_local_crossing_count_brute_force(&self,adj_ref_u:usize,adj_ref_v:usize)-> i32{
    //     let V_u = &self.adjcency_list[adj_ref_u];
    //     let V_v = &self.adjcency_list[adj_ref_v];
    //     let mut count = 0;
    //     for i in V_u{
    //         for j in V_v{
    //             if(i > j){
    //                 count += 1;
    //             }
    //         }
    //     }
    //     return count;
    // }

    //Calculated the number of crossings created by two vertices u and v when arranged uv and vu (uv,vu)
    pub fn calc_local_cross_count_touple_between_edgelists(&self,adj_ref_u:usize,adj_ref_v:usize)-> (u32,u32){
               
        //get reference to interconnection arrray
        let V_u = &self.adjcency_list[adj_ref_u];
        let V_v = &self.adjcency_list[adj_ref_v];

        if V_v.len() == 0 || V_u.len() == 0{
            return (0,0);
        }

         //init Pointer in interconnection array
         let mut p_v : usize = 0;
         let mut p_u : usize = 0;

        //init Tally for total number of crossings
        let mut C_u : u32 = 0; //C_u = C_uv
        let mut C_v : u32 = 0; //C_v = C_vu

         if V_u[p_u] > V_v[p_v] {
             C_u = 1;
             while V_v.len() > p_v + 1 && V_u[p_u] > V_v[p_v + 1]{
                 C_u += 1;
                 p_v += 1;
             }
         } else if V_u[p_u] < V_v[p_v] {
             C_v = 1;
             while V_u.len() > p_u +1 && V_u[p_u + 1] < V_v[p_v]  {
                C_v += 1;
                p_u += 1;
            }
         }
        

        loop {
         
            if V_u.len() <= (p_u +1)  && V_v.len() <= (p_v + 1) {
                // if V_u[p_u] > V_v[p_v] {
                //     let u = 9;
                // }
                // if V_u[p_u] < V_v[p_v] {
                //     let u = 4;

                // }
                
                return (C_u,C_v);
            }
            else if V_u.len() <= (p_u + 1){
                //Move V
                p_v += 1;
                
                if V_u[p_u] != V_v[p_v] {
                    C_v += (p_u + 1) as u32;

                }
                
            }
            else if V_v.len() <= (p_v + 1){
                //Move U
                p_u += 1;

                if V_u[p_u] != V_v[p_v] {
                    C_u += (p_v + 1) as u32;
                }
                
            } else if V_u[p_u + 1] < V_v[p_v + 1] {
                //Move U
                p_u += 1;
                if V_u[p_u] != V_v[p_v] {
                    C_u += (p_v + 1) as u32;
                }
                
     
            } else if V_u[p_u + 1] > V_v[p_v + 1] {
                //Move V
                p_v += 1;

                if V_u[p_u] != V_v[p_v] {
                    C_v += (p_u + 1) as u32;

                }
                

            } else if V_u[p_u + 1] == V_v[p_v + 1]{
                //Move Both
                C_u += (p_v + 1) as u32;
                C_v += (p_u + 1) as u32;
                p_v += 1;
                p_u += 1;
            } else {
                println!("something went extremly wrong");
            }
        }

    }

    //Caluclate the crossings between the two specified vertices
    //returns tuple where first value is the crossing in the given position and the second is the crossing if switched
    pub fn calc_local_cross_count_touple_between_positions(&self,pos_u:usize,pos_v:usize) -> (u32,u32){
        return self.calc_local_cross_count_touple_between_edgelists(self.loose[pos_u].1,self.loose[pos_v].1);
       

    }





    


    // pub fn switch_beneficial(pos_a:usize,pos_b:usize) -> bool{
        
    // }

    pub fn calculate_median(&self, loose_vertex:&(i32,usize)) -> usize{
        let arraylen = self.adjcency_list[loose_vertex.1].len();
        let median = (arraylen-1) / 2;
        return self.adjcency_list[loose_vertex.1][median]
    }

    pub fn calculate_mean(&self, loose_vertex:&(i32,usize)) -> usize{
        let neighbours = &self.adjcency_list[loose_vertex.1];
        let mean = neighbours.into_iter().sum::<usize>()  / neighbours.len() ;
        return mean
    }

    pub fn calculate_mean_position(&self) -> Vec<Vec<(i32,usize)>>{
        let resultsize = self.fixed.len();
        let mut position_vector: Vec<Vec<(i32,usize)>> = vec![Vec::new(); resultsize + 1 as usize];
        for looseVertex in &self.loose{
            let arraylen = self.adjcency_list[looseVertex.1].len();
            if arraylen > 0 {
                position_vector[self.calculate_mean(looseVertex)].push((looseVertex.0,looseVertex.1));
            }
            else {
                position_vector[resultsize].push((looseVertex.0,looseVertex.1))
            }
        }
        return position_vector;
    }

    // pub fn perform_median_rearrangement(&mut self){
    //     let pos = self.CalculateMedianPosition();
    //     let newloose = self.collapse_loose_edge_list(pos,TieBreaker::Native);
    //     self.loose = newloose;
    // }

    // pub fn perform_mean_rearrangement(&mut self){
    //     let pos = self.CalculateMeanPosition();
    //     let newloose = self.collapse_loose_edge_list(pos,TieBreaker::Median);
    //     self.loose = newloose;

    // }

   

    // pub fn calculate_current_crossing_count_with_slow_fertig(&self)-> u32{
    //     let mut result = 0;
    //     for i in 0..self.get_loose_order().len()-1{
    //         for j in (i+1) .. self.get_loose_order().len(){
    //             let countz = self.calc_local_cross_count_touple_between_positions(i, j);
    //             let count = countz.0;
    //             result += count
    //         }
    //     }
    //     return result;
    // }


    // pub fn DrycleanPass(&mut self) -> bool{
    //     let mut has_switched = false;
    //     for i in 0..self.loose.len() -1 {
    //         has_switched = has_switched || self.switch_with_neighbour_if_beneficial(i,Direction::Right);
    //     }

    //     return has_switched;
    // }

    // pub fn dry_clean_count_on_sublist(&self,sublist: &mut Vec<(i32,usize)>) -> u32{
    //     let mut result = 0;
    //     for i in 0.. (sublist.len() -1){
    //         //let mut active = i;
    //         for j in (i+1) .. sublist.len(){
    //             let cr = self.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1);
    //             if j-i == 1 && cr.1 < cr.0 {

    //                     result += cr.1;
    //                     let temp = sublist[i];
    //                     sublist[i] = sublist[j];
    //                     sublist[j] = temp;

                    
    //             } else {
    //                 result += cr.0;
    //             }
               
    //         }

    //     }
    //     return result;
    // }

    pub fn permutate_on_sublist(&self,sublist: &[(i32,usize)]) -> (bool,u32,Vec<(i32,usize)>){
        let mut A: Vec<(i32, usize)> = sublist.to_vec();

        let mut baseValue = self.calculate_current_crossing_count_with_slow_fertig_on_sublist(&A);
        if(baseValue == 0){
            return (false,0,A);
        }
       
        let mut BasePerm = A.clone();
        let mut did_change = false;

        let mut result = 0;
        let len = sublist.len();
        let mut p = vec![0; len];

        let mut i = 1;
        for k in 0..(1..=len).product(){
            while i < len {
                if(p[i]< i){
                    let mut j = 0;
                    if(p[i] % 2 == 1){
                        j = p[i];
                    }
                    let temp = A[i];
                    A[i] = A[j];
                    A[j] = temp;
                    p[i] += 1;
                    i = 1;
                }
                else{
                    p[i] = 0;
                    i += 1;
                }
            }

            let crossings = self.calculate_current_crossing_count_with_slow_fertig_on_sublist_to_max(&A,baseValue);
            if(crossings < baseValue){
                BasePerm = A.clone();
                baseValue = crossings;
                did_change = true;
                if(crossings == 0){
                    break;
                }

            }
            if should_terminate() {
                eprintln!("abort");
                break;
            }
        }
  
     
        return (did_change,baseValue,BasePerm);
    }

//     The Counting QuickPerm Algorithm:

//    let a[] represent an arbitrary list of objects to permute
//    let N equal the length of a[]
//    create an integer array p[] of size N to control the iteration       
//    initialize p[0] to 0, p[1] to 0, p[2] to 0, ..., and p[N-1] to 0
//    initialize index variable i to 1
//    while (i < N) do {
//       if (p[i] < i) then {
//          if i is odd, then let j = p[i] otherwise let j = 0
//          swap(a[j], a[i])
//          increment p[i] by 1
//          let i = 1 (reset i to 1)
//       } // end if
//       else { // (p[i] equals i)
//          let p[i] = 0 (reset p[i] to 0)
//          increment i by 1
//       } // end else (p[i] equals i)
//    } // end while (i < N)

    pub fn calculate_current_crossing_count_with_slow_fertig_on_sublist(&self,sublist:&Vec<(i32,usize)>)-> u32{
        let mut result = 0;
        for i in 0..sublist.len()-1{
            for j in (i+1) .. sublist.len(){
                result += self.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1).0;
            }
        }
        return result;
    }

    pub fn calculate_current_crossing_count_with_slow_fertig_on_sublist_to_max(&self,sublist:&Vec<(i32,usize)>,max:u32)-> u32{
        let mut result = 0;
        for i in 0..sublist.len()-1{
            for j in (i+1) .. sublist.len(){
                result += self.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1).0;
                if(result >= max){
                    return result;
                }
            }
        }
        return result;
    }


    // pub fn DrycleanerCrossingCount(&mut self) -> u32{

    //     let mut result = 0;
    //     for i in 0.. (self.loose.len() -1){
    //         let mut active = i;
    //         for j in (i+1) .. self.loose.len(){
    //             let cr = self.calc_local_cross_count_touple_between_positions(i, j);
    //             if(j-i == 1 && cr.1 < cr.0){

    //                     result += cr.1;
    //                     let temp = self.loose[i];
    //                     self.loose[i] = self.loose[j];
    //                     self.loose[j] = temp;

                    
    //             } else {
    //                 result += cr.0;
    //             }
               
    //         }

    //     }
    //     return result;
    // }

    // pub fn MedianHeuristic(&self)-> Vec<Vec<(i32,usize)>>{
    //     return self.median_heuristic_from_sublist(&self.loose); 
    // }

   

    pub fn median_heuristic_from_sublist(&self,sublist: &Vec<(i32,usize)>) -> Vec<Vec<(i32,usize)>>{
        let resultsize = self.fixed.len();
        let mut positionVector: Vec<Vec<(i32,usize)>> = vec![Vec::new(); resultsize + 1 as usize];
        for loose_vertex in sublist{
            let arraylen = self.adjcency_list[loose_vertex.1].len();
            if arraylen > 0 {
                positionVector[self.calculate_median(loose_vertex)].push(*loose_vertex)
            }
            else {
                positionVector[resultsize].push(*loose_vertex)
            }
        }
        return positionVector;
    }

    pub fn mean_heuristic(&self)-> Vec<Vec<(i32,usize)>>{
        return self.mean_heuristic_from_sublist(&self.loose); 
    }


    pub fn mean_heuristic_from_sublist(&self,sublist: &Vec<(i32,usize)>) -> Vec<Vec<(i32,usize)>>{
        let resultsize = self.fixed.len();
        let mut positionVector: Vec<Vec<(i32,usize)>> = vec![Vec::new(); resultsize + 1 as usize];
        for loose_vertex in sublist{
            let arraylen = self.adjcency_list[loose_vertex.1].len();
            if arraylen > 0 {
                positionVector[self.calculate_mean(loose_vertex)].push(*loose_vertex);
            }
            else {
                positionVector[resultsize].push(*loose_vertex)
            }
        }
        return positionVector;
    }

    // pub fn MedianMeanHeuristic(&self,verbose : bool,useDrycleaner : bool)-> (u32,Vec<(i32,usize)>){
    //     return self.MedianMeanOnSublist(&self.loose,verbose,false);
    // }



    // pub fn MedianMeanOnSublist(&self,sublist: &Vec<(i32,usize)>,verbose : bool,useDrycleaner : bool)-> (u32,Vec<(i32,usize)>){
    //     let mean = self.collapse_loose_edge_list(self.MeanHeuristicFromSublist(sublist),TieBreaker::Median);
    //     let median = self.collapse_loose_edge_list(self.MedianHeuristicFromSublist(sublist),TieBreaker::Native);
   
    //     let mean_count = self.calculate_current_crossing_count_with_slow_fertig_on_sublist(&mean);
    //     let median_count = self.calculate_current_crossing_count_with_slow_fertig_on_sublist(&median);

    //     let mut result = Vec::new();
    //     let mut bestcount;

    //     if(median_count < mean_count){
    //         if(verbose){
    //             println!("Median < Mean with {} vs {}",median_count,mean_count);
    //         }
    //         result= median;
    //         bestcount = median_count;
    //     }
    //     else {
    //         if(verbose){
    //             println!("Median > Mean with {} vs {}",median_count,mean_count);
    //         }
    //         result= mean;
    //         bestcount = mean_count
    //     }

    //     if(useDrycleaner){
    //         bestcount = self.dry_clean_count_on_sublist(&mut result);
    //         if(verbose){
    //             println!("Count after Dryclean {}",bestcount);
    //         }
    //     }
    //     return (bestcount,result);
    // }

   

    // pub fn MedianMean(&mut self,verbose : bool,useDrycleaner : bool) -> (Vec<i32>,u32){
    //     self.perform_mean_rearrangement();
    //     let mut mean_count = 0;
    //     if useDrycleaner{
    //         mean_count = self.DrycleanerCrossingCount();
    //     }
    //     else {
    //         mean_count = self.calculate_current_crossing_count_with_slow_fertig();
    //     }
    //     let meanarrangement = self.get_loose_order();

    //     self.perform_median_rearrangement();
    //     let mut median_count = 0;
    //     if useDrycleaner{
    //         median_count = self.DrycleanerCrossingCount();
    //     }
    //     else {
    //         median_count = self.calculate_current_crossing_count_with_slow_fertig();
    //     }

    //     if(median_count < mean_count){
    //         if(verbose){
    //             println!("Median < Mean with {} vs {}",median_count,mean_count);
    //         }
    //         return (self.get_loose_order(),median_count);
    //     }
    //     else {
    //         if(verbose){
    //             println!("Median > Mean with {} vs {}",median_count,mean_count);
    //         }
    //         self.perform_mean_rearrangement();
    //         return (self.get_loose_order(),mean_count);
    //     }
        
    // }



    // pub fn median_random_greedy_switch_heuristic(&mut self, steps: i32) -> Vec<i32>{

    //     self.perform_median_rearrangement();
         
    //     let loose_size = self.loose.len();

    //     let mut rng = rand::thread_rng();

    //     for i in 0..steps {
    //         let pos = rng.gen_range(0..loose_size - 2);
    //         self.switch_with_neighbour_if_beneficial(pos, Direction::Right);
    //     }

    //     return self.get_loose_order();

        
    // }

    // pub fn mean_random_greedy_switch_heuristic(&mut self, steps: i32) -> Vec<i32>{

    //     self.perform_mean_rearrangement();
         
    //     let looseSize = self.loose.len();

    //     let mut rng = rand::thread_rng();

    //     for i in 0..steps {
    //         let pos = rng.gen_range(0..looseSize - 2);
    //         self.switch_with_neighbour_if_beneficial(pos, Direction::Right);
    //     }

    //     return self.get_loose_order();
    // }

    

   pub fn collapse_loose_edge_list(&self,pos_vec : Vec<Vec<(i32,usize)>>,tie_breaker:TieBreaker) -> Vec<(i32,usize)>{
        let mut result : Vec<(i32,usize)> = Vec::new();
        for posset in  pos_vec{
            if posset.len() == 0 {
                continue;
            }
            match tie_breaker {
                TieBreaker::Native => {
                        for pos in posset{
                            result.push(pos);
                        }
                },
                TieBreaker::Median => {
                    if posset.len() == 1 {
                        result.push(posset[0]);
                        
                    }
                    else {
                        //eprintln!("Calculating Tiebreak result...");
                        let temp = self.median_heuristic_from_sublist(&posset);
                       // eprintln!("Collapsing Tiebreak result...");
                        let res = self.collapse_loose_edge_list(temp,TieBreaker::Native);
        
                        for pos in res{
                            result.push(pos);
                        }
        
                    }
                },
            }
            
        }
        return result;
    }

    // pub fn collapse(&self,pos_vec : &Vec<Vec<(i32,usize)>>) -> Vec<i32>{
    //     let mut result : Vec<i32> = Vec::new();
    //     for posset in  pos_vec{
    //         if posset.len() > 0 {
    //             for pos in posset{
    //                 result.push(pos.0.clone());
    //             }
    //         }
    //     }
    //     return result;
    // }

    


    
    // pub fn compress(&mut self) -> HashMap<i32,Vec<i32>>{
    //     let mut hashmap : HashMap<Vec<usize>,i32> = HashMap::new();
    //     let mut result:HashMap<i32,Vec<i32>> = HashMap::new(); 
    //     let mut new_loose:Vec<(i32,usize)> = Vec::new(); //(edgeName,indexOfFexedSet)
    //     let mut new_adjcency_list: Vec<Vec<usize>> = Vec::new();
        
    //     let mut index = 0;
    
    //     for i in &self.loose{
    //         if hashmap.contains_key(&self.adjcency_list[i.1]) {
    //             result.get_mut(hashmap.get(&self.adjcency_list[i.1]).unwrap()).unwrap().push(i.0);
                

    //         }
    //         else {
    //             hashmap.insert(self.adjcency_list[i.1].clone(),i.0);
    //             new_loose.push((i.0,index));
    //             new_adjcency_list.push(self.adjcency_list[i.1].clone());
    //             result.insert(i.0,Vec::new());
                
    //         }
            
    //     }
    //     self.loose = new_loose;
    //     self.adjcency_list = new_adjcency_list;

    //     return result;

    // }

    // pub fn decompress(&mut self ,compressmap: &HashMap<i32,Vec<i32>>){
    //     let mut new_loose:Vec<(i32,usize)> = Vec::new(); //(edgeName,indexOfFexedSet)

    //     let mut index = 0;
    //     for i in &self.loose {

    //         new_loose.push((i.0,i.1));
    //         index = index +1;

    //         if compressmap.contains_key(&i.0){
    //             for insertVertex in compressmap.get(&i.0).unwrap(){
    //                 new_loose.push((*insertVertex,index));
    //                 self.adjcency_list.push(self.adjcency_list[i.1].clone());
    //                 index = index +1;
    //             }
    //         }
            
    //     }

    //     self.loose = new_loose;
        
    // }

    


}

impl InterconnectionMatrix for SmartCOOInterconnectionMatrix{
    fn parse(graph:&GraphInput) -> Self where Self: Sized {
        let fixedset = graph.fixed_vertices.clone();

        let avg_loose_degr = graph.number_of_loose / graph.number_of_edges;
        let mut loose_temp: Vec<(i32,usize)> = Vec::new();
        let mut adj_list = Vec::new();

        let mut index:usize = 0;
        for loose_vertex in graph.loose_vertices.clone(){
            adj_list.push(Vec::new());
            loose_temp.push((loose_vertex,index));
            index = index +1;
        }


        
        //these are sorted by fixed and then loose vertices so we have to sort them correctly.
        //we convert them because at this stage those coordinates are the same as in the respective sets
        //for edge in graph.edge_set.clone(){
            
        for i in 0..graph.edge_set.len() {
            let edge = graph.edge_set[i].clone();
            
            let (fixed_index,loose_index) = default_edgelabel_to_interconnection_coordinates(edge.0,edge.1,graph.number_of_fixed);
            adj_list[loose_index].push(fixed_index);
            
        }
        
        let result = SmartCOOInterconnectionMatrix{ 
            fixed:  fixedset, loose: loose_temp, adjcency_list: adj_list ,average_loose_vertex_degree:avg_loose_degr as usize};
       
       return result;

    }

    fn get_fixed_order(&self) -> &Vec<i32> {
        return &self.fixed;
    }

    fn get_loose_order(&self) -> Vec<i32> {

        let mut res: Vec<i32> = Vec::new();
        for entry in &self.loose {
            res.push(entry.0);
        }

        return res;

        
    }

    fn switch_loose_by_vertexlabel(&mut self,a:i32,b:i32) {
        let mut found_a : bool = false;
        let mut found_b : bool = false;
        let mut apos = 0;
        let mut bpos = 0;


        for i in 0..self.loose.len(){
            let label = self.loose[i].0;
            if label == a {
                found_a = true;
                apos = i
            }
            if label == b {
                found_b = true;
                bpos = i
            }

            if found_a && found_b {
                break;
            }
        }

        self.switch_loose_by_position(apos, bpos)
    }

    fn switch_loose_by_position(&mut self,a:usize,b:usize) {
        let temp = self.loose[a];
        self.loose[a] = self.loose[b];
        self.loose[b] = temp;
    }

    // fn get_value_at(&self,fixedIndex:usize,looseIndex:usize)-> bool {
    //     // let vecindex = self.loose[looseIndex].1;

    //     // //you can do a binary search instead of forloop
    //     // for point in &self.adjcency_list[vecindex]{
    //     //     if(point == fixedIndex){
    //     //         return true;
    //     //     }
    //     //     if point > &fixedIndex {
    //     //         break;
    //     //     }
    //     // }
    //     return false;
    // }

    // fn get_edge_set(&self) -> Vec<(i32,i32)> {
    //     let mut result : Vec<(i32,i32)> = Vec::new();

    //     // for looseVertex in &self.loose{
    //     //     for fixedVertexIndex in &self.adjcency_list[looseVertex.1]{
    //     //         result.push((*self.fixed[fixedVertexIndex],looseVertex.0));
    //     //     }
    //     // }
    //     // result.sort_by(|(x0,x1),(y0,y1)| x0.cmp(y0));
    //     return result
    // }

    fn to_string(&self) -> String {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}

// impl InterconnectionMatrix for SimpleInterconnectionMatrix{
//     fn parse(graph:&GraphInput) -> Self {
//         let looseTemp= graph.loose_vertices.clone();
//         let fixedTemp= graph.fixed_vertices.clone();

//         let mut array = vec![vec![false; looseTemp.len()]; fixedTemp.len()];

//         for edge in &graph.edge_set {
//             let coord = default_edgelabel_to_interconnection_coordinates(edge.0,edge.1,graph.number_of_fixed);
//             array[coord.0 ][coord.1 ]=true;
//         }

//         let result = SimpleInterconnectionMatrix{ 
//              fixed:  fixedTemp, loose: looseTemp, matrix: array };
        
//         return result;

//     }

//     fn get_fixed_order(&self) -> &Vec<i32> {
//         return &self.fixed;
//     }

//     fn get_loose_order(&self) -> Vec<i32> {
//         return self.loose.clone();
//     }


//     fn switch_loose_by_vertexlabel(&mut self,a:i32,b:i32) {
//         let mut aIndex:usize = 0;
//         let mut bIndex:usize = 0;

//         let mut aindexFound = false;
//         let mut bindexFound = false;


//         for i  in 0..self.loose.len() {
//             if (aindexFound  && bindexFound){
//                 self.switch_loose_by_position(aIndex, bIndex);
//                 break;
//             }
//             if aindexFound ==false && self.loose[i] == a {
//                 aIndex = i;
//                 aindexFound = true;
//             }
//             if bindexFound ==false && self.loose[i] == b {
//                 bIndex = i;
//                 bindexFound = true;
//             }
//         }
//     }

//     fn switch_loose_by_position(&mut self,a:usize,b:usize) {
//         //switch loose index
//         let temp = self.loose[a];
//         self.loose[a] = self.loose[b];
//         self.loose[b] = temp;

//         //Switch loose array
//         for i in 0..self.fixed.len(){
//             let temp = self.matrix[i][a];
//             self.matrix[i][a] = self.matrix[i][b];
//             self.matrix[i][b] = temp;
//         }
//     }
    
//     fn to_string(&self) -> String {
//         let mut stringresult = String::new();
//         stringresult.push_str(&format!("    {:?}\r", self.loose));  
//         for i  in 0..self.fixed.len() {
//             stringresult.push_str( &format!("{:-<1$}",self.fixed[i],5));
//             let z : Vec<u32> =  self.matrix[i].iter().map(|&e| e as u32).collect();
//             stringresult.push_str( &format!("{:?}",z));
//             stringresult.push_str("\r");
//         }
//         return  stringresult;
//     }
    
//     fn print(&self) {
//         print!("{}", self.to_string());
//     }
    
//     fn get_value_at(&self,fixedIndex:usize,looseIndex:usize)-> bool {
//         self.matrix[fixedIndex][looseIndex]
//     }

//     fn get_edge_set(&self) -> Vec<(i32,i32)> {
//         let mut result = Vec::new();
//         for i in 0..self.fixed.len(){
//             for j in 0..self.loose.len(){
//                 if (self.matrix[i][j] == true){
//                     result.push((self.fixed[i],self.loose[j]));
//                 }

//             }
            
//         }

//         return result;
//     }
// }