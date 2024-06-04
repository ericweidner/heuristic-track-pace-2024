use std::{collections::{HashMap, HashSet}, result, f32::INFINITY, path::Component, cmp::{self}, sync::atomic::Ordering};

use rand::Rng;
use crate::{utils, TERMINATION_SIGNAL};

use crate::{problem::GraphInput, utils::Direction, interconnection::SmartCOOInterconnectionMatrix};//, heuristics::Heuristics};

pub(crate) struct PenaltyGraph{
    pub vertices: Vec<PenaltyGraphVertex>,
}

pub enum VertexArcBaseHeuristic {
    Eades,
    EadesForward,
    EadesForward_Weighted,
    EadesBackward,
    EadesBackward_Weighted,
    Baharev
}







pub(crate) struct PenaltyGraphVertex{
    pub Name: i32,
    pub InterconRef : usize,
    pub OutNeighbours : Vec<(usize,u32)>,
    pub InNeighbours : Vec<(usize,u32)>,
    pub isCondensed : bool,
    pub CondensedVertices : Option<PenaltyGraph>
}

impl PenaltyGraphVertex{
    pub fn GetOutWeightSum(&self) -> u32{
        let mut count = 0;
        for neigh in &self.OutNeighbours{
            count += neigh.1;
        }
        return count;
    }

    pub fn GetInWeightSum(&self) -> u32{
        let mut count = 0;
        for neigh in &self.InNeighbours{
            count += neigh.1;
        }
        return count;
    }
}

impl PenaltyGraph{
    pub fn parse(input : &SmartCOOInterconnectionMatrix) -> Option<PenaltyGraph>{
        let mut vert = Vec::new();
        let looslen = input.loose.len();
        let mut index = 0;
        for v in &input.loose {
            let vertex = PenaltyGraphVertex{
                Name : v.0, OutNeighbours: Vec::new(),InNeighbours:Vec::new(),isCondensed:false,CondensedVertices:None,InterconRef: index

            };
            vert.push(vertex);
            index += 1;
        }

        for i in 0..looslen - 1 {
            for j in i..looslen {
                let crossings = input.calc_local_cross_count_touple_between_positions(i, j);
                if(crossings.0 > crossings.1){
                    vert[i].InNeighbours.push((j,crossings.0-crossings.1));
                    vert[j].OutNeighbours.push((i,crossings.0-crossings.1));
                }else if (crossings.0 < crossings.1) {
                    vert[j].InNeighbours.push((i,crossings.1 - crossings.0));
                    vert[i].OutNeighbours.push((j,crossings.1 - crossings.0));
                }
                if TERMINATION_SIGNAL.load(Ordering::Relaxed) {
                    return None;
                }
            }

            
        }

        let result = PenaltyGraph{ 
            vertices:  vert };
       
       return Some(result);

    }

    pub fn generateStronglyConnectedComponents(&self) -> Vec<Vec<usize>>{
        let mut vertices:HashSet<usize> = HashSet::new();
        let mut VisitingList:Vec<usize> = Vec::new();
        let mut tempresult: HashMap<usize,usize> = HashMap::new();

        for vertex in 0..self.vertices.len() {
            self.Visit(&mut VisitingList, &mut vertices, vertex);
        }
        
        while !VisitingList.is_empty() {
            let u = VisitingList.pop().unwrap();
            self.Assign(&mut tempresult, u, u);
        }

        let mut result_hashmap : HashMap<usize,Vec<usize>> = HashMap::new();

        for a in tempresult  {
            if !result_hashmap.contains_key(&a.1) {
                let mut vec :Vec<usize> =  Vec::new();
                vec.push(a.0);
                result_hashmap.insert(a.1, vec);
            }else {
                result_hashmap.get_mut(&a.1).unwrap().push(a.0);
            }
        }
        



        return result_hashmap.into_iter()
                            .map(|(_id, x)| x)
                            .collect();
    }

