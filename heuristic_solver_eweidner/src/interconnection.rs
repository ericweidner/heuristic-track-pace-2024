


use std::{collections::HashMap, result, f32::INFINITY};

use rand::Rng;

use crate::{problem::GraphInput, utils::Direction};



pub(crate) trait InterconnectionMatrix {
    fn parse(graph:&GraphInput) -> Self where Self: Sized;
    fn get_fixed_order(&self) -> &Vec<i32>;
    fn get_loose_order(&self) -> Vec<i32>;
    fn switch_loose_by_vertexlabel(&mut self,a:i32,b:i32);
    fn switch_loose_by_position(&mut self,a:usize,b:usize);
    fn get_value_at(&self,fixedIndex:usize,looseIndex:usize)-> bool;
    fn get_edge_set(&self) -> Vec<(i32,i32)>;
    fn to_string(&self) -> String;
    fn print(&self);
}


fn default_edgelabel_to_interconnection_coordinates(fixed:i32,loose:i32,fixedSize:i32)->(usize,usize){
    let fixedPos = fixed - 1;
    let loosePos   = (loose - fixedSize) - 1;
    return (fixedPos as usize,loosePos as usize);
}
pub(crate) struct SimpleInterconnectionMatrix{
    pub fixed: Vec<i32>,
    pub loose:Vec<i32>,
    pub matrix : Vec<Vec<bool>>
}

pub(crate) struct SmartCOOInterconnectionMatrix{
    pub fixed: Vec<i32>,
    pub loose:Vec<(i32,usize)>, //(edgeName,indexOfFexedSet)
    
    //Matrix in COO Format daved as edgelist for the loose Vertices sorted by coordinate 
    pub adjcency_list: Vec<Vec<usize>>
}

impl SmartCOOInterconnectionMatrix{
    pub fn CalculateMedianPosition(&self) -> Vec<Vec<(i32,usize)>>{
        let resultsize = self.fixed.len();
        let mut positionVector: Vec<Vec<(i32,usize)>> = vec![Vec::new(); resultsize + 1 as usize];
        for looseVertex in &self.loose{
            let arraylen = self.adjcency_list[looseVertex.1].len();
            if(arraylen > 0){
                positionVector[self.CalculateMedian(looseVertex)].push((looseVertex.0,looseVertex.1))
            }
            else {
                positionVector[resultsize].push((looseVertex.0,looseVertex.1))
            }
        }
        return positionVector;
    }


    pub fn switch_with_neighbour_if_beneficial(& mut self, pos:usize,direction : Direction){
        let (mut pos_u, mut pos_v) = (0,1);
        if direction == Direction::Left {
            (pos_u,pos_v) = (pos - 1, pos);
        }else {
            (pos_u,pos_v) = (pos , pos + 1);
        }

        let (stay,switch) = self.calc_local_cross_count_touple_between(pos_u , pos_v);

        if stay > switch{
            self.switch_loose_by_position(pos_u, pos_v);
        }
    }

