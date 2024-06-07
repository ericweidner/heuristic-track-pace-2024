use std::path::Path;

use crate::utils;


pub(crate) struct GraphInput{
    pub number_of_fixed:i32,
    pub number_of_loose:i32,
    pub number_of_edges:i32,
    pub fixed_vertices:Vec<i32>,
    pub name: String,
    pub loose_vertices:Vec<i32>,
    pub edge_set:Vec<(i32,i32)>,
    pub has_cutwith_parameter:bool,
    pub cutwith_parameter:Option<CutwidthParameterisation>
}

pub(crate) struct CutwidthParameterisation{
    pub cutwidth:i32,
    pub optimal_order:Vec<i32>
}

// pub trait Graph{
//     fn get_neighbours_of_vertex(&self,Vertex:i32)->Vec<i32>;
//     fn get_fixed(&self) -> Vec<i32>;
//     fn get_loose(&self) -> Vec<i32>;
// }






 impl GraphInput {
    
    //Just retuns the neighbours without optimisation
    pub(crate) fn get_neighbourse_bf(&self,vertex:i32)->Vec<i32>{
        let mut result : Vec<i32> = Vec::new();
        for (e1,e2) in self.edge_set.clone() {
            if e1 == vertex 
            {
                result.push(e2);
            }else if e2 == vertex{

                result.push(e1);
            }
 
        }

        return result;
    }

    pub(crate) fn parse_from_path(path:&str,with_cutwith_parameter:bool,verbose:bool) -> GraphInput{
        let lines:Vec<String> = utils::lines_from_file(path);
        return GraphInput::parse_from_lines(&lines,with_cutwith_parameter,verbose);
    }

    pub(crate) fn parse_from_lines(lines:&Vec<String>,with_cutwith_parameter:bool,verbose:bool) -> GraphInput{
        //Data
        if verbose {
            eprintln!("Parsing graph....");
        }
        let mut number_of_fixed_vertices = 0;
        let mut number_of_loose_vertices = 0;
        let mut count_of_edges :i32  = 0;
        let mut edges : Vec<(i32,i32)> = Vec::new();
        let mut cutwidthvalue = 0;
        let mut problem_descriptor: String = "".to_string();
        let mut cutwith_parameterisation: Option<CutwidthParameterisation> = None;

        let mut needsort = false;
        let mut lastedge = 0;
        
        let mut cutwidthOrdering:Vec<i32>  = Vec::new();
        
        for lne  in lines{
            let ln = lne.clone();
            if ln.starts_with('c'){ //Comment -> Ignore
                if verbose {
                    eprintln!("Comment is :{}",&ln[1..]);
                }
                continue;
            } else if ln.starts_with('p') { //Problem Description -> ReadData
                let p_line_data : Vec<String> = ln.split(' ').map(|x| x.to_string()).collect();
                problem_descriptor = p_line_data[1].clone();
                number_of_fixed_vertices = p_line_data[2].parse::<i32>().unwrap();
                number_of_loose_vertices = p_line_data[3].parse::<i32>().unwrap();
                count_of_edges = p_line_data[4].parse::<i32>().unwrap();
                if with_cutwith_parameter {
                    cutwidthvalue = p_line_data[5].parse::<i32>().unwrap();
                }
                

            }else if with_cutwith_parameter && (cutwidthOrdering.len() as i32) < (number_of_fixed_vertices + number_of_loose_vertices){
                let cutw_element_temp : i32 = ln.parse::<i32>().unwrap();
                cutwidthOrdering.push(cutw_element_temp)
            } else { //Edge
  

                let mut temp : Vec<String> = ln.split(' ').map(|x| x.to_string()).collect();
                let mut edge_left = &temp[0].parse::<i32>().unwrap();
                let mut edge_right = &temp[1].parse::<i32>().unwrap();
                if(!needsort){
                    if(lastedge <= *edge_left){
                        lastedge = edge_left.clone();
                    }
                    else {
                        needsort = true;
                    }
                }
                edges.push((*edge_left,*edge_right));
            }
        }

        if(needsort){
            edges.sort_by_key(|k| k.0);
        }

        if verbose {
            eprintln!("Graph has {} Fixed and {} Loose Vertices",&number_of_fixed_vertices,&number_of_loose_vertices);
            eprintln!("Problem Descriptor is :{}",problem_descriptor);
            if with_cutwith_parameter {
                eprintln!("Cutwith {}  " ,cutwidthvalue);
            }

            if(needsort){
                eprint!("Need to sort edges");
            }
        }

        //Assert that parsing was right
        if edges.len() as i32  != count_of_edges{
            panic!("Number of edges wrong")
        }

        //Calculate rust set
        let mut fixed_verts:Vec<i32> = Vec::new();
        for i in 0..number_of_fixed_vertices{
            fixed_verts.push(i+1 );
        }

        let mut loose_verts:Vec<i32> = Vec::new();
        for i in 0..number_of_loose_vertices{
            loose_verts.push(i+1+number_of_fixed_vertices);
        }

        //Set parameterisation trait
        if with_cutwith_parameter {
            cutwith_parameterisation = Some(CutwidthParameterisation{
                cutwidth : cutwidthvalue,
                optimal_order : cutwidthOrdering
            });
        }

        let filename = "live";

        let result:GraphInput = GraphInput{
            number_of_fixed : number_of_fixed_vertices,
            number_of_loose : number_of_loose_vertices,
            number_of_edges : count_of_edges,
            fixed_vertices : fixed_verts,
            loose_vertices : loose_verts,
            edge_set : edges,
            has_cutwith_parameter : with_cutwith_parameter,
            cutwith_parameter:cutwith_parameterisation,
            name : filename.to_string()

        };
        if(verbose){
            eprintln!("Finished!.");
        }
        return result;
    }

    pub(crate) fn parse(path:&str,with_cutwith_parameter:bool,verbose:bool,) -> GraphInput{
        return GraphInput::parse_from_path(path, with_cutwith_parameter, verbose);

    }

    
}