    pub fn CondenseGraph(&self) -> PenaltyGraph{
        let scc = self.generateStronglyConnectedComponents();
        let mut positionMap:HashMap<usize,usize> = HashMap::new();
        let mut newVertices:Vec<PenaltyGraphVertex> = Vec::new();

        
        //Add all new vertivces and fill translationMap
        for ComponentPos in 0..scc.len() {
            let component = &scc[ComponentPos];
            if component.len() == 1{
                //just add as same vertex
                let vertex = PenaltyGraphVertex{
                    Name : self.vertices[component[0]].Name, InterconRef:self.vertices[component[0]].InterconRef, OutNeighbours: Vec::new(),InNeighbours:Vec::new(),isCondensed:false,CondensedVertices:None
    
                };

                newVertices.push(vertex);
                positionMap.insert(component[0], ComponentPos);
            }
            else {
                let mut tempPositionMap :HashMap<usize,usize> = HashMap::new();
                let mut tempVec : Vec<PenaltyGraphVertex> = Vec::new();
                for subcomponentindex in 0..component.len(){
                    tempPositionMap.insert(component[subcomponentindex], subcomponentindex);
                    let vertex = PenaltyGraphVertex{
                        Name : self.vertices[component[subcomponentindex]].Name, InterconRef: self.vertices[component[subcomponentindex]].InterconRef, OutNeighbours: Vec::new(),InNeighbours:Vec::new(),isCondensed:false,CondensedVertices:None
        
                    };
                    positionMap.insert(component[subcomponentindex], ComponentPos);
                    tempVec.push(vertex);
                   
                }

                for subcomponentindex in 0..component.len(){
                    let origVertex = &self.vertices[component[subcomponentindex]];
                    for edge in &origVertex.InNeighbours{
                        if(tempPositionMap.contains_key(&edge.0)){
                            let translated_pos = tempPositionMap.get(&edge.0).unwrap();
                            tempVec[subcomponentindex].InNeighbours.push((*translated_pos,edge.1));
                        }
                    }

                    for edge in &origVertex.OutNeighbours{
                        if(tempPositionMap.contains_key(&edge.0)){
                            let translated_pos = tempPositionMap.get(&edge.0).unwrap();
                            tempVec[subcomponentindex].OutNeighbours.push((*translated_pos,edge.1));
                        }
                    }
                }

                let subPGraph = PenaltyGraph{
                    vertices: tempVec, 
                };

                let vertex = PenaltyGraphVertex{
                    Name : self.vertices[component[0]].Name, InterconRef: self.vertices[component[0]].InterconRef, OutNeighbours: Vec::new(),InNeighbours:Vec::new(),isCondensed:true,CondensedVertices:Some(subPGraph)
    
                };

                newVertices.push(vertex);
                
            }
        }

        //reroute all the edges V2
        

        //reroute all Edges

        for ComponentPos in 0..scc.len() {
            let component = &scc[ComponentPos];
            if component.len() == 1{
                let original_vertex = &self.vertices[component[0]];
                let mut already_inserted_in :HashSet<usize> = HashSet::new();
                for inVert in &original_vertex.InNeighbours {
                    let mapped = positionMap.get(&inVert.0).unwrap().clone();
                    if !already_inserted_in.contains(&mapped){
                        newVertices[ComponentPos].InNeighbours.push((mapped.clone(),inVert.1.clone()));
                        already_inserted_in.insert(mapped);
                    }
                }

                let mut already_inserted_out :HashSet<usize> = HashSet::new();
                for outVert in &original_vertex.OutNeighbours {
                    let mapped = positionMap.get(&outVert.0).unwrap().clone();
                    if !already_inserted_out.contains(&mapped){
                        newVertices[ComponentPos].OutNeighbours.push((mapped.clone(),outVert.1.clone()));
                        already_inserted_out.insert(mapped);
                    }
                }
            }
            else {
                let mut already_inserted_in :HashSet<usize> = HashSet::new();
                let mut already_inserted_out :HashSet<usize> = HashSet::new();
                let mut internalVertices : HashSet<usize> = HashSet::new();

                for condcomp in component{
                    internalVertices.insert(condcomp.clone());
                }

                for condcomp in 0..component.len(){

                    let original_vertex = &self.vertices[component[condcomp]];
                    for inVert in &original_vertex.InNeighbours {
                        if(!internalVertices.contains(&inVert.0)){
                            let mapped = positionMap.get(&inVert.0).unwrap().clone();
    
                            if !already_inserted_in.contains(&mapped){
                                newVertices[ComponentPos].InNeighbours.push((mapped.clone(),inVert.1.clone()));
                                already_inserted_in.insert(mapped);
                            }
                        }
                       

                    }

                    for outVert in &original_vertex.OutNeighbours {
                        if(!internalVertices.contains(&outVert.0)){
                            let mapped = positionMap.get(&outVert.0).unwrap().clone();
                            if !already_inserted_out.contains(&mapped){
                                newVertices[ComponentPos].OutNeighbours.push((mapped.clone(),outVert.1.clone()));
                                already_inserted_out.insert(mapped);
                            }
                        }
                    }
                }
           
                
            }
        }


        return PenaltyGraph{vertices:newVertices };
    }

// L ← Empty list that will contain the sorted elements
// S ← Set of all nodes with no incoming edge

// while S is not empty do
//     remove a node n from S
//     add n to L
//     for each node m with an edge e from n to m do
//         remove edge e from the graph
//         if m has no other incoming edges then
//             insert m into S

// if graph has edges then
//     return error   (graph has at least one cycle)
// else 
//     return L   (a topologically sorted order)
    pub fn SortWithKahnsAlgorithm(&self) -> Vec<usize>{
        let mut result:Vec<usize> = Vec::new();
        let mut removedVerts:HashSet<usize> = HashSet::new();

        //find startNodes
        let mut startNodes : Vec<usize> = Vec::new();
        for i in 0..self.vertices.len() {
            if(self.vertices[i].InNeighbours.len() == 0){
                startNodes.push(i);
                
            }
        }

        while startNodes.len() > 0{
            let tempNode = startNodes.pop().unwrap();
            result.push(tempNode);
            removedVerts.insert(tempNode);

            let tempNodeOutNeigh = &self.vertices[tempNode].OutNeighbours;
            for outneighbour in tempNodeOutNeigh{
                let mut allInEdgesMarked = true;
                for incomming_neighbours in &self.vertices[outneighbour.0].InNeighbours{
                    if !removedVerts.contains(&incomming_neighbours.0) {
                        allInEdgesMarked = false;
                        break;
                    }

                }

                if allInEdgesMarked {
                    startNodes.push(outneighbour.0)
                }
            }

        }




        return result;
    }


   

