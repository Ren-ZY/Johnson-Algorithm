use std::collections::HashSet;

pub struct SCCResult{
    nodeIDsOfSCC: HashSet<usize>,
    adjList: Vec<Vec<i32>>,
    lowestNodeId: usize
}

impl SCCResult{
    pub fn New(adjL: &Vec<Vec<i32>>, lId: usize) -> Self{
        let mut s = HashSet::new();
        if adjL.len() > 0{
            for i in lId..adjL.len(){
                if adjL[i as usize].len() > 0 {
                    s.insert(i);
                }
            }
        }
        
        Self{
            adjList: adjL.clone(),
            lowestNodeId: lId,
            nodeIDsOfSCC: s
        }
    }

    pub fn getAdjList(&self) -> Vec<Vec<i32>>{
        self.adjList.clone()
    }

    pub fn getLowestNodeId(&self) -> usize{
        self.lowestNodeId
    }
}

