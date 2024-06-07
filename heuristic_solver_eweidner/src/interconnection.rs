



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

/**
 * Tiebreaker for Statistical heuristics
 * (For example, when calculating the mean heuristic, several vertices usually have the same value. 
 * This enum determines the criteria according to which vertices with the same heuristic position are arranged)
 */
pub enum TieBreaker{
    Native, //Arbitrary ordering
    Median // Ordering according to Median heuristic
}


/**
 * finds default position of vertex in interconnection matrix
 */
fn default_edgelabel_to_interconnection_coordinates(fixed:i32,loose:i32,fixedSize:i32)->(usize,usize){
    let fixedPos = fixed - 1;
    let loosePos   = (loose - fixedSize) - 1;
    return (fixedPos as usize,loosePos as usize);
}


/**
Datastructure for an Interconnection matrix which uses the COOrdinate format for sparse matrices to save memory.
*/
pub(crate) struct COOInterconnectionMatrix{
    pub fixed: Vec<i32>,
    pub loose:Vec<(i32,usize)>, //(Name,index in adjcency_list)
    
    //Matrix in COO Format saved as edgelist for the loose vertices sorted by coordinate 
    pub adjacency_list: Vec<Vec<usize>>,
}

impl COOInterconnectionMatrix{

    /**
     * Removes all loose vertices from the datastructure with degree=0
     * (Orphaned vertices do not need to be involved in the computation and can be just added to the result later without generating any crossings)
     */
    pub fn extract_orphan_nodes(&mut self) -> Vec<(i32,usize)>{
        let mut newNodes : Vec<(i32,usize)> = Vec::new();
        let mut extractedOrphanNodes : Vec<(i32,usize)> = Vec::new();
        for node in &self.loose  {
            if self.adjacency_list[node.1].len() > 0 {
                newNodes.push(*node);
            }
            else {
                extractedOrphanNodes.push(node.clone());
            }
        }
        self.loose = newNodes;
        return  extractedOrphanNodes;
    }

   
    /**
     * Calculated the number of crossings created by two vertices u and v when arranged uv and vu (uv,vu) in one single pass.
     */
    pub fn calc_local_cross_count_touple_between_edgelists(&self,adj_ref_u:usize,adj_ref_v:usize)-> (u32,u32){
               
        //get reference to interconnection arrray
        let V_u = &self.adjacency_list[adj_ref_u];
        let V_v = &self.adjacency_list[adj_ref_v];

        if V_v.len() == 0 || V_u.len() == 0{
            return (0,0);
        }

         //init pointer in interconnection array
         let mut p_v : usize = 0;
         let mut p_u : usize = 0;

        //init tally for total number of crossings
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
                eprintln!("something went extremly wrong");
            }
        }

    }


    /**
     * Calculate the crossings between the two specified vertices.
     * Returns a tuple where the first value is the number of crossings in the given position and the second is the number of crossings if switched.
     */
    pub fn calc_local_cross_count_touple_between_positions(&self,pos_u:usize,pos_v:usize) -> (u32,u32){
        return self.calc_local_cross_count_touple_between_edgelists(self.loose[pos_u].1,self.loose[pos_v].1);
    }


    /**
     * Calculates median heuristic.
     */
    pub fn calculate_median(&self, loose_vertex:&(i32,usize)) -> usize{
        let arraylen = self.adjacency_list[loose_vertex.1].len();
        let median = (arraylen-1) / 2;
        return self.adjacency_list[loose_vertex.1][median]
    }

     /**
     * Calculates mean heuristic.
     */
    pub fn calculate_mean(&self, loose_vertex:&(i32,usize)) -> usize{
        let neighbours = &self.adjacency_list[loose_vertex.1];
        let mean = neighbours.into_iter().sum::<usize>()  / neighbours.len() ;
        return mean
    }


    /**
     * Calculates all crossings generated by a given sublist.
     */
    pub fn calculate_current_crossing_count_on_sublist(&self,sublist:&Vec<(i32,usize)>)-> u32{
        let mut result = 0;
        for i in 0..sublist.len()-1{
            for j in (i+1) .. sublist.len(){
                result += self.calc_local_cross_count_touple_between_edgelists(sublist[i].1, sublist[j].1).0;
            }
        }
        return result;
    }

 

  
    /**
     * Calculates median heuristic on a given sublist.
     */
    pub fn median_heuristic_from_sublist(&self,sublist: &Vec<(i32,usize)>) -> Vec<Vec<(i32,usize)>>{
        let resultsize = self.fixed.len();
        let mut positionVector: Vec<Vec<(i32,usize)>> = vec![Vec::new(); resultsize + 1 as usize];
        for loose_vertex in sublist{
            let arraylen = self.adjacency_list[loose_vertex.1].len();
            if arraylen > 0 {
                positionVector[self.calculate_median(loose_vertex)].push(*loose_vertex)
            }
            else {
                positionVector[resultsize].push(*loose_vertex)
            }
        }
        return positionVector;
    }

    /**
     * Calculates mean heuristic. 
     */
    pub fn mean_heuristic(&self)-> Vec<Vec<(i32,usize)>>{
        return self.mean_heuristic_from_sublist(&self.loose); 
    }

    /**
     * Calculates mean heuristic on a given sublist.
     */
    pub fn mean_heuristic_from_sublist(&self,sublist: &Vec<(i32,usize)>) -> Vec<Vec<(i32,usize)>>{
        let resultsize = self.fixed.len();
        let mut positionVector: Vec<Vec<(i32,usize)>> = vec![Vec::new(); resultsize + 1 as usize];
        for loose_vertex in sublist{
            let arraylen = self.adjacency_list[loose_vertex.1].len();
            if arraylen > 0 {
                positionVector[self.calculate_mean(loose_vertex)].push(*loose_vertex);
            }
            else {
                positionVector[resultsize].push(*loose_vertex)
            }
        }
        return positionVector;
    }


    /**
     * Collapses the results of the median/mean heuristic into a proper ordering according to a given tie-breaker strategy.
     */
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
                        let temp = self.median_heuristic_from_sublist(&posset);
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


}