    pub fn sort_eads_heuristic(& self,heur:VertexArcBaseHeuristic)->Vec<&PenaltyGraphVertex>{
        let mut head:Vec<&PenaltyGraphVertex> = Vec::new();
        let mut tail:Vec<&PenaltyGraphVertex> = Vec::new();

        let mut middle:Vec<&PenaltyGraphVertex> = Vec::new();

        for i in 0.. self.vertices.len() {
            let vertex = &self.vertices[i];
            if(vertex.InNeighbours.len() == 0){
                head.push(vertex);
            }else if vertex.OutNeighbours.len() == 0 {
                tail.push(vertex);
            }else {
                // let mut weight = 0;
                // // for inN in vertex.InNeighbours  {
                // //     weight += inN.1;
                // // }
                // for outN in vertex.OutNeighbours  {
                //     weight += outN.1;
                // }
                middle.push(vertex);
            }
        }


       

        match heur {
            VertexArcBaseHeuristic::Eades => {
                middle.sort_by_key(|v| v.OutNeighbours.len() as i32 - v.InNeighbours.len() as i32);
            }
            VertexArcBaseHeuristic::EadesForward => {
                middle.sort_by_key(|v| v.OutNeighbours.len() as i32);
            }
            VertexArcBaseHeuristic::EadesForward_Weighted => {
                middle.sort_by_key(|v|  v.GetOutWeightSum());
            }
            VertexArcBaseHeuristic::EadesBackward => {
                middle.sort_by_key(|v| - (v.InNeighbours.len() as i32));
            }
            VertexArcBaseHeuristic::EadesBackward_Weighted => {
                middle.sort_by_key(|v|  -(v.GetInWeightSum() as i32));
            }
            VertexArcBaseHeuristic::Baharev => {
                middle.sort_by_key(|v| ((v.OutNeighbours.len() as f32 / v.InNeighbours.len() as f32 ) * 1000 as f32) as i32);
                //middle.reverse();
            },
        }

       middle.reverse();
        
        for m in middle{
            head.push(m);
        }

        head.append(&mut tail);


       
        return head;

    }
    // pub fn CheckIfConedensationIsStartNode(&self,input : &Vec<usize>) -> bool{
    //     let CheckHashset: HashSet<usize> = HashSet::from_iter(input.iter().cloned());

    //     for vertex in input {
    //         for inneigh in &self.vertices[*vertex].InNeighbours{
    //             if !CheckHashset.contains(&inneigh.0) {
    //                 return false;
    //             }
    //         }
    //     }

    //     return true;
        
    // }

    fn Visit(&self, VisitingList:&mut Vec<usize>, vertice_hash_set:&mut HashSet<usize>,visiting_vertex : usize){
        if !vertice_hash_set.contains(&visiting_vertex) {
            vertice_hash_set.insert(visiting_vertex);
            for outNeigh in &self.vertices[visiting_vertex].OutNeighbours{
                self.Visit(VisitingList, vertice_hash_set, outNeigh.0);
            }
            VisitingList.push(visiting_vertex);
        }
    }

    fn Assign(&self,Result: &mut HashMap<usize,usize>,u:usize,root:usize){
        if !Result.contains_key(&u) {
            Result.insert(u, root);
            for neighbour in &self.vertices[u].InNeighbours  {
                self.Assign(Result, neighbour.0, root);
            }
        }

    }
}