    //Caluclate the crossings between the two specified vertices
    //returns touple where first value is the crossing in the given position and the second is the crossing if switched
    pub fn calc_local_cross_count_touple_between(&self,pos_u:usize,pos_v:usize) -> (i32,i32){
        
        //get reference to interconnection arrray
        let V_u = &self.adjcency_list[self.loose[pos_u].1];
        let V_v = &self.adjcency_list[self.loose[pos_v].1];

        if V_v.len() == 0 || V_u.len() == 0{
            return (0,0);
        }

         //init Pointer in interconnection array
         let mut p_v : usize = 0;
         let mut p_u : usize = 0;

        //init Tally for total number of crossings
        let mut C_u : i32 = 0; //C_u = C_uv
        let mut C_v : i32 = 0; //C_v = C_vu

         if(V_u[p_u] > V_v[p_v]){
             C_u = 1;
             while V_v.len() > p_v + 1 && V_u[p_u] > V_v[p_v + 1]{
                 C_u += 1;
                 p_v += 1;
             }
         } else if (V_u[p_u] < V_v[p_v] ) {
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

                C_v += (p_u + 1) as i32;
                
            }
            else if V_v.len() <= (p_v + 1){
                //Move U
                p_u += 1;

                C_u += (p_v + 1) as i32;
                
            } else if V_u[p_u + 1] < V_v[p_v + 1] {
                //Move U
                p_u += 1;
  
                C_u += (p_v + 1) as i32;
     
            } else if V_u[p_u + 1] > V_v[p_v + 1] {
                //Move V
                p_v += 1;

                C_v += (p_u + 1) as i32;

            } else if V_u[p_u + 1] == V_v[p_v + 1]{
                //Move Both
                C_u += (p_v + 1) as i32;
                C_v += (p_u + 1) as i32;
                p_v += 1;
                p_u += 1;
            } else {
                println!("FUCK something happened");
            }
        }

    }





    


    // pub fn switch_beneficial(pos_a:usize,pos_b:usize) -> bool{
        
    // }

    pub fn CalculateMedian(&self, loose_vertex:&(i32,usize)) -> usize{
        let arraylen = self.adjcency_list[loose_vertex.1].len();
        let median = (arraylen-1) / 2;
        return self.adjcency_list[loose_vertex.1][median]
    }

    pub fn CalculateMean(&self, loose_vertex:&(i32,usize)) -> usize{
        let neighbours = &self.adjcency_list[loose_vertex.1];
        let mean = neighbours.into_iter().sum::<usize>()  / neighbours.len() ;
        return mean
    }

    pub fn CalculateMeanPosition(&self) -> Vec<Vec<(i32,usize)>>{
        let resultsize = self.fixed.len();
        let mut position_vector: Vec<Vec<(i32,usize)>> = vec![Vec::new(); resultsize + 1 as usize];
        for looseVertex in &self.loose{
            let arraylen = self.adjcency_list[looseVertex.1].len();
            if arraylen > 0 {
                position_vector[self.CalculateMean(looseVertex)].push((looseVertex.0,looseVertex.1));
            }
            else {
                position_vector[resultsize].push((looseVertex.0,looseVertex.1))
            }
        }
        return position_vector;
    }

    pub fn PerformMedianRearrangement(&mut self){
        let pos = self.CalculateMedianPosition();
        let newloose = self.collapseLooseEdgeList(pos);
        self.loose = newloose;

    }

    pub fn PerformMeanRearrangement(&mut self){
        let pos = self.CalculateMeanPosition();
        let newloose = self.collapseLooseEdgeList(pos);
        self.loose = newloose;

    }

    pub fn MedianHeuristic(&self) -> Vec<Vec<i32>>{
        let resultsize = self.fixed.len();
        let mut positionVector: Vec<Vec<i32>> = vec![Vec::new(); resultsize + 1 as usize];
        for looseVertex in &self.loose{
            let arraylen = self.adjcency_list[looseVertex.1].len();
            if(arraylen > 0){
                let median = (arraylen - 1) / 2;
                positionVector[self.CalculateMedian(looseVertex)].push(looseVertex.0)
            }
            else {
                positionVector[resultsize].push(looseVertex.0)
            }
        }
        return positionVector;
    }

    pub fn MeanHeuristic(&self) -> Vec<Vec<i32>>{
        let resultsize = self.fixed.len();
        let mut positionVector: Vec<Vec<i32>> = vec![Vec::new(); resultsize + 1 as usize];
        for looseVertex in &self.loose{
            let arraylen = self.adjcency_list[looseVertex.1].len();
            if(arraylen > 0){
                positionVector[self.CalculateMean(looseVertex)].push(looseVertex.0);
            }
            else {
                positionVector[resultsize].push(looseVertex.0)
            }
        }
        return positionVector;
    }

    pub fn median_random_greedy_switch_heuristic(&mut self, steps: i32) -> Vec<i32>{

        self.PerformMedianRearrangement();
         
        let looseSize = self.loose.len();

        let mut rng = rand::thread_rng();

        for i in 0..steps {
            let pos = rng.gen_range(0..looseSize - 2);
            self.switch_with_neighbour_if_beneficial(pos, Direction::Right);
        }

        return self.get_loose_order();

        
    }

    pub fn mean_random_greedy_switch_heuristic(&mut self, steps: i32) -> Vec<i32>{

        self.PerformMeanRearrangement();
         
        let looseSize = self.loose.len();

        let mut rng = rand::thread_rng();

        for i in 0..steps {
            let pos = rng.gen_range(0..looseSize - 2);
            self.switch_with_neighbour_if_beneficial(pos, Direction::Right);
        }

        return self.get_loose_order();
    }

    

   pub fn collapseLooseEdgeList(&self,posVec : Vec<Vec<(i32,usize)>>) -> Vec<(i32,usize)>{
        let mut result : Vec<(i32,usize)> = Vec::new();
        for posset in  posVec{
            if posset.len() > 0 {
                for pos in posset{
                    result.push(pos);
                }
            }
        }
        return result;
    }

    pub fn collapse(&self,posVec : &Vec<Vec<i32>>) -> Vec<i32>{
        let mut result : Vec<i32> = Vec::new();
        for posset in  posVec{
            if posset.len() > 0 {
                for pos in posset{
                    result.push(pos.clone());
                }
            }
        }
        return result;
    }


    
    pub fn compress(&mut self) -> HashMap<i32,Vec<i32>>{
        let mut hashmap : HashMap<Vec<usize>,i32> = HashMap::new();
        let mut result:HashMap<i32,Vec<i32>> = HashMap::new(); 
        let mut new_loose:Vec<(i32,usize)> = Vec::new(); //(edgeName,indexOfFexedSet)
        let mut new_adjcency_list: Vec<Vec<usize>> = Vec::new();
        
        let mut index = 0;
    
        for i in &self.loose{
            if hashmap.contains_key(&self.adjcency_list[i.1]) {
                result.get_mut(hashmap.get(&self.adjcency_list[i.1]).unwrap()).unwrap().push(i.0);
                

            }
            else {
                hashmap.insert(self.adjcency_list[i.1].clone(),i.0);
                new_loose.push((i.0,index));
                new_adjcency_list.push(self.adjcency_list[i.1].clone());
                result.insert(i.0,Vec::new());
                
            }
            
        }
        self.loose = new_loose;
        self.adjcency_list = new_adjcency_list;

        return result;

    }

    pub fn decompress(&mut self ,compressmap: &HashMap<i32,Vec<i32>>){
        let mut new_loose:Vec<(i32,usize)> = Vec::new(); //(edgeName,indexOfFexedSet)

        let mut index = 0;
        for i in &self.loose {

            new_loose.push((i.0,i.1));
            index = index +1;

            if compressmap.contains_key(&i.0){
                for insertVertex in compressmap.get(&i.0).unwrap(){
                    new_loose.push((*insertVertex,index));
                    self.adjcency_list.push(self.adjcency_list[i.1].clone());
                    index = index +1;
                }
            }
            
        }

        self.loose = new_loose;
        
    }

    


}