impl InterconnectionMatrix for COOInterconnectionMatrix{
    fn parse(graph:&GraphInput) -> Self where Self: Sized {
        let fixedset = graph.fixed_vertices.clone();

        let mut loose_temp: Vec<(i32,usize)> = Vec::new();
        let mut adj_list = Vec::new();

        let mut index:usize = 0;
        for loose_vertex in graph.loose_vertices.clone(){
            adj_list.push(Vec::new());
            loose_temp.push((loose_vertex,index));
            index = index +1;
        }

        //Edges are sorted by fixed and then loose vertices so we have to sort them correctly.
        //We convert them because at this stage those coordinates are the same as in the respective sets.
            
        for i in 0..graph.edge_set.len() {
            let edge = graph.edge_set[i].clone();
            
            let (fixed_index,loose_index) = default_edgelabel_to_interconnection_coordinates(edge.0,edge.1,graph.number_of_fixed);
            adj_list[loose_index].push(fixed_index);
            
        }
        
        let result = COOInterconnectionMatrix{ 
            fixed:  fixedset, loose: loose_temp, adjacency_list: adj_list };
       
       return result;

    }

    /** Get current fixed vertice order. */
    fn get_fixed_order(&self) -> &Vec<i32> {
        return &self.fixed;
    }

    /** Get current loose vertice order. */
    fn get_loose_order(&self) -> Vec<i32> {

        let mut res: Vec<i32> = Vec::new();
        for entry in &self.loose {
            res.push(entry.0);
        }

        return res;

        
    }

    /**
     * switch two loose vertices by name.
     */
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

    /**
     * Switch two vertices indexed on position a and b.
     */
    fn switch_loose_by_position(&mut self,a:usize,b:usize) {
        let temp = self.loose[a];
        self.loose[a] = self.loose[b];
        self.loose[b] = temp;
    }



    fn to_string(&self) -> String {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}


