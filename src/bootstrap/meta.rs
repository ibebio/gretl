use std::collections::{HashSet};
use gfa_reader::{GraphWrapper, NCGfa, NCPath};
use crate::bootstrap::helper::random_numbers;

/// Wrapper for combinations
///
/// Creates meta data for the bootstrap
/// One entry consists of: [number of genomes, number of iteration, combination (HashSet)]
pub fn combinations_maker_wrapper(size: &usize, amount: &usize) -> Vec<(usize, usize, HashSet<usize>)> {
    let mut data = vec![];
    for number in 2..size+1 {
        let test_comb = combinations_maker(&size, &number, amount);
        for (run, combination) in test_comb.iter().enumerate(){
            data.push((number, run, combination.clone()))
        }
    }
    data
}


/// Makes multiple random combinations
/// size = total sample size [0,1,2,3,4,5,6,7,8,9]
/// number = size of the sample (3 -> [1,2,3])
/// amount = amount og samples that should be drawn (2 -> [[1,2,3], [4,5,6]])
///
/// TODO
/// Make a check when not more different samples can be drawn (very unlikely)
pub fn combinations_maker(size: &usize, core_number: &usize, amount: &usize) -> Vec<HashSet<usize>>{
    let mut result = Vec::new();
    let mut counter = 0;
    while result.len() != *amount {
        let v = random_numbers(size, core_number);
        let v: HashSet<usize> = v.iter().cloned().collect();
        if ! result.contains(&v){
            result.push(v);
        }
        if counter > 10000{
            return result
        }
        counter += 1;
    }
    return result
}

/// Removes lines (combinations) based on given condition (meta file or core)
pub fn reduce_meta(meta: &mut Vec<(usize, usize, HashSet<usize>)>, line: i32, core: i32){
    if line != -1{
        let value_to_retain = meta[line as usize].clone();
        meta.retain(|x| *x == value_to_retain );
    } else {
        if core != -1 {
            meta.retain(|x| x.0 == core as usize);
        }
    }
}



/// Calculation for one iteration
///
/// Take core and then remove stuff from it
pub fn one_iteration(gw: &GraphWrapper<NCPath>, graph: &NCGfa<()>, combination: &[usize], _metric: &str, information: &Vec<u32>) -> (Vec<usize>, Vec<usize>){

    let info2 = test1(gw, graph, information, combination);
    let max_value = info2.iter().max().unwrap().clone();

    let mut result: Vec<usize> = vec![0; max_value as usize +1];             // NODES
    let mut result2 = vec![0; max_value as usize +1];             // Sequence


    // Add amount and sequence
    for (i, x) in info2.iter().enumerate(){
        result[*x as usize] += 1;
        result2[*x as usize] += graph.nodes[i].seq.len();
    }
    result2.remove(0);
    result.remove(0);


    return (result, result2)

}

/// Reduce vector wrapper
/// - Iterate over the genome with one combination
/// - Check if the genome is in the combination
/// - If not, remove it from the vector
pub fn test1(wrapper: &GraphWrapper<NCPath>, graph: &NCGfa<()>, info: &Vec<u32>, comb: &[usize]) -> Vec<u32>{
    let mut info2 = info.clone();
    for (i, x) in wrapper.genomes.iter().enumerate(){
        if ! comb.contains(&i){
            let a = make_vec(&x.1, graph.nodes.len());
            remove_from_vec(& mut info2, &a);
        }
    }
    return info2
}

pub fn make_vec(t: &Vec<&NCPath>, length: usize) -> Vec<u32>{
    let mut vec1 = vec![0; length];
    for a in t.iter(){
        a.nodes.iter().for_each(|x| vec1[*x as usize - 1] = 1);
    }
    vec1
}


/// Remove one vector from the other
/// Vectors must be of same size
/// Inplace operation
pub fn remove_from_vec(origin: &mut Vec<u32>, sub: &Vec<u32>){
    if origin.len() != sub.len(){
        panic!("Vectors must be of same size")
    }
    origin.iter_mut().zip(sub.iter()).for_each(|(o, s)| *o = *o - *s);
}