impl InterconnectionMatrix for SmartCOOInterconnectionMatrix{
    fn parse(graph:&GraphInput) -> Self where Self: Sized {
        let fixedset = graph.fixed_vertices.clone();

        let mut looseTemp: Vec<(i32,usize)> = Vec::new();
        let mut adj_list = Vec::new();

        let mut index:usize = 0;
        for looseVertex in graph.loose_vertices.clone(){
            adj_list.push(Vec::new());
            looseTemp.push((looseVertex,index));
            index = index +1;
        }


        //these are sorted by fixed and then loose vertices so we have to sort them correctly.
        //we convert them because at this stage those coordinates are the same as in the respective sets
        for edge in graph.edge_set.clone(){
            let (fixedIndex,looseIndex) = default_edgelabel_to_interconnection_coordinates(edge.0,edge.1,graph.number_of_fixed);
            adj_list[looseIndex].push(fixedIndex);
        }

        let result = SmartCOOInterconnectionMatrix{ 
            fixed:  fixedset, loose: looseTemp, adjcency_list: adj_list };
       
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

            if(found_a && found_b){
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

    fn get_value_at(&self,fixedIndex:usize,looseIndex:usize)-> bool {
        // let vecindex = self.loose[looseIndex].1;

        // //you can do a binary search instead of forloop
        // for point in &self.adjcency_list[vecindex]{
        //     if(point == fixedIndex){
        //         return true;
        //     }
        //     if point > &fixedIndex {
        //         break;
        //     }
        // }
        return false;
    }

    fn get_edge_set(&self) -> Vec<(i32,i32)> {
        let mut result : Vec<(i32,i32)> = Vec::new();

        // for looseVertex in &self.loose{
        //     for fixedVertexIndex in &self.adjcency_list[looseVertex.1]{
        //         result.push((*self.fixed[fixedVertexIndex],looseVertex.0));
        //     }
        // }
        // result.sort_by(|(x0,x1),(y0,y1)| x0.cmp(y0));
        return result
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}

impl InterconnectionMatrix for SimpleInterconnectionMatrix{
    fn parse(graph:&GraphInput) -> Self {
        let looseTemp= graph.loose_vertices.clone();
        let fixedTemp= graph.fixed_vertices.clone();

        let mut array = vec![vec![false; looseTemp.len()]; fixedTemp.len()];

        for edge in &graph.edge_set {
            let coord = default_edgelabel_to_interconnection_coordinates(edge.0,edge.1,graph.number_of_fixed);
            array[coord.0 ][coord.1 ]=true;
        }

        let result = SimpleInterconnectionMatrix{ 
             fixed:  fixedTemp, loose: looseTemp, matrix: array };
        
        return result;

    }

    fn get_fixed_order(&self) -> &Vec<i32> {
        return &self.fixed;
    }

    fn get_loose_order(&self) -> Vec<i32> {
        return self.loose.clone();
    }


    fn switch_loose_by_vertexlabel(&mut self,a:i32,b:i32) {
        let mut aIndex:usize = 0;
        let mut bIndex:usize = 0;

        let mut aindexFound = false;
        let mut bindexFound = false;


        for i  in 0..self.loose.len() {
            if (aindexFound  && bindexFound){
                self.switch_loose_by_position(aIndex, bIndex);
                break;
            }
            if aindexFound ==false && self.loose[i] == a {
                aIndex = i;
                aindexFound = true;
            }
            if bindexFound ==false && self.loose[i] == b {
                bIndex = i;
                bindexFound = true;
            }
        }
    }

    fn switch_loose_by_position(&mut self,a:usize,b:usize) {
        //switch loose index
        let temp = self.loose[a];
        self.loose[a] = self.loose[b];
        self.loose[b] = temp;

        //Switch loose array
        for i in 0..self.fixed.len(){
            let temp = self.matrix[i][a];
            self.matrix[i][a] = self.matrix[i][b];
            self.matrix[i][b] = temp;
        }
    }
    
    fn to_string(&self) -> String {
        let mut stringresult = String::new();
        stringresult.push_str(&format!("    {:?}\r", self.loose));  
        for i  in 0..self.fixed.len() {
            stringresult.push_str( &format!("{:-<1$}",self.fixed[i],5));
            let z : Vec<u32> =  self.matrix[i].iter().map(|&e| e as u32).collect();
            stringresult.push_str( &format!("{:?}",z));
            stringresult.push_str("\r");
        }
        return  stringresult;
    }
    
    fn print(&self) {
        print!("{}", self.to_string());
    }
    
    fn get_value_at(&self,fixedIndex:usize,looseIndex:usize)-> bool {
        self.matrix[fixedIndex][looseIndex]
    }

    fn get_edge_set(&self) -> Vec<(i32,i32)> {
        let mut result = Vec::new();
        for i in 0..self.fixed.len(){
            for j in 0..self.loose.len(){
                if (self.matrix[i][j] == true){
                    result.push((self.fixed[i],self.loose[j]));
                }

            }
            
        }

        return result;
    }
}